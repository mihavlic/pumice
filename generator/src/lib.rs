#![allow(unused)]

// vk.xml documentation
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html

use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    io::Write,
    ops::RangeBounds,
    sync::{
        atomic::{AtomicPtr, Ordering},
        RwLock,
    },
};

use lasso::{Rodeo, RodeoResolver, Spur};
use type_declaration::{parse_type, TypeDecl};
use vk_parse::{
    EnumSpec, Error, ExtensionChild, FormatChild, RegistryChild, TypeCodeMarkup, TypeMember,
    TypeSpec,
};

pub mod debug_impl;
pub mod type_declaration;
#[macro_use]
pub mod format_utils;

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
    Enum {
        members: Vec<(Spur, EnumValue)>,
    },
    BitmaskBits {
        members: Vec<(Spur, EnumValue)>,
    },
    Command {
        return_type: Spur,
        params: Vec<CommandParameter>,
    },
}

#[derive(Debug)]
// FIXME kind of ugly, the empty variants of 'bitpos' are value rather than bitpos, and aliases
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

#[derive(Debug)]
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_attributes_of_extension_tags
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
        // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_pub enum_tags
        // the item removed will always be `InterfaceItem::Simple`, api property that can be in <pub enum> is not applicable I think?
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
    pub vendors: Vec<vk_parse::CommentedChildren<vk_parse::VendorId>>,
    pub platforms: Vec<vk_parse::CommentedChildren<vk_parse::Platform>>,
    pub tags: Vec<vk_parse::CommentedChildren<vk_parse::Tag>>,
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
}

