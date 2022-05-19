// vk.xml documentation
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use lasso::{Rodeo, Spur};
use roxmltree::Node;
use type_declaration::{parse_type, TypeDecl};

pub mod debug_impl;
pub mod type_declaration;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToplevelKind {
    Handle,
    Constant,
    Enum,
    Bitmask,
    Command,
    // the folowing kinds cannot? be aliases
    Alias,
    Included,
    Basetype,
    Funcpointer,
    Struct,
    BitmaskBits,
}

#[derive(Debug)]
pub struct Toplevel(pub Spur, pub ToplevelBody);

#[derive(Debug)]
pub enum ToplevelBody {
    Alias {
        alias_of: Spur,
        kind: ToplevelKind,
    },
    Included {
        header: Spur,
    },
    Basetype {
        /// if the type is created by a preprocessor macro the type is not available
        ty: Option<Spur>,
        code: String,
    },
    Bitmask {
        ty: Spur,
        /// the bits can be missing if it the flags exist but are so far unused
        bits_enum: Option<Spur>,
    },
    Handle {
        object_type: Spur,
        dispatchable: bool,
    },
    Funcpointer {
        return_type: TypeDecl,
        args: Vec<(Spur, TypeDecl)>,
    },
    // TODO reconsider splitting off into another item
    Struct {
        union: bool,
        members: Vec<(Spur, TypeDecl)>,
    },
    Constant {
        ty: Spur,
        val: Spur,
    },
    // FIXME merge Enum and Bitmaskbits just like we've done to Struct and Union
    Enum {
        members: Vec<(Spur, EnumValue)>,
    },
    BitmaskBits {
        members: Vec<(Spur, EnumValue)>,
    },
    Command {
        return_type: TypeDecl,
        params: Vec<CommandParameter>,
    },
}

#[derive(Debug)]
pub enum EnumValue {
    Bitpos(u32),
    Value(i64),
    Alias(Spur),
}

#[derive(Debug)]
pub struct Feature {
    pub name: Spur,
    pub api: Spur,
    pub number: Spur,
    pub protect: Option<Spur>,
    pub children: Vec<FeatureExtensionItem>,
}

// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_attributes_of_extension_tags
#[derive(Debug)]
pub struct Extension {
    pub name: Spur,
    pub number: u32,
    pub sortorder: Option<u32>,
    pub author: Option<String>,
    pub contact: Option<String>,

    pub ext_type: Option<Spur>,
    pub requires: Vec<Spur>,
    pub requires_core: Option<Spur>,
    pub protect: Option<Spur>,
    pub platform: Option<Spur>,
    pub supported: Vec<Spur>,
    pub promotedto: Option<Spur>,
    pub deprecatedby: Option<Spur>,
    pub obsoletedby: Option<Spur>,
    pub provisional: bool,
    pub specialuse: Vec<Spur>,
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
    pub name: Spur,
    pub bits: Option<u8>, // is none for "compressed"
    pub numeric_format: NumericFormat,
    pub plane_index: Option<u8>,
}

#[derive(Debug)]
pub struct Plane {
    pub index: u8,
    pub width_divisor: u8,
    pub height_divisor: u8,
    pub compatible: Spur,
}

#[derive(Debug)]
pub struct Format {
    pub name: Spur,
    pub class: Spur,
    pub blocksize: u8,
    pub texels_per_block: u8,
    pub block_extent: Option<[u8; 3]>,
    pub packed: Option<u8>,
    pub compressed: Option<Spur>,
    pub chroma: Option<YCBREncoding>,

    pub components: Vec<Component>,
    pub planes: Vec<Plane>,
    pub spirvimageformats: Vec<Spur>,
}

#[derive(Debug)]
pub struct CommandParameter {
    pub name: Spur,
    pub len: Option<String>,
    pub alt_len: Option<String>,
    pub optional: bool,
    pub externsync: Option<String>,
    pub ty: TypeDecl,
}

#[derive(Debug)]
pub struct Define {
    pub name: Spur,
    pub body: String,
}

