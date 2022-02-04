#![allow(unused)]

use std::{collections::HashMap, io::Write};
// vk.xml documentation
// https://www.khronos.org/registry/vulkan/specs/1.3/registry.html

use interner::UniqueStr;
use vk_parse::{Error, TypeCodeMarkup, TypeSpec};

pub mod c_preprocessor;
pub mod interner;

struct TyId(usize);

impl TyId {
    pub fn from(id: usize) -> Self {
        Self(id)
    }
    pub fn raw(&self) -> usize {
        self.0
    }
}

struct Function {
    name: UniqueStr,
    args: Vec<(UniqueStr, TyId)>,
    return_ty: TyId,
}

struct Member {}

struct Struct {
    fields: Vec<(UniqueStr, TyId)>,
}

struct Enum {
    // (name, extension name, value)
    enumerants: Vec<(UniqueStr, UniqueStr, u64)>,
}

struct GlobalDeclarations {
    // (name, type, extension name, value)
    declarations: Vec<(UniqueStr, UniqueStr, UniqueStr, u64)>,
}

enum Thing {
    GlobalDeclarations(GlobalDeclarations),
    Enum(Enum),
    Struct(Struct),
    Function(Function),
}

struct Extension {
    name: UniqueStr,
    things: Vec<Thing>,
}

enum TypeCategory {
    Basetype,
    Bitmask,
    Define,
    Enum,
    Funcpointer,
    Group,
    Handle,
    Include,
    Struct,
    Union,
}

// struct Type {
//     alias: Option<UniqueStr>,
//     api: Vec<UniqueStr>,
//     category: Option<TypeCategory>,
//     // type name of struct to which this is a handle
//     parent: Option<UniqueStr>,
//     returnedonly: bool,
//     // pnext
//     structureextends: Vec<UniqueStr>,
//     allowduplicate: bool,
//     // if category is Handle, this is the VkObjectType enumerant which corresponds to this type
//     objtypeenum: Option<UniqueStr>
// }

struct Thing2 {
    thing: Thing,
    type_info: UniqueStr,
    extension: UniqueStr,
}

//////

enum Type {
    Included { header: UniqueStr },
}

struct Define {
    name: UniqueStr,
    body: String,
}

use interner::intern;

fn add_with_map<T>(map: &mut HashMap<UniqueStr, u32>, buf: &mut Vec<T>, wha: T, name: &str) {
    let index = TryInto::<u32>::try_into(buf.len()).unwrap();
    buf.push(wha);

    let uniquestr = intern(name);
    map.insert(uniquestr, index);
}

fn process_registry(registry: vk_parse::Registry) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = std::fs::File::create("./output.rs").unwrap();

    let mut vendors = Vec::new();
    let mut platforms = Vec::new();
    let mut tags = Vec::new();
    let mut headers = Vec::new();

    let mut types = Vec::new();
    let mut types_map = HashMap::new();

    let mut defines = Vec::new();

    for node in registry.0 {
        match node {
            vk_parse::RegistryChild::Comment(_) => {}
            vk_parse::RegistryChild::VendorIds(vendors_) => {
                vendors.push(vendors_);
            }
            vk_parse::RegistryChild::Platforms(platforms_) => {
                platforms.push(platforms_);
            }
            vk_parse::RegistryChild::Tags(tags_) => {
                tags.push(tags_);
            }
            vk_parse::RegistryChild::Types(types_) => {
                for type_ in types_.children {
                    match type_ {
                        vk_parse::TypesChild::Type(ty) => {
                            match ty.category.as_ref().map(|a| a.as_str()) {
                                Some("include") => headers.push(intern(&ty.name.unwrap())),
                                Some("") => {
                                    let typ = Type::Included {
                                        header: intern(&ty.requires.unwrap()),
                                    };

                                    add_with_map(
                                        &mut types_map,
                                        &mut types,
                                        typ,
                                        &ty.name.unwrap(),
                                    );
                                }
                                Some("define") => {
                                    let type_code = match ty.spec {
                                        TypeSpec::Code(a) => a,
                                        _ => unreachable!(),
                                    };
                                    let name = match &type_code.markup[0] {
                                        TypeCodeMarkup::Name(name) => name,
                                        _ => unreachable!(),
                                    };
                                    let define = Define {
                                        name: intern(name),
                                        body: type_code.code,
                                    };
                                    defines.push(define);
                                }
                                Some(_) => (),
                                None => (),
                            }
                        }
                        vk_parse::TypesChild::Comment(comment) => writeln!(out, "{}", comment)?,
                        _ => todo!(),
                    }
                }
            }
            vk_parse::RegistryChild::Enums(_) => todo!(),
            vk_parse::RegistryChild::Commands(_) => todo!(),
            vk_parse::RegistryChild::Feature(_) => todo!(),
            vk_parse::RegistryChild::Extensions(_) => todo!(),
            vk_parse::RegistryChild::Formats(_) => todo!(),
            vk_parse::RegistryChild::SpirvExtensions(_) => todo!(),
            vk_parse::RegistryChild::SpirvCapabilities(_) => todo!(),
            _ => todo!(),
        }
        out.write(b"\n")?;
    }

    Ok(())
}
