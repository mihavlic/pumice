use crate::{
    format_utils::{Cond, ExtendedFormat, Fun, Iter, SectionWriter, Separated},
    ownership::resolve_ownership,
    undangle::undangle,
    workarounds::apply_workarounds,
    wrapper::fmt_command_wrapper,
};

use derives::DeriveData;
use format_macro::code;
use fs_utils::{copy_dir_recursive, delete_dir_children};
use generator_lib::{
    configuration::GenConfig,
    interner::{Intern, UniqueStr},
    type_declaration::{TyToken, Type, TypeRef},
    ConstantValue, Declaration, ExtensionKind, FeatureExtensionItem, InterfaceItem,
    RedeclarationMethod, Registry, Symbol, SymbolBody,
};
use ownership::skip_conf_conditions;
use rename::apply_renames;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write,
    ops::{Deref, Range},
    path::{Path, PathBuf},
};

pub mod derives;
pub mod format_utils;
pub mod fs_utils;
pub mod ownership;
pub mod rename;
pub mod undangle;
pub mod workarounds;
pub mod wrapper;

pub const PLATFORM_TYPES_IMPORT: &str =
    "use crate::{cstr, enum_impl, bitflags_impl, dispatchable_handle, non_dispatchable_handle, util::{char, double, float, int, void}};";
pub const SLAG_PLATFORM_TYPES_IMPORT: &str =
    "use slag_sys::util::{char, double, float, int, void};";

type SectionIdent = Range<UniqueStr>;

pub trait SectionFunctions {
    fn lib(&self) -> UniqueStr;
    fn name(&self) -> UniqueStr;
}

impl SectionFunctions for SectionIdent {
    fn lib(&self) -> UniqueStr {
        self.start
    }
    fn name(&self) -> UniqueStr {
        self.end
    }
}

#[derive(Clone)]
pub enum SectionKind {
    Feature(u32),
    Extension(u32),
    Path(String),
}

pub struct Section {
    pub ident: SectionIdent,
    pub kind: SectionKind,
}

impl Section {
    pub fn path(&self) -> impl Iterator<Item = &str> {
        let mut path = None;
        let slice: &[&str] = match &self.kind {
            SectionKind::Feature(_) => &[],
            SectionKind::Extension(_) => &["extensions"],
            SectionKind::Path(s) => {
                path = Some(s.as_str());
                &[]
            }
        };
        slice.iter().cloned().chain(
            path.into_iter()
                .flat_map(|s| s.split('/').filter(|s| !s.is_empty())),
        )
    }
}

impl SectionFunctions for Section {
    fn lib(&self) -> UniqueStr {
        self.ident.start
    }
    fn name(&self) -> UniqueStr {
        self.ident.end
    }
}

macro_rules! common_strings {
    ($($string:ident),+) => {
        #[allow(non_snake_case)]
        pub struct CommonStrings {
            $(
                pub $string: UniqueStr,
            )+
        }
        impl CommonStrings {
            fn new(int: &generator_lib::interner::Interner) -> Self {
                Self {
                    $(
                        $string: stringify!($string).intern(int),
                    )+
                }
            }
        }
    };
}

macro_rules! common_types {
    ($($name:ident: $string:expr),+) => {
        #[allow(non_snake_case)]
        pub struct CommonTypes {
            $(
                pub $name: Type,
            )+
        }
        impl CommonTypes {
            fn new(int: &generator_lib::interner::Interner) -> Self {
                Self {
                    $(
                        $name: generator_lib::type_declaration::parse_type_decl($string, int).1,
                    )+
                }
            }
        }
    };
}

common_strings! {
    void, int, char, float, double, bool,
    size_t, uint8_t, uint16_t, uint32_t, uint64_t, int8_t, int16_t, int32_t, int64_t,
    usize, u8, u16, u32, u64, i8, i16, i32, i64,
    vk_platform, disabled,
    VkDevice, VkCommandBuffer, VkQueue,
    VkInstance, VkPhysicalDevice,
    slag_sys, slag,
    pNext, sType
}

common_types! {
    cstring:  "const char *",
    void:     "void",
    void_ptr: "void *",
    StructureType: "StructureType",
    VkResult: "VkResult",
    VulkanResult: "VulkanResult",
    VkBool32: "VkBool32",
    bool: "bool"
}

pub struct Context {
    pub reg: Registry,
    pub conf: GenConfig,
    pub symbol_ownership: HashMap<UniqueStr, u32>,
    pub sections: Vec<Section>,
    // previously we had sorted the sections vector and then binary searched thta
    // however the ownership algorithm is order sensitive and this may potentially
    // cause issues later, instead we now have a hashmap
    // (section name, section's index in self.sections)
    pub section_map: HashMap<SectionIdent, u32>,
    pub strings: CommonStrings,
    pub types: CommonTypes,
}

