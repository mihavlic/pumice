pub mod deep_copy;
pub mod dumb_hash;
pub mod symbols;
pub mod wrappers;

use crate::codegen::dumb_hash::write_dumb_hash;
use crate::codegen::symbols::fmt_command_preamble;
use crate::codegen_support::format_utils::SectionWriter;
use crate::codegen_support::rename::apply_renames;
use crate::codegen_support::type_analysis::get_underlying_symbol;
use crate::codegen_support::type_query::DeriveData;
use crate::codegen_support::{
    get_command_kind, get_enum_added_variants, merge_bitfield_members, AddedVariants, CommandKind,
};
use crate::context::{Context, SectionFunctions};
use crate::context::{Section, SectionKind};
use crate::{cat, cstring, doc_boilerplate, fun, import, import_str, string};

use codewrite::{Cond, Iter, Separated};
use codewrite_macro::code;
use deep_copy::write_deep_copy;
use generator_lib::ConstantValue;
use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::Type,
    Extension, ExtensionKind, Symbol, SymbolBody,
};
use slice_group_by::GroupBy;
use std::fmt::Write;
use std::ops::Not;
use std::rc::Rc;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use self::symbols::write_symbol;

pub fn write_bindings(mut ctx: Context, out: &Path) {
    // remove all files in pumice/src/* which do not have a CODEGEN START
    // which is assumed to mean that they are wholly generated,
    // we will be writing them from scratch anyway, so it's a good idea to clean the directory first
    // to prevent us from accumulating unused files in case their name changes
    crate::fs_utils::for_all_files(&out.join("src"), |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        if contents.find("// CODEGEN START").is_none() {
            std::fs::remove_file(path).unwrap();
        }
    });

    // manually input sections and their contained symbols for the template handwritten files
    macro_rules! manual_symbols {
        ($($path:literal: $symbols:expr),+) => {
            &[
                $(
                    ($path, &$symbols))
                ,+
            ]
        };
    }

    let register_symbols: &[(&str, &[&str])] = manual_symbols!(
        "crate": [
            "EntryWrapper",
            "InstanceWrapper",
            "DeviceWrapper",
            "VulkanResult",
            // these macros are in ::crate because they are [macro_export]
            // but they are actually defined in util/impl_macros
            "enum_impl",
            "bitflags_impl",
            "non_dispatchable_handle",
            "dispatchable_handle",
        ],
        "util": [
            "cstr",
            "vkcall",
            "ObjectHandle",
            "pnext_visit",
            "ApiLoadConfig",
            "ApiLoadConfigErr",
        ],
        "surface": [
            "create_surface",
            "enumerate_required_extensions",
        ],
        "loader": [
            "FunctionLoad",
            "EntryLoad",
            "InstanceLoad",
            "DeviceLoad",
            "EntryLoader",
            "LinkedEntryLoader",
            "LoadedEntryLoader",
            "InstanceLoader",
            "DeviceLoader",
        ],
        "loader/tables": [
            "GLOBAL_ENTRY_TABLE",
            "GLOBAL_INSTANCE_TABLE",
            "GLOBAL_DEVICE_TABLE",
            "EntryTable",
            "InstanceTable",
            "DeviceTable",
        ],
        "extensions/metadata": [
            "ExtensionMetadata",
            "get_metadata",
            "EXTENSION_METADATA",
        ]
    );

    for &(path, items) in register_symbols {
        let divide = path
            .char_indices()
            .rev()
            .find(|&(_, c)| c == '/')
            .map(|(i, _)| i + 1)
            .unwrap_or(0);
        let (path, name) = path.split_at(divide);
        let ident = ctx.strings.pumice..name.intern(&ctx);
        ctx.register_section(Section {
            ident: ident.clone(),
            kind: SectionKind::Path(path.to_owned()),
        });
        ctx.section_add_symbols_str(ident, items.iter().cloned());
    }

    {
        // since rust can't express C bitfields we emulate the behaviour by just merging such struct
        // fields with the same algorithm and leaving to the programmer to deal with packing and
        // upacking the values
        // any of the other passes could be doing something with struct fields so we do this at the earliest time possible
        for &(index, _) in &ctx.symbols {
            let Symbol(_, body) = &ctx.reg.symbols[index];
            if let SymbolBody::Struct {
                union: false,
                members,
            } = body
            {
                let merged = merge_bitfield_members(&members, &ctx);

                // FIXME do this more elegantly, borrow checker woes
                let Symbol(_, body) = &mut ctx.reg.symbols[index];
                if let SymbolBody::Struct { members, .. } = body {
                    *members = merged;
                } else {
                    unreachable!()
                }
            }
        }
    }

    // we move ctx into an rc so that we don't have to keep spamming lifetimes everywhere, particularly SectionWriter
    let ctx = Rc::new(ctx);

    let mut added_variants = get_enum_added_variants(&ctx);
    {
        // QUIRK structures added by extensions may use the non-extension version of their StructureType variant,
        // there is no pretty way to solve this so we just enable all of its variants
        added_variants
            .get_mut(&"VkStructureType".intern(&ctx))
            .unwrap()
            .iter_mut()
            .for_each(|a| a.applicable = true);
    }

    let flags_to_flag_bits = ctx
        .flag_bits_to_flags
        .iter()
        .map(|(&k, &(v, _))| (v, k))
        .collect();

    apply_renames(&added_variants, &ctx);

    let (features, extensions) = {
        let mut features = ctx
            .reg
            .features
            .iter()
            .filter(|f| ctx.conf.is_feature_used(f.name))
            .map(|f| f.name)
            .collect::<Vec<_>>();

        features.sort_by(|a, b| a.resolve().cmp(b.resolve()));

        let mut extensions = ctx
            .reg
            .extensions
            .iter()
            .filter(|e| ctx.conf.is_extension_used(e.name))
            // aditional extension may end up being disabled by the workarounds
            .filter(|e| e.kind != ExtensionKind::Disabled)
            .collect::<Vec<_>>();

        extensions.sort_by(|a, b| a.name.resolve().cmp(b.name.resolve()));

        (features, extensions)
    };

    write_wrappers(&features, &extensions, out, &ctx);

    write_extension_metadata(&extensions, out, &ctx);

    write_vk_module(&features, &extensions, out, &ctx);

    write_access_flags_util(out, &ctx);
    pipeline_stage_flags_util(out, &ctx);
    write_format_util(out, &ctx);

    // sort the symbols by their owning section (its index)
    let sorted_symbols = {
        let mut clone = ctx.symbols.clone();
        clone.sort_by_key(|i| i.1);
        clone
    };

    write_tables(&sorted_symbols, out, &ctx);

    write_pnext_visit(&sorted_symbols, out, &ctx);

    let mut derives = DeriveData::new(&ctx);

    write_deep_copy(&mut derives, out, &ctx);
    write_dumb_hash(&mut derives, out, &ctx);

    write_sections(
        &sorted_symbols,
        &added_variants,
        &flags_to_flag_bits,
        &mut derives,
        out,
        &ctx,
    );
}

