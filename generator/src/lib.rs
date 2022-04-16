#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    io::Write,
    ops::RangeBounds,
};
// vk.xml documentation
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html

use interner::{intern, UniqueStr};
use type_declaration::{parse_type, TypeDecl};
use vk_parse::{
    EnumSpec, Error, ExtensionChild, FormatChild, RegistryChild, TypeCodeMarkup, TypeMember,
    TypeSpec,
};

pub mod c_preprocessor;
pub mod interner;
pub mod type_declaration;

#[derive(Debug)]
enum TypeKind {
    Handle,
    Constant,
    Enum,
    Bitmask,
    Command,
}

#[derive(Debug)]
struct Type(UniqueStr, TypeBody);

#[derive(Debug)]
enum TypeBody {
    Alias {
        alias_of: UniqueStr,
        kind: TypeKind,
    },
    Included {
        header: UniqueStr,
    },
    Basetype {
        code: String,
    },
    Bitmask {
        ty: UniqueStr,
        bits_enum: Option<UniqueStr>,
    },
    Handle {
        ty: UniqueStr,
        dispatchable: bool,
    },
    Funcpointer {
        return_type: UniqueStr,
        pointer_type: UniqueStr,
        args: Vec<(UniqueStr, TypeDecl)>,
    },
    Struct {
        members: Vec<(UniqueStr, TypeDecl)>,
    },
    Constant {
        ty: UniqueStr,
        val: UniqueStr,
    },
    Enum {
        members: Vec<(EnumValue, UniqueStr)>,
    },
    BitmaskBits {
        members: Vec<(EnumValue, UniqueStr)>,
    },
    Command {
        return_type: UniqueStr,
        params: Vec<CommandParameter>,
    },
}

#[derive(Debug)]
// FIXME kind of ugly, the empty variants of 'bitpos' are value rather than bitpos, and aliases
enum EnumValue {
    Bitpos(u32),
    Value(i64),
    Alias(UniqueStr),
}

#[derive(Debug)]
struct Feature {
    name: UniqueStr,
    api: UniqueStr,
    number: UniqueStr,
    protect: Option<UniqueStr>,
    children: Vec<FeatureExtensionItem>,
}

#[derive(Debug)]
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_attributes_of_extension_tags
struct Extension {
    name: UniqueStr,
    number: u32,
    sortorder: Option<u32>,
    author: Option<String>,
    contact: Option<String>,

    ext_type: Option<UniqueStr>,
    requires: Vec<UniqueStr>,
    requires_core: Option<UniqueStr>,
    protect: Option<UniqueStr>,
    platform: Option<UniqueStr>,
    supported: Vec<UniqueStr>,
    promotedto: Option<UniqueStr>,
    deprecatedby: Option<UniqueStr>,
    obsoletedby: Option<UniqueStr>,
    provisional: bool,
    specialuse: Vec<UniqueStr>,
}

#[derive(Debug)]
enum YCBREncoding {
    E420,
    E422,
    E444,
}

#[derive(Debug)]
enum NumericFormat {
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
struct Component {
    name: UniqueStr,
    bits: Option<u8>, // is none for "compressed"
    numeric_format: NumericFormat,
    plane_index: Option<u8>,
}

#[derive(Debug)]
struct Plane {
    index: u8,
    width_divisor: u8,
    height_divisor: u8,
    compatible: UniqueStr,
}

#[derive(Debug)]
struct Format {
    name: UniqueStr,
    class: UniqueStr,
    blocksize: u8,
    texels_per_block: u8,
    block_extent: Option<[u8; 3]>,
    packed: Option<u8>,
    compressed: Option<UniqueStr>,
    chroma: Option<YCBREncoding>,

