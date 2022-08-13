// vk.xml documentation
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    ops::Deref,
};

use configuration::GenConfig;
use foreach_uniquestr::ForeachUniquestr;
use interner::{Intern, Interner, UniqueStr};
use roxmltree::Node;
use type_declaration::{parse_type_decl, Type};

pub extern crate smallvec;

pub mod configuration;
pub mod foreach_uniquestr;
pub mod interner;
pub mod type_declaration;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    Alias,
    Included,
    Define,
    Basetype,
    Bitmask,
    Handle,
    Funcpointer,
    Struct,
    Constant,
    Enum,
    BitmaskBits,
    Command,
}

#[derive(Clone)]
pub enum RedeclarationMethod {
    Type(Type),
    Custom(fn(&mut dyn std::fmt::Write) -> std::fmt::Result),
}

impl Debug for RedeclarationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(arg0) => f.debug_tuple("Type").field(arg0).finish(),
            Self::Custom(_) => f.debug_tuple("Custom").field(&"opaque fn").finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol(pub UniqueStr, pub SymbolBody);

#[derive(Debug, Clone)]
pub enum SymbolBody {
    Alias(UniqueStr),
    Redeclaration(RedeclarationMethod),
    Included {
        header: UniqueStr,
    },
    Define {
        body: String,
    },
    Basetype {
        code: String,
    },
    Enum {
        bitmask: bool,
        ty: UniqueStr,
        members: Vec<(UniqueStr, ConstantValue)>,
    },
    Handle {
        object_type: UniqueStr,
        dispatchable: bool,
    },
    Funcpointer {
        ret: Type,
        args: Vec<(UniqueStr, Type)>,
    },
    Struct {
        union: bool,
        members: Vec<StructMember>,
    },
    Constant {
        val: ConstantValue,
        ty: Option<UniqueStr>,
    },
    Command {
        return_type: Type,
        params: Vec<CommandParameter>,
    },
}

#[derive(Debug, Clone)]
pub struct StructMember {
    pub name: UniqueStr,
    pub ty: Type,
    pub bitfield: Option<u8>,
}

#[derive(Debug)]
pub struct Feature {
    pub name: UniqueStr,
    pub api: Vec<UniqueStr>,
    pub major: u8,
    pub minor: u8,
    pub sortorder: Option<u32>,
    pub protect: Option<UniqueStr>,
    pub children: Vec<FeatureExtensionItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionKind {
    Disabled,
    Instance,
    Device,
    /// the "extension" vulkan_video_codecs_common gets this
    None,
}

// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_attributes_of_extension_tags
#[derive(Debug, Clone)]
pub struct Extension {
    pub name: UniqueStr,
    pub number: u32,
    pub sortorder: Option<u32>,
    pub author: Option<String>,
    pub contact: Option<String>,

    pub kind: ExtensionKind,
    pub requires: Vec<UniqueStr>,
    pub requires_core: Option<UniqueStr>,
    pub protect: Option<UniqueStr>,
    pub platform: Option<UniqueStr>,
    pub supported: Vec<UniqueStr>,
    pub promotedto: Option<UniqueStr>,
    pub deprecatedby: Option<UniqueStr>,
    pub obsoletedby: Option<UniqueStr>,
    pub provisional: bool,
    pub specialuse: Vec<UniqueStr>,
    pub children: Vec<FeatureExtensionItem>,
}

#[derive(Debug)]
pub enum YCBREncoding {
    E420,
    E422,
    E444,
}

#[derive(Debug)]
pub enum NumericFormat {
    SFLOAT,
    SINT,
    SNORM,
    SRGB,
    SSCALED,
    UFLOAT,
    UINT,
    UNORM,
    USCALED,
}

#[derive(Debug)]
pub struct Component {
    pub name: UniqueStr,
    pub bits: Option<u8>, // is none for "compressed"
    pub numeric_format: NumericFormat,
    pub plane_index: Option<u8>,
}

#[derive(Debug)]
pub struct Plane {
    pub index: u8,
    pub width_divisor: u8,
    pub height_divisor: u8,
    pub compatible: UniqueStr,
}

#[derive(Debug)]
pub struct Format {
    pub name: UniqueStr,
    pub class: UniqueStr,
    pub blocksize: u8,
    pub texels_per_block: u8,
    pub block_extent: Option<[u8; 3]>,
    pub packed: Option<u8>,
    pub compressed: Option<UniqueStr>,
    pub chroma: Option<YCBREncoding>,