fn write_sections(
    sorted_symbols: &[(usize, u32)],
    added_variants: &HashMap<UniqueStr, Vec<AddedVariants>>,
    flags_to_flag_bits: &HashMap<UniqueStr, UniqueStr>,
    derives: &mut DeriveData,
    out: &Path,
    ctx: &Rc<Context>,
) {
    let mut path = PathBuf::new();
    for symbols in sorted_symbols.binary_group_by_key(|&(_, section_index)| section_index) {
        let section_index = symbols[0].1;
        let section = &ctx.sections[section_index as usize];

        let mut writer = {
            // build the section path, for example:
            // {out}/src/extensions/amd_buffer_marker.rs
            path.clear();
            path.extend(out);
            path.push("src");
            path.extend(section.path());
            path.push(section.name().resolve());
            path.set_extension("rs");

            SectionWriter::new(section.ident.clone(), &path, ctx)
        };

        for &(symbol, _) in symbols {
            let Symbol(name, body) = &ctx.reg.symbols[symbol];
            write_symbol(
                &mut writer,
                *name,
                body,
                derives,
                &added_variants,
                &flags_to_flag_bits,
                &ctx,
            );
        }
    }
}

fn write_tables(sorted_symbols: &[(usize, u32)], out: &Path, ctx: &Rc<Context>) {
    let mut tables = SectionWriter::new(
        ctx.create_section("tables"),
        out.join("src/loader/tables.rs"),
        &ctx,
    );
    // (index of owning section, name of function, actual name of function)
    let mut entry = Vec::new();
    let mut instance = Vec::new();
    let mut device = Vec::new();
    for &(index, _) in sorted_symbols {
        let &Symbol(name, ref body) = &ctx.reg.symbols[index];
        let section_idx = ctx.symbol_get_section_idx(name).unwrap();
        if let SymbolBody::Command { params, .. } = body {
            let what = (section_idx, name, body);
            match get_command_kind(&params, &ctx) {
                CommandKind::Entry => entry.push(what),
                CommandKind::Instance => instance.push(what),
                CommandKind::Device => device.push(what),
            }
        } else if let &SymbolBody::Alias(of) = body {
            let (_, body) = get_underlying_symbol(of, &ctx);
            if let SymbolBody::Command { params, .. } = body {
                let what = (section_idx, name, body);
                match get_command_kind(&params, &ctx) {
                    CommandKind::Entry => entry.push(what),
                    CommandKind::Instance => instance.push(what),
                    CommandKind::Device => device.push(what),
                }
            }
        }
    }
    for vec in [&mut entry, &mut instance, &mut device] {
        vec.sort_by_key(|a| a.0)
    }
    let variations = [
        (&entry, "Entry", "ENTRY"),
        (&instance, "Instance", "INSTANCE"),
        (&device, "Device", "DEVICE"),
    ];
    for &(_, name, caps_name) in &variations {
        let table_type = cat!(name, "Table");
        let table_const = cat!("GLOBAL_", caps_name, "_TABLE");

        code!(
            tables,
            #[cfg(feature = "global")]
            pub static mut $table_const: $table_type = $table_type::new_empty();
        );
    }
    for &(commands, name, _) in &variations {
        let table_name = cat!(name, "Table");
        let loader_trait = cat!(name, "Load");

        let fields = fun!(|w| {
            for &(_, name, body) in commands {
                if let SymbolBody::Command {
                    success_codes: _,
                    error_codes: _,
                    return_type,
                    params,
                } = body
                {
                    let preamble = fun!(|w| {
                        fmt_command_preamble(
                            w,
                            "",
                            params.iter().map(|p| p.name),
                            params.iter().map(|p| &*p.ty),
                            true,
                            "",
                            return_type,
                            &ctx,
                        )
                    });

                    code!(
                        w,
                        pub $name: Option<unsafe extern "system" $preamble>,
                    )
                } else {
                    unreachable!()
                }
            }
        });

        let commands_funs = fun!(|w| {
            for &(_, name, body) in commands {
                if let SymbolBody::Command {
                    success_codes: _,
                    error_codes: _,
                    return_type,
                    params,
                } = body
                {
                    let preamble = fun!(|w| {
                        fmt_command_preamble(
                            w,
                            name.resolve(),
                            params.iter().map(|p| p.name),
                            params.iter().map(|p| &*p.ty),
                            true,
                            "&self, ",
                            return_type,
                            &ctx,
                        )
                    });
                    let params = Separated::display(params.iter().map(|p| p.name), ",");

                    code!(
                        w,
                        #[track_caller]
                        $doc_boilerplate!(name)
                        pub unsafe $preamble {
                            (self.$name.unwrap())(
                                $params
                            )
                        }
                    );
                } else {
                    unreachable!()
                }
            }
        });

        let load_entry_functions = fun!(|w| {
            let fields = fun!(|w| for (_, name, _) in commands {
                code!(
                    w,
                    ($name, $string!(name.resolve_original()))
                );
            });

            code!(
                w,
                pub fn load(&mut self, loader: &impl $loader_trait) {
                    unsafe {
                        load_fns!{
                            self, loader, $fields
                        }
                    }
                }
            );
        });

        let load_other_functions = fun!(|w| {
            let fields = fun!(move |w| {
                let groups = commands.binary_group_by_key(|(section_idx, ..)| *section_idx);
                for group in groups {
                    let section = &ctx.sections[group[0].0 as usize];

                    fn write_fns(w: &mut SectionWriter, symbols: &[(u32, UniqueStr, &SymbolBody)]) {
                        for (_, name, _) in symbols {
                            code!(
                                w,
                                ($name, $string!(name.resolve_original()))
                            );
                        }
                    }

                    let fns = fun!(move |w| {
                        write_fns(w, group);
                    });

                    match section.kind {
                        SectionKind::Feature(i) => {
                            let feature = &ctx.reg.features[i as usize];
                            // https://registry.khronos.org/vulkan/specs/1.3/html/chap31.html$extendingvulkan-coreversions-versionnumbers
                            code!(
                                w,
                                if conf.api_version_enabled(crate::vk10::make_api_version(0, $(feature.major), $(feature.minor), 0)) {
                                    load_fns! {
                                        self, loader,
                                        $fns
                                    }
                                }
                            );
                        }
                        SectionKind::Extension(_) => {
                            let name = section.name();
                            let name = name.resolve_original();
                            code!(
                                w,
                                if conf.extension_enabled(cstr!($string!(name))) {
                                    load_fns! {
                                        self, loader,
                                        $fns
                                    }
                                }
                            );
                        }
                        SectionKind::Path(_) => {
                            unreachable!("Custom-pathed sections cannot contain loadable pointers!")
                        }
                    }
                }
            });

            code!(
                w,
                pub fn load(&mut self, loader: &impl $loader_trait, conf: &ApiLoadConfig) {
                    unsafe {
                        $fields
                    }
                }
            );
        });

        let do_entry = name == "Entry";
        let load_entry_functions = Cond::new(do_entry, load_entry_functions);
        let load_other_functions = Cond::new(!do_entry, load_other_functions);

        // it is valid to use zeroed Option to create a None
        // see https://doc.rust-lang.org/std/option/index.html$representation
        code!(
            tables,
            pub struct $table_name {
                $fields
            }
            impl $table_name {
                pub const fn new_empty() -> Self {
                    unsafe {
                        // https://github.com/maxbla/const-zero$how-does-it-work
                        // for some reason calling const functions in place of generics is invalid
                        const SIZE: usize = std::mem::size_of::<$table_name>();
                        ConstZeroedHack::<$table_name, SIZE>::zero()
                    }
                }
                $load_entry_functions
                $load_other_functions
            }
            #[cfg(feature = "raw")]
            impl $table_name {
                $commands_funs
            }
            impl Default for $table_name {
                fn default() -> Self {
                    Self::new_empty()
                }
            }
        );
    }
}

