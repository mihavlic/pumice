pub mod deep_copy;
pub mod dumb_hash;
pub mod symbols;
pub mod wrappers;

use crate::codegen::dumb_hash::write_dumb_hash;
use crate::codegen::symbols::fmt_command_preamble;
use crate::codegen_support::format_utils::{
    Cond, ExtendedFormat, Fun, Iter, SectionWriter, Separated,
};
use crate::codegen_support::rename::apply_renames;
use crate::codegen_support::type_analysis::resolve_alias;
use crate::codegen_support::type_query::DeriveData;
use crate::codegen_support::{
    get_command_kind, get_enum_added_variants, merge_bitfield_members, AddedVariants, CommandKind,
};
use crate::context::{Context, SectionFunctions};
use crate::context::{Section, SectionKind};
use crate::fs_utils::{copy_dir_recursive, delete_dir_children};
use crate::{cat, cstring, doc_boilerplate, import, string};
use deep_copy::write_deep_copy;
use format_macro::code;
use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::Type,
    Extension, ExtensionKind, Symbol, SymbolBody,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::Write,
    path::{Path, PathBuf},
};

use self::symbols::write_symbol;

pub fn write_bindings(
    ctx: &mut Context,
    template: &dyn AsRef<Path>,
    out: &dyn AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let out = out.as_ref();

    {
        std::fs::create_dir_all(out).unwrap();
        for entry in std::fs::read_dir(out).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                std::fs::remove_dir_all(path).unwrap();
            } else {
                std::fs::remove_file(path).unwrap();
            }
        }

        delete_dir_children(out).unwrap();
        std::fs::create_dir_all(out.join("src/extensions")).unwrap();
        copy_dir_recursive(template, out).unwrap();
    }

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
            // these macros are in ::crate because they have [macro_export]
            // but they are actually defined in util/impl_macros
            "enum_impl",
            "bitflags_impl",
            "non_dispatchable_handle",
            "dispatchable_handle",
        ],
        "util": [
            "cstr",
            "vkcall",
        ],
        "util/impl_macros": [
            "ObjectHandle",
        ],
        "util/result": [
            "VulkanResult"
        ],
        "util/pnext": [
            "pnext_visit"
        ],
        "util/config": [
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
        // since rust can't express C bitfields we emulate the behaviour by just merging such struct fields with the same algorithm
        // and leaving to the programmer to deal with packing and upacking the values, also since any of the passes could be doing something
        // with struct fields we do this at the earliest time possible
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

    write_wrappers(&features, &extensions, out, ctx);

    write_extension_metadata(&extensions, out, ctx);

    write_vk_module(&features, &extensions, out, ctx);

    // sort the symbols by their owning section (its index)
    let sorted_symbols = {
        let mut clone = ctx.symbols.clone();
        clone.sort_by_key(|i| i.1);
        clone
    };

    write_tables(&sorted_symbols, out, ctx);

    write_pnext_visit(&sorted_symbols, out, ctx);

    let mut derives = DeriveData::new(&ctx);

    write_deep_copy(&mut derives, out, ctx);
    write_dumb_hash(&mut derives, out, ctx);

    write_sections(&sorted_symbols, &added_variants, &mut derives, out, ctx);

    Ok(())
}

fn write_sections(
    sorted_symbols: &[(usize, u32)],
    added_variants: &HashMap<UniqueStr, Vec<AddedVariants>>,
    derives: &mut DeriveData,
    out: &Path,
    ctx: &Context,
) {
    let mut buf = PathBuf::new();
    let mut prev_section = u32::MAX;
    let mut section_writer = None;

    for &(symbol_index, section_index) in sorted_symbols {
        if section_index != prev_section {
            let section = &ctx.sections[section_index as usize];

            {
                buf.clear();
                buf.extend(out);
                buf.push("src");
                buf.extend(section.path());
                buf.push(section.name().resolve());
                buf.set_extension("rs");

                section_writer = Some(SectionWriter::new(section.ident.clone(), &buf, false, &ctx));
            }

            prev_section = section_index;
        }

        let writer = section_writer.as_mut().unwrap();

        let Symbol(name, body) = &ctx.reg.symbols[symbol_index];

        write_symbol(*name, body, writer, derives, &added_variants, &ctx);
    }
}