#[derive(Debug)]
pub enum FeatureExtensionItem {
    Comment(String),
    // consider splitting this off
    Require {
        profile: Option<Spur>,
        // The api attribute is only supported inside extension tags, since feature tags already define a specific API.
        api: Vec<Spur>,
        // The extension attribute currently does not affect output generators in any way, and is simply metadata. This will be addressed as we better define different types of dependencies between extensions.
        // bruh
        extension: Option<Spur>,
        feature: Option<Spur>,
        items: Vec<InterfaceItem>,
    },
    Remove {
        profile: Option<Spur>,
        // The api attribute is only supported inside extension tags, since feature tags already define a specific API.
        api: Vec<Spur>,
        // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_enum_tags
        // the item removed will always be `InterfaceItem::Simple`, api property that can be in <enum> is not applicable I think?
        // https://github.com/osor-io/Vulkan/blob/e4bc1b9125645f3db3c8342edc24d81dc497f252/generate_code/generate_vulkan_code.jai#L1586
        // here it seems that the author doesn't handle `api`
        items: Vec<Spur>,
    },
}

#[derive(Debug)]
pub enum ExtendMethod {
    Bitpos(u32),
    // extnumber, offset, is positive direction, the notion of a direction when offset could have been negative is stupid
    BitposExtnumber { extnumber: u32, offset: i32 },
    // value can in fact be whatever
    Value(String),
    Alias(Spur),
}

#[derive(Debug)]
pub enum ConstantValue {
    Value(String),
    Alias(Spur),
}

#[derive(Debug)]
pub enum InterfaceItem {
    // just importing already finished items
    // for some reason Enums can have an Api condition
    // even though it can already be scoped by the parent <require>, why??
    Simple {
        name: Spur,
        api: Option<Spur>,
    },
    Extend {
        name: Spur,
        extends: Spur,
        api: Option<Spur>,
        method: ExtendMethod,
    },
    AddConstant {
        name: Spur,
        value: ConstantValue,
    },
}

// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#tag-spirvenable
#[derive(Debug)]
pub enum SpirvEnable {
    Version(Spur),
    Extension(Spur),
    Feature {
        structure: Spur,
        feature: Spur,
        requires: Vec<Spur>,
        alias: Option<Spur>,
    },
    Property {
        property: Spur,
        member: Spur,
        value: Spur,
        requires: Vec<Spur>,
    },
}

#[derive(Debug)]
pub struct SpirvExtCap(pub Spur, pub Vec<SpirvEnable>);

pub struct Platform {
    pub name: Spur,
    pub protect: Spur,
    pub comment: String,
}

pub struct Tag {
    pub name: Spur,
    pub author: String,
    pub contact: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemKind {
    Toplevel,
    Feature,
    Extension,
    Format,
    SpirvCapability,
    SpirvExtension,
}

pub struct Registry {
    pub platforms: Vec<Platform>,
    pub tags: Vec<Tag>,
    pub headers: Vec<Spur>,
    pub defines: Vec<Define>,

    pub toplevel: Vec<Toplevel>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
    pub formats: Vec<Format>,
    pub spirv_capabilities: Vec<SpirvExtCap>,
    pub spirv_extensions: Vec<SpirvExtCap>,
    pub item_map: HashMap<Spur, (u32, ItemKind)>,
    pub interner: RefCell<Rodeo>,
    // a map for substituting certain Spurs with Different spurs
    // this is used at code generation to consistently rename different identifiers
    // to make them match rust naming conventions better
    // this is a pretty dumb solution and it is debatable whether this should be implemented here
    // but keeping everything withou wrapper types makes it by far the least painful solution
    // another option was preparing all the renames beforehand and then iterating through everything here
    // and replacing the renamed spurs but that would be quite tiring to maintain and would break any spurs
    // that the user may have kept in variables for comparison
    pub renames: RefCell<HashMap<Spur, Spur>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            platforms: Default::default(),
            tags: Default::default(),
            headers: Default::default(),
            defines: Default::default(),