fn write_pnext_visit(sorted_symbols: &[(usize, u32)], out: &Path, ctx: &Rc<Context>) {
    let mut stype_to_struct = Vec::new();

    for &(symbol_index, _) in sorted_symbols {
        let Symbol(name, body) = &ctx.reg.symbols[symbol_index];
        match body {
            SymbolBody::Struct { union, members } => {
                if *union == false {
                    if let Some(decl) = members
                        .iter()
                        .find(|m| m.name == ctx.strings.sType && m.ty == ctx.types.VkStructureType)
                    {
                        // QUIRK VkBaseOutStructure does not have a value? It seem to always contain the same struct in its pNext
                        if let Some(stype) = decl.metadata.values.first() {
                            stype_to_struct.push((*stype, *name));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut pnext = SectionWriter::new(
        ctx.create_section("pnext"),
        out.join("src/util/pnext.rs"),
        ctx,
    );

    let variants = Iter::new(
        stype_to_struct,
        |w: &mut SectionWriter, (stype, structure)| {
            code!(w,
                ~$crate::vk10::StructureType::$stype => {
                    let ~$object = ~$pnext.cast::<$import!(structure)>();
                    ~$op;
                    ~$pnext = (*~$object).p_next;
                }
            );
        },
    );

    code!(
        pnext,
        #[macro_export]
        macro_rules! pnext_visit {
            (~$pnext:expr, ~$stype:ident, ~$object:ident, ~$op:expr) => {
                let ~$stype = *~$pnext.cast::<~$crate::vk10::StructureType>();
                match ~$stype  {
                    $variants
                    _ => panic!("Unknown StructureType value ({:?})", ~$stype)
                }
            };
        }
    )
}

fn write_vk_module(
    features: &[UniqueStr],
    extensions: &[&Extension],
    out: &Path,
    ctx: &Rc<Context>,
) {
    let mut vk = SectionWriter::new(ctx.create_section("vk"), out.join("src/vk.rs"), &ctx);
    for feature in features {
        code!(
            vk,
            #[doc(no_inline)]
            pub use crate::$feature::*;
        );
    }
    for extension in extensions {
        code!(
            vk,
            #[doc(no_inline)]
            pub use crate::extensions::$(extension.name)::*;
        );
    }
}

fn write_extension_metadata(extensions: &[&Extension], out: &Path, ctx: &Rc<Context>) {
    let mut meta = SectionWriter::new(
        ctx.create_section("metadata"),
        out.join("src/extensions/metadata.rs"),
        &ctx,
    );
    let exts = fun!(|w| {
        for e in extensions {
            let instance = match e.kind {
                ExtensionKind::Disabled => {
                    unreachable!("'{}' is disabled, it should never get generated!", e.name)
                }
                ExtensionKind::Instance => true,
                ExtensionKind::Device => false,
                // vulkan_video_codecs_common has this, it's not really an extension though
                ExtensionKind::None => continue,
            };

            let (major, minor) = if let Some(core) = e.requires_core {
                let mut split = core.resolve().split('.');
                (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            } else {
                (1, 0)
            };
            let core = format!("VK_API_VERSION_{}_{}", major, minor).intern(&ctx);
            let requires = Separated::args(&e.requires, |w: &mut SectionWriter, n| {
                code!(w,
                    $cstring!(n.resolve_original())
                );
            });

            code!(
                w,
                ExtensionMetadata {
                    name: $cstring!(e.name.resolve_original()),
                    instance: $instance,
                    core_version: $import!(core),
                    requires_extensions: &[
                        $requires
                    ]
                },
            );
        }
    });
    code!(
        meta,
        pub const EXTENSION_METADATA: &[ExtensionMetadata] = &[
            $exts
        ];
    );
}

fn write_wrappers(
    features: &[UniqueStr],
    extensions: &[&Extension],
    out: &Path,
    ctx: &Rc<Context>,
) {
    let mut lib = SectionWriter::new(ctx.create_section("lib"), out.join("src/lib.rs"), &ctx);
    let mut exts = SectionWriter::new(
        ctx.create_section("extensions"),
        out.join("src/extensions/mod.rs"),
        &ctx,
    );
    code!(
        lib,
        pub mod vk;
        $(Iter::new(features, |w: &mut SectionWriter, t| code!(w, pub mod $t;)))
    );
    code!(
        exts,
        $(Iter::new(extensions, |w: &mut SectionWriter, t| code!(w, pub mod $(t.name);)))
    );
    for stage in 0..3 {
        let (wrapper, table) = match stage {
            0 => ("EntryWrapper", "EntryTable".intern(&ctx)),
            1 => ("InstanceWrapper", "InstanceTable".intern(&ctx)),
            2 => ("DeviceWrapper", "DeviceTable".intern(&ctx)),
            _ => unreachable!(),
        };
        let handle_type = match stage {
            0 => None,
            1 => Some(Type::from_only_basetype(ctx.strings.VkInstance)),
            2 => Some(Type::from_only_basetype(ctx.strings.VkDevice)),
            _ => unreachable!(),
        };

        let handle_field = fun!(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    pub(crate) handle: $import!(handle),
                );
            }
        });
        let handle_param = fun!(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    handle: $import!(handle),
                );
            }
        });
        let handle_getter = fun!(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    #[inline]
                    pub fn handle(&self) -> $import!(handle) {
                        self.handle
                    }
                );
            }
        });

        code!(
            lib,
            #[derive(Clone)]
            pub struct $wrapper {
                $handle_field
                pub(crate) table: *const $import!(table),
            }

            impl $wrapper {
                pub unsafe fn new($handle_param table: *const $import!(table)) -> Self {
                    Self {
                        $(Cond::new(handle_type.is_some(), "handle,"))
                        table
                    }
                }
                $handle_getter
                #[inline]
                pub fn table(&self) -> *const $import!(table) {
                    self.table
                }
            }

            $(cat!("/// When creating a ", wrapper, ", you are promising to keep the pointed to table alive for the lifetime of the wrapper\n"))
            unsafe impl Send for $wrapper {}
            unsafe impl Sync for $wrapper {}
        );
    }
}

