pub mod ownership;
pub mod undangle;
pub mod workarounds;

use generator_lib::{
    configuration::GenConfig,
    interner::{Intern, UniqueStr},
    type_declaration::Type,
    ExtensionKind, Registry, Symbol,
};
use std::{
    collections::HashMap,
    ops::{Deref, Range},
};

use self::{ownership::resolve_ownership, undangle::undangle, workarounds::apply_workarounds};

macro_rules! common_strings {
    ($($string:ident),+ $(@ $($name:ident: $string2:literal),+)?) => {
        #[allow(non_snake_case)]
        pub struct CommonStrings {
            $(
                pub $string: UniqueStr,
            )+
            $(
                $(
                    pub $name: UniqueStr,
                )+
            )?
        }
        impl CommonStrings {
            fn new(int: &generator_lib::interner::Interner) -> Self {
                Self {
                    $(
                        $string: stringify!($string).intern(int),
                    )+
                    $(
                        $(
                            $name: $string2.intern(int),
                        )+
                    )?
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
    VkFormat,
    pumice,
    pNext, sType,
    // just some string that we can use as a placeholder that will have no collisions with existing strings
    __RESERVED_INVALID_PLACEHOLDER,
    // an edge case for cstring constants because we're using `std::ffi::CStr` for them and not `*const c_char`
    __cstring_constant_type,
    VK_VERSION_1_0,
    VK_SUCCESS
    @
    null_terminated: "null-terminated"
}

common_types! {
    cstring:  "const char *",
    void:     "void",
    void_ptr: "void *",
    void_const_ptr: "const void *",
    VkStructureType: "VkStructureType",
    VkResult: "VkResult",
    VulkanResult: "VulkanResult",
    VkBool32: "VkBool32",
    bool: "bool"
}

pub struct Context {
    pub reg: Registry,
    pub conf: GenConfig,
    pub symbols: Vec<(usize, u32)>,
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
            symbols: Vec::new(),
            symbol_ownership: HashMap::new(),
            sections: Vec::with_capacity(sections_len),
            section_map: HashMap::with_capacity(sections_len),
            strings: CommonStrings::new(&reg),
            types: CommonTypes::new(&reg),
            reg,
            conf,
        };

        {
            let pumice = s.strings.pumice;
            for (i, feature) in s.reg.features.iter().enumerate() {
                s.sections.push(Section {
                    ident: pumice..feature.name,
                    kind: SectionKind::Feature(i as u32),
                });
            }

            for (i, extension) in s.reg.extensions.iter().enumerate() {
                s.sections.push(Section {
                    ident: pumice..extension.name,
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

        // all the symbols that we will be generating
        s.symbols = get_used_symbols(&s);

        // some aliases may point at symbols that are not getting generated due to the config
        // so we must propagate them to the aliases that are actually getting generated
        undangle(&mut s);

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
    pub fn create_section(&self, name: &str) -> SectionIdent {
        self.strings.pumice..name.intern(self)
    }
}

impl Deref for Context {
    type Target = Registry;

    fn deref(&self) -> &Self::Target {
        &self.reg
    }
}

/// Vec<(symbol index, section index)>
pub fn get_used_symbols(s: &Context) -> Vec<(usize, u32)> {
    let mut symbols = Vec::new();
    for (i, &Symbol(name, _)) in s.reg.symbols.iter().enumerate() {
        if let Some(section_idx) = s.symbol_get_section_idx(name) {
            let section = &s.sections[section_idx as usize];

            let used = match section.kind {
                SectionKind::Feature(_) => s.conf.is_feature_used(section.name()),
                SectionKind::Extension(i) => {
                    let e = &s.reg.extensions[i as usize];
                    s.conf.is_extension_used(section.name()) && e.kind != ExtensionKind::Disabled
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
    symbols
}

pub type SectionIdent = Range<UniqueStr>;

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