            toplevel: Default::default(),
            features: Default::default(),
            extensions: Default::default(),
            formats: Default::default(),
            spirv_capabilities: Default::default(),
            spirv_extensions: Default::default(),
            item_map: Default::default(),
            interner: RefCell::new(Rodeo::new()),
            renames: RefCell::new(Default::default()),
        }
    }
    pub fn add_rename_with(&self, original: Spur, with: impl FnOnce() -> Spur) {
        // we need to drop the Ref here because the closure may try to borrow the RefCell inside the registry,
        // this is non-ideal because we can't use the entry api, it would be neccessary to give the cloure
        // an already borrowed reference but that would require some lines of code to keep the api consistent
        let get = self.renames.borrow().get(&original).map(|s| *s);

        if let Some(existing) = get {
            let rename = with();
            if existing != rename {
                panic!("Attempt to rename Spur multiple times.")
            }
        } else {
            let rename = with();
            let mut renames_ref = self.renames.borrow_mut();

            // currently we're using renames to do simple substitutions, this is unnecessary and we may want to preserve the original rename
            // // if we are renaming a spur that is already renamed we directly replace the spur with that rename
            // // this will spin forever if any cycles are encountered, too bad!
            // while let Some(next) = renames_ref.get(&rename) {
            //     rename = *next;
            // }

            let none = renames_ref.insert(original, rename);
            assert!(none.is_none());
        }
    }
    // renaming changes the string that a spur will resolve to, however it keeps all spurs the same
    // thus we can have multiple *different* spurs resolve to the same string
    // this is useful for example when merging the two-part bitfield declarations: *Bits and *Flags structs
    // we use only the *Flags name but all vulkan identifiers use the *Bits as the type, thus we can merge them easily
    pub fn add_rename(&self, original: Spur, spur: Spur) {
        self.add_rename_with(original, || spur);
    }
    pub fn resolve<'a>(&'a self, spur: &Spur) -> Ref<'a, str> {
        let renames = self.renames.borrow();
        let spur = renames.get(spur).unwrap_or(spur);
        Ref::map(self.interner.borrow(), |a| a.resolve(spur))
    }
    pub fn get(&self, str: &str) -> Option<Spur> {
        self.interner.borrow().get(str)
    }
    pub fn intern(&self, str: &str) -> Spur {
        self.interner.borrow_mut().get_or_intern(str)
    }
}

pub trait Intern {
    fn intern(&self, reg: &Registry) -> Spur;
}

impl<T: AsRef<str>> Intern for T {
    fn intern(&self, reg: &Registry) -> Spur {
        reg.intern(self.as_ref())
    }
}

pub trait Resolve {
    fn resolve<'a>(&self, reg: &'a Registry) -> Ref<'a, str>;
}

impl Resolve for Spur {
    fn resolve<'a>(&self, reg: &'a Registry) -> Ref<'a, str> {
        reg.resolve(self)
    }
}

fn add_item<T>(
    vec: &mut Vec<T>,
    map: &mut HashMap<Spur, (u32, ItemKind)>,
    what: T,
    name: Spur,
    kind: ItemKind,
) {
    let index = TryInto::<u32>::try_into(vec.len()).unwrap();
    vec.push(what);

    let none = map.insert(name, (index, kind));

    // assert that no collisions happen
    assert!(none.is_none());
}