    pub components: Vec<Component>,
    pub planes: Vec<Plane>,
    pub spirvimageformats: Vec<UniqueStr>,
}

#[derive(Debug, Clone)]
pub struct CommandParameter {
    pub name: UniqueStr,
    pub len: Option<String>,
    pub alt_len: Option<String>,
    pub optional: bool,
    pub externsync: Option<String>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum FeatureExtensionItem {
    Comment(String),
    // consider splitting this off
    Require {
        profile: Option<UniqueStr>,
        // The api attribute is only supported inside extension tags, since feature tags already define a specific API.
        api: Vec<UniqueStr>,
        // The extension attribute currently does not affect output generators in any way, and is simply metadata. This will be addressed as we better define different types of dependencies between extensions.
        // bruh
        extension: Option<UniqueStr>,
        feature: Option<UniqueStr>,
        items: Vec<InterfaceItem>,
    },
    Remove {
        profile: Option<UniqueStr>,
        // The api attribute is only supported inside extension tags, since feature tags already define a specific API.
        api: Vec<UniqueStr>,
        // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_enum_tags
        // the item removed will always be `InterfaceItem::Simple`, api property that can be in <enum> is not applicable I think?
        // https://github.com/osor-io/Vulkan/blob/e4bc1b9125645f3db3c8342edc24d81dc497f252/generate_code/generate_vulkan_code.jai#L1586
        // here it seems that the author doesn't handle `api`
        items: Vec<UniqueStr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstantValue {
    Bitpos(u32),
    Literal(i64),
    Expression(String),
    Symbol(UniqueStr),
    String(String),
}

#[derive(Debug, Clone)]
pub enum InterfaceItem {
    // just importing already finished items
    // for some reason Enums can have an Api condition
    // even though it can already be scoped by the parent <require>, why??
    Simple {
        name: UniqueStr,
    },
    Extend {
        name: UniqueStr,
        extends: UniqueStr,
        value: ConstantValue,
    },
}

// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#tag-spirvenable
#[derive(Debug)]
pub enum SpirvEnable {
    Version(UniqueStr),
    Extension(UniqueStr),
    Feature {
        structure: UniqueStr,
        feature: UniqueStr,
        requires: Vec<UniqueStr>,
        alias: Option<UniqueStr>,
    },
    Property {
        property: UniqueStr,
        member: UniqueStr,
        value: UniqueStr,
        requires: Vec<UniqueStr>,
    },
}

#[derive(Debug)]
pub struct SpirvExtCap {
    pub name: UniqueStr,
    pub enables: Vec<SpirvEnable>,
}

#[derive(Debug)]
pub struct Platform {
    pub name: UniqueStr,
    pub protect: UniqueStr,
    pub comment: String,
}

#[derive(Debug)]
pub struct Tag {
    pub name: UniqueStr,
    pub author: String,
    pub contact: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemKind {
    Symbol,
    Feature,
    Extension,
    Format,
    SpirvCapability,
    SpirvExtension,
}

impl Deref for Registry {
    type Target = Interner;

    fn deref(&self) -> &Self::Target {
        &self.interner
    }
}

pub struct Registry {
    pub platforms: Vec<Platform>,
    pub tags: Vec<Tag>,
    pub headers: Vec<UniqueStr>,

    pub symbols: Vec<Symbol>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
    pub formats: Vec<Format>,
    pub spirv_capabilities: Vec<SpirvExtCap>,
    pub spirv_extensions: Vec<SpirvExtCap>,

    pub item_map: HashMap<UniqueStr, (u32, ItemKind)>,
    pub interner: Interner,
    pub flag_bits_to_flags: HashMap<UniqueStr, (UniqueStr, UniqueStr)>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            platforms: Default::default(),
            tags: Default::default(),
            headers: Default::default(),

            symbols: Default::default(),
            features: Default::default(),
            extensions: Default::default(),
            formats: Default::default(),
            spirv_capabilities: Default::default(),
            spirv_extensions: Default::default(),

            item_map: Default::default(),
            interner: Interner::new(),
            flag_bits_to_flags: Default::default(),
        }
    }
    pub fn get_symbol_index(&self, name: UniqueStr) -> Option<u32> {
        let &(index, ty) = self.item_map.get(&name)?;
        if ty != ItemKind::Symbol {
            return None;
        }
        Some(index)
    }
    pub fn get_symbol(&self, name: UniqueStr) -> Option<&SymbolBody> {
        let item = self.get_symbol_index(name)?;
        Some(&self.symbols[item as usize].1)
    }
    pub fn get_feature_index(&self, name: UniqueStr) -> Option<u32> {
        let &(index, ty) = self.item_map.get(&name)?;
        if ty != ItemKind::Feature {
            return None;
        }
        Some(index)
    }
    pub fn get_feature(&self, name: UniqueStr) -> Option<&Feature> {
        let item = self.get_feature_index(name)?;
        Some(&self.features[item as usize])
    }
    pub fn get_extension_index(&self, name: UniqueStr) -> Option<u32> {
        let &(index, ty) = self.item_map.get(&name)?;
        if ty != ItemKind::Extension {
            return None;
        }
        Some(index)
    }
    pub fn get_extension(&self, name: UniqueStr) -> Option<&Extension> {
        let item = self.get_extension_index(name)?;
        Some(&self.extensions[item as usize])
    }
    pub fn add_symbol(&mut self, name: UniqueStr, body: SymbolBody) {
        let entry = self.item_map.entry(name);
        match entry {
            Entry::Occupied(o) => {
                let &(index, kind) = o.get();
                if kind == ItemKind::Symbol {
                    match self.symbols[index as usize].1 {
                        // QUIRK we allow overwriting `Included` items because those contained in video.xml are declared as such in vk.xml
                        // and their actual definition occurs later in video.xml
                        SymbolBody::Included { .. } => {
                            self.symbols[index as usize].1 = body;
                            return;
                        }
                        _ => {}
                    }
                }
                panic!(
                    "Attempt to insert duplicate item '{}' into registry.",
                    name.resolve()
                )
            }
            Entry::Vacant(v) => {
                let index = u32::try_from(self.symbols.len()).unwrap();
                v.insert((index, ItemKind::Symbol));
                self.symbols.push(Symbol(name, body));
            }
        }
    }
    pub fn add_feature(&mut self, add: Feature) {
        add_impl(
            &mut self.features,
            add.name,
            add,
            ItemKind::Feature,
            &mut self.item_map,
        );
    }
    pub fn add_extension(&mut self, add: Extension) {
        add_impl(
            &mut self.extensions,
            add.name,
            add,
            ItemKind::Extension,
            &mut self.item_map,
        );
    }
    pub fn add_format(&mut self, add: Format) {
        add_impl(
            &mut self.formats,
            add.name,
            add,
            ItemKind::Format,
            &mut self.item_map,
        );
    }
    pub fn add_spirv_capability(&mut self, add: SpirvExtCap) {
        add_impl(
            &mut self.spirv_capabilities,
            add.name,
            add,
            ItemKind::SpirvCapability,
            &mut self.item_map,
        );
    }
    pub fn add_spirv_extension(&mut self, add: SpirvExtCap) {
        add_impl(
            &mut self.spirv_extensions,
            add.name,
            add,
            ItemKind::SpirvExtension,
            &mut self.item_map,
        );
    }
    pub fn remove_symbol(&mut self, name: UniqueStr) {
        let (index, kind) = self.item_map.remove(&name).unwrap();
        assert_eq!(kind, ItemKind::Symbol);

        self.symbols.remove(index as usize);

        // need to adjust all the following indexes in the item_map because we've just deleted an element
        for Symbol(name, _) in &self.symbols[index as usize..] {
            self.item_map.get_mut(name).unwrap().0 -= 1;
        }
    }
    pub fn get_item_entry(&self, name: UniqueStr) -> Option<&(u32, ItemKind)> {
        self.item_map.get(&name)
    }
    pub fn finalize(&mut self) {
        let mut fun = |str: &mut UniqueStr| {
            if let Some((flags, _)) = self.flag_bits_to_flags.get(&*str) {
                *str = *flags;
            }
        };
        self.symbols.foreach(&mut fun);
        self.features.foreach(&mut fun);
        self.extensions.foreach(&mut fun);
    }
}

fn add_impl<T>(
    vec: &mut Vec<T>,
    name: UniqueStr,
    what: T,
    kind: ItemKind,
    item_map: &mut HashMap<UniqueStr, (u32, ItemKind)>,
) {
    let index = u32::try_from(vec.len()).unwrap();
    let none = item_map.insert(name, (index, kind));
    assert!(
        none.is_none(),
        "Attempt to insert duplicate item '{}' into registry.",
        name.resolve()
    );
    vec.push(what);
}

impl Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("platforms", &self.platforms)
            .field("tags", &self.tags)
            .field("headers", &self.headers)
            .field("symbols", &self.symbols)
            .field("features", &self.features)
            .field("extensions", &self.extensions)
            .field("formats", &self.formats)
            .field("spirv_capabilities", &self.spirv_capabilities)
            .field("spirv_extensions", &self.spirv_extensions)
            .finish()
    }
}

trait NodeUtils<'a> {
    fn intern(&self, attribute: &str, int: &Interner) -> UniqueStr;
    fn owned(&self, attribute: &str) -> String;
    fn get(&'a self, attribute: &str) -> &'a str;
    fn child_text(&'a self, child: &str) -> &'a str;
    fn try_child_text(&'a self, child: &str) -> Option<&'a str>;
}

impl<'a, 'b> NodeUtils<'b> for Node<'a, 'b> {
    fn intern(&self, attribute: &str, int: &Interner) -> UniqueStr {
        self.attribute(attribute).unwrap().intern(int)
    }
    fn owned(&self, attribute: &str) -> String {
        self.attribute(attribute).unwrap().to_owned()
    }
    fn get(&'a self, attribute: &str) -> &'a str {
        self.attribute(attribute).unwrap()
    }
    fn child_text(&'a self, child: &str) -> &'a str {
        node_iter_children(*self)
            .find(|n| n.tag_name().name() == child)
            .unwrap()
            .text()
            .unwrap()
    }
    fn try_child_text(&'a self, child: &str) -> Option<&'a str> {
        Some(
            node_iter_children(*self)
                .find(|n| n.tag_name().name() == child)?
                .text()
                .unwrap(),
        )
    }
}

// the kind must be a closure so that it may be lazily evaluated as in some cases it needs to
//  do some computation that would be invalid if the node is not an alias in the first place
fn try_alias(t: Node, reg: &mut Registry) -> bool {
    if let Some(alias) = t.attribute("alias") {
        reg.add_symbol(t.intern("name", reg), SymbolBody::Alias(alias.intern(reg)));

        return true;
    }

    return false;
}

fn node_iter_children<'a, 'b: 'a>(node: Node<'a, 'b>) -> impl Iterator<Item = Node<'a, 'b>> {
    // nice and readable
    node.first_child().into_iter().flat_map(|c| {
        c.next_siblings()
            .filter(|n| !n.tag_name().name().is_empty())
    })
}

fn node_collect_text(n: Node, buf: &mut String) {
    for d in n.children() {
        // struct member definitions contain comments, we don't want these
        if d.tag_name().name() != "comment" {
            d.text().map(|s| {
                // there may not be any whitespace in the text within the separate elements
                // as a heuristic to preserve formatting, this is only applied if the split
                // is between two text characters as otherwise it likely doesn't matter
                if buf.ends_with(|c: char| c.is_ascii_alphanumeric())
                    && s.starts_with(|c: char| c.is_ascii_alphanumeric())
                {
                    buf.push(' ');
                }

                buf.push_str(s)
            });
        }
    }
}

fn api_guard_skip(node: Node, conf: Option<&GenConfig>, int: &Interner) -> bool {
    if let Some(conf) = conf {
        if let Some(str) = node.attribute("api") {
            for api in iter_comma_separated(str, int) {
                if conf.is_api_used(api) {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

fn try_skip(n: Node, conf: Option<&GenConfig>, int: &Interner) -> bool {
    if let Some(conf) = conf {
        let apis = n.attribute("api").or_else(|| n.attribute("supported"));
        if let Some(str) = apis {
            let mut tmp = false;
            for api in iter_comma_separated(str, int) {
                if conf.is_api_used(api) {
                    tmp = true;
                    break;
                }
            }
            if tmp == false {
                return true;
            }
        }

        if let Some(protect) = n.attribute("protect").map(|s| s.intern(int)) {
            if !conf.is_protect_used(protect) {
                return true;
            }
        }

        if let Some(profile) = n.attribute("profile").map(|s| s.intern(int)) {
            if !conf.is_profile_used(profile) {
                return true;
            }
        }

        let tag_name = n.tag_name().name();
        if tag_name == "feature" {
            let name = n.intern("name", int);
            if !conf.is_feature_used(name) {
                return true;
            }
        } else if tag_name == "extension" {
            let name = n.intern("name", int);
            if !conf.is_extension_used(name) {
                return true;
            }
        }
    }
    false
}

pub fn process_registry_xml(reg: &mut Registry, xml: &str, conf: Option<&GenConfig>) {
    let doc = roxmltree::Document::parse(xml).unwrap();

    let r = doc
        .descendants()
        .find(|n| n.has_tag_name("registry"))
        .unwrap();

    // quite a few places require a string to collect some text into and then further process it
    let mut buf = String::new();

    for n in node_iter_children(r) {
        match n.tag_name().name() {
            "comment" => {} // TODO propagate this information
            "platforms" => {
                for c in node_iter_children(n) {
                    let p = Platform {
                        name: c.intern("name", reg),
                        protect: c.intern("protect", reg),
                        comment: c.owned("comment"),
                    };
                    reg.platforms.push(p);
                }
            }
            "tags" => {
                for c in node_iter_children(n) {
                    let t = Tag {
                        name: c.intern("name", reg),
                        author: c.owned("author"),
                        contact: c.owned("contact"),
                    };
                    reg.tags.push(t);
                }
            }
            "types" => {
                for t in node_iter_children(n) {
                    if t.tag_name().name() == "comment" {
                        continue;
                    }

                    if api_guard_skip(t, conf, reg) {
                        continue;
                    }

                    if try_alias(t, reg) {
                        continue;
                    }

                    match t.attribute("category") {
                        // <type name="vk_platform" category="include">#include "vk_platform.h"</type>
                        // <type category="include" name="X11/Xlib.h"/>
                        Some("include") => {
                            let name = t.get("name");
                            if name.contains(".h") {
                                reg.headers.push(name.intern(reg));
                            } else {
                                // #include "vk_platform.h"
                                // #include <stdint.h>
                                let code = t.text().unwrap();
                                let name = code.split_terminator('"').nth(1).unwrap_or_else(|| {
                                    code.split_terminator('<')
                                        .nth(1)
                                        .unwrap()
                                        .trim_end_matches('>')
                                });
                                reg.headers.push(name.intern(reg));
                            }
                        }
                        // <type requires="X11/Xlib.h" name="Display"/>
                        // <type requires="vk_platform" name="void"/>
                        // <type name="int"/>
                        None => {
                            let name = t.get("name");

                            // QUIRK there a plain '<type name="int"/>' in the registry
                            // this doesn't really map to any item declaration, just skip it
                            if name == "int" {
                                continue;
                            }

                            reg.add_symbol(
                                name.intern(reg),
                                SymbolBody::Included {
                                    header: t.intern("requires", reg),
                                },
                            );
                        }
                        // <type category="define">
                        //   #define
                        //   <name>VK_MAKE_API_VERSION</name>
                        //   (variant, major, minor, patch) \ ((((uint32_t)(variant)) << 29) | (((uint32_t)(major)) << 22) | (((uint32_t)(minor)) << 12) | ((uint32_t)(patch)))
                        // </type>
                        // <type category="define" requires="VK_NULL_HANDLE" name="VK_DEFINE_NON_DISPATCHABLE_HANDLE">
                        //   #ifndef VK_DEFINE_NON_DISPATCHABLE_HANDLE #if (VK_USE_64_BIT_PTR_DEFINES==1) #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef pub struct object##_T *object; #else #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef uint64_t object; #endif #endif
                        // </type>
                        Some("define") => {
                            // the name is either specified as an attribute or delimited with an element within the contained text
                            let name = t
                                .attribute("name")
                                .unwrap_or_else(|| t.child_text("name"))
                                .intern(reg);

                            let body = {
                                buf.clear();
                                node_collect_text(t, &mut buf);
                                buf.trim().to_owned()
                            };

                            reg.add_symbol(name, SymbolBody::Define { body });
                        }
                        // <type category="basetype">struct <name>ANativeWindow</name>;</type>
                        // <type category="basetype">typedef <type>uint32_t</type> <name>VkSampleMask</name>;</type>
                        Some("basetype") => {
                            let name = t.child_text("name").intern(reg);
                            let ty = t.try_child_text("type");

                            // QUIRK
                            // the `basetype` tag may contain a:
                            //   typedef
                            //   struct pre declaration
                            //   preprocessor code
                            // currently the first option is mapped to an alias, while the third is preserved as a basetype
                            let body = if let Some(ty) = ty {
                                SymbolBody::Alias(ty.intern(reg))
                            } else {
                                buf.clear();
                                node_collect_text(t, &mut buf);

                                SymbolBody::Basetype {
                                    code: buf.trim().to_owned(),
                                }
                            };

                            reg.add_symbol(name, body);
                        }
                        // <type requires="VkFramebufferCreateFlagBits" category="bitmask">
                        //   typedef
                        //   <type>VkFlags</type>
                        //   <name>VkFramebufferCreateFlags</name>
                        //   ;
                        // </type>
                        // <type category="bitmask" name="VkGeometryFlagsNV" alias="VkGeometryFlagsKHR"/>
                        Some("bitmask") => {
                            // due to vk.xml being a horror produced from C headers
                            // bitmasks are actually an enum with the values and a typedef of the actual integer that is passed around vulkan
                            // as such, this element is for the integer typedef and the values pub enum is listed as a
                            // 'requires' when it is a 32 bit bitmask or in 'bitvalues' when it is 64 bit
                            // the bits can be missing if it the flags exist but are so far unused
                            let bits = if let Some(req) = t.attribute("requires") {
                                Some(req)
                            } else if let Some(req) = t.attribute("bitvalues") {
                                Some(req)
                            } else {
                                None
                            };

                            let name = t.child_text("name").intern(reg);
                            let ty = t.child_text("type").intern(reg);

                            if let Some(bits) = bits {
                                reg.flag_bits_to_flags.insert(bits.intern(&reg), (name, ty));
                            } else {
                                // QUIRK bitmasks are divided into two parts:
                                // a "bitmask" element and a "bitmaskbits" element, the latter of which specifies the variants in the enum
                                // and may be missing if there are none, as such we add the symbol here only if `bits` is None, because otherwise we should encounter
                                // the actually useful definition later
                                reg.add_symbol(
                                    name,
                                    SymbolBody::Enum {
                                        bitmask: true,
                                        ty: ty,
                                        members: Vec::new(),
                                    },
                                );
                            }
                        }
                        // <type category="handle" objtypeenum="VK_OBJECT_TYPE_INSTANCE"><type>VK_DEFINE_HANDLE</type>(<name>VkInstance</name>)</type>
                        // <type category="handle" name="VkDescriptorUpdateTemplateKHR" alias="VkDescriptorUpdateTemplate"/>
                        Some("handle") => {
                            let object_type = t.intern("objtypeenum", reg);

                            let kind = t.child_text("type");
                            let dispatchable = match kind {
                                "VK_DEFINE_HANDLE" => true,
                                "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => false,
                                _ => unreachable!(),
                            };

                            reg.add_symbol(
                                t.child_text("name").intern(reg),
                                SymbolBody::Handle {
                                    object_type,
                                    dispatchable,
                                },
                            );
                        }
                        // <type name="VkAttachmentLoadOp" category="enum"/>
                        // <type category="enum" name="VkPrivateDataSlotCreateFlagBitsEXT" alias="VkPrivateDataSlotCreateFlagBits"/>
                        Some("enum") => {
                            // the alias variant has already been handled above, if it's not an alias we skip
                            // the node as it is essentially just a C prototype and the actual definition is later on
                        }
                        // <type category="funcpointer">
                        //   typedef void (VKAPI_PTR *<name>PFN_vkInternalAllocationNotification</name>)(
                        //     <type>void</type>*                      pUserData,
                        //     <type>size_t</type>                     size,
                        //     <type>VkInternalAllocationType</type>   allocationType,
                        //     <type>VkSystemAllocationScope</type>    allocationScope
                        //   );
                        // </type>
                        Some("funcpointer") => {
                            // must parse C code, thanks khronos
                            // all 9 instances of this follow this structure:
                            // typedef 'return type' (VKAPI_PTR *'ptr type')($('argument type'),*)

                            // since the incredible way that the xml is structured, pointer syntax and const decorations are not specially marked
                            // thus it actually becomes easier to collect all the text into a String and parse it manually

                            let mut brackets = {
                                buf.clear();
                                node_collect_text(t, &mut buf);
                                buf.split('(')
                            };

                            let return_type = {
                                let first = brackets.next().unwrap();
                                let clean = first.trim_start_matches("typedef");
                                parse_type_decl(clean, reg).1
                            };

                            // waste the second parenthesis
                            brackets.next().unwrap();

                            let args_text = brackets
                                .next()
                                .unwrap()
                                .trim_end()
                                .trim_end_matches(&[')', ';']);

                            let mut args = Vec::new();
                            for arg in args_text.split(',') {
                                // QUIRK
                                //   the signature `typedef void (VKAPI_PTR *PFN_vkVoidFunction)(void);`
                                //   has no arguments yet we try to parse them, for this one case we check the argument text

                                if arg == "void" {
                                    continue;
                                }

                                let (name, ty, _) = parse_type_decl(&arg, &reg);
                                args.push((name.unwrap(), ty));
                            }

                            reg.add_symbol(
                                t.child_text("name").intern(reg),
                                SymbolBody::Funcpointer {
                                    ret: return_type,
                                    args,
                                },
                            );
                        }
                        // <type category="struct" name="VkBaseOutStructure">
                        //   <member>
                        //    <type>VkStructureType</type>
                        //    <name>sType</name>
                        //   </member>
                        //   <member optional="true">
                        //    struct
                        //    <type>VkBaseOutStructure</type>
                        //    *
                        //    <name>pNext</name>
                        //   </member>
                        // </type>
                        // <type category="struct" name="VkImageStencilUsageCreateInfoEXT" alias="VkImageStencilUsageCreateInfo"/>
                        category @ Some("struct" | "union") => {
                            let mut members = Vec::new();

                            for member in node_iter_children(t) {
                                match member.tag_name().name() {
                                    "member" => {}
                                    "comment" => continue,
                                    _ => unreachable!(),
                                }

                                if api_guard_skip(member, conf, reg) {
                                    continue;
                                }

                                buf.clear();
                                node_collect_text(member, &mut buf);
                                let (name, ty, bitfield) = parse_type_decl(&buf, reg);

                                members.push(StructMember {
                                    name: name.unwrap(),
                                    ty,
                                    bitfield,
                                });
                            }

                            reg.add_symbol(
                                t.intern("name", reg),
                                SymbolBody::Struct {
                                    union: category == Some("union"),
                                    members,
                                },
                            );
                        }
                        _ => {}
                    }
                }
            }
            "enums" => {
                match n.attribute("type") {
                    // <enums name="API Constants" comment="Vulkan hardcoded constants - not an enumerated type, part of the header boilerplate">
                    None => {
                        for e in node_iter_children(n) {
                            assert_eq!(e.tag_name().name(), "enum");
                            // <enum name="VK_LUID_SIZE_KHR" alias="VK_LUID_SIZE"/>
                            if try_alias(e, reg) {
                                continue;
                            }

                            // // <enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                            let val = guess_constant_val(e.get("value"), reg);
                            let ty = e.get("type").intern(reg);

                            reg.add_symbol(
                                e.intern("name", reg),
                                SymbolBody::Constant { val, ty: Some(ty) },
                            );
                        }
                    }
                    // <enums name="VkImageLayout" type="enum">
                    Some(category @ ("enum" | "bitmask")) => {
                        assert_eq!(n.tag_name().name(), "enums");
                        let mut members = Vec::new();
                        for e in node_iter_children(n) {
                            match e.tag_name().name() {
                                "enum" => {}
                                "comment" | "unused" => continue, // I don't think tracking unused values is ever useful
                                _ => unreachable!(),
                            }

                            if api_guard_skip(e, conf, reg) {
                                continue;
                            }

                            let variant_name = e.intern("name", reg);

                            // <enum value="0" name="VK_IMAGE_LAYOUT_UNDEFINED" comment="..."/>
                            // <enum bitpos="0" name="VK_QUEUE_GRAPHICS_BIT" comment="..."/>
                            // <enum name="VK_STENCIL_FRONT_AND_BACK" alias="VK_STENCIL_FACE_FRONT_AND_BACK"/>
                            let val = if let Some(value) = e.attribute("value") {
                                ConstantValue::Literal(parse_detect_radix(value) as i64)
                            } else if let Some(bitpos) = e.attribute("bitpos") {
                                ConstantValue::Bitpos(bitpos.parse().unwrap())
                            } else if let Some(alias) = e.attribute("alias") {
                                ConstantValue::Symbol(alias.intern(reg))
                            } else {
                                unreachable!()
                            };

                            members.push((variant_name, val));
                        }

                        let bitmask = category == "bitmask";

                        let mut name = n.intern("name", reg);
                        let mut ty = None;

                        // see the comment in the match case for <types category="bitmask">

                        if bitmask {
                            if let Some(&(flags_name, flags_ty)) = reg.flag_bits_to_flags.get(&name)
                            {
                                name = flags_name;
                                ty = Some(flags_ty);
                            } else {
                                assert!(members.is_empty());
                                continue;
                            }
                        }

                        reg.add_symbol(
                            name,
                            SymbolBody::Enum {
                                // in C it isn't really clear what type backs an enum without futher constraints,
                                // however vulkan never crosses i32::max and reddit says that it's indeed i32, erupt also uses i32
                                //   https://www.reddit.com/r/vulkan/comments/4710rm/comment/d09gprb/
                                //   https://github.com/KhronosGroup/Vulkan-Docs/issues/124#issuecomment-192878892
                                ty: ty.unwrap_or_else(|| "int32_t".intern(&reg)),
                                bitmask,
                                members,
                            },
                        );
                    }
                    _ => unimplemented!(),
                }
            }
            "commands" => {
                for c in node_iter_children(n) {
                    assert_eq!(c.tag_name().name(), "command");

                    if api_guard_skip(c, conf, reg) {
                        continue;
                    }

                    // <command name="vkResetQueryPoolEXT" alias="vkResetQueryPool"/>
                    if try_alias(c, reg) {
                        continue;
                    }

                    // <command successcodes="VK_SUCCESS" errorcodes="VK_ERROR_OUT_OF_HOST_MEMORY,VK_ERROR_OUT_OF_DEVICE_MEMORY,VK_ERROR_INITIALIZATION_FAILED,VK_ERROR_LAYER_NOT_PRESENT,VK_ERROR_EXTENSION_NOT_PRESENT,VK_ERROR_INCOMPATIBLE_DRIVER">
                    //     <proto><type>VkResult</type> <name>vkCreateInstance</name></proto>
                    //     <param>const <type>VkInstanceCreateInfo</type>* <name>pCreateInfo</name></param>
                    //     <param optional="true">const <type>VkAllocationCallbacks</type>* <name>pAllocator</name></param>
                    //     <param><type>VkInstance</type>* <name>pInstance</name></param>
                    // </command>
                    let mut children = node_iter_children(c);

                    let (name, return_type) = {
                        let proto = children.next().unwrap();

                        buf.clear();
                        node_collect_text(proto, &mut buf);

                        let (name, ty, _) = parse_type_decl(&buf, reg);
                        (name.unwrap(), ty)
                    };

                    let mut params = Vec::new();
                    for p in children {
                        match p.tag_name().name() {
                            "param" => {}
                            "implicitexternsyncparams" => continue,
                            _ => unreachable!(),
                        }

                        if api_guard_skip(p, conf, reg) {
                            continue;
                        }

                        buf.clear();
                        node_collect_text(p, &mut buf);
                        let (name, ty, _) = parse_type_decl(&buf, reg);

                        params.push(CommandParameter {
                            name: name.unwrap(),
                            len: p.attribute("len").map(|s| s.to_owned()),
                            alt_len: p.attribute("altlen").map(|s| s.to_owned()),
                            // FIXME this can be a comma separated list, help
                            optional: p
                                .attribute("optional")
                                .map(|s| s.starts_with("true"))
                                .unwrap_or(false),
                            externsync: p.attribute("externsync").map(|s| s.to_owned()),
                            ty,
                        })
                    }

                    reg.add_symbol(
                        name,
                        SymbolBody::Command {
                            return_type,
                            params,
                        },
                    );
                }
            }
            // <feature api="vulkan" name="VK_VERSION_1_0" number="1.0" comment="Vulkan core API interface definitions">
            //     <require comment="Header boilerplate">
            //         <type name="vk_platform"/>
            //         <type name="VK_DEFINE_HANDLE"/>
            //         <type name="VK_USE_64_BIT_PTR_DEFINES"/>
            //         <type name="VK_DEFINE_NON_DISPATCHABLE_HANDLE"/>
            //         <type name="VK_NULL_HANDLE"/>
            //     </require>
            "feature" => {
                if try_skip(n, conf, reg) {
                    continue;
                }

                let mut number = n.get("number").split('.');

                let children = convert_section_children(n, None, conf, reg);
                reg.add_feature(Feature {
                    name: n.intern("name", reg),
                    api: parse_comma_separated(n.attribute("api"), reg),
                    major: number.next().unwrap().parse().unwrap(),
                    minor: number.next().unwrap().parse().unwrap(),
                    sortorder: n.attribute("sortorder").map(|s| s.parse().unwrap()),
                    protect: n.attribute("protect").map(|s| s.intern(reg)),
                    children,
                });
            }
            // <extensions comment="Vulkan extension interface definitions">
            //     <extension name="VK_KHR_surface" number="1" type="instance" author="KHR" contact="James Jones @cubanismo,Ian Elliott @ianelliottus" supported="vulkan">
            //         <require>
            //             <enum value="25"                                                name="VK_KHR_SURFACE_SPEC_VERSION"/>
            //             <enum value="&quot;VK_KHR_surface&quot;"                        name="VK_KHR_SURFACE_EXTENSION_NAME"/>
            //             <enum offset="0" extends="VkResult" dir="-"                     name="VK_ERROR_SURFACE_LOST_KHR"/>
            //             <enum offset="1" extends="VkResult" dir="-"                     name="VK_ERROR_NATIVE_WINDOW_IN_USE_KHR"/>
            //             <enum offset="0" extends="VkObjectType"                         name="VK_OBJECT_TYPE_SURFACE_KHR"/>
            //             <type name="VkSurfaceKHR"/>
            //             <type name="VkSurfaceTransformFlagBitsKHR"/>
            //             <type name="VkPresentModeKHR"/>
            "extensions" => {
                for e in node_iter_children(n) {
                    if try_skip(n, conf, reg) {
                        continue;
                    }

                    let name = e.intern("name", reg);
                    let protect = n.attribute("protect").map(|s| s.intern(reg));

                    let extnumber = e.attribute("number").map(|s| s.parse().unwrap());
                    let children = convert_section_children(e, extnumber, conf, reg);

                    let mut supported = Vec::new();
                    let kind = loop {
                        if let Some(s) = e.attribute("supported") {
                            if s == "disabled" {
                                break ExtensionKind::Disabled;
                            } else {
                                supported = parse_comma_separated(Some(s), &reg);
                            }
                        }

                        match e.attribute("type") {
                            Some("instance") => break ExtensionKind::Instance,
                            Some("device") => break ExtensionKind::Device,
                            _ => break ExtensionKind::None,
                        };
                    };

                    reg.add_extension(Extension {
                        name,
                        // video.xml does not have this number yay!
                        number: extnumber.unwrap_or(0),
                        sortorder: e.attribute("sortorder").map(|s| s.parse().unwrap()),
                        author: e.attribute("author").map(|s| s.to_owned()),
                        contact: e.attribute("contact").map(|s| s.to_owned()),
                        kind,
                        requires: parse_comma_separated(e.attribute("requires"), reg),
                        requires_core: e.attribute("requiresCore").map(|s| s.intern(reg)),
                        protect,
                        platform: e.attribute("platform").map(|s| s.intern(reg)),
                        supported,
                        promotedto: e.attribute("promotedto").map(|s| s.intern(reg)),
                        deprecatedby: e.attribute("deprecatedby").map(|s| s.intern(reg)),
                        obsoletedby: e.attribute("obsoletedby").map(|s| s.intern(reg)),
                        provisional: e.attribute("provisional") == Some("true"),
                        specialuse: parse_comma_separated(e.attribute("specialuse"), reg),
                        children,
                    });
                }
            }
            // <formats>
            //     <format name="VK_FORMAT_R4G4_UNORM_PACK8" class="8-bit" blockSize="1" texelsPerBlock="1" packed="8">
            //         <component name="R" bits="4" numericFormat="UNORM"/>
            //         <component name="G" bits="4" numericFormat="UNORM"/>
            //     </format>
            "formats" => {
                for format in node_iter_children(n) {
                    let block_extent = format.attribute("blockExtent").map(|s| {
                        let mut split = s.split_terminator(',').map(|s| s.parse::<u8>().unwrap());
                        [
                            split.next().unwrap(),
                            split.next().unwrap(),
                            split.next().unwrap(),
                        ]
                    });

                    let chroma = format.attribute("chroma").map(|s| match s {
                        "420" => YCBREncoding::E420,
                        "422" => YCBREncoding::E422,
                        "444" => YCBREncoding::E444,
                        _ => unreachable!(),
                    });

                    let mut components = Vec::new();
                    let mut planes = Vec::new();
                    let mut spirvimageformats = Vec::new();

                    for child in node_iter_children(format) {
                        match child.tag_name().name() {
                            "component" => {
                                let numeric_format = match child.get("numericFormat") {
                                    "SFLOAT" => NumericFormat::SFLOAT,
                                    "SINT" => NumericFormat::SINT,
                                    "SNORM" => NumericFormat::SNORM,
                                    "SRGB" => NumericFormat::SRGB,
                                    "SSCALED" => NumericFormat::SSCALED,
                                    "UFLOAT" => NumericFormat::UFLOAT,
                                    "UINT" => NumericFormat::UINT,
                                    "UNORM" => NumericFormat::UNORM,
                                    "USCALED" => NumericFormat::USCALED,
                                    _ => unreachable!(),
                                };
                                let bits = match child.get("bits") {
                                    "compressed" => None,
                                    other => Some(other.parse::<u8>().unwrap()),
                                };
                                components.push(Component {
                                    name: child.intern("name", reg),
                                    bits,
                                    numeric_format,
                                    plane_index: child
                                        .attribute("planeIndex")
                                        .map(|s| s.parse().unwrap()),
                                });
                            }
                            "plane" => {
                                planes.push(Plane {
                                    index: child.get("index").parse().unwrap(),
                                    width_divisor: child.get("widthDivisor").parse().unwrap(),
                                    height_divisor: child.get("heightDivisor").parse().unwrap(),
                                    compatible: child.intern("compatible", reg),
                                });
                            }
                            "spirvimageformat" => {
                                spirvimageformats.push(child.intern("name", reg));
                            }
                            _ => unreachable!(),
                        }
                    }

                    reg.add_format(Format {
                        name: format.intern("name", reg),
                        class: format.intern("class", reg),
                        blocksize: format.get("blockSize").parse().unwrap(),
                        texels_per_block: format.get("texelsPerBlock").parse().unwrap(),
                        block_extent,
                        packed: format.attribute("packed").map(|s| s.parse().unwrap()),
                        compressed: format.attribute("compressed").map(|s| s.intern(reg)),
                        chroma,
                        components,
                        planes,
                        spirvimageformats,
                    });
                }
            }
            tag @ ("spirvextensions" | "spirvcapabilities") => {
                for c in node_iter_children(n) {
                    let ext_cap = SpirvExtCap {
                        name: c.intern("name", reg),
                        enables: convert_spirv_enable(c, reg),
                    };

                    match tag {
                        "spirvextensions" => reg.add_spirv_extension(ext_cap),
                        "spirvcapabilities" => reg.add_spirv_capability(ext_cap),
                        _ => unreachable!(),
                    };
                }
            }
            other => unimplemented!("Aaaaa {} aaa", other),
        }
    }
}

// tranforms C numeric expressions into those accepted by rust
fn numeric_expression_rustify(buf: &mut String) {
    // junk like '(~0ULL)' (ie. unsigned long long ie. u64) is not valid rust
    // the NOT operator is ! instead of ~ and specifying bit width is not neccessary (I hope)
    // replace ~ with !
    assert!(buf.is_ascii());
    // operating on bytes like this is safe only for ascii
    unsafe {
        for b in buf.as_bytes_mut() {
            if *b == '~' as u8 {
                *b = '!' as u8;
            }
        }
    }
    // if the expression is wrapped in parentheses, remove them
    if buf.chars().next().unwrap() == '(' {
        buf.pop();
        buf.remove(0);
    }
    // remove the bit width specifiers - I assume this is valid since rust doesn't allow integers
    // to be implicitly cast implying that the literal must start at the target bitwidth
    buf.retain(|c| (c != 'L') && (c != 'U') && (c != 'F'));
}

fn convert_spirv_enable(n: Node, int: &Interner) -> Vec<SpirvEnable> {
    let mut converted = Vec::new();
    for enable in node_iter_children(n) {
        assert!(enable.tag_name().name() == "enable");

        let attrs = enable.attributes();
        assert!(attrs.len() > 0);

        let val = attrs[0].value().intern(int);
        // there are four variants of the enable tag, here we discriminate by the first attribute
        // FIXME this is rather fragile
        let out = match attrs[0].name() {
            "version" => SpirvEnable::Version(val),
            "extension" => SpirvEnable::Extension(val),
            "struct" => SpirvEnable::Feature {
                structure: val,
                feature: enable.intern("feature", int),
                requires: parse_comma_separated(enable.attribute("requires"), int),
                alias: enable.attribute("alias").map(|s| s.intern(int)),
            },
            "property" => SpirvEnable::Property {
                property: val,
                member: enable.intern("member", int),
                value: enable.intern("value", int),
                requires: parse_comma_separated(enable.attribute("requires"), int),
            },
            _ => panic!("This is likely a result of bad code and fragile assumptions."),
        };

        converted.push(out);
    }
    converted
}

fn convert_section_children(
    n: Node,
    inherited_extnumber: Option<u32>,
    conf: Option<&GenConfig>,
    reg: &mut Registry,
) -> Vec<FeatureExtensionItem> {
    let mut converted = Vec::new();
    for child in node_iter_children(n) {
        if try_skip(child, conf, reg) {
            continue;
        }

        if let Some(comment) = child.attribute("comment") {
            converted.push(FeatureExtensionItem::Comment(comment.to_owned()));
        }

        match child.tag_name().name() {
            "require" => {
                let mut items = Vec::new();
                for item in node_iter_children(child) {
                    let tag_name = item.tag_name().name();

                    if tag_name == "comment" {
                        continue;
                    }

                    let name = item.intern("name", reg);
                    let iitem = match tag_name {
                        "type" | "command" => {
                            assert!(child.attribute("api").is_none());

                            // QUIRK extensions can introduce aliases, in pursuit of consistency we add them to "the soup"
                            // though this doesn't seem to be excersized in the registry
                            if let Some(value) = item.attribute("alias") {
                                reg.add_symbol(name, SymbolBody::Alias(value.intern(reg)));
                            }

                            InterfaceItem::Simple { name }
                        }
                        "enum" => {
                            // I don't think this is applicable here as it is already in a <require> which has its own api property
                            // however the spec says "used to address subtle incompatibilities"
                            // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_enum_tags
                            // todo convert potential usage in a separate <require> block, that should have the same semantics
                            assert!(child.attribute("api").is_none());

                            if let Some(extends) = item.attribute("extends") {
                                // This was so incredibly hard to find! It took like 40 minutes!
                                // https://registry.khronos.org/vulkan/specs/1.3/styleguide.html#_assigning_extension_token_values
                                let method = if let Some(offset) = item.attribute("offset") {
                                    let offset = offset.parse::<i64>().unwrap();
                                    let extnumber = item
                                        .attribute("extnumber")
                                        .map(|s| s.parse().unwrap())
                                        .or(inherited_extnumber)
                                        .unwrap()
                                        as i64;

                                    // enum_offset(extension_number, offset) = base_value + (extension_number - 1) × range_size + offset
                                    let mut offset = 1000000000 + (extnumber - 1) * 1000 + offset;

                                    if item.attribute("dir") == Some("-") {
                                        offset = -offset;
                                    }

                                    ConstantValue::Literal(offset)
                                } else if let Some(bitpos) = item.attribute("bitpos") {
                                    let bitpos = bitpos.parse().unwrap();
                                    ConstantValue::Bitpos(bitpos)
                                } else if let Some(value) = item.attribute("value") {
                                    if let Ok(int) = value.parse() {
                                        ConstantValue::Literal(int)
                                    } else {
                                        let mut val = value.to_owned();
                                        numeric_expression_rustify(&mut val);
                                        ConstantValue::Expression(val)
                                    }
                                } else if let Some(value) = item.attribute("alias") {
                                    ConstantValue::Symbol(value.intern(reg))
                                } else {
                                    unreachable!()
                                };

                                InterfaceItem::Extend {
                                    name,
                                    extends: extends.intern(reg),
                                    value: method,
                                }
                            } else {
                                if let Some(val) = item.attribute("value") {
                                    let val = guess_constant_val(val, reg);
                                    reg.add_symbol(name, SymbolBody::Constant { ty: None, val });
                                } else if let Some(value) = item.attribute("alias") {
                                    reg.add_symbol(name, SymbolBody::Alias(value.intern(reg)));
                                }

                                InterfaceItem::Simple { name }
                            }
                        }
                        _ => unimplemented!(),
                    };
                    items.push(iitem);
                }

                let profile = child.attribute("profile").map(|s| s.intern(reg));
                let api = parse_comma_separated(child.attribute("api"), reg);
                let extension = child.attribute("extension").map(|s| s.intern(reg));
                let feature = child.attribute("feature").map(|s| s.intern(reg));

                converted.push(FeatureExtensionItem::Require {
                    profile,
                    api,
                    extension,
                    feature,
                    items,
                });
            }
            "remove" => {
                // extensions removing things is dubious at best, thanks to the spec for being explicit
                // "It is unlikely that a type would ever be removed, although this usage is allowed by the schema."

                let mut items = Vec::new();
                for item in node_iter_children(child) {
                    // if you're going to be removing something, obbviously only the name
                    // is neccessary and no other information is given
                    let name = item.intern("name", reg);
                    items.push(name);
                }

                let profile = child.attribute("profile").map(|s| s.intern(reg));
                let api = parse_comma_separated(child.attribute("api"), reg);

                converted.push(FeatureExtensionItem::Remove {
                    profile,
                    api,
                    items,
                });
            }
            _ => unreachable!(),
        }
    }
    converted
}

fn iter_comma_separated<'a>(
    str: &'a str,
    int: &'a Interner,
) -> impl Iterator<Item = UniqueStr> + 'a {
    str.split_terminator(',').map(|s| s.intern(int))
}

fn parse_comma_separated(str: Option<&str>, int: &Interner) -> Vec<UniqueStr> {
    if let Some(str) = str {
        iter_comma_separated(str, int).collect()
    } else {
        Vec::new()
    }
}

fn parse_detect_radix(str: &str) -> i32 {
    if str.len() > 2 && &str[0..2] == "0x" {
        i32::from_str_radix(&str[2..], 16).unwrap()
    } else {
        i32::from_str_radix(str, 10).unwrap()
    }
}

fn guess_constant_val(val: &str, reg: &mut Registry) -> ConstantValue {
    let val = if val.starts_with('"') {
        ConstantValue::String(val.trim_matches('"').to_owned())
    } else if val.starts_with("VK_") {
        ConstantValue::Symbol(val.intern(reg))
    } else if let Ok(int) = val.parse() {
        ConstantValue::Literal(int)
    } else {
        let mut val = val.to_owned();
        numeric_expression_rustify(&mut val);
        ConstantValue::Expression(val)
    };
    val
}