fn write_access_flags_util(out: &Path, ctx: &Rc<Context>) {
    let mut lib = SectionWriter::new(
        ctx.create_section("access"),
        out.join("src/util/access.rs"),
        &ctx,
    );

    let flags = ["VkAccessFlags", "VkAccessFlags2", "VkAccessFlags2KHR"];

    for name in flags {
        let Some(SymbolBody::Enum { members, .. }) = ctx.get_symbol(name.intern(ctx)) else {
            continue
        };

        let flags_item = |keyword: &'static str| {
            let mut names = members
                .iter()
                .filter(|(name, _)| name.resolve().contains(keyword))
                .map(|(name, _)| name.resolve())
                .collect::<Vec<_>>();

            names.sort();
            names.dedup();

            fun!(move |w| {
                let mut first = true;

                // this will result in code like
                //   const WRITING: Self = Self(Self::SHADER_WRITE.0 | Self::COLOR_ATTACHMENT_WRITE.0 ...)
                // we do this through the raw integers because BitOr cannot currently be implemented for const contexts
                // https://github.com/rust-lang/rust/issues/67792

                for name in &names {
                    if !first {
                        w.write_char('|').unwrap();
                    }
                    first = false;
                    code!(
                        w,
                        Self::$name.0
                    );
                }
            })
        };

        let read = flags_item("READ");
        let write = flags_item("WRITE");
        let access_flags = import_str!(name, ctx);

        code!(
            lib,
            impl $access_flags {
                pub const READ_FLAGS: Self = Self($read);
                pub const WRITE_FLAGS: Self = Self($write);
                /// Whether the AccessFlags contains flags containing "READ"
                pub const fn contains_read(&self) -> bool {
                    self.intersects(Self::READ_FLAGS)
                }
                /// Whether the AccessFlags contains flags containing "WRITE"
                pub const fn contains_write(&self) -> bool {
                    self.intersects(Self::WRITE_FLAGS)
                }
            }
        )
    }
}