trait NodeUtils<'a> {
    fn intern(&self, attribute: &str, reg: &Registry) -> Spur;
    fn owned(&self, attribute: &str) -> String;
    fn get(&'a self, attribute: &str) -> &'a str;
    fn child_text(&'a self, child: &str) -> &'a str;
    fn try_child_text(&'a self, child: &str) -> Option<&'a str>;
}

impl<'a, 'b> NodeUtils<'b> for Node<'a, 'b> {
    fn intern(&self, attribute: &str, reg: &Registry) -> Spur {
        self.attribute(attribute).unwrap().intern(reg)
    }
    fn owned(&self, attribute: &str) -> String {
        self.attribute(attribute).unwrap().to_owned()
    }
    fn get(&'a self, attribute: &str) -> &'a str {
        self.attribute(attribute).unwrap()
    }
    fn child_text(&'a self, child: &str) -> &'a str {
        iter_children(*self)
            .find(|n| n.tag_name().name() == child)
            .unwrap()
            .text()
            .unwrap()
    }
    fn try_child_text(&'a self, child: &str) -> Option<&'a str> {
        Some(
            iter_children(*self)
                .find(|n| n.tag_name().name() == child)?
                .text()
                .unwrap(),
        )
    }
}

// the kind must be a closure so that it may be lazily evaluated as in some cases it needs to
//  do some computation that would be invalid if the node is not an alias in the first place
fn try_alias(t: Node, kind: &dyn Fn() -> ToplevelKind, reg: &mut Registry) -> bool {
    if let Some(alias) = t.attribute("alias") {
        let typ = ToplevelBody::Alias {
            alias_of: alias.intern(&*reg),
            kind: kind(),
        };

        let name = t.intern("name", &*reg);

        add_item(
            &mut reg.toplevel,
            &mut reg.item_map,
            Toplevel(name, typ),
            name,
            ItemKind::Toplevel,
        );

        return true;
    }

    return false;
}

fn iter_children<'a, 'b: 'a>(node: Node<'a, 'b>) -> impl Iterator<Item = Node<'a, 'b>> {
    // nice and readable
    node.first_child().into_iter().flat_map(|c| {
        c.next_siblings()
            .filter(|n| !n.tag_name().name().is_empty())
    })
}

fn collect_node_text(n: Node, buf: &mut String) {
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

pub fn process_registry(xml: &str) -> Registry {
    let mut reg = Registry::new();

    let doc = roxmltree::Document::parse(xml).unwrap();

    let r = doc
        .descendants()
        .find(|n| n.has_tag_name("registry"))
        .unwrap();

    // quite a few places require a string to collect some text into and then further process it
    let mut buf = String::new();

    for n in iter_children(r) {
        match n.tag_name().name() {
            "comment" => {} // TODO propagate this information
            "platforms" => {
                for c in iter_children(n) {
                    let p = Platform {
                        name: c.intern("name", &reg),
                        protect: c.intern("protect", &reg),
                        comment: c.owned("comment"),
                    };
                    reg.platforms.push(p);
                }
            }
            "tags" => {
                for c in iter_children(n) {
                    let t = Tag {
                        name: c.intern("name", &reg),
                        author: c.owned("author"),
                        contact: c.owned("contact"),
                    };
                    reg.tags.push(t);
                }
            }
            "types" => {
                for t in iter_children(n) {
                    if t.tag_name().name() == "comment" {
                        continue;
                    }

                    let category = t.attribute("category");

                    // handle all aliases here, all of them have the same form
                    let kind = || match category {
                        Some("bitmask") => ToplevelKind::Bitmask,
                        Some("handle") => ToplevelKind::Handle,
                        Some("enum") => ToplevelKind::Enum,
                        // struct and union representation is the same
                        // a bool flag discriminates between the two
                        Some("struct" | "union") => ToplevelKind::Struct,
                        _ => todo!(),
                    };

                    if try_alias(t, &kind, &mut reg) {
                        continue;
                    }

                    match category {
                        // <type name="vk_platform" category="include">#include "vk_platform.h"</type>
                        // <type category="include" name="X11/Xlib.h"/>
                        Some("include") => {
                            let name = t.get("name");
                            if name.contains(".h") {
                                reg.headers.push(name.intern(&reg));
                            } else {
                                // #include "vk_platform.h"
                                let code = t.text().unwrap();
                                let name = code.split_terminator('"').nth(1).unwrap();
                                reg.headers.push(name.intern(&reg));
                            }
                        }
                        // <type requires="X11/Xlib.h" name="Display"/>
                        // <type requires="vk_platform" name="void"/>
                        // <type name="int"/>
                        None => {
                            let name = t.get("name");

                            // there is just an 'int' - Why? - Just skip it and hardcode an alias to std::ffi::c_int later
                            if name == "int" {
                                continue;
                            }

                            let name = name.intern(&reg);

                            let typ = ToplevelBody::Included {
                                header: t.intern("requires", &reg),
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
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
                            // wow, the name is either specified as an attribute or delimited with an element within the contained text
                            let name = t.attribute("name").unwrap_or_else(|| t.child_text("name"));

                            buf.clear();
                            collect_node_text(t, &mut buf);

                            let define = Define {
                                name: name.intern(&reg),
                                body: buf.trim().to_owned(),
                            };

                            reg.defines.push(define);
                        }
                        // <type category="basetype">struct <name>ANativeWindow</name>;</type>
                        // <type category="basetype">typedef <type>uint32_t</type> <name>VkSampleMask</name>;</type>
                        Some("basetype") => {
                            let name = t.child_text("name").intern(&reg);
                            let ty = t.try_child_text("type").map(|s| s.intern(&reg));

                            let mut buf = String::new();
                            collect_node_text(t, &mut buf);

                            let typ = ToplevelBody::Basetype {
                                ty,
                                code: buf.trim().to_owned(),
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
                            );
                        }
                        // <type requires="VkFramebufferCreateFlagBits" category="bitmask">
                        //   typedef
                        //   <type>VkFlags</type>
                        //   <name>VkFramebufferCreateFlags</name>
                        //   ;
                        // </type>
                        // <type category="bitmask" name="VkGeometryFlagsNV" alias="VkGeometryFlagsKHR"/>
                        Some("bitmask") => {
                            let name = t.child_text("name").intern(&reg);
                            let ty = t.child_text("type").intern(&reg);

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

                            let typ = ToplevelBody::Bitmask {
                                ty,
                                bits_enum: bits.map(|b| b.intern(&reg)),
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
                            );
                        }
                        // <type category="handle" objtypeenum="VK_OBJECT_TYPE_INSTANCE"><type>VK_DEFINE_HANDLE</type>(<name>VkInstance</name>)</type>
                        // <type category="handle" name="VkDescriptorUpdateTemplateKHR" alias="VkDescriptorUpdateTemplate"/>
                        Some("handle") => {
                            let name = t.child_text("name").intern(&reg);
                            let object_type = t.intern("objtypeenum", &reg);

                            let ty = t.child_text("type"); // type as in kind
                            let dispatchable = match ty {
                                "VK_DEFINE_HANDLE" => true,
                                "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => false,
                                _ => unreachable!(),
                            };

                            let typ = ToplevelBody::Handle {
                                object_type,
                                dispatchable,
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
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

                            let name = t.child_text("name").intern(&reg);

                            // since the incredible way that the xml is structured, pointer syntax and const decorations are not specially marked
                            // thus it actually becomes easier to collect all the text into a String and parse it manually

                            let mut text = String::new();
                            collect_node_text(t, &mut text);

                            let mut brackets = text.split('(');

                            // println!(
                            //     "\n {} \n{:#?}\n",
                            //     text,
                            //     brackets.clone().collect::<Vec<_>>()
                            // );

                            let return_type = {
                                let first = brackets.next().unwrap();
                                let clean = first.trim_start_matches("typedef");
                                parse_type(clean, false, &reg).1
                            };

                            // waste the second parenthesis
                            brackets.next().unwrap();

                            let args_text = brackets
                                .next()
                                .unwrap()
                                .trim_end()
                                .trim_end_matches(&[')', ';']);

                            // println!("  '{}'", &args_text);

                            let mut args = Vec::new();
                            for arg in args_text.split(',') {
                                // println!("  wee");
                                // println!("  // {} //", &arg);

                                let (name, ty) = parse_type(&arg, true, &reg);
                                args.push((name.unwrap(), ty));
                            }

                            let typ = ToplevelBody::Funcpointer { return_type, args };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
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
                            let name = t.intern("name", &reg);
                            let mut members = Vec::new();

                            for member in iter_children(t) {
                                match member.tag_name().name() {
                                    "member" => {}
                                    "comment" => continue,
                                    _ => unreachable!(),
                                }

                                buf.clear();
                                collect_node_text(member, &mut buf);
                                let (name, ty) = parse_type(&buf, true, &reg);

                                members.push((name.unwrap(), ty));
                            }

                            let typ = ToplevelBody::Struct {
                                union: category == Some("union"),
                                members,
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
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
                        for e in iter_children(n) {
                            assert_eq!(e.tag_name().name(), "enum");
                            let name = e.intern("name", &reg);

                            // <enum name="VK_LUID_SIZE_KHR" alias="VK_LUID_SIZE"/>
                            if try_alias(e, &|| ToplevelKind::Constant, &mut reg) {
                                continue;
                            }

                            // <enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                            let ty = e.intern("type", &reg);
                            let mut value = e.get("value").to_owned(); // owned due to needing to mutate

                            // junk like '(~0ULL)' (ie. unsigned long long ie. u64) is not valid rust
                            // the NOT operator is ! instead of ~ and specifying bit width is not neccessary (I hope)

                            // replace ~ with !
                            assert!(value.is_ascii()); // operating on bytes like this is safe only for ascii
                            unsafe {
                                for b in value.as_bytes_mut() {
                                    if *b == '~' as u8 {
                                        *b = '!' as u8;
                                    }
                                }
                            }

                            // if the expression is wrapped in parentheses, remove them
                            if value.chars().next().unwrap() == '(' {
                                value.pop();
                                value.remove(0);
                            }

                            // remove the bit width specifiers - I assume this is valid since rust doesn't allow integers
                            // to be implicitly cast implying that the literal must start at the target bitwidth
                            value.retain(|c| c != 'L' && c != 'L' && c != 'F');

                            let typ = ToplevelBody::Constant {
                                ty,
                                val: value.intern(&reg),
                            };

                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
                            );
                        }
                    }
                    // <enums name="VkImageLayout" type="enum">
                    category @ Some("enum" | "bitmask") => {
                        assert_eq!(n.tag_name().name(), "enums");
                        let name = n.intern("name", &reg);

                        let mut members = Vec::new();
                        for e in iter_children(n) {
                            match e.tag_name().name() {
                                "enum" => {}
                                "comment" | "unused" => continue, // I don't think tracking unused values is ever useful
                                _ => unreachable!(),
                            }

                            let variant_name = e.intern("name", &reg);

                            // <enum value="0" name="VK_IMAGE_LAYOUT_UNDEFINED" comment="..."/>
                            // <enum bitpos="0" name="VK_QUEUE_GRAPHICS_BIT" comment="..."/>
                            // <enum name="VK_STENCIL_FRONT_AND_BACK" alias="VK_STENCIL_FACE_FRONT_AND_BACK"/>
                            let val = if let Some(value) = e.attribute("value") {
                                EnumValue::Value(parse_detect_radix(value))
                            } else if let Some(bitpos) = e.attribute("bitpos") {
                                EnumValue::Bitpos(bitpos.parse().unwrap())
                            } else if let Some(alias) = e.attribute("alias") {
                                EnumValue::Alias(alias.intern(&reg))
                            } else {
                                unreachable!()
                            };

                            members.push((variant_name, val));
                        }

                        // FIXME merge the enum variants for more straightforward code
                        let typ = match category {
                            Some("enum") => ToplevelBody::Enum { members },
                            Some("bitmask") => ToplevelBody::BitmaskBits { members },
                            _ => unreachable!(),
                        };

                        add_item(
                            &mut reg.toplevel,
                            &mut reg.item_map,
                            Toplevel(name, typ),
                            name,
                            ItemKind::Toplevel,
                        );
                    }
                    _ => unimplemented!(),
                }
            }
            "commands" => {
                for c in iter_children(n) {
                    assert_eq!(c.tag_name().name(), "command");

                    // <command name="vkResetQueryPoolEXT" alias="vkResetQueryPool"/>
                    if try_alias(c, &|| ToplevelKind::Command, &mut reg) {
                        continue;
                    }

                    // <command successcodes="VK_SUCCESS" errorcodes="VK_ERROR_OUT_OF_HOST_MEMORY,VK_ERROR_OUT_OF_DEVICE_MEMORY,VK_ERROR_INITIALIZATION_FAILED,VK_ERROR_LAYER_NOT_PRESENT,VK_ERROR_EXTENSION_NOT_PRESENT,VK_ERROR_INCOMPATIBLE_DRIVER">
                    //     <proto><type>VkResult</type> <name>vkCreateInstance</name></proto>
                    //     <param>const <type>VkInstanceCreateInfo</type>* <name>pCreateInfo</name></param>
                    //     <param optional="true">const <type>VkAllocationCallbacks</type>* <name>pAllocator</name></param>
                    //     <param><type>VkInstance</type>* <name>pInstance</name></param>
                    // </command>
                    let mut declaration = iter_children(c);

                    let (name, return_type) = {
                        let proto = declaration.next().unwrap();

                        buf.clear();
                        collect_node_text(proto, &mut buf);

                        let (name, ty) = parse_type(&buf, true, &reg);
                        (name.unwrap(), ty)
                    };

                    let mut params = Vec::new();
                    for p in declaration {
                        match p.tag_name().name() {
                            "param" => {}
                            "implicitexternsyncparams" => continue,
                            _ => unreachable!(),
                        }

                        buf.clear();
                        collect_node_text(p, &mut buf);
                        let (name, ty) = parse_type(&buf, true, &reg);

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

                    let typ = ToplevelBody::Command {
                        return_type,
                        params,
                    };

                    add_item(
                        &mut reg.toplevel,
                        &mut reg.item_map,
                        Toplevel(name, typ),
                        name,
                        ItemKind::Toplevel,
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
                let name = n.intern("name", &reg);

                let children = convert_extension_children(n, None, &mut reg);

                let typ = Feature {
                    name,
                    api: n.intern("api", &reg),
                    number: n.intern("number", &reg),
                    protect: n.attribute("protect").map(|s| s.intern(&reg)),
                    children,
                };

                add_item(
                    &mut reg.features,
                    &mut reg.item_map,
                    typ,
                    name,
                    ItemKind::Feature,
                );
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
            //             <typ
            "extensions" => {
                for e in iter_children(n) {
                    let name = e.intern("name", &reg);

                    // this is in fact an extension, it needs to have a number
                    let extnumber = e.get("number").parse().unwrap();
                    // FIXME the children are not actually included in the output
                    let _children = convert_extension_children(e, Some(extnumber), &reg);
                    let deprecatedby = e.attribute("deprecatedby").and_then(|s| {
                        // thanks VK_NV_glsl_shader
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.intern(&reg))
                        }
                    });

                    let typ = Extension {
                        name,
                        number: extnumber,
                        sortorder: e.attribute("sortorder").map(|s| s.parse().unwrap()),
                        author: e.attribute("author").map(|s| s.to_owned()),
                        contact: e.attribute("contact").map(|s| s.to_owned()),
                        ext_type: e.attribute("type").map(|s| s.intern(&reg)),
                        requires: parse_comma_separated(e.attribute("requires"), &reg),
                        requires_core: e.attribute("requiresCore").map(|s| s.intern(&reg)),
                        protect: e.attribute("protect").map(|s| s.intern(&reg)),
                        platform: e.attribute("platform").map(|s| s.intern(&reg)),
                        supported: parse_comma_separated(e.attribute("supported"), &reg),
                        promotedto: e.attribute("promotedto").map(|s| s.intern(&reg)),
                        deprecatedby,
                        obsoletedby: e.attribute("obsoletedby").map(|s| s.intern(&reg)),
                        provisional: e.attribute("provisional") == Some("true"),
                        specialuse: parse_comma_separated(e.attribute("specialuse"), &reg),
                    };

                    add_item(
                        &mut reg.extensions,
                        &mut reg.item_map,
                        typ,
                        name,
                        ItemKind::Extension,
                    );
                }
            }
            // <formats>
            //     <format name="VK_FORMAT_R4G4_UNORM_PACK8" class="8-bit" blockSize="1" texelsPerBlock="1" packed="8">
            //         <component name="R" bits="4" numericFormat="UNORM"/>
            //         <component name="G" bits="4" numericFormat="UNORM"/>
            //     </format>
            "formats" => {
                for format in iter_children(n) {
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

                    for child in iter_children(format) {
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
                                    name: child.intern("name", &reg),
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
                                    compatible: child.intern("compatible", &reg),
                                });
                            }
                            "spirvimageformat" => {
                                spirvimageformats.push(child.intern("name", &reg));
                            }
                            _ => unreachable!(),
                        }
                    }

                    let name = format.intern("name", &reg);
                    let typ = Format {
                        name,
                        class: format.intern("class", &reg),
                        blocksize: format.get("blockSize").parse().unwrap(),
                        texels_per_block: format.get("texelsPerBlock").parse().unwrap(),
                        block_extent,
                        packed: format.attribute("packed").map(|s| s.parse().unwrap()),
                        compressed: format.attribute("compressed").map(|s| s.intern(&reg)),
                        chroma,
                        components,
                        planes,
                        spirvimageformats,
                    };

                    add_item(
                        &mut reg.formats,
                        &mut reg.item_map,
                        typ,
                        name,
                        ItemKind::Format,
                    );
                }
            }
            tag @ ("spirvextensions" | "spirvcapabilities") => {
                for c in iter_children(n) {
                    let name = c.intern("name", &reg);
                    let enables = convert_spirv_enable(c, &mut reg);

                    let typ = SpirvExtCap(name, enables);

                    let (kind, vec) = match tag {
                        "spirvextensions" => (ItemKind::SpirvExtension, &mut reg.spirv_extensions),
                        "spirvcapabilities" => {
                            (ItemKind::SpirvCapability, &mut reg.spirv_capabilities)
                        }
                        _ => unreachable!(),
                    };

                    add_item(vec, &mut reg.item_map, typ, name, kind);
                }
            }
            other => unimplemented!("Aaaaa {} aaa", other),
        }
    }
    reg
}

fn convert_spirv_enable(n: Node, reg: &Registry) -> Vec<SpirvEnable> {
    let mut converted = Vec::new();
    for enable in iter_children(n) {
        assert!(enable.tag_name().name() == "enable");

        let attrs = enable.attributes();
        assert!(attrs.len() > 0);

        let val = attrs[0].value().intern(&reg);
        // there are four variants of the enable tag, here we discriminate by the first attribute
        // FIXME this is rather fragile
        let out = match attrs[0].name() {
            "version" => SpirvEnable::Version(val),
            "extension" => SpirvEnable::Extension(val),
            "struct" => SpirvEnable::Feature {
                structure: val,
                feature: enable.intern("feature", &reg),
                requires: parse_comma_separated(enable.attribute("requires"), &reg),
                alias: enable.attribute("alias").map(|s| s.intern(&reg)),
            },
            "property" => SpirvEnable::Property {
                property: val,
                member: enable.intern("member", &reg),
                value: enable.intern("value", &reg),
                requires: parse_comma_separated(enable.attribute("requires"), &reg),
            },
            _ => unreachable!(),
        };

        converted.push(out);
    }
    converted
}

fn convert_extension_children(
    n: Node,
    ext_number: Option<u32>,
    reg: &Registry,
) -> Vec<FeatureExtensionItem> {
    let mut converted = Vec::new();
    for child in iter_children(n) {
        if let Some(comment) = child.attribute("comment") {
            converted.push(FeatureExtensionItem::Comment(comment.to_owned()));
        }

        match child.tag_name().name() {
            "require" => {
                let mut items = Vec::new();
                for item in iter_children(child) {
                    let tag_name = item.tag_name().name();

                    if tag_name == "comment" {
                        continue;
                    }

                    let name = item.intern("name", &reg);
                    let iitem = match tag_name {
                        "type" | "command" => InterfaceItem::Simple { name, api: None },
                        "enum" => {
                            // I don't think this is applicable here as it is already in a <require> which has its own api property
                            // however the spec says "used to address subtle incompatibilities"
                            // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_pub enum_tags
                            assert!(child.attribute("api").is_none());

                            if let Some(extends) = item.attribute("extends") {
                                let method = if let Some(offset) = item.attribute("offset") {
                                    let offset = offset.parse::<i32>().unwrap();
                                    let extnumber = item
                                        .attribute("extnumber")
                                        .map(|s| s.parse::<u32>().unwrap());
                                    let dir = item.attribute("dir");

                                    ExtendMethod::BitposExtnumber {
                                        // if this is a feature which itself (as opposed to an extension) doesn't have an extumber, extnumber is always defined
                                        extnumber: extnumber.or(ext_number).unwrap(),
                                        offset: if dir == Some("-") { -offset } else { offset }
                                            as i32,
                                    }
                                } else if let Some(bitpos) = item.attribute("bitpos") {
                                    let bitpos = bitpos.parse::<u32>().unwrap();
                                    // extends can't be None right? how can a global constant be a bitpos? if yes then copy the EnumSpec::Value case and set value to a bitshifted 1?
                                    ExtendMethod::Bitpos(bitpos)
                                } else if let Some(value) = item.attribute("value") {
                                    ExtendMethod::Value(value.to_owned())
                                } else if let Some(value) = item.attribute("alias") {
                                    ExtendMethod::Alias(value.intern(&reg))
                                } else {
                                    unreachable!()
                                };

                                InterfaceItem::Extend {
                                    name,
                                    extends: extends.intern(&reg),
                                    api: None,
                                    method,
                                }
                            } else {
                                if let Some(value) = item.attribute("value") {
                                    InterfaceItem::AddConstant {
                                        name,
                                        value: ConstantValue::Value(value.to_owned()),
                                    }
                                } else {
                                    InterfaceItem::Simple { name, api: None }
                                }
                            }
                        }
                        _ => unimplemented!(),
                    };
                    items.push(iitem);
                }

                let profile = child.attribute("profile").map(|s| s.intern(&reg));
                let api = parse_comma_separated(child.attribute("api"), reg);
                let extension = child.attribute("extension").map(|s| s.intern(&reg));
                let feature = child.attribute("feature").map(|s| s.intern(&reg));

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
                for item in iter_children(child) {
                    // if you're going to be removing something, obbviously only the name
                    // is neccessary and no other information is given
                    let name = item.intern("name", &reg);
                    items.push(name);
                }

                let profile = child.attribute("profile").map(|s| s.intern(&reg));
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

fn parse_comma_separated(what: Option<&str>, reg: &Registry) -> Vec<Spur> {
    match what {
        Some(s) => s.split_terminator(',').map(|s| s.intern(&reg)).collect(),
        None => Vec::new(),
    }
}

fn parse_detect_radix(str: &str) -> i64 {
    if str.len() > 2 && &str[0..2] == "0x" {
        i64::from_str_radix(&str[2..], 16).unwrap()
    } else {
        i64::from_str_radix(str, 10).unwrap()
    }
}