impl Context {
    pub fn new(conf: GenConfig, reg: Registry) -> Self {
        let sections_len = reg.features.len() + reg.extensions.len();

        let mut s = Self {
            symbol_ownership: HashMap::new(),
            sections: Vec::with_capacity(sections_len),
            section_map: HashMap::with_capacity(sections_len),
            strings: CommonStrings::new(&reg),
            types: CommonTypes::new(&reg),
            reg,
            conf,
        };

        {
            let slag_sys = s.strings.slag_sys;
            for (i, feature) in s.reg.features.iter().enumerate() {
                s.sections.push(Section {
                    ident: slag_sys..feature.name,
                    kind: SectionKind::Feature(i as u32),
                });
            }

            for (i, extension) in s.reg.extensions.iter().enumerate() {
                s.sections.push(Section {
                    ident: slag_sys..extension.name,
                    kind: SectionKind::Extension(i as u32),
                });
            }

            s.section_map.extend(
                s.sections
                    .iter()
                    .enumerate()
                    .map(|(i, e)| (e.ident.clone(), i as u32)),
            );
        }

        apply_workarounds(&mut s);
        resolve_ownership(&mut s);

        s
    }
    pub fn get_section_idx(&self, ident: SectionIdent) -> Option<u32> {
        self.section_map.get(&ident).cloned()
    }
    pub fn get_section(&self, ident: SectionIdent) -> Option<&Section> {
        self.sections.get(self.get_section_idx(ident)? as usize)
    }
    pub fn symbol_get_section_idx(&self, name: UniqueStr) -> Option<u32> {
        self.symbol_ownership.get(&name).cloned()
    }
    pub fn symbol_get_section(&self, name: UniqueStr) -> Option<&Section> {
        self.sections
            .get(self.symbol_get_section_idx(name)? as usize)
    }
    pub fn remove_symbol(&mut self, name: UniqueStr) {
        self.reg.remove_symbol(name);
        self.symbol_ownership.remove(&name);
    }
    pub fn register_section(&mut self, section: Section) {
        assert!(!self.section_map.contains_key(&section.ident));
        self.section_map
            .insert(section.ident.clone(), self.sections.len() as u32);
        self.sections.push(section);
    }
    pub fn section_add_symbols(
        &mut self,
        ident: SectionIdent,
        symbols: impl IntoIterator<Item = UniqueStr>,
    ) {
        let index = *self.section_map.get(&ident).unwrap();
        for symbol in symbols {
            assert!(!self.symbol_ownership.contains_key(&symbol));
            self.symbol_ownership.insert(symbol, index);
        }
    }
    pub fn section_add_symbols_str<'a, 'b>(
        &'a mut self,
        ident: SectionIdent,
        symbols: impl IntoIterator<Item = &'b str>,
    ) {
        let index = *self.section_map.get(&ident).unwrap();
        for symbol in symbols {
            let symbol = symbol.intern(&self);
            assert!(!self.symbol_ownership.contains_key(&symbol));
            self.symbol_ownership.insert(symbol, index);
        }
    }
}

impl Deref for Context {
    type Target = Registry;

    fn deref(&self) -> &Self::Target {
        &self.reg
    }
}

pub enum CommandKind {
    Entry,
    Instance,
    Device,
}

pub fn get_command_kind(params: &[Declaration], ctx: &Context) -> CommandKind {
    // idea shamelessly stolen from erupt
    if let Some(param) = params.get(0) {
        if let Some(basetype) = param.ty.try_only_basetype() {
            let s = &ctx.strings;
            switch!( basetype;
                s.VkInstance, s.VkPhysicalDevice => return CommandKind::Instance;
                s.VkDevice, s.VkCommandBuffer, s.VkQueue => return CommandKind::Device;
                @ {}
            )
        }
    }
    CommandKind::Entry
}