fn write_tables(sorted_symbols: &[(usize, u32)], out: &Path, ctx: &Context) {
    let mut tables = SectionWriter::new(
        ctx.create_section("tables"),
        out.join("src/loader/tables.rs"),
        true,
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
            let (_, body) = resolve_alias(of, &ctx);
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
            ##[cfg(feature = "global")]
            pub static mut #table_const: #table_type = #table_type::new_empty();
        );
    }
    for &(commands, name, _) in &variations {
        let table_name = cat!(name, "Table");
        let loader_trait = cat!(name, "Load");

        let fields = Fun(|w| {
            for &(_, name, body) in commands {
                if let SymbolBody::Command {
                    return_type,
                    params,
                } = body
                {
                    let preamble = Fun(|w| {
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
                        pub #name: Option<extern "system" #preamble>,
                    )
                } else {
                    unreachable!()
                }
            }
            Ok(())
        });

        let commands_funs = Fun(|w| {
            for &(_, name, body) in commands {
                if let SymbolBody::Command {
                    return_type,
                    params,
                } = body
                {
                    let preamble = Fun(|w| {
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
                        ##[track_caller]
                        #doc_boilerplate!(name)
                        pub unsafe #preamble {
                            (self.#name.unwrap())(
                                #params
                            )
                        }
                    );
                } else {
                    unreachable!()
                }
            }
            Ok(())
        });

        let load_entry_functions = Fun(|w| {
            let fields = Iter(commands, |w, &(_, name, _)| {
                code!(
                    w,
                    (#name, #string!(name.resolve_original()))
                );
            });

            code!(
                w,
                pub fn load(&mut self, loader: &impl #loader_trait) {
                    unsafe {
                        load_fns!{
                            self, loader, #fields
                        }
                    }
                }
            );

            Ok(())
        });

        let load_other_functions = Fun(|w| {
            let mut state = None;
            let fields = Fun(|w| {
                for &(section_idx, name, _) in commands {
                    if Some(section_idx) != state {
                        if state.is_some() {
                            w.write_str("}}")?;
                        }
                        let section = &ctx.sections[section_idx as usize];
                        match section.kind {
                            SectionKind::Feature(i) => {
                                let feature = &ctx.reg.features[i as usize];
                                // vulkan always has 0 variant
                                // https://registry.khronos.org/vulkan/specs/1.3/html/chap31.html#extendingvulkan-coreversions-versionnumbers
                                write!(
                                    w,
                                    r#"
                                        if conf.core_enabled(crate::vk10::make_api_version(0, {}, {}, 0)) {{
                                            load_fns!{{self, loader,
                                            "#,
                                    feature.major, feature.minor
                                )?;
                            }
                            SectionKind::Extension(_) => {
                                write!(
                                    w,
                                    r#"if conf.extension_enabled(cstr!("{}")) {{
                                            load_fns!{{self, loader,
                                            "#,
                                    section.name().resolve_original()
                                )?;
                            }
                            SectionKind::Path(_) => unreachable!(
                                "Custom-pathed sections cannot contain loadable pointers!"
                            ),
                        }
                        state = Some(section_idx);
                    }
                    code!(
                        w,
                        (#name, #string!(name.resolve_original()))
                    );
                }
                w.write_str("}}")?;
                Ok(())
            });

            code!(
                w,
                pub fn load(&mut self, loader: &impl #loader_trait, conf: &ApiLoadConfig) {
                    unsafe {
                        #fields
                    }
                }
            );

            Ok(())
        });

        let do_entry = name == "Entry";
        let load_entry_functions = Cond(do_entry, load_entry_functions);
        let load_other_functions = Cond(!do_entry, load_other_functions);

        // it is valid to use zeroed Option to create a None
        // see https://doc.rust-lang.org/std/option/index.html#representation
        code!(
            tables,
            pub struct #table_name {
                #fields
            }
            impl #table_name {
                pub const fn new_empty() -> Self {
                    unsafe {
                        // https://github.com/maxbla/const-zero#how-does-it-work
                        // for some reason calling const functions in place of generics is invalid
                        const SIZE: usize = std::mem::size_of::<#table_name>();
                        ConstZeroedHack::<#table_name, SIZE>::zero()
                    }
                }
                #load_entry_functions
                #load_other_functions
            }
            ##[cfg(feature = "raw")]
            impl #table_name {
                #commands_funs
            }
        );
    }
}

fn write_pnext_visit(sorted_symbols: &[(usize, u32)], out: &Path, ctx: &Context) {
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
        false,
        &ctx,
    );

    let variants = Iter(stype_to_struct, |w, (stype, structure)| {
        code!(w,
            $crate::vk10::StructureType::#stype => {
                let $object = $pnext.cast::<#import!(structure)>();
                $op;
                $pnext = (*$object).p_next;
            }
        );
    });

    code!(
        pnext,
        ##[macro_export]
        macro_rules! pnext_visit {
            ($pnext:expr, $stype:ident, $object:ident, $op:expr) => {
                let $stype = *$pnext.cast::<$crate::vk10::StructureType>();
                match $stype  {
                    #variants
                    _ => panic!("Unknown StructureType value ({:?})", $stype)
                }
            };
        }
    )
}

fn write_vk_module(features: &[UniqueStr], extensions: &[&Extension], out: &Path, ctx: &Context) {
    let mut vk = SectionWriter::new(ctx.create_section("vk"), out.join("src/vk.rs"), false, &ctx);
    for feature in features {
        code!(
            vk,
            ##[doc(no_inline)]
            pub use crate::#feature::*;
        );
    }
    for extension in extensions {
        code!(
            vk,
            ##[doc(no_inline)]
            pub use crate::extensions::#(extension.name)::*;
        );
    }
}

fn write_extension_metadata(extensions: &[&Extension], out: &Path, ctx: &Context) {
    let mut meta = SectionWriter::new(
        ctx.create_section("metadata"),
        out.join("src/extensions/metadata.rs"),
        true,
        &ctx,
    );
    let exts = Fun(|w| {
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
            let requires = Separated::args(&e.requires, |w, n| {
                code!(w,
                    #cstring!(n.resolve_original())
                );
                Ok(())
            });

            code!(
                w,
                ExtensionMetadata {
                    name: #cstring!(e.name.resolve_original()),
                    instance: #instance,
                    core_version: #import!(core),
                    requires_extensions: &[
                        #requires
                    ]
                },
            );
        }
        Ok(())
    });
    code!(
        meta,
        pub const EXTENSION_METADATA: &[ExtensionMetadata] = &[
            #exts
        ];
    );
}

fn write_wrappers(features: &[UniqueStr], extensions: &[&Extension], out: &Path, ctx: &Context) {
    let mut lib = SectionWriter::new(
        ctx.create_section("lib"),
        out.join("src/lib.rs"),
        true,
        &ctx,
    );
    let mut exts = SectionWriter::new(
        ctx.create_section("extensions"),
        out.join("src/extensions/mod.rs"),
        true,
        &ctx,
    );
    code!(
        lib,
        pub mod vk;
        #(Iter(features, |w, t| code!(w, pub mod #t;)))
    );
    code!(
        exts,
        #(Iter(extensions, |w, t| code!(w, pub mod #(t.name);)))
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

        let handle_field = Fun(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    pub(crate) handle: #import!(handle),
                );
            }
            Ok(())
        });
        let handle_param = Fun(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    handle: #import!(handle),
                );
            }
            Ok(())
        });
        let handle_getter = Fun(|w| {
            if let Some(handle) = &handle_type {
                code!(
                    w,
                    ##[inline]
                    pub fn handle(&self) -> #import!(handle) {
                        self.handle
                    }
                );
            }
            Ok(())
        });

        code!(
            lib,
            pub struct #wrapper {
                #handle_field
                pub(crate) table: *const #import!(table),
            }

            impl #wrapper {
                pub unsafe fn new(#handle_param table: *const #import!(table)) -> Self {
                    Self {
                        #(Cond(handle_type.is_some(), "handle,"))
                        table
                    }
                }
                #handle_getter
                ##[inline]
                pub fn table(&self) -> *const #import!(table) {
                    self.table
                }
            }
        );
    }
}