fn pipeline_stage_flags_util(out: &Path, ctx: &Rc<Context>) {
    let mut lib = SectionWriter::new(
        ctx.create_section("access"),
        out.join("src/util/stage.rs"),
        &ctx,
    );

    // TODO do this for VkPipelineStageFlags as well
    let flags = ["VkPipelineStageFlags2", "VkPipelineStageFlags2KHR"];
    let mut sanity_done = false;
    for name in flags {
        let Some(SymbolBody::Enum { members, .. }) = ctx.get_symbol(name.intern(ctx)) else {
            continue
        };

        if sanity_done == true {
            panic!("There should be only one non-alias enum!");
        }
        sanity_done = true;

        // ON_UPDATE: add variants from documentation
        //   https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html

        let special: &[(&str, &[&str])] = &[
            (
                "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT",
                &[
                    "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT",
                    "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT",
                ],
            ),
            (
                "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT",
                &[
                    "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT",
                ],
            ),
            (
                "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT",
                &[
                    "VK_PIPELINE_STAGE_2_COPY_BIT",
                    "VK_PIPELINE_STAGE_2_BLIT_BIT",
                    "VK_PIPELINE_STAGE_2_RESOLVE_BIT",
                    "VK_PIPELINE_STAGE_2_CLEAR_BIT",
                    "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR",
                ],
            ),
            (
                "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT",
                &[
                    "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT",
                    "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT",
                    "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT",
                    "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT",
                    "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT",
                    "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT",
                    "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV",
                    "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT",
                    "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI",
                ],
            ),
        ];

        let special_bits = fun!(|w| {
            let top = ctx
                .try_intern("VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")
                .or_else(|| ctx.try_intern("VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR"))
                .unwrap();
            let bottom = ctx
                .try_intern("VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")
                .or_else(|| ctx.try_intern("VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR"))
                .unwrap();

            write!(w, "Self::{top}.0 | Self::{bottom}.0").unwrap();
            for &(special, _) in special {
                let name = ctx.apply_rename(special);
                code!(
                    w,
                    | Self::$name.0
                );
            }
        });

        let special_equivalents = fun!(|w| {
            for &(special, equivalents) in special {
                let values = fun!(|w| {
                    let mut first = true;
                    for &str in equivalents {
                        let name = ctx.try_intern(str).unwrap();
                        // we emit the value only if it actually is in the enum, this can happen due to the member being added by an extension that is not enabled
                        if members
                            .iter()
                            .find(|&&(member, _)| member == name)
                            .is_some()
                        {
                            if !first {
                                w.write_char('|').unwrap();
                            }
                            first = false;
                            code!(
                                w,
                                Self::$name.0
                            );
                        }
                    }
                });

                let rename = ctx.apply_rename(special);
                let name = cat!("UTIL_", rename, "_EQUIVALENT");
                code!(
                    w,
                    pub const $name: Self = Self($values);
                );
            }
        });

        let translate_fn_impl = fun!(|w| {
            for &(special, _) in special {
                let rename = ctx.apply_rename(special);
                let name = cat!("UTIL_", rename, "_EQUIVALENT");

                code!(
                    w,
                    if self.contains(Self::$rename) {
                        out |= Self::$name.0;
                    }
                )
            }
        });

        let stage_flags = import_str!(name, ctx);

        code!(
            lib,
            impl $stage_flags {
                pub const UTIL_SPECIAL_FLAGS: Self = Self($special_bits);
                pub const UTIL_ALL_COMMANDS_EQUIVALENT: Self = Self(Self::all().0 & !Self::UTIL_SPECIAL_FLAGS.0);
                $special_equivalents

                pub const fn translate_special_bits(self) -> Self {
                    let mut out = self.0 & Self::UTIL_ALL_COMMANDS_EQUIVALENT.0;

                    if self.contains(Self::ALL_COMMANDS) {
                        // return all but the special flags
                        return Self::UTIL_ALL_COMMANDS_EQUIVALENT;
                    }

                    $translate_fn_impl

                    Self(out)
                }
            }
        )
    }
}

