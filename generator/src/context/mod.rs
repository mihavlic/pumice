pub mod ownership;
pub mod undangle;
pub mod workarounds;

use generator_lib::{
    configuration::GenConfig,
    interner::{Intern, UniqueStr},
    type_declaration::Type,
    DependsExpr, ExtensionKind, Registry, Symbol,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, Index, Range},
    rc::Rc,
};

use self::{ownership::resolve_ownership, undangle::undangle, workarounds::apply_workarounds};

pub trait TypedHandle {
    fn new(index: usize) -> Self;
    fn index(self) -> usize;
}

macro_rules! simple_handle {
    ($($visibility:vis $name:ident),+) => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            $visibility struct $name(u32);
            #[allow(unused)]
            impl TypedHandle for $name {
                fn new(index: usize) -> Self {
                    assert!(index <= u32::MAX as usize);
                    Self(index as u32)
                }
                #[inline]
                fn index(self) -> usize {
                    self.0 as usize
                }
            }
        )+
    };
}

pub struct HandleVec<H, T>(Vec<T>, PhantomData<H>);

impl<H: TypedHandle, T> HandleVec<H, T> {
    pub fn new() -> Self {
        HandleVec(Vec::new(), PhantomData)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        HandleVec(Vec::with_capacity(capacity), PhantomData)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, value: T) -> H {
        let len = self.0.len();
        self.0.push(value);
        H::new(len)
    }
    pub fn get(&self, index: H) -> Option<&T> {
        self.0.get(index.index())
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
    pub fn iter_kv(
        &self,
    ) -> impl Iterator<Item = (H, &T)> + Clone + ExactSizeIterator + DoubleEndedIterator {
        self.0.iter().enumerate().map(|(i, t)| (H::new(i), t))
    }
    pub fn iter_keys(
        &self,
    ) -> impl Iterator<Item = H> + Clone + ExactSizeIterator + DoubleEndedIterator {
        (0..self.len()).map(H::new)
    }
}

impl<H: TypedHandle, T> Index<H> for HandleVec<H, T> {
    type Output = T;
    fn index(&self, index: H) -> &Self::Output {
        &self.0[index.index()]
    }
}

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

simple_handle!(
    pub SectionHandle
);

pub struct SymbolMeta {
    pub owner: SectionHandle,
    pub depends: Option<Rc<DependsExpr>>,
}

pub struct Context {
    pub reg: Registry,
    pub conf: GenConfig,
    pub used_symbols: Vec<(usize, SectionHandle)>,
    pub symbol_ownership: HashMap<UniqueStr, SymbolMeta>,
    pub sections: HandleVec<SectionHandle, Section>,
    // however the ownership algorithm is order sensitive so we must preserve the order in which sections are declared in the registry
    // (section name, section's index in self.sections)
    pub section_map: HashMap<QualifiedSectionName, SectionHandle>,
    pub strings: CommonStrings,
    pub types: CommonTypes,
}

impl Context {
    pub fn new(reg: Registry, conf: GenConfig) -> Self {
        let sections_len = reg.features.len() + reg.extensions.len();

        let mut s = Self {
            used_symbols: Vec::new(),
            symbol_ownership: HashMap::new(),
            sections: HandleVec::with_capacity(sections_len),
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

            s.section_map
                .extend(s.sections.iter_kv().map(|(i, e)| (e.ident.clone(), i)));
        }

        // apply some manual fixes to the collected item soup
        apply_workarounds(&mut s);
        // assign a "section" (an extensions or core version) to every item in the registry
        resolve_ownership(&mut s);

        // all the symbols that we will be generating
        s.used_symbols = get_used_symbols(&s);

        // some aliases may point at symbols that are not getting generated due to the config
        // so we must propagate them to the aliases that are actually getting generated
        undangle(&mut s);

        s
    }
    pub fn get_section_by_name(&self, ident: QualifiedSectionName) -> Option<SectionHandle> {
        self.section_map.get(&ident).copied()
    }
    pub fn get_section_body_by_name(&self, ident: QualifiedSectionName) -> Option<&Section> {
        self.sections.get(self.get_section_by_name(ident)?)
    }
    pub fn get_symbol_section(&self, name: UniqueStr) -> Option<SectionHandle> {
        self.symbol_ownership.get(&name).map(|s| s.owner)
    }
    pub fn get_symbol_meta(&self, name: UniqueStr) -> Option<&SymbolMeta> {
        self.symbol_ownership.get(&name)
    }
    pub fn get_symbol_section_body(&self, name: UniqueStr) -> Option<&Section> {
        self.sections.get(self.get_symbol_section(name)?)
    }
    pub fn remove_symbol(&mut self, name: UniqueStr) {
        self.reg.remove_symbol(name);
        self.symbol_ownership.remove(&name);
    }
    pub fn register_section(&mut self, section: Section) {
        assert!(!self.section_map.contains_key(&section.ident));
        self.section_map.insert(
            section.ident.clone(),
            SectionHandle::new(self.sections.len()),
        );
        self.sections.push(section);
    }
    pub fn section_add_symbols(
        &mut self,
        ident: QualifiedSectionName,
        symbols: impl IntoIterator<Item = UniqueStr>,
    ) {
        let section = *self.section_map.get(&ident).unwrap();
        for symbol in symbols {
            assert!(!self.symbol_ownership.contains_key(&symbol));
            self.symbol_ownership.insert(
                symbol,
                SymbolMeta {
                    owner: section,
                    depends: None,
                },
            );
        }
    }
    pub fn section_add_symbols_str<'a, 'b>(
        &'a mut self,
        ident: QualifiedSectionName,
        symbols: impl IntoIterator<Item = &'b str>,
    ) {
        let section = *self.section_map.get(&ident).unwrap();
        for symbol in symbols {
            let symbol = symbol.intern(&self);
            assert!(!self.symbol_ownership.contains_key(&symbol));
            self.symbol_ownership.insert(
                symbol,
                SymbolMeta {
                    owner: section,
                    depends: None,
                },
            );
        }
    }
    pub fn create_section(&self, name: &str) -> QualifiedSectionName {
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
pub fn get_used_symbols(s: &Context) -> Vec<(usize, SectionHandle)> {
    let mut symbols = Vec::new();
    for (i, &Symbol(name, _)) in s.reg.symbols.iter().enumerate() {
        if let Some(section_handle) = s.get_symbol_section(name) {
            let section = &s.sections[section_handle];

            let used = match section.kind {
                SectionKind::Feature(_) => s.conf.is_feature_used(section.name()),
                SectionKind::Extension(i) => {
                    let e = &s.reg.extensions[i as usize];
                    e.kind != ExtensionKind::Disabled && s.conf.is_extension_used(section.name())
                }
                SectionKind::Path(_) => unimplemented!(),
            };

            if used {
                symbols.push((i, section_handle));
            }
        } else {
            log::warn!("[{name}] has no owner, skipping");
        }
    }
    symbols
}

pub type QualifiedSectionName = Range<UniqueStr>;

pub trait SectionFunctions {
    fn lib(&self) -> UniqueStr;
    fn name(&self) -> UniqueStr;
}

impl SectionFunctions for QualifiedSectionName {
    fn lib(&self) -> UniqueStr {
        self.start
    }
    fn name(&self) -> UniqueStr {
        self.end
    }
}

#[derive(Clone)]
pub enum SectionKind {
    /// A Vulkan core version
    Feature(u32),
    /// A Vulkan extension
    Extension(u32),
    /// A section with a custom path, for example the rust module with the function pointer tables
    Path(String),
}

pub struct Section {
    pub ident: QualifiedSectionName,
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