pub fn write_bindings(
    mut ctx: Context,
    template: &dyn AsRef<Path>,
    out: &dyn AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let slag_sys = out.as_ref().join("slag-sys");
    let slag = out.as_ref().join("slag");
    let mut buf = PathBuf::new();

    std::fs::create_dir_all(&out).unwrap();
    for entry in std::fs::read_dir(&out).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            std::fs::remove_dir_all(path).unwrap();
        } else {
            std::fs::remove_file(path).unwrap();
        }
    }

    delete_dir_children(out).unwrap();
    std::fs::create_dir_all(slag_sys.join("src/extensions")).unwrap();
    std::fs::create_dir_all(slag.join("src/extensions")).unwrap();
    copy_dir_recursive(template, out).unwrap();

    // (symbol index, section index)
    // all the symbols that we will be generating
    let mut symbols = Vec::new();
    for (i, &Symbol(name, _)) in ctx.reg.symbols.iter().enumerate() {
        if let Some(section_idx) = ctx.symbol_get_section_idx(name) {
            let section = &ctx.sections[section_idx as usize];

            let used = match section.kind {
                SectionKind::Feature(_) => ctx.conf.is_feature_used(section.name()),
                SectionKind::Extension(i) => {
                    let e = &ctx.reg.extensions[i as usize];
                    ctx.conf.is_extension_used(section.name()) && e.kind != ExtensionKind::Disabled
                }
                SectionKind::Path(_) => {
                    unreachable!("Custom-pathed sections cannot be generated by this method!")
                }
            };

            if used {
                symbols.push((i, section_idx));
            }
        }
    }

    // manually input sections and their contained symbols
    // for the codegen-template handwritten files
    let handwritten_sections: &[(UniqueStr, &str, &str, &[&str])] = &[
        (
            ctx.strings.slag_sys,
            "",
            "util",
            &[
                "cstr",
                "vkcall",
                "enum_impl",
                "bitflags_impl",
                "ObjectHandle",
                "non_dispatchable_handle",
                "dispatchable_handle",
                "ApiLoadConfig",
                "ApiLoadConfigErr",
                "VulkanResult",
            ],
        ),
        (
            ctx.strings.slag_sys,
            "",
            "surface",
            &["create_surface", "enumerate_required_extensions"],
        ),
        (
            ctx.strings.slag_sys,
            "",
            "loader",
            &[
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
        ),
        (
            ctx.strings.slag_sys,
            "loader",
            "tables",
            &[
                "GLOBAL_ENTRY_TABLE",
                "GLOBAL_INSTANCE_TABLE",
                "GLOBAL_DEVICE_TABLE",
                "EntryTable",
                "InstanceTable",
                "DeviceTable",
            ],
        ),
        (
            ctx.strings.slag_sys,
            "extensions",
            "metadata",
            &["ExtensionMetadata", "get_metadata", "EXTENSION_METADATA"],
        ),
    ];

    for &(lib, path, name, items) in handwritten_sections {
        let ident = lib..name.intern(&ctx);
        ctx.register_section(Section {
            ident: ident.clone(),
            kind: SectionKind::Path(path.to_owned()),
        });
        ctx.section_add_symbols_str(ident, items.iter().cloned());
    }

    undangle(&symbols, &mut ctx);
    let added_variants = get_enum_added_variants(&ctx);
    apply_renames(&symbols, &added_variants, &ctx);

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
        .inspect(|e| {
            if e.name.resolve_original() == "VK_AMD_extension_17" {
                panic!("{:#?}", e)
            }
        })
        .collect::<Vec<_>>();

    extensions.sort_by(|a, b| a.name.resolve().cmp(b.name.resolve()));

    let create_sys_section =
        |name: &str| -> SectionIdent { ctx.strings.slag_sys..name.intern(&ctx) };

    {
        let mut lib = SectionWriter::new(
            create_sys_section("lib"),
            slag_sys.join("src/lib.rs"),
            true,
            &ctx,
        );
        let mut exts = SectionWriter::new(
            create_sys_section("extensions"),
            slag_sys.join("src/extensions/mod.rs"),
            true,
            &ctx,
        );

        code!(
            lib,
            pub mod vk;
            #(Iter(&features, |w, t| code!(w, pub mod #t;)))
        );

        code!(
            exts,
            #(Iter(&extensions, |w, t| code!(w, pub mod #(t.name);)))
        );
    }

    {
        let mut meta = SectionWriter::new(
            create_sys_section("metadata"),
            slag_sys.join("src/extensions/metadata.rs"),
            true,
            &ctx,
        );

        let exts = Fun(|w| {
            for e in &extensions {
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

    {
        let mut vk = SectionWriter::new(
            create_sys_section("vk"),
            slag_sys.join("src/vk.rs"),
            false,
            &ctx,
        );

        for feature in &features {
            code!(
                vk,
                ##[doc(no_inline)]
                pub use crate::#feature::*;
            );
        }
        for extension in &extensions {
            code!(
                vk,
                ##[doc(no_inline)]
                pub use crate::extensions::#(extension.name)::*;
            );
        }
    }

    symbols.sort_by_key(|i| i.1);

    {
        let mut tables = SectionWriter::new(
            create_sys_section("tables"),
            slag_sys.join("src/loader/tables.rs"),
            true,
            &ctx,
        );

        // (index of owning section, name of function, actual name of function)
        let mut entry = Vec::new();
        let mut instance = Vec::new();
        let mut device = Vec::new();

        for &(index, _) in &symbols {
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
                if is_std_type(of, &ctx) {
                    continue;
                }
                let (of, body) = resolve_alias(of, &ctx);
                if let SymbolBody::Command { params, .. } = body {
                    let what = (section_idx, of, body);
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
                ##[cfg(feature = "global_commands")]
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
                                params.iter().map(|p| &p.ty),
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
                                params.iter().map(|p| &p.ty),
                                true,
                                "&self, ",
                                return_type,
                                &ctx,
                            )
                        });
                        let params = Separated::display(params.iter().map(|p| p.name), ",");

                        code!(
                            w,
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

            let entry_functions = Fun(|w| {
                let fields = Iter(commands, |w, &(_, name, _)| {
                    code!(
                        w,
                        (#name, #string!(name.resolve_original()))
                    );
                });

                code!(w, pub fn load(&mut self, loader: &impl #loader_trait) {
                    unsafe {
                        load_fns!{
                            self, loader, #fields
                        }
                    }
                });

                Ok(())
            });

            let other_functions = Fun(|w| {
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
            let entry_functions = Cond(do_entry, entry_functions);
            let other_functions = Cond(!do_entry, other_functions);

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
                    #commands_funs
                    #entry_functions
                    #other_functions
                }
            );
        }

        let mut sections = HashSet::new();
        let mut lib = SectionWriter::new(
            ctx.strings.slag.."lib".intern(&ctx),
            slag.join("src/lib.rs"),
            true,
            &ctx,
        );
        let mut extensions = SectionWriter::new(
            ctx.strings.slag.."extensions".intern(&ctx),
            slag.join("src/extensions/mod.rs"),
            false,
            &ctx,
        );
        let mut vk = SectionWriter::new(
            ctx.strings.slag.."vk".intern(&ctx),
            slag.join("src/vk.rs"),
            false,
            &ctx,
        );
        for &(commands, name, _) in &variations {
            let mut i = 0;

            while i < commands.len() {
                let &(section_index, ..) = &commands[i];
                let section = &ctx.sections[section_index as usize];

                let mut prev_section = None;
                let mut commands_iter = commands[i..].iter();
                let commands_iter = std::iter::from_fn(|| {
                    if let Some((section, name, body)) = commands_iter.next() {
                        if prev_section.is_none() {
                            prev_section = Some(section);
                        }
                        if Some(section) != prev_section {
                            return None;
                        }
                        i += 1;
                        return Some((section, name, body));
                    }
                    return None;
                });

                buf.clone_from(&slag);
                buf.push("src");
                buf.extend(section.path());
                buf.push(section.name().resolve());
                buf.set_extension("rs");

                let mut writer =
                    SectionWriter::new(ctx.strings.slag..section.name(), &buf, true, &ctx);

                let functions = Iter(commands_iter, |w, (_, &name, &body)| {
                    fmt_command_wrapper(w, name, body, &ctx);
                });

                if sections.insert(section_index) {
                    let name = section.name();
                    match section.kind {
                        SectionKind::Feature(_) => {
                            code!(
                                lib,
                                pub mod #name;
                            );
                            code!(
                                vk,
                                ##[doc(no_inline)]
                                pub use crate::#name::*;
                            )
                        }
                        SectionKind::Extension(_) => {
                            code!(
                                extensions,
                                pub mod #name;
                            );
                            code!(
                                vk,
                                ##[doc(no_inline)]
                                pub use crate::extensions::#name::*;
                            )
                        }
                        SectionKind::Path(_) => unreachable!(),
                    };

                    code!(
                        writer,
                        #(SLAG_PLATFORM_TYPES_IMPORT)
                    )
                }

                code!(
                    writer,
                    impl crate::#name {
                        #functions
                    }
                );
            }
        }

        for &(_, name, _) in &variations {
            let table_type = format!("{name}Table").intern(&ctx);
            let handle_type = match name {
                "Entry" => None,
                "Instance" => Some(Type::from_only_basetype(ctx.strings.VkInstance)),
                "Device" => Some(Type::from_only_basetype(ctx.strings.VkDevice)),
                _ => unreachable!(),
            };
            let handle = Fun(|w| {
                if let Some(handle) = &handle_type {
                    code!(w, handle: #import!(handle),);
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
                pub struct #name {
                    #(handle.clone())
                    table: *const #import!(table_type),
                }

                impl #name {
                    pub unsafe fn new(#handle table: *const #import!(table_type)) -> Self {
                        Self {
                            #(Cond(handle_type.is_some(), "handle,"))
                            table
                        }
                    }
                    ##[inline]
                    pub fn table(&self) -> *const #import!(table_type) {
                        self.table
                    }
                    #handle_getter
                }
            );
        }
    }

    let mut derives = DeriveData::new(&ctx);

    let mut prev_section = u32::MAX;
    let mut slag_sys_writer = None;

    for &(symbol_index, section_index) in &symbols {
        if section_index != prev_section {
            let section = &ctx.sections[section_index as usize];

            {
                buf.clone_from(&slag_sys);
                buf.push("src");
                buf.extend(section.path());
                buf.push(section.name().resolve());
                buf.set_extension("rs");

                let mut writer = SectionWriter::new(section.ident.clone(), &buf, false, &ctx);

                code!(writer, #PLATFORM_TYPES_IMPORT);

                slag_sys_writer = Some(writer);
            }

            prev_section = section_index;
        }

        let sys_writer = slag_sys_writer.as_mut().unwrap();

        let Symbol(name, body) = &ctx.reg.symbols[symbol_index];

        write_item(*name, body, sys_writer, &mut derives, &added_variants, &ctx);
    }

    Ok(())
}

pub struct AddedVariants<'a> {
    source_extension: UniqueStr,
    applicable: bool,
    variants: Vec<(UniqueStr, &'a ConstantValue)>,
}

fn get_enum_added_variants(ctx: &Context) -> HashMap<UniqueStr, Vec<AddedVariants<'_>>> {
    let mut enum_supplements: HashMap<UniqueStr, Vec<AddedVariants<'_>>> = HashMap::new();

    let features = ctx
        .reg
        .features
        .iter()
        .map(|f| (ctx.conf.is_feature_used(f.name), f.name, &f.children));
    let extensions = ctx
        .reg
        .extensions
        .iter()
        .filter(|e| e.kind != ExtensionKind::Disabled)
        .map(|e| {
            (
                ctx.conf.is_extension_used(e.name)
                // when an extention is promoted to core, all its enum values are copied into a
                // feature <require> thus variants from the extensions are no longer applicable if
                // this occurs note that this isn't done for constants because extensions specify
                // their name and version that way
                && !e.promotedto
                    .map(|core| ctx.conf.is_feature_used(core))
                    .unwrap_or(false),
                e.name,
                &e.children,
            )
        });

    for (applicable, section_name, children) in features.chain(extensions) {
        for item in children {
            match item {
                FeatureExtensionItem::Comment(_) => {}
                FeatureExtensionItem::Require {
                    profile,
                    api,
                    extension,
                    feature,
                    items,
                } => {
                    let applicable = applicable
                        && !skip_conf_conditions(
                            api, *extension, None, *feature, *profile, &ctx.conf,
                        );
                    'outer: for item in items {
                        match item {
                            InterfaceItem::Simple { .. } => {}
                            &InterfaceItem::Extend {
                                name,
                                extends,
                                ref value,
                            } => {
                                let entry = enum_supplements.entry(extends).or_insert(Vec::new());

                                let add = (name, value);

                                // we deduplicate the variants here, because khronos was so nice to willingly put
                                // duplicates in the registry, like VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR
                                'middle: for added in &mut *entry {
                                    for i in 0..added.variants.len() {
                                        let &(n, _) = &added.variants[i];
                                        if n == name {
                                            if !applicable {
                                                continue 'outer;
                                            }
                                            // if the added variant is not applicable, ie. soft-deleted
                                            // we remove it and overwrite it with the current one, otherwise
                                            else if !added.applicable {
                                                added.variants.remove(i);
                                                break 'middle;
                                            } else {
                                                continue;
                                            }
                                        }
                                    }
                                }

                                if let Some(a) = entry
                                    .iter_mut()
                                    .find(|a| a.source_extension == section_name)
                                {
                                    a.variants.push(add);
                                } else {
                                    entry.push(AddedVariants {
                                        source_extension: section_name,
                                        applicable: applicable,
                                        variants: vec![add],
                                    });
                                }
                            }
                        }
                    }
                }
                FeatureExtensionItem::Remove { .. } => unimplemented!(),
            }
        }
    }
    enum_supplements
}

fn write_item(
    name: UniqueStr,
    body: &SymbolBody,
    sys_writer: &mut SectionWriter<'_>,
    derives: &mut DeriveData,
    supplements: &HashMap<UniqueStr, Vec<AddedVariants>>,
    ctx: &Context,
) {
    macro_rules! select {
        ($cond:expr, $true:expr, $false:expr) => {
            if $cond {
                $true
            } else {
                $false
            }
        };
    }

    match body {
        &SymbolBody::Alias(of) => {
            if !is_std_type(of, &ctx) {
                let target = resolve_alias(of, &ctx);
                match target.1 {
                    SymbolBody::Alias { .. } | SymbolBody::Define { .. } => {
                        unreachable!();
                    }
                    SymbolBody::Constant { .. } => {
                        let ty = get_underlying_type(target.0, &ctx);
                        code!(
                            sys_writer,
                            pub const #name: #ty = #import!(target.0);
                        );
                        return;
                    }
                    SymbolBody::Command {
                        return_type,
                        params,
                    } => {
                        fmt_global_command(sys_writer, name, target.0, params, return_type, ctx);
                        return;
                    }
                    _ => {}
                }
            };

            code!(
                sys_writer,
                #doc_boilerplate!(name)
                pub type #name = #import!(of);
            );
        }
        SymbolBody::Redeclaration(method) => match method {
            RedeclarationMethod::Type(ty) => {
                code!(
                    sys_writer,
                    pub type #name = #import!(ty);
                );
            }
            RedeclarationMethod::Custom(fun) => {
                fun(sys_writer).unwrap();
            }
        },
        // there is nothing to do with defines in rust, just skip them
        SymbolBody::Define { body } => {
            // FIXME burn this code down

            let mut code = String::new();
            let mut chars = body.chars().peekable();

            // remove comments
            'outer: while let Some(next) = chars.next() {
                match next {
                    '/' => {
                        if let Some('/') = chars.peek() {
                            while let Some(next) = chars.next() {
                                if next == '\n' {
                                    break;
                                }
                            }
                            continue 'outer;
                        }
                    }
                    _ => {}
                }
                code.push(next);
            }

            // String::remove_matches is unstable so this is what you get
            code = code.replace("#define", "");
            code = code.replace(name.resolve_original(), "");
            code = code.replace(
                "VK_MAKE_VIDEO_STD_VERSION",
                "crate::extensions::video_common::make_video_std_version",
            );
            code = code.replace("VK_MAKE_API_VERSION", "crate::vk10::make_api_version");
            code = code.replace("VK_API_VERSION_VARIANT", "crate::vk10::api_version_variant");
            code = code.replace("VK_API_VERSION_MAJOR", "crate::vk10::api_version_major");
            code = code.replace("VK_API_VERSION_MINOR", "crate::vk10::api_version_minor");
            code = code.replace("VK_API_VERSION_PATCH", "crate::vk10::api_version_patch");
            code = code.replace(
                "VK_HEADER_VERSION",
                // the constant gets renamed and we need to know its new name
                "VK_HEADER_VERSION".intern(ctx).resolve(),
            );

            code!(
                sys_writer,
                pub const #name: u32 = #code;
            );
        }
        SymbolBody::Included { .. } => {
            unreachable!("[{}]", name);
        }
        SymbolBody::Basetype { .. } => {
            unreachable!("[{}] Cannot process C preprocessor code, this type should be manually replaced in a workaround.", name);
        }
        &SymbolBody::Handle {
            object_type,
            dispatchable,
        } => {
            let raw = name.resolve_original();
            let handle = select!(
                dispatchable,
                "non_dispatchable_handle!",
                "dispatchable_handle!"
            );

            code!(
                sys_writer,
                #handle (
                    #name, #object_type, #string!(raw),
                    #cat!("\"[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/", raw, ".html)\"")
                );
            );
        }
        SymbolBody::Funcpointer { ret, args } => {
            let preamble = Fun(|w| {
                fmt_command_preamble(
                    w,
                    "",
                    args.iter().map(|p| p.0),
                    args.iter().map(|p| &p.1),
                    true,
                    "",
                    ret,
                    ctx,
                )
            });
            code!(sys_writer,
                #doc_boilerplate!(name)
                pub type #name = unsafe extern "system" #preamble;
            );
        }
        SymbolBody::Struct { union, members } => {
            let keyword = match union {
                true => "union",
                false => "struct",
            };
            let members = merge_bitfield_members(&members, &ctx);
            let members = Separated::args(members.iter(), |w, &(name, ty)| {
                code!(w, pub #name: #import!(ty));
                Ok(())
            });
            let copy = Cond(derives.is_copy(name, ctx), ", Copy");
            let eq = Cond(derives.is_eq(name, ctx), ", PartialEq, Eq, Hash");
            let zeroable = derives.is_zeroable(name, ctx);

            code!(
                sys_writer,
                #doc_boilerplate!(name)
                ##[derive(Clone #copy #eq)]
                ##[repr(C)]
                pub #keyword #name {
                    #members
                }
                #(Cond(zeroable, Fun(|w| {code!(w,
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe {
                                std::mem::MaybeUninit::zeroed().assume_init()
                            }
                        }
                    }
                ); Ok(())} )))
            );
        }
        SymbolBody::Constant { val, .. } => {
            let ty = get_underlying_type(name, &ctx);

            let val = Fun(|w| {
                use ConstantValue::*;
                match val {
                    Bitpos(p) => code!(w, 1 << #p),
                    Literal(i) => code!(w, #i),
                    Expression(e) => code!(w, #e),
                    Symbol(s) => code!(w, #import!(*s)),
                    String(s) => code!(w, #cstring!(s)),
                }
                Ok(())
            });

            code!(sys_writer, pub const #name: #ty = #val;)
        }
        SymbolBody::Enum {
            ty,
            members,
            bitmask,
        } => {
            let ty = get_underlying_type(*ty, &ctx);

            let eq = select!(derives.is_eq(name, ctx), ", Eq, Hash", "");

            let supl = supplements.get(&name);

            code!(
                sys_writer,
                #doc_boilerplate!(name)
                ##[derive(Clone, Copy, PartialEq #eq, Default)]
                ##[repr(transparent)]
                pub struct #name(pub #ty);
            );

            let member_iter = members.iter().map(|a| (name, a.0, &a.1));
            let supl_iter = supl
                .into_iter()
                .flatten()
                .filter(|a| a.applicable)
                .flat_map(|a| a.variants.iter().map(|b| (a.source_extension, b.0, b.1)));

            let mut members = member_iter.chain(supl_iter).collect::<Vec<_>>();

            let mut i = 0;
            'a: while i < members.len() {
                let mut j = i + 1;
                'b: while j < members.len() {
                    let a = &members[i];
                    let b = &members[j];
                    if a.1.eq_resolve(b.1) {
                        match (&a.2, &b.2) {
                            (_, ConstantValue::Symbol(to)) => {
                                assert_eq!(*to, a.1);
                                members.remove(j);
                                continue 'a;
                            }
                            (ConstantValue::Symbol(to), _) => {
                                assert_eq!(*to, b.1);
                                members.remove(i);
                                continue 'b;
                            }
                            (a_val, b_val) => {
                                // multiple conditional <require> may be applicable and result
                                // in duplicate variants, here we remove them
                                // example: VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR
                                if a_val == b_val {
                                    members.remove(j);
                                    continue 'a;
                                } else {
                                    unreachable!(
                                        "Duplicates '{}' and '{}' should share a value!",
                                        a.1.resolve_original(),
                                        b.1.resolve_original()
                                    )
                                }
                            }
                        }
                    }
                    j += 1;
                }
                i += 1;
            }

            // Vulkan enums allow for some variant to be an alias of another these are mostly used
            // for backwards compatibility when some enum is promoted to core, the _KHR and such
            // variants remain. Hoiwever if we are only generating the extensions which used to have
            // these variants natively we may happen to not generate the core version that currently
            // has the actual variants of which these are aliases. Thus we only "softly" no-generate
            // these variants and here we look for aliases to softly removed ("not applicable")
            // variants and if this happens we replace the alias to its supposed value
            for (_, _, val) in &mut members {
                'propagate: loop {
                    match val {
                        ConstantValue::Symbol(alias) => {
                            if let Some(supl) = supl {
                                for added in supl {
                                    for &(name, other_val) in &added.variants {
                                        if name == *alias {
                                            if !added.applicable {
                                                *val = other_val;
                                                continue 'propagate;
                                            } else {
                                                break 'propagate;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => break,
                    }

                    break;
                }
            }

            let state = Cell::new(name);
            let variants = Iter(&members, |w, &(ext, name, val)| {
                if state.get() != ext {
                    write!(w, "/// {ext}\n").unwrap();
                    state.set(ext);
                }

                use ConstantValue::*;
                match val {
                    Bitpos(pos) => code!(w, pub const #name: Self = Self(1 << #pos);),
                    Literal(val) => code!(w, pub const #name: Self = Self(#val);),
                    Expression(str) => code!(w, pub const #name: Self = Self(#str);),
                    Symbol(alias) => code!(w, pub const #name: Self = Self::#alias;),
                    String(_) => unreachable!(),
                }
            });

            if !members.is_empty() {
                code!(
                    sys_writer,
                    impl #name {
                        #variants
                    }
                );
            }

            let variants = Separated::display(
                members
                    .iter()
                    .filter(|m| !matches!(m.2, ConstantValue::Symbol(_)))
                    .map(|m| m.1),
                ",",
            );
            if *bitmask {
                let mut all = 0;
                for &(_, _, val) in &members {
                    match val {
                        &ConstantValue::Bitpos(v) => all |= 1 << v,
                        &ConstantValue::Literal(v) => all |= v,
                        ConstantValue::Expression(_) => unreachable!(),
                        ConstantValue::Symbol(_) => {}
                        ConstantValue::String(_) => {}
                    }
                }
                let all = Fun(|w| write!(w, "0x{:x}", all));

                code!(
                    sys_writer,
                    bitflags_impl! {
                        #name: #ty, #all, #variants
                    }
                )
            } else {
                code!(
                    sys_writer,
                    enum_impl! {
                        #name: #ty, #variants
                    }
                )
            }
        }
        SymbolBody::Command {
            return_type,
            params,
        } => {
            fmt_global_command(sys_writer, name, name, params, return_type, ctx);
        }
    }
}

fn fmt_global_command(
    writer: &mut SectionWriter<'_>,
    name: UniqueStr,
    fptr_name: UniqueStr,
    params: &Vec<Declaration>,
    return_type: &Type,
    ctx: &Context,
) {
    let preamble = Fun(|w| {
        fmt_command_preamble(
            w,
            name.resolve(),
            params.iter().map(|p| p.name),
            params.iter().map(|p| &p.ty),
            true,
            "",
            return_type,
            ctx,
        )
    });
    let table = match get_command_kind(&params, ctx) {
        CommandKind::Entry => "GLOBAL_ENTRY_TABLE",
        CommandKind::Instance => "GLOBAL_INSTANCE_TABLE",
        CommandKind::Device => "GLOBAL_DEVICE_TABLE",
    };
    let params = Separated::display(params.iter().map(|p| p.name), ",");

    code!(
        writer,
        ##[cfg(feature = "global_commands")]
        #doc_boilerplate!(name)
        pub unsafe #preamble {
            (crate::loader::tables::#table.#fptr_name.unwrap())(
                #params
            )
        }
    );
}

fn fmt_command_preamble<'a>(
    writer: &mut SectionWriter,
    name: &str,
    param_names: impl Iterator<Item = UniqueStr> + Clone,
    param_types: impl Iterator<Item = &'a Type> + Clone,
    output_names: bool,
    self_str: &str,
    return_type: &Type,
    ctx: &Context,
) -> std::fmt::Result {
    let iter = param_names.into_iter().zip(param_types.into_iter());
    let args = Separated::args(iter, |w, (name, ty)| {
        if output_names {
            code!(w, #name: #import!(ty));
        } else {
            code!(w, #import!(ty));
        }
        Ok(())
    });

    code!(writer, fn #name(#self_str #args));

    if return_type == &ctx.types.void {
        return Ok(());
    }

    code!(writer, -> #import!(return_type));
    Ok(())
}

// whether the type is provided from the rust standard library and as such has no entry in the Registry
pub fn is_std_type(ty: UniqueStr, ctx: &Context) -> bool {
    let s = &ctx.strings;
    switch!(ty;
        s.void, s.int, s.char, s.float, s.double, s.bool,
        s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => true;
        @ false
    )
}

// essentially allows matching against runtime values
macro_rules! switch {
    ($what:expr; $( $($e:expr),+ => $to:expr;)+ @ $esle:expr) => {
        $(
            if $($what == $e) ||+  {
                $to
            } else
        )+
        {
            $esle
        }
    };
}
pub(crate) use switch;

fn merge_bitfield_members<'a>(
    members: &'a [Declaration],
    ctx: &Context,
) -> Vec<(UniqueStr, &'a Type)> {
    let mut resolved = Vec::new();
    let mut last_ty: Option<&Type> = None;
    let mut current_bits = 0;
    let mut max_bits = 0;
    let mut merged_members: Vec<UniqueStr> = Vec::new();

    for member in members {
        let Declaration {
            name, ty, bitfield, ..
        } = member;

        // the type matches and it is a bitfield
        if Some(ty) == last_ty && bitfield.is_some() {
            let bits = bitfield.unwrap();
            assert!(bits <= max_bits);
            current_bits += bits;
            // we still have space to merge this member
            if current_bits <= max_bits {
                merged_members.push(*name);
                continue;
            }
            // otherwise we just pass through and the merged members are picked up and merged
            // and the current member is added to the next batch
        };

        // merge the accumulated members into one member that will have to be packed and unpacked by the user
        // TODO consider propagating this information and making helper functions
        if let Some(ty) = last_ty.take() {
            assert!(!merged_members.is_empty());

            // TODO consider some better naming rather than just concatenating everything
            let name = if merged_members.len() == 1 {
                merged_members[0]
            } else {
                let mut concat = merged_members[0].resolve().to_owned();
                for member in &merged_members[1..] {
                    concat += "_";
                    concat += member.resolve();
                }
                concat.intern(ctx)
            };

            resolved.push((name, ty));
            merged_members.clear();
        }

        // start accumulating a new type, if it isn't a bitfield, we add it to the resolved vec straight away,
        // since last_ty is still None, the next member that comes skips both of the block above and can either
        // start accumulating because it is a bitfield or is again just passed through to resolved
        if let Some(bits) = bitfield {
            // microsoft (https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170) says that only primitive types
            // can be bitfields, in practice this means that the type tokens will be just an ident
            let basetype = ty
                .try_only_basetype()
                .expect("Only a base raw integer can be a bitfield.");

            let s = &ctx.strings;
            let underlying = get_underlying_type(basetype, ctx).try_basetype().unwrap();
            let type_bits = switch!(underlying;
                s.uint8_t,  s.int8_t => 8;
                s.uint16_t, s.int16_t => 16;
                s.uint32_t, s.int32_t => 32;
                s.uint64_t, s.int64_t => 64;
                @ unimplemented!("Don't know the bit-width of '{}'", underlying)
            );

            assert!(*bits <= type_bits);

            max_bits = type_bits;
            current_bits = *bits;
            last_ty = Some(ty);
            merged_members.push(*name);
        } else {
            resolved.push((*name, ty));
        }
    }

    resolved
}

// jumps through as many aliases (Symbol::Alias) as needed and returns the resulting non-alias symbol,
// in cases where the provided identifier is not an alias it is immediatelly returned back
fn resolve_alias<'a>(alias: UniqueStr, reg: &'a Registry) -> (UniqueStr, &'a SymbolBody) {
    let mut next = alias;
    loop {
        let symbol = reg
            .get_symbol(next)
            .unwrap_or_else(|| panic!("'{}' Not in registry", { next }));
        match symbol {
            &SymbolBody::Alias(of) => {
                next = of;
            }
            _ => return (next, symbol),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnderlyingType {
    Basetype(UniqueStr),
    CString,
}

impl UnderlyingType {
    pub fn try_basetype(&self) -> Option<UniqueStr> {
        match self {
            UnderlyingType::Basetype(b) => Some(*b),
            UnderlyingType::CString => None,
        }
    }
}

impl ExtendedFormat for UnderlyingType {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        match self {
            UnderlyingType::Basetype(b) => import!(b).efmt(writer),
            UnderlyingType::CString => write!(writer, "&std::ffi::CStr"),
        }
    }
}

// Get the underlying type of a symbol.
// The difference between this and `resolve_alias()` is that this also jumps through "transparent" symbols, such as handles or constants.
fn get_underlying_type(name: UniqueStr, ctx: &Context) -> UnderlyingType {
    let mut symbol = name;
    loop {
        if is_std_type(symbol, &ctx) {
            return UnderlyingType::Basetype(symbol);
        }

        let top = ctx.get_symbol(symbol).unwrap();
        match top {
            SymbolBody::Alias(of) => symbol = *of,
            SymbolBody::Enum { ty, .. } => symbol = *ty,
            SymbolBody::Handle { .. } => symbol = ctx.strings.uint64_t, // the underlying type of all handles is this
            SymbolBody::Constant { ty, val } => {
                if let Some(ty) = ty {
                    symbol = *ty;
                } else {
                    match val {
                        ConstantValue::Bitpos(_) => unreachable!(),
                        ConstantValue::Literal(_) | ConstantValue::Expression(_) => {
                            return UnderlyingType::Basetype(ctx.strings.uint32_t)
                        }
                        ConstantValue::Symbol(s) => symbol = *s,
                        ConstantValue::String(_) => return UnderlyingType::CString,
                    }
                }
            }
            // really the only macros that are left are version constants so this is good enough for now
            SymbolBody::Define { .. } => return UnderlyingType::Basetype(ctx.strings.uint32_t),
            SymbolBody::Redeclaration(_)
            | SymbolBody::Basetype { .. }
            | SymbolBody::Included { .. }
            | SymbolBody::Struct { .. }
            | SymbolBody::Funcpointer { .. } => return UnderlyingType::Basetype(symbol),
            SymbolBody::Command { .. } => unreachable!(),
        }
    }
}

pub fn get_default_value<'a>(decl: &'a Declaration, ctx: &Context) -> &'a str {
    if decl.name == ctx.strings.sType
        && decl.ty == ctx.types.StructureType
        && !decl.metadata.values.is_empty()
    {
        decl.metadata.values.get(0).unwrap().resolve()
    } else {
        match decl.ty.first() {
            TyToken::Ref => unreachable!("Cannot default-construct a reference!"),
            TyToken::Ptr => {
                if let Some(TyToken::Const) = decl.ty.0.get(1) {
                    "std::ptr::null()"
                } else {
                    "std::ptr::null_mut()"
                }
            }
            TyToken::Array(_) => "std::mem::zeroed()",
            TyToken::BaseType(_) | TyToken::Const => "Default::default()",
        }
    }
}

/// Same functionality as `get_default_value` but uses a different type than the declaration's after not matching sType.
pub fn get_default_value_with_overlay<'a>(
    decl: &'a Declaration,
    ty: &TypeRef,
    ctx: &Context,
) -> &'a str {
    if decl.name == ctx.strings.sType
        && decl.ty == ctx.types.StructureType
        && !decl.metadata.values.is_empty()
    {
        decl.metadata.values.get(0).unwrap().resolve()
    } else {
        match ty.first() {
            TyToken::Ref => unreachable!("Cannot default-construct a reference!"),
            TyToken::Ptr => {
                if let Some(TyToken::Const) = decl.ty.0.get(1) {
                    "std::ptr::null()"
                } else {
                    "std::ptr::null_mut()"
                }
            }
            TyToken::Array(_) => "std::mem::zeroed()",
            TyToken::BaseType(_) | TyToken::Const => "Default::default()",
        }
    }
}