    components: Vec<Component>,
    planes: Vec<Plane>,
    spirvimageformats: Vec<UniqueStr>,
}

#[derive(Debug)]
struct CommandParameter {
    len: Option<String>,
    alt_len: Option<String>,
    optional: bool,
    externsync: Option<String>,
    ty: TypeDecl,
    name: UniqueStr,
}

#[derive(Debug)]
struct Define {
    name: UniqueStr,
    body: String,
}

#[derive(Debug)]
enum FeatureExtensionItem {
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

#[derive(Debug)]
enum ExtendMethod {
    Bitpos(u32),
    // extnumber, offset, is positive direction, the notion of a direction when offset could have been negative is stupid
    BitposExtnumber { extnumber: u32, offset: i32 },
    // value can in fact be whatever
    Value(String),
    Alias(UniqueStr),
}

#[derive(Debug)]
enum ConstantValue {
    Value(String),
    Alias(UniqueStr),
}

#[derive(Debug)]
enum InterfaceItem {
    // just importing already finished items
    // for some reason Enums can have an Api condition
    // even though it can already be scoped by the parent <require>, why??
    Simple {
        name: UniqueStr,
        api: Option<UniqueStr>,
    },
    Extend {
        name: UniqueStr,
        extends: UniqueStr,
        api: Option<UniqueStr>,
        method: ExtendMethod,
    },
    AddConstant {
        name: UniqueStr,
        value: ConstantValue,
    },
}

// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#tag-spirvenable
#[derive(Debug)]
enum SpirvEnable {
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
struct SpirvExtensionOrCapability(UniqueStr, Vec<SpirvEnable>);

enum ItemKind {
    Type,
    Feature,
    Extension,
    Format,
    SpirvCapability,
    SpirvExtension,
}

fn add_item<T>(
    vec: &mut Vec<T>,
    map: &mut HashMap<UniqueStr, (u32, ItemKind)>,
    what: T,
    name: UniqueStr,
    kind: ItemKind,
) {
    let index = TryInto::<u32>::try_into(vec.len()).unwrap();
    vec.push(what);

    let none = map.insert(name, (index, kind));

    // assert that no collisions happen
    assert!(none.is_none());
}

pub fn process_registry(registry: vk_parse::Registry) -> Result<(), Box<dyn std::error::Error>> {
    let mut vendors = Vec::new();
    let mut platforms = Vec::new();
    let mut tags = Vec::new();
    let mut headers = Vec::new();
    let mut defines = Vec::new();

    // the objects themselves and a hashmap mapping their names to the object
    // this preserves definition order
    let mut types = Vec::new();
    let mut features = Vec::new();
    let mut extensions = Vec::new();
    let mut formats = Vec::new();
    let mut spirv_capabilities = Vec::new();
    let mut spirv_extensions = Vec::new();

    let mut object_map = HashMap::new();

    for node in registry.0 {
        match node {
            RegistryChild::Comment(_) => {}
            RegistryChild::VendorIds(vendors_) => {
                vendors.push(vendors_);
            }
            RegistryChild::Platforms(platforms_) => {
                platforms.push(platforms_);
            }
            RegistryChild::Tags(tags_) => {
                tags.push(tags_);
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
                                        headers.push(intern(&name));
                                    } else {
                                        match ty.spec {
                                            TypeSpec::Code(code) => {
                                                // #include "vk_platform.h"
                                                let name = code
                                                    .code
                                                    .split_terminator("\"")
                                                    .nth(1)
                                                    .unwrap();
                                                headers.push(intern(name));
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
                                    let name = intern(&ty.name.unwrap());

                                    let header = match name.as_str() {
                                        "int" => "",
                                        _ => ty.requires.as_ref().unwrap(),
                                    };

                                    let typ = TypeBody::Included {
                                        header: intern(header),
                                    };

                                    add_item(
                                        &mut types,
                                        &mut object_map,
                                        Type(name, typ),
                                        name,
                                        ItemKind::Type,
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
                                //   #ifndef VK_DEFINE_NON_DISPATCHABLE_HANDLE #if (VK_USE_64_BIT_PTR_DEFINES==1) #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef struct object##_T *object; #else #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef uint64_t object; #endif #endif
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
                                        name: intern(name),
                                        body: type_code.code,
                                    };
                                    defines.push(define);
                                }
                                // <type category="basetype">
                                //   struct
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
                                    let name = match &type_code.markup[0] {
                                        TypeCodeMarkup::Name(name) => name,
                                        _ => match &type_code.markup[1] {
                                            TypeCodeMarkup::Name(name) => name,
                                            _ => unreachable!(),
                                        },
                                    };
                                    let name = intern(name);

                                    let typ = TypeBody::Basetype {
                                        code: type_code.code.clone(),
                                    };

                                    add_item(
                                        &mut types,
                                        &mut object_map,
                                        Type(name, typ),
                                        name,
                                        ItemKind::Type,
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
                                        let name = intern(&ty.name.unwrap());
                                        let typ = TypeBody::Alias {
                                            alias_of: intern(&alias),
                                            kind: TypeKind::Bitmask,
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
                                        );
                                    } else {
                                        let type_code = match ty.spec {
                                            TypeSpec::Code(a) => a,
                                            _ => unreachable!(),
                                        };

                                        // due to vk.xml being a horror produced from C headers
                                        // bitmasks are actually an enum with the values
                                        // and a typedef of the actual integer that is passed around vulkan
                                        // as such this element is for the integer typedef and the values enum is
                                        // listed as a 'requires' when it is a 32 bit bitmask or in 'bitvalues' when it is 64 bit
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
                                            TypeCodeMarkup::Name(name) => intern(name),
                                            _ => unreachable!(),
                                        };

                                        let typ = TypeBody::Bitmask {
                                            ty: intern(ty),
                                            bits_enum: bits.map(|b| intern(&b)),
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
                                        );
                                    }
                                }
                                // <type category="handle" objtypeenum="VK_OBJECT_TYPE_INSTANCE">
                                //   <type>VK_DEFINE_HANDLE</type>
                                //   (
                                //   <name>VkInstance</name>
                                //   )
                                // </type>
                                Some("handle") => {
                                    if let Some(alias) = ty.alias {
                                        let name = intern(&ty.name.unwrap());
                                        let typ = TypeBody::Alias {
                                            alias_of: intern(&alias),
                                            kind: TypeKind::Handle,
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
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
                                            TypeCodeMarkup::Name(name) => intern(&name),
                                            _ => unreachable!(),
                                        };

                                        let type_enum = ty.objtypeenum.unwrap();

                                        let typ = TypeBody::Handle {
                                            ty: intern(&type_enum),
                                            dispatchable,
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
                                        );
                                    }
                                }
                                // <type name="VkAttachmentLoadOp" category="enum"/>
                                Some("enum") => {
                                    if let Some(alias) = ty.alias {
                                        // add_with_name(&mut enum_bitmask_aliases, &mut enum_bitmask_aliases_map, intern(&alias), &name);
                                        let name = intern(&ty.name.unwrap());
                                        let typ = TypeBody::Alias {
                                            alias_of: intern(&alias),
                                            kind: TypeKind::Enum,
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
                                        );
                                    }
                                }
                                // <type category="funcpointer">
                                //   typedef void (VKAPI_PTR *
                                //   <name>PFN_vkInternalAllocationNotification</name>
                                //     )(
                                //   <type>void</type>
                                //     * pUserData,
                                //   <type>size_t</type>
                                //     size,
                                //   <type>VkInternalAllocationType</type>
                                //     allocationType,
                                //   <type>VkSystemAllocationScope</type>
                                //     allocationScope);
                                // </type>
                                Some("funcpointer") => {
                                    // must parse C code, thanks khronos
                                    // all 9 declarations? follow this structure:
                                    // typedef 'return type' (VKAPI_PTR *'ptr type')($('argument type'),)
                                    let type_code = match ty.spec {
                                        TypeSpec::Code(a) => a,
                                        _ => unreachable!(),
                                    };

                                    let mut split = type_code.code.split_ascii_whitespace();
                                    let fun_type = split.nth(1).unwrap();
                                    let ptr_type = split.nth(1).unwrap();
                                    // TODO ptr_type is probably not the correct thing to use
                                    let name = intern(ptr_type);

                                    let mut args = Vec::new();
                                    // TODO this is dumb
                                    let mut buffer = String::new();
                                    'outer: for arg in split {
                                        for char in arg.chars() {
                                            match char {
                                                ',' | ')' | ';' => {
                                                    let (name, ty) = parse_type(&buffer);

                                                    args.push((name, ty));
                                                    buffer.clear();

                                                    continue 'outer;
                                                }
                                                _ => buffer.push(char),
                                            }
                                        }
                                    }

                                    let typ = TypeBody::Funcpointer {
                                        return_type: intern(fun_type),
                                        pointer_type: intern(ptr_type),
                                        args,
                                    };

                                    add_item(
                                        &mut types,
                                        &mut object_map,
                                        Type(name, typ),
                                        name,
                                        ItemKind::Type,
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
                                Some("struct") => {
                                    let name = intern(&ty.name.unwrap());

                                    if let Some(alias) = ty.alias {
                                        let typ = TypeBody::Alias {
                                            alias_of: intern(&alias),
                                            kind: TypeKind::Bitmask,
                                        };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
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
                                                // structextends is comma separated list of structs whose pNext can contain this struct
                                                // returnedonly - consider not generating a Default implementation
                                                TypeMember::Definition(m) => {
                                                    let (name, ty) = parse_type(&m.code);

                                                    vec.push((name, ty));
                                                }
                                                _ => todo!(),
                                            }
                                        }

                                        let typ = TypeBody::Struct { members: vec };

                                        add_item(
                                            &mut types,
                                            &mut object_map,
                                            Type(name, typ),
                                            name,
                                            ItemKind::Type,
                                        );
                                    }
                                }
                                Some(_) => {}
                            }
                        }
                        vk_parse::TypesChild::Comment(string) => {}
                        _ => todo!(),
                    }
                }
            }
            RegistryChild::Enums(e) => {
                match e.kind.as_ref().map(|s| s.as_str()) {
                    // <enums name="API Constants" comment="Vulkan hardcoded constants - not an enumerated type, part of the header boilerplate">
                    //   <enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                    // ..
                    None => {
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let name = intern(&e.name);

                                    match e.spec {
                                        // <enum type="uint32_t" value="256" name="VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"/>
                                        EnumSpec::Value { mut value, extends } => {
                                            let ty = intern(&e.type_suffix.unwrap());

                                            // junk like '(~0ULL)' is not valid rust
                                            // the NOT operator is ! instead of ~ and specifying bit width is not neccessary (I hope)
                                            value.replace("~", "!");
                                            // if the expression is wrapped in parentheses, remove them
                                            if value.chars().next().unwrap() == '(' {
                                                value.pop();
                                                value.remove(0);
                                            }
                                            value.retain(|c| c != 'L' && c != 'L' && c != 'F');

                                            let typ = TypeBody::Constant {
                                                ty,
                                                val: intern(&value),
                                            };

                                            add_item(
                                                &mut types,
                                                &mut object_map,
                                                Type(name, typ),
                                                name,
                                                ItemKind::Type,
                                            );
                                        }
                                        // <enum name="VK_LUID_SIZE_KHR" alias="VK_LUID_SIZE"/>
                                        EnumSpec::Alias { alias, extends } => {
                                            let typ = TypeBody::Alias {
                                                alias_of: intern(&alias),
                                                kind: TypeKind::Constant,
                                            };

                                            add_item(
                                                &mut types,
                                                &mut object_map,
                                                Type(name, typ),
                                                name,
                                                ItemKind::Type,
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
                    // actually an enum
                    Some("enum") => {
                        let mut members = Vec::new();
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let child_name = intern(&e.name);
                                    let value = match e.spec {
                                        // EnumSpec::Bitpos { bitpos, extends } => EnumValue::Bitpos(bitpos as u32),
                                        EnumSpec::Value { value, extends } => {
                                            EnumValue::Value(parse_detect_radix(&value))
                                        }
                                        EnumSpec::Alias { alias, extends } => {
                                            EnumValue::Alias(intern(&alias))
                                        }
                                        _ => unreachable!(),
                                    };
                                    members.push((value, child_name));
                                }
                                vk_parse::EnumsChild::Unused(_) => {}
                                vk_parse::EnumsChild::Comment(_) => {}
                                _ => todo!(),
                            }
                        }

                        let name = intern(&e.name.unwrap());
                        let typ = TypeBody::Enum { members };

                        add_item(
                            &mut types,
                            &mut object_map,
                            Type(name, typ),
                            name,
                            ItemKind::Type,
                        );
                    }
                    // actually an enum
                    Some("bitmask") => {
                        let mut members = Vec::new();
                        for child in e.children {
                            match child {
                                vk_parse::EnumsChild::Enum(e) => {
                                    let child_name = intern(&e.name);
                                    let bitpos = match e.spec {
                                        EnumSpec::Bitpos { bitpos, extends } => {
                                            EnumValue::Bitpos(bitpos as u32)
                                        }
                                        EnumSpec::Value { value, extends } => {
                                            EnumValue::Value(parse_detect_radix(&value))
                                        }
                                        EnumSpec::Alias { alias, extends } => {
                                            EnumValue::Alias(intern(&alias))
                                        }
                                        _ => unreachable!("{:#?}", e),
                                    };
                                    members.push((bitpos, child_name));
                                }
                                vk_parse::EnumsChild::Unused(_) => {}
                                vk_parse::EnumsChild::Comment(_) => {}
                                _ => todo!(),
                            }
                        }

                        let name = intern(&e.name.unwrap());
                        let typ = TypeBody::BitmaskBits { members };

                        add_item(
                            &mut types,
                            &mut object_map,
                            Type(name, typ),
                            name,
                            ItemKind::Type,
                        );
                    }
                    _ => todo!(),
                }
            }
            RegistryChild::Commands(c) => {
                for child in c.children {
                    match child {
                        vk_parse::Command::Definition(d) => {
                            // does the lib really reconstruct C code instead of giving me the full parameters??
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
                                            let (name, ty) = parse_type(&buffer);

                                            args.push((name, ty));
                                            buffer.clear();

                                            continue 'outer2;
                                        }
                                        _ => buffer.push(char),
                                    }
                                }
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

                            let name = intern(&d.proto.name);
                            let return_type = d.proto.type_name.unwrap();

                            let typ = TypeBody::Command {
                                return_type: intern(&return_type),
                                params,
                            };

                            add_item(
                                &mut types,
                                &mut object_map,
                                Type(name, typ),
                                name,
                                ItemKind::Type,
                            );
                        }
                        vk_parse::Command::Alias { name, alias } => {
                            let typ = TypeBody::Alias {
                                alias_of: intern(&alias),
                                kind: TypeKind::Command,
                            };
                            let name = intern(&name);
                            add_item(
                                &mut types,
                                &mut object_map,
                                Type(name, typ),
                                name,
                                ItemKind::Type,
                            );
                        }
                        _ => todo!(),
                    }
                }
            }
            RegistryChild::Feature(f) => {
                let children = convert_extension_children(&f.children, None);

                let name = intern(&f.name);
                let typ = Feature {
                    name,
                    api: intern(&f.api),
                    number: intern(&f.number),
                    protect: f.protect.map(|b| intern(&b)),
                    children,
                };

                add_item(&mut features, &mut object_map, typ, name, ItemKind::Feature);
            }
            RegistryChild::Extensions(e) => {
                for ext in e.children {
                    assert!(ext.number.is_some());
                    let children = convert_extension_children(&ext.children, ext.number);

                    let name = intern(&ext.name);
                    let typ = Extension {
                        name,
                        number: ext.number.unwrap() as u32,
                        sortorder: ext.sortorder.map(|n| n as u32),
                        author: ext.author,
                        contact: ext.contact,
                        ext_type: ext.ext_type.map(|s| intern(&s)),
                        requires: intern_comma_separated(ext.requires.as_deref()),
                        requires_core: ext.requires_core.map(|s| intern(&s)),
                        protect: ext.protect.map(|s| intern(&s)),
                        platform: ext.platform.map(|s| intern(&s)),
                        supported: intern_comma_separated(ext.supported.as_deref()),
                        promotedto: ext.promotedto.map(|s| intern(&s)),
                        deprecatedby: ext.deprecatedby.map(|s| intern(&s)),
                        obsoletedby: ext.obsoletedby.map(|s| intern(&s)),
                        provisional: ext.provisional,
                        specialuse: intern_comma_separated(ext.specialuse.as_deref()),
                    };

                    add_item(
                        &mut extensions,
                        &mut object_map,
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
                                    name: intern(&name),
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
                                    compatible: intern(&compatible),
                                });
                            }
                            FormatChild::SpirvImageFormat { name, .. } => {
                                spirvimageformats.push(intern(&name));
                            }
                            _ => todo!(),
                        }
                    }

                    let name = intern(&f.name);
                    let typ = Format {
                        name,
                        class: intern(&f.class),
                        blocksize: f.blockSize,
                        texels_per_block: f.texelsPerBlock,
                        block_extent,
                        packed: f.packed,
                        compressed: f.compressed.map(|s| intern(&s)),
                        chroma,
                        components,
                        planes,
                        spirvimageformats,
                    };

                    add_item(&mut formats, &mut object_map, typ, name, ItemKind::Format);
                }
            }
            RegistryChild::SpirvExtensions(s) => {
                for ext in s.children {
                    let enables = convert_spirv_enable(&ext.enables);
                    let name = intern(&ext.name);
                    let typ = SpirvExtensionOrCapability(name, enables);

                    add_item(
                        &mut spirv_extensions,
                        &mut object_map,
                        typ,
                        name,
                        ItemKind::SpirvExtension,
                    );
                }
            }
            RegistryChild::SpirvCapabilities(s) => {
                for cap in s.children {
                    let enables = convert_spirv_enable(&cap.enables);
                    let name = intern(&cap.name);
                    let typ = SpirvExtensionOrCapability(name, enables);

                    add_item(
                        &mut spirv_capabilities,
                        &mut object_map,
                        typ,
                        name,
                        ItemKind::SpirvCapability,
                    );
                }
            }
            _ => todo!(),
        }
    }

    println!("{:#?}", types);
    println!("{:#?}", features);
    println!("{:#?}", extensions);
    println!("{:#?}", formats);
    println!("{:#?}", spirv_extensions);
    println!("{:#?}", spirv_capabilities);

    Ok(())
}