fn write_format_util(out: &Path, ctx: &Rc<Context>) {
    let mut lib = SectionWriter::new(
        ctx.create_section("format"),
        out.join("src/util/format.rs"),
        &ctx,
    );

    let Some(SymbolBody::Enum { members, .. }) = ctx.get_symbol("VkFormat".intern(ctx)) else {
        unreachable!();
    };

    let variants = fun!(move |w| {
        let names = members.iter().filter_map(|(name, value)| {
            matches!(value, ConstantValue::Symbol(_))
                .not()
                .then(|| name.resolve())
        });

        for name in names {
            let bits = parse_format_string(name);
            let mut color_aspect = false;
            let mut depth_aspect = false;
            let mut stencil_aspect = false;
            let mut plane0_aspect = false;
            let mut plane1_aspect = false;
            let mut plane2_aspect = false;

            let [c, d, s, u] = bits;
            // the format is some compressed block and the parser failed to extract bit counts, assume color
            if c > 0 || (bits.iter().all(|c| *c == 0) && name != "UNDEFINED") {
                color_aspect = true;
            }
            if d > 0 {
                depth_aspect = true;
            }
            if s > 0 {
                stencil_aspect = true;
            }
            if name.contains("PLANE1") {
                plane0_aspect = true;
            }
            if name.contains("PLANE2") {
                plane0_aspect = true;
                plane1_aspect = true;
            }
            if name.contains("PLANE3") {
                plane0_aspect = true;
                plane1_aspect = true;
                plane2_aspect = true;
            }

            let ca = Cond::new(color_aspect, ", COLOR");
            let da = Cond::new(depth_aspect, ", DEPTH");
            let sa = Cond::new(stencil_aspect, ", STENCIL");
            let p0a = Cond::new(plane0_aspect, ", PLANE0");
            let p1a = Cond::new(plane1_aspect, ", PLANE1");
            let p2a = Cond::new(plane2_aspect, ", PLANE2");

            code!(
                w,
                Self::$name => aspects!($c, $d, $s, $u $ca $da $sa $p0a $p1a $p2a),
            );
        }
    });

    let format = import!(ctx.strings.VkFormat);

    code!(
        lib,
        impl $format {
            pub fn get_format_aspects(self) -> (crate::vk10::ImageAspectFlags, FormatAspectBits) {
                match self {
                    $variants
                    _ => panic!("Unknown format {}", self.0)
                }
            }
        }
    )
}

fn parse_format_string(str: &str) -> [u16; 4] {
    let mut counts = [None; 7];

    let mut chars = str.chars();
    while let Some(ch) = chars.next() {
        let next_is_digit = chars
            .clone()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false);

        if next_is_digit {
            let rem = chars.as_str();
            let end = rem
                .char_indices()
                .find(|(_, c)| !c.is_ascii_digit())
                .map(|(i, _)| i)
                .unwrap_or(rem.len());
            let number = rem[..end].parse::<u16>().unwrap();

            let index = match ch {
                'R' => 0,
                'G' => 1,
                'B' => 2,
                'A' => 3,
                'D' => 4,
                'S' => 5,
                'X' => 6,
                _ => 7,
            };

            chars = rem[end..].chars();

            if index < 7 {
                let count = &mut counts[index];
                assert!(count.is_none(), "{str}");
                *count = Some(number);
            }
        }
    }

    #[rustfmt::skip]
    let counts = [
        counts[0].unwrap_or(0) +
        counts[1].unwrap_or(0) +
        counts[2].unwrap_or(0) +
        counts[3].unwrap_or(0),
        counts[4].unwrap_or(0),
        counts[5].unwrap_or(0),
        counts[6].unwrap_or(0),
    ];
    counts
}