impl Registry {
    pub fn resolve<'a>(&'a self, spur: &Spur) -> Ref<'a, str> {
        Ref::map(self.interner.borrow(), |a| a.resolve(spur))
    }
    pub fn get(&self, str: &str) -> Option<Spur> {
        self.interner.borrow().get(str)
    }
    pub fn get_or_intern(&self, str: &str) -> Spur {
        self.interner.borrow_mut().get_or_intern(str)
    }
    pub fn new() -> Self {
        Self {
            vendors: Default::default(),
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
        }
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

pub fn process_registry(registry: vk_parse::Registry) -> Registry {
    // let mut reg = Rodeo::new();

    // let mut vendors = Vec::new();
    // let mut platforms = Vec::new();
    // let mut tags = Vec::new();
    // let mut headers = Vec::new();
    // let mut defines = Vec::new();

    // the objects themselves and a hashmap mapping their names to the object
    // this preserves definition order
    // let mut toplevel = Vec::new();
    // let mut features = Vec::new();
    // let mut extensions = Vec::new();
    // let mut formats = Vec::new();
    // let mut spirv_capabilities = Vec::new();
    // let mut spirv_extensions = Vec::new();

    // let mut item_map = HashMap::new();

    let mut reg = Registry::new();
    // let Registry { vendors, platforms, tags, headers, defines, toplevel, features, extensions, formats, spirv_capabilities, spirv_extensions, item_map, interner } = &mut reg;

    for node in registry.0 {
        match node {
            RegistryChild::Comment(_) => {}
            RegistryChild::VendorIds(vendors_) => {
                reg.vendors.push(vendors_);
            }
            RegistryChild::Platforms(platforms_) => {
                reg.platforms.push(platforms_);
            }
            RegistryChild::Tags(tags_) => {
                reg.tags.push(tags_);
            }
            RegistryChild::Types(types_) => {
                for child in types_.children {
                    match child {
                        vk_parse::TypesChild::Type(ty) => {
                            match ty.category.as_ref().map(|s| s.as_str()) {
                                // list of potential headers to be included
                                Some("include") => {
                                    // distinguish between:
                                    //   <type name="vk_platform" category="include">#include "vk_platform.h"</type>
                                    //   <type category="include" name="X11/Xlib.h"/>
                                    let name = ty.name.unwrap();
                                    if name.contains(".h") {
                                        reg.headers.push(reg.get_or_intern(&name));
                                    } else {
                                        match ty.spec {
                                            TypeSpec::Code(code) => {
                                                // #include "vk_platform.h"
                                                let name = code
                                                    .code
                                                    .split_terminator("\"")
                                                    .nth(1)
                                                    .unwrap();
                                                reg.headers.push(reg.get_or_intern(name));
                                            }
                                            TypeSpec::None | TypeSpec::Members(_) => unreachable!(),
                                            _ => todo!(),
                                        };
                                    }
                                }
                                // platform types with required headers
                                // <type requires="X11/Xlib.h" name="Display"/>
                                //
                                // there's also, which is of course missing a '.h' so prepare to have a special case
                                // <type requires="vk_platform" name="void"/>
                                //
                                // incredibly enough there is this as well, ##@$@!
                                // <type name="int"/>
                                None => {
                                    // TODO consider hardcoding the ck_platform types and 'int' and ignore them here
                                    let name = ty.name.unwrap();

                                    let header = match &*name {
                                        "int" => "FIXME_WHAT_TO_DO",
                                        _ => ty.requires.as_ref().unwrap(),
                                    };

                                    let name = reg.get_or_intern(&name);
                                    let typ = ToplevelBody::Included {
                                        header: reg.get_or_intern(header),
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
                                //
                                // for some reason there is also
                                // <type category="define" requires="VK_NULL_HANDLE" name="VK_DEFINE_NON_DISPATCHABLE_HANDLE">
                                //   #ifndef VK_DEFINE_NON_DISPATCHABLE_HANDLE #if (VK_USE_64_BIT_PTR_DEFINES==1) #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef pub struct object##_T *object; #else #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef uint64_t object; #endif #endif
                                // </type>
                                Some("define") => {
                                    // FIXME handle requires field, currently hope they are  input in order
                                    let type_code = match ty.spec {
                                        TypeSpec::Code(a) => a,
                                        _ => unreachable!(),
                                    };

                                    let name = match &ty.name {
                                        Some(n) => n,
                                        None => match &type_code.markup[0] {
                                            TypeCodeMarkup::Name(name) => name,
                                            _ => unreachable!(),
                                        },
                                    };

                                    let define = Define {
                                        name: reg.get_or_intern(name),
                                        body: type_code.code.trim().to_owned(),
                                    };
                                    reg.defines.push(define);
                                }
                                // <type category="basetype">
                                //   pub struct
                                //   <name>ANativeWindow</name>
                                //   ;
                                // </type>
                                // <type category="basetype">
                                //   typedef
                                //   <type>uint32_t</type>
                                //   <name>VkSampleMask</name>
                                //   ;
                                // </type>
                                Some("basetype") => {
                                    let type_code = match &ty.spec {
                                        TypeSpec::Code(a) => a,
                                        _ => unreachable!(),
                                    };

                                    // search for a name attribute
                                    let name;
                                    let mut ty = None;
                                    match &type_code.markup[0] {
                                        TypeCodeMarkup::Name(n) => name = n,
                                        TypeCodeMarkup::Type(t) => {
                                            ty = Some(t);
                                            name = match &type_code.markup[1] {
                                                TypeCodeMarkup::Name(n) => n,
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
                                    };
                                    let name = reg.get_or_intern(name);
                                    let ty = ty.map(|s| reg.get_or_intern(&s));

                                    let typ = ToplevelBody::Basetype {
                                        ty,
                                        code: type_code.code.as_str().trim().to_owned(),
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
                                Some("bitmask") => {
                                    if let Some(alias) = ty.alias {
                                        let name = reg.get_or_intern(&ty.name.unwrap());
                                        let typ = ToplevelBody::Alias {
                                            alias_of: reg.get_or_intern(&alias),
                                            kind: ToplevelKind::Bitmask,
                                        };

                                        add_item(
                                            &mut reg.toplevel,
                                            &mut reg.item_map,
                                            Toplevel(name, typ),
                                            name,
                                            ItemKind::Toplevel,
                                        );
                                    } else {
                                        let type_code = match ty.spec {
                                            TypeSpec::Code(a) => a,
                                            _ => unreachable!(),
                                        };

                                        // due to vk.xml being a horror produced from C headers
                                        // bitmasks are actually an enum with the values and a typedef of the actual integer that is passed around vulkan
                                        // as such, this element is for the integer typedef and the values pub enum is listed as a
                                        // 'requires' when it is a 32 bit bitmask or in 'bitvalues' when it is 64 bit
                                        // the bits can be missing if it the flags exist but are so far unused
                                        let bits = if let Some(req) = ty.requires {
                                            Some(req)
                                        } else if let Some(req) = ty.bitvalues {
                                            Some(req)
                                        } else {
                                            None
                                        };

                                        let ty = match &type_code.markup[0] {
                                            TypeCodeMarkup::Type(ty) => ty,
                                            _ => unreachable!(),
                                        };

                                        let name = match &type_code.markup[1] {
                                            TypeCodeMarkup::Name(name) => reg.get_or_intern(name),
                                            _ => unreachable!(),
                                        };

                                        let typ = ToplevelBody::Bitmask {
                                            ty: reg.get_or_intern(ty),
                                            bits_enum: bits.map(|b| reg.get_or_intern(&b)),
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
                                // <type category="handle" objtypepub enum="VK_OBJECT_TYPE_INSTANCE">
                                //   <type>VK_DEFINE_HANDLE</type>
                                //   (
                                //   <name>VkInstance</name>
                                //   )
                                // </type>
                                Some("handle") => {
                                    if let Some(alias) = ty.alias {
                                        let name = reg.get_or_intern(&ty.name.unwrap());
                                        let typ = ToplevelBody::Alias {
                                            alias_of: reg.get_or_intern(&alias),
                                            kind: ToplevelKind::Handle,
                                        };

                                        add_item(
                                            &mut reg.toplevel,
                                            &mut reg.item_map,
                                            Toplevel(name, typ),
                                            name,
                                            ItemKind::Toplevel,
                                        );
                                    } else {
                                        let type_code = match &ty.spec {
                                            TypeSpec::Code(a) => a,
                                            _ => unreachable!(),
                                        };

                                        let dispatchable = match &type_code.markup[0] {
                                            TypeCodeMarkup::Type(string) => match string.as_str() {
                                                "VK_DEFINE_HANDLE" => true,
                                                "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => false,
                                                _ => unreachable!(),
                                            },
                                            _ => unreachable!(),
                                        };

                                        let name = match &type_code.markup[1] {
                                            TypeCodeMarkup::Name(name) => reg.get_or_intern(&name),
                                            _ => unreachable!(),
                                        };

                                        let type_enum = ty.objtypeenum.unwrap();

                                        let typ = ToplevelBody::Handle {
                                            object_type: reg.get_or_intern(&type_enum),
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
                                }
                                // <type name="VkAttachmentLoadOp" category="pub enum"/>
                                Some("enum") => {
                                    if let Some(alias) = ty.alias {
                                        // add_with_name(&mut pub enum_bitmask_aliases, &mut pub enum_bitmask_aliases_map, int.get_or_intern(&alias), &name);
                                        let name = reg.get_or_intern(&ty.name.unwrap());
                                        let typ = ToplevelBody::Alias {
                                            alias_of: reg.get_or_intern(&alias),
                                            kind: ToplevelKind::Enum,
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
                                // <type category="funcpointer">
                                //   typedef void (VKAPI_PTR *
                                //   <name>PFN_vkint.get_or_internalAllocationNotification</name>
                                //     )(
                                //   <type>void</type>
                                //     * pUserData,
                                //   <type>size_t</type>
                                //     size,
                                //   <type>Vkint.get_or_internalAllocationType</type>
                                //     allocationType,
                                //   <type>VkSystemAllocationScope</type>
                                //     allocationScope);
                                // </type>
                                Some("funcpointer") => {
                                    // must parse C code, thanks khronos
                                    // all 9 declarations? follow this pub structure:
                                    // typedef 'return type' (VKAPI_PTR *'ptr type')($('argument type'),*)
                                    let type_code = match ty.spec {
                                        TypeSpec::Code(a) => a,
                                        _ => unreachable!(),
                                    };

                                    let name = match &type_code.markup[0] {
                                        TypeCodeMarkup::Name(string) => reg.get_or_intern(string),
                                        _ => unreachable!(),
                                    };

                                    let mut split = type_code.code.split_ascii_whitespace();
                                    let fun_type = split.nth(1).unwrap();
                                    split.nth(1);

                                    let mut args = Vec::new();
                                    // TODO this is dumb
                                    let mut buffer = String::new();
                                    'outer: for arg in split {
                                        for char in arg.chars() {
                                            match char {
                                                ',' | ')' | ';' => {
                                                    let (name, ty) =
                                                        parse_type(&buffer, true, &reg);

                                                    args.push((name.unwrap(), ty));
                                                    buffer.clear();

                                                    continue 'outer;
                                                }
                                                _ => buffer.push(char),
                                            }
                                        }
                                        buffer.push(' ');
                                    }

                                    let typ = ToplevelBody::Funcpointer {
                                        return_type: parse_type(fun_type, false, &reg).1,
                                        args,
                                    };

                                    add_item(
                                        &mut reg.toplevel,
                                        &mut reg.item_map,
                                        Toplevel(name, typ),
                                        name,
                                        ItemKind::Toplevel,
                                    );
                                }
                                // <type category="pub struct" name="VkBaseOutStructure">
                                //   <member>
                                //    <type>VkStructureType</type>
                                //    <name>sType</name>
                                //   </member>
                                //   <member optional="true">
                                //    pub struct
                                //    <type>VkBaseOutStructure</type>
                                //    *
                                //    <name>pNext</name>
                                //   </member>
                                // </type>
                                category @ Some("struct" | "union") => {
                                    let name = reg.get_or_intern(&ty.name.unwrap());

                                    if let Some(alias) = ty.alias {
                                        let typ = ToplevelBody::Alias {
                                            alias_of: reg.get_or_intern(&alias),
                                            kind: ToplevelKind::Bitmask,
                                        };

                                        add_item(
                                            &mut reg.toplevel,
                                            &mut reg.item_map,
                                            Toplevel(name, typ),
                                            name,
                                            ItemKind::Toplevel,
                                        );
                                    } else {
                                        let members = match ty.spec {
                                            TypeSpec::Members(m) => m,
                                            _ => unreachable!(),
                                        };

                                        let mut vec = Vec::new();
                                        for member in members {
                                            match member {
                                                TypeMember::Comment(_) => {} // TODO,
                                                // TODO include more member information from Definition
                                                // the <member> called sType has a property "values" which is a comma separated list of valid values
                                                // pub structextends is comma separated list of pub structs whose pNext can contain this pub struct
                                                // returnedonly - consider not generating a Default implementation
                                                TypeMember::Definition(m) => {
                                                    let (name, ty) =
                                                        parse_type(&m.code, true, &reg);

                                                    vec.push((name.unwrap(), ty));
                                                }
                                                _ => todo!(),
                                            }
                                        }

                                        let typ = ToplevelBody::Struct {
                                            union: category == Some("union"),
                                            members: vec,
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
                                Some(other) => todo!("{:?}", other), // Some(_) => todo!()
                            }
                        }
                        vk_parse::TypesChild::Comment(string) => {}
                        _ => todo!(),
                    }
                }
            }
            RegistryChild::Enums(e) => {
                match e.kind.as_deref() {
                    // <pub enums name="API Constants" comment="Vulkan hardcoded constants - not an pub enumerated type, part of the header boilerplate">
                    //   <pub enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                    // ..
                    None => {
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let name = reg.get_or_intern(&e.name);

                                    match e.spec {
                                        // <pub enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                                        EnumSpec::Value { mut value, extends } => {
                                            let ty = reg.get_or_intern(&e.type_suffix.unwrap());

                                            // junk like '(~0ULL)' (ie. unsigned long long ie. u64) is not valid rust
                                            // the NOT operator is ! instead of ~ and specifying bit width is not neccessary (I hope)
                                            value.replace("~", "!");
                                            // if the expression is wrapped in parentheses, remove them
                                            if value.chars().next().unwrap() == '(' {
                                                value.pop();
                                                value.remove(0);
                                            }
                                            value.retain(|c| c != 'L' && c != 'L' && c != 'F');

                                            let typ = ToplevelBody::Constant {
                                                ty,
                                                val: reg.get_or_intern(&value),
                                            };

                                            add_item(
                                                &mut reg.toplevel,
                                                &mut reg.item_map,
                                                Toplevel(name, typ),
                                                name,
                                                ItemKind::Toplevel,
                                            );
                                        }
                                        // <pub enum name="VK_LUID_SIZE_KHR" alias="VK_LUID_SIZE"/>
                                        EnumSpec::Alias { alias, extends } => {
                                            let typ = ToplevelBody::Alias {
                                                alias_of: reg.get_or_intern(&alias),
                                                kind: ToplevelKind::Constant,
                                            };

                                            add_item(
                                                &mut reg.toplevel,
                                                &mut reg.item_map,
                                                Toplevel(name, typ),
                                                name,
                                                ItemKind::Toplevel,
                                            );
                                        }
                                        _ => unreachable!(),
                                    };
                                }
                                // useless for binding and only 2 occurences in xml
                                vk_parse::EnumsChild::Unused(_) => {}
                                vk_parse::EnumsChild::Comment(_) => {}
                                _ => todo!(),
                            }
                        }
                    }
                    // actually an pub enum
                    Some("enum") => {
                        let mut members = Vec::new();
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let child_name = reg.get_or_intern(&e.name);
                                    let value = match e.spec {
                                        // EnumSpec::Bitpos { bitpos, extends } => EnumValue::Bitpos(bitpos as u32),
                                        EnumSpec::Value { value, extends } => {
                                            EnumValue::Value(parse_detect_radix(&value))
                                        }
                                        EnumSpec::Alias { alias, extends } => {
                                            EnumValue::Alias(reg.get_or_intern(&alias))
                                        }
                                        _ => unreachable!(),
                                    };
                                    members.push((child_name, value));
                                }
                                vk_parse::EnumsChild::Unused(_) => {}
                                vk_parse::EnumsChild::Comment(_) => {}
                                _ => todo!(),
                            }
                        }

                        let name = reg.get_or_intern(&e.name.unwrap());
                        let typ = ToplevelBody::Enum { members };

                        add_item(
                            &mut reg.toplevel,
                            &mut reg.item_map,
                            Toplevel(name, typ),
                            name,
                            ItemKind::Toplevel,
                        );
                    }
                    // actually an pub enum
                    Some("bitmask") => {
                        let mut members = Vec::new();
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let child_name = reg.get_or_intern(&e.name);
                                    let bitpos = match e.spec {
                                        EnumSpec::Bitpos { bitpos, extends } => {
                                            EnumValue::Bitpos(bitpos as u32)
                                        }
                                        EnumSpec::Value { value, extends } => {
                                            EnumValue::Value(parse_detect_radix(&value))
                                        }
                                        EnumSpec::Alias { alias, extends } => {
                                            EnumValue::Alias(reg.get_or_intern(&alias))
                                        }
                                        _ => unreachable!("{:#?}", e),
                                    };
                                    members.push((child_name, bitpos));
                                }
                                vk_parse::EnumsChild::Unused(_) => {}
                                vk_parse::EnumsChild::Comment(_) => {}
                                _ => todo!(),
                            }
                        }

                        let name = reg.get_or_intern(&e.name.unwrap());
                        let typ = ToplevelBody::BitmaskBits { members };

                        add_item(
                            &mut reg.toplevel,
                            &mut reg.item_map,
                            Toplevel(name, typ),
                            name,
                            ItemKind::Toplevel,
                        );
                    }
                    _ => todo!(),
                }
            }
            RegistryChild::Commands(c) => {
                for child in c.children {
                    match child {
                        vk_parse::Command::Definition(d) => {
                            // does the lib really reconpub struct C code instead of giving me the full parameters??
                            // repurpose the code from 'funcpointer' parsing
                            // TODO cleanup, switch to using xml directly
                            // VkResult  vkCreateInstance (const  VkInstanceCreateInfo*  pCreateInfo , const  VkAllocationCallbacks*  pAllocator ,  VkInstance*  pInstance );

                            let params = d.code.split_terminator('(').nth(1).unwrap();
                            let mut split = params.split_ascii_whitespace();

                            let mut args = Vec::new();
                            // TODO this is dumb
                            let mut buffer = String::new();
                            'outer2: for arg in split {
                                for char in arg.chars() {
                                    match char {
                                        ',' | ')' | ';' => {
                                            let (name, ty) = parse_type(&buffer, true, &reg);

                                            args.push((name.unwrap(), ty));
                                            buffer.clear();

                                            continue 'outer2;
                                        }
                                        _ => buffer.push(char),
                                    }
                                }
                                buffer.push(' ');
                            }

                            let mut params = Vec::with_capacity(d.params.len());
                            for (i, p) in d.params.into_iter().enumerate() {
                                params.push(CommandParameter {
                                    len: p.len,
                                    alt_len: p.altlen,
                                    optional: p.optional.as_deref() == Some("true"),
                                    externsync: p.externsync,
                                    ty: args[i].1.clone(),
                                    name: args[i].0,
                                })
                            }

                            let name = reg.get_or_intern(&d.proto.name);
                            let return_type = d.proto.type_name.unwrap();

                            let typ = ToplevelBody::Command {
                                return_type: reg.get_or_intern(&return_type),
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
                        vk_parse::Command::Alias { name, alias } => {
                            let typ = ToplevelBody::Alias {
                                alias_of: reg.get_or_intern(&alias),
                                kind: ToplevelKind::Command,
                            };
                            let name = reg.get_or_intern(&name);
                            add_item(
                                &mut reg.toplevel,
                                &mut reg.item_map,
                                Toplevel(name, typ),
                                name,
                                ItemKind::Toplevel,
                            );
                        }
                        _ => todo!(),
                    }
                }
            }
            RegistryChild::Feature(f) => {
                let children = convert_extension_children(&f.children, None, &mut reg);

                let name = reg.get_or_intern(&f.name);
                let typ = Feature {
                    name,
                    api: reg.get_or_intern(&f.api),
                    number: reg.get_or_intern(&f.number),
                    protect: f.protect.map(|b| reg.get_or_intern(&b)),
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
            RegistryChild::Extensions(e) => {
                for ext in e.children {
                    assert!(ext.number.is_some());
                    let children = convert_extension_children(&ext.children, ext.number, &mut reg);
                    let deprecatedby = match ext.deprecatedby {
                        Some(s) => {
                            if !s.is_empty() {
                                Some(reg.get_or_intern(&s))
                            } else {
                                None
                            }
                        }
                        None => None,
                    };

                    let name = reg.get_or_intern(&ext.name);
                    let typ = Extension {
                        name,
                        number: ext.number.unwrap() as u32,
                        sortorder: ext.sortorder.map(|n| n as u32),
                        author: ext.author,
                        contact: ext.contact,
                        ext_type: ext.ext_type.map(|s| reg.get_or_intern(&s)),
                        requires: parse_comma_separated(ext.requires.as_deref(), &mut reg),
                        requires_core: ext.requires_core.map(|s| reg.get_or_intern(&s)),
                        protect: ext.protect.map(|s| reg.get_or_intern(&s)),
                        platform: ext.platform.map(|s| reg.get_or_intern(&s)),
                        supported: parse_comma_separated(ext.supported.as_deref(), &mut reg),
                        promotedto: ext.promotedto.map(|s| reg.get_or_intern(&s)),
                        deprecatedby,
                        obsoletedby: ext.obsoletedby.map(|s| reg.get_or_intern(&s)),
                        provisional: ext.provisional,
                        specialuse: parse_comma_separated(ext.specialuse.as_deref(), &mut reg),
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
            RegistryChild::Formats(children) => {
                for f in children.children {
                    let block_extent = f.blockExtent.map(|s| {
                        let mut split = s.split_terminator(',').map(|s| s.parse::<u8>().unwrap());
                        [
                            split.next().unwrap(),
                            split.next().unwrap(),
                            split.next().unwrap(),
                        ]
                    });
                    let chroma = f.chroma.map(|s| match s.as_str() {
                        "420" => YCBREncoding::E420,
                        "422" => YCBREncoding::E422,
                        "444" => YCBREncoding::E444,
                        _ => unreachable!(),
                    });

                    let mut components = Vec::new();
                    let mut planes = Vec::new();
                    let mut spirvimageformats = Vec::new();

                    for child in f.children {
                        match child {
                            FormatChild::Component {
                                name,
                                bits,
                                numericFormat,
                                planeIndex,
                                ..
                            } => {
                                let numeric_format = match numericFormat.as_str() {
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
                                let bits = match bits.as_str() {
                                    "compressed" => None,
                                    _ => Some(bits.parse::<u8>().unwrap()),
                                };
                                components.push(Component {
                                    name: reg.get_or_intern(&name),
                                    bits,
                                    numeric_format,
                                    plane_index: planeIndex,
                                });
                            }
                            FormatChild::Plane {
                                index,
                                widthDivisor,
                                heightDivisor,
                                compatible,
                                ..
                            } => {
                                planes.push(Plane {
                                    index,
                                    width_divisor: widthDivisor,
                                    height_divisor: heightDivisor,
                                    compatible: reg.get_or_intern(&compatible),
                                });
                            }
                            FormatChild::SpirvImageFormat { name, .. } => {
                                spirvimageformats.push(reg.get_or_intern(&name));
                            }
                            _ => todo!(),
                        }
                    }

                    let name = reg.get_or_intern(&f.name);
                    let typ = Format {
                        name,
                        class: reg.get_or_intern(&f.class),
                        blocksize: f.blockSize,
                        texels_per_block: f.texelsPerBlock,
                        block_extent,
                        packed: f.packed,
                        compressed: f.compressed.map(|s| reg.get_or_intern(&s)),
                        chroma,
                        components,
                        planes,
                        spirvimageformats,
                    };

                    if &f.name == "VkGeometryInstanceFlagsKHR" {
                        let a = "help";
                    }

                    add_item(
                        &mut reg.formats,
                        &mut reg.item_map,
                        typ,
                        name,
                        ItemKind::Format,
                    );
                }
            }
            RegistryChild::SpirvExtensions(s) => {
                for ext in s.children {
                    let enables = convert_spirv_enable(&ext.enables, &mut reg);
                    let name = reg.get_or_intern(&ext.name);
                    let typ = SpirvExtCap(name, enables);

                    add_item(
                        &mut reg.spirv_extensions,
                        &mut reg.item_map,
                        typ,
                        name,
                        ItemKind::SpirvExtension,
                    );
                }
            }
            RegistryChild::SpirvCapabilities(s) => {
                for cap in s.children {
                    let enables = convert_spirv_enable(&cap.enables, &mut reg);
                    let name = reg.get_or_intern(&cap.name);
                    let typ = SpirvExtCap(name, enables);

                    add_item(
                        &mut reg.spirv_capabilities,
                        &mut reg.item_map,
                        typ,
                        name,
                        ItemKind::SpirvCapability,
                    );
                }
            }
            _ => todo!(),
        }
    }

    reg
}

fn convert_spirv_enable(enables: &[vk_parse::Enable], reg: &Registry) -> Vec<SpirvEnable> {
    let mut converted = Vec::new();
    for enable in enables {
        let out;
        match enable {
            vk_parse::Enable::Version(v) => out = SpirvEnable::Version(reg.get_or_intern(&v)),
            vk_parse::Enable::Extension(e) => out = SpirvEnable::Extension(reg.get_or_intern(&e)),
            vk_parse::Enable::Feature(f) => {
                out = SpirvEnable::Feature {
                    structure: reg.get_or_intern(&f.struct_),
                    feature: reg.get_or_intern(&f.feature),
                    requires: parse_comma_separated(f.requires.as_deref(), reg),
                    alias: f.alias.as_ref().map(|s| reg.get_or_intern(&s)),
                }
            }
            vk_parse::Enable::Property(p) => {
                out = SpirvEnable::Property {
                    property: reg.get_or_intern(&p.property),
                    member: reg.get_or_intern(&p.member),
                    value: reg.get_or_intern(&p.value),
                    requires: parse_comma_separated(p.requires.as_deref(), reg),
                }
            }
            _ => todo!(),
        }
        converted.push(out);
    }
    converted
}

fn convert_extension_children(
    children: &[ExtensionChild],
    ext_number: Option<i64>,
    reg: &Registry,
) -> Vec<FeatureExtensionItem> {
    let mut converted = Vec::new();
    for child in children {
        match child {
            ExtensionChild::Require {
                api,
                profile,
                extension,
                feature,
                comment,
                items,
            } => {
                if let Some(comment) = comment {
                    converted.push(FeatureExtensionItem::Comment(comment.clone()));
                }

                let items = {
                    let items: &[vk_parse::InterfaceItem] = &items;
                    let mut converted = Vec::new();
                    for item in items {
                        match item {
                            vk_parse::InterfaceItem::Type { name, comment } => {
                                converted.push(InterfaceItem::Simple {
                                    name: reg.get_or_intern(&name),
                                    api: None,
                                })
                            }
                            vk_parse::InterfaceItem::Command { name, comment } => {
                                converted.push(InterfaceItem::Simple {
                                    name: reg.get_or_intern(&name),
                                    api: None,
                                })
                            }
                            vk_parse::InterfaceItem::Enum(e) => {
                                // I don't think this is applicable here as it is already in a <require> which has its own api property
                                // however the spec says "used to address subtle incompatibilities"
                                // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_pub enum_tags
                                assert!(e.api.is_none());

                                let name = reg.get_or_intern(&e.name);
                                match &e.spec {
                                    // just a constant, because of course
                                    EnumSpec::None => {
                                        converted.push(InterfaceItem::Simple { name, api: None })
                                    }
                                    EnumSpec::Offset {
                                        offset,
                                        extends,
                                        extnumber,
                                        dir,
                                    } => {
                                        converted.push(InterfaceItem::Extend {
                                            name,
                                            extends: reg.get_or_intern(&extends),
                                            api: None,
                                            method: ExtendMethod::BitposExtnumber {
                                                // if this is a feature which itself (as opposed to an extension) doesn't have an extumber, extnumber is always defined
                                                extnumber: extnumber.or(ext_number).unwrap() as u32,
                                                offset: if *dir == true {
                                                    *offset
                                                } else {
                                                    -*offset
                                                }
                                                    as i32,
                                            },
                                        })
                                    }
                                    EnumSpec::Bitpos { bitpos, extends } => {
                                        converted.push(InterfaceItem::Extend {
                                            name,
                                            // extends can't be None right? how can a global constant be a bitpos? if yes then copy the EnumSpec::Value case and set value to a bitshifted 1?
                                            extends: reg.get_or_intern(extends.as_ref().unwrap()),
                                            api: None,
                                            method: ExtendMethod::Bitpos(*bitpos as u32),
                                        });
                                    }
                                    EnumSpec::Value { value, extends } => {
                                        match extends {
                                            Some(e) => converted.push(InterfaceItem::Extend {
                                                name,
                                                extends: reg.get_or_intern(e),
                                                api: None,
                                                method: ExtendMethod::Value(value.clone()),
                                            }),
                                            // global constant
                                            None => converted.push(InterfaceItem::AddConstant {
                                                name,
                                                value: ConstantValue::Value(value.clone()),
                                            }),
                                        }
                                    }
                                    EnumSpec::Alias { alias, extends } => {
                                        match extends {
                                            Some(e) => converted.push(InterfaceItem::Extend {
                                                name,
                                                extends: reg.get_or_intern(e),
                                                api: None,
                                                method: ExtendMethod::Alias(
                                                    reg.get_or_intern(alias),
                                                ),
                                            }),
                                            // global constant
                                            None => converted.push(InterfaceItem::AddConstant {
                                                name,
                                                value: ConstantValue::Alias(
                                                    reg.get_or_intern(alias),
                                                ),
                                            }),
                                        }
                                    }
                                    _ => todo!(),
                                }
                            }
                            // TODO consider adding comment to InterfaceItem
                            vk_parse::InterfaceItem::Comment(_) => continue,
                            _ => todo!(),
                        }
                    }
                    converted
                };

                let api = parse_comma_separated(api.as_deref(), reg);

                converted.push(FeatureExtensionItem::Require {
                    profile: profile.as_ref().map(|b| reg.get_or_intern(&b)),
                    api,
                    extension: extension.as_ref().map(|s| reg.get_or_intern(&s)),
                    feature: feature.as_ref().map(|b| reg.get_or_intern(&b)),
                    items,
                })
            }
            ExtensionChild::Remove {
                api,
                profile,
                comment,
                items,
            } => {
                // extensions removing things is dubious at best, thanks to the spec for being explicit
                // "It is unlikely that a type would ever be removed, although this usage is allowed by the schema."
                if let Some(comment) = comment {
                    converted.push(FeatureExtensionItem::Comment(comment.clone()));
                }

                let items = {
                    let items: &[vk_parse::InterfaceItem] = &items;
                    let mut converted = Vec::new();
                    for item in items {
                        let item_name;
                        match item {
                            vk_parse::InterfaceItem::Comment(_) => continue,
                            vk_parse::InterfaceItem::Type { name, comment } => item_name = name,
                            vk_parse::InterfaceItem::Enum(e) => item_name = &e.name,
                            vk_parse::InterfaceItem::Command { name, comment } => item_name = name,
                            _ => todo!(),
                        }
                        converted.push(reg.get_or_intern(&item_name));
                    }
                    converted
                };

                let api = parse_comma_separated(api.as_deref(), reg);

                converted.push(FeatureExtensionItem::Remove {
                    profile: profile.as_ref().map(|b| reg.get_or_intern(&b)),
                    api,
                    items,
                })
            }
            _ => todo!(),
        }
    }
    converted
}

fn parse_comma_separated(what: Option<&str>, reg: &Registry) -> Vec<Spur> {
    match what {
        Some(s) => s
            .split_terminator(',')
            .map(|s| reg.get_or_intern(s))
            .collect(),
        None => Vec::new(),
    }
}

fn parse_detect_radix(what: &str) -> i64 {
    if what.len() > 2 && what[0..2] == (*"0x") {
        i64::from_str_radix(&what[2..], 16).unwrap()
    } else {
        i64::from_str_radix(what, 10).unwrap()
    }
}