fn convert_spirv_enable(enables: &[vk_parse::Enable]) -> Vec<SpirvEnable> {
    let mut converted = Vec::new();
    for enable in enables {
        let out;
        match enable {
            vk_parse::Enable::Version(v) => out = SpirvEnable::Version(intern(&v)),
            vk_parse::Enable::Extension(e) => out = SpirvEnable::Extension(intern(&e)),
            vk_parse::Enable::Feature(f) => {
                out = SpirvEnable::Feature {
                    structure: intern(&f.struct_),
                    feature: intern(&f.feature),
                    requires: intern_comma_separated(f.requires.as_deref()),
                    alias: f.alias.as_ref().map(|s| intern(&s)),
                }
            }
            vk_parse::Enable::Property(p) => {
                out = SpirvEnable::Property {
                    property: intern(&p.property),
                    member: intern(&p.member),
                    value: intern(&p.value),
                    requires: intern_comma_separated(p.requires.as_deref()),
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
                                    name: intern(&name),
                                    api: None,
                                })
                            }
                            vk_parse::InterfaceItem::Command { name, comment } => {
                                converted.push(InterfaceItem::Simple {
                                    name: intern(&name),
                                    api: None,
                                })
                            }
                            vk_parse::InterfaceItem::Enum(e) => {
                                // I don't think this is applicable here as it is already in a <require> which has its own api property
                                // however the spec says "used to address subtle incompatibilities"
                                // https://www.khronos.org/registry/vulkan/specs/1.3/registry.html#_enum_tags
                                assert!(e.api.is_none());

                                let name = intern(&e.name);
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
                                            extends: intern(&extends),
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
                                            extends: intern(extends.as_ref().unwrap()),
                                            api: None,
                                            method: ExtendMethod::Bitpos(*bitpos as u32),
                                        });
                                    }
                                    EnumSpec::Value { value, extends } => {
                                        match extends {
                                            Some(e) => converted.push(InterfaceItem::Extend {
                                                name,
                                                extends: intern(e),
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
                                                extends: intern(e),
                                                api: None,
                                                method: ExtendMethod::Alias(intern(alias)),
                                            }),
                                            // global constant
                                            None => converted.push(InterfaceItem::AddConstant {
                                                name,
                                                value: ConstantValue::Alias(intern(alias)),
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

                let api = intern_comma_separated(api.as_deref());

                converted.push(FeatureExtensionItem::Require {
                    profile: profile.as_ref().map(|b| intern(&b)),
                    api,
                    extension: extension.as_ref().map(|s| intern(&s)),
                    feature: feature.as_ref().map(|b| intern(&b)),
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
                        converted.push(intern(&item_name));
                    }
                    converted
                };

                let api = intern_comma_separated(api.as_deref());

                converted.push(FeatureExtensionItem::Remove {
                    profile: profile.as_ref().map(|b| intern(&b)),
                    api,
                    items,
                })
            }
            _ => todo!(),
        }
    }
    converted
}

fn intern_comma_separated(what: Option<&str>) -> Vec<UniqueStr> {
    match what {
        Some(s) => s.split_terminator(',').map(|s| intern(s)).collect(),
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
