#![allow(unused)]
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use generator_lib::{
    code, process_registry,
    type_declaration::{TypeDecl, TypeToken},
    EnumValue, InterfaceItem, ItemKind, Registry, Toplevel, ToplevelBody, ToplevelKind,
};
use lasso::{Rodeo, Spur};

use generator_lib::format_utils::{RegistryDisplay, RegistryWrap, Separated, WithRegistry};

fn main() -> Result<(), Box<dyn Error>> {
    let (reg, errors) = vk_parse::parse_file(&Path::new("/home/eg/Downloads/vk.xml")).unwrap();
    if !errors.is_empty() {
        eprintln!("{:#?}", errors);
    }

    let bindgen_file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/bindgen.c"
    ))
    .unwrap();

    let rust_file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/generated.rs"
    ))
    .unwrap();

    let reg = process_registry(reg);

    // let resolve = |key| {
    //     reg.resolve(key)
    // };

    // let int = &reg;

    // todo filter items by maximum api version, extensions, and all the other <requires> stuff
    // esentially crawl through all the possible definition places and reject those that don't match the hard limits
    // and refcount all the global items, then emit additional things like when apis can add constants and annotate
    // them with the accumulated required properties, for example a constant in VK 1.1 would get a requirement api=1.1
    // this should be reconsidered though as it would be nice to keep these in different files and would allow us to reject
    // api definitions at a rough granuality first without shoving it all into the compiler

    let mut include_headers = Vec::new();
    let mut c = BufWriter::new(bindgen_file);
    let mut rust = BufWriter::new(rust_file);

    // in vulkan bitmasks are implemented as a typedef of an integer that serves as the actual object of the bitmask
    // and a distinct enum that hold the various bitflags, with rust we want to have only the integer with the flags as associated values
    // this needs to know which Bitmask a BitmaskBits specifies the bits for
    let mut bitmask_pairing = HashMap::new();
    for item in &reg.toplevel {
        let name = item.0;
        match item.1 {
            ToplevelBody::Bitmask { ty, bits_enum } => {
                if let Some(bits_enum) = bits_enum {
                    bitmask_pairing.insert(bits_enum, name);
                }
            }
            _ => {}
        }
    }

    for item in &reg.toplevel {
        let name = item.0;
        match &item.1 {
            ToplevelBody::Alias { alias_of, kind } => match kind {
                ToplevelKind::Command => {
                    code!(
                        rust,
                        "// TODO alias of command '{}'"
                        @ name.reg(&reg)
                    )?;
                }
                ToplevelKind::Constant => {
                    let item = resolve_alias(*alias_of, ToplevelKind::Constant, &reg);
                    let (ty, val) = match item.1 {
                        ToplevelBody::Constant { ty, val } => (ty, val),
                        _ => unreachable!(),
                    };

                    code!(
                        rust,
                        "pub const {}: {} = {};"
                        @ name.reg(&reg), ty.reg(&reg), alias_of.reg(&reg)
                    )?;
                }
                _ => {
                    code!(
                        rust,
                        "pub type {} = {};"
                        @ name.reg(&reg), alias_of.reg(&reg)
                    )?;
                }
            },
            ToplevelBody::Included { header } => {
                // FIXME inefficient but I want to preserve definition order
                if include_headers.iter().find(|h| **h == *header).is_none() {
                    include_headers.push(*header);
                }
            }
            // this is most often just a typedef
            // TODO parse the typedef (and replace it with an alias) so that we have more complete information
            ToplevelBody::Basetype { ty, code } => {
                if let Some(ty) = ty {
                    code!(
                        rust,
                        "pub type {} = {};"
                        @ name.reg(&reg), ty.reg(&reg)
                    )?;
                } else {
                    c.write(code.as_bytes());
                    c.write(b"\n");
                }
            }
            ToplevelBody::Bitmask { ty, bits_enum } => {
                code!(
                    rust,
                    "#[repr(transparent)]"
                    "pub struct {}(pub {});"
                    @ name.reg(&reg), ty.reg(&reg)
                )?;
            }
            ToplevelBody::Handle {
                object_type,
                dispatchable,
            } => {
                let dispatchable = match dispatchable {
                    true => "dispatchable, ",
                    false => "",
                };
                code!(
                    rust,
                    "/// {}{}"
                    "#[repr(transparent)]"
                    "pub struct {}(u64);"
                    @ dispatchable, object_type.reg(&reg), name.reg(&reg)
                )?;
            }
            ToplevelBody::Funcpointer { return_type, args } => {
                let ret = return_type.as_rust_order(&reg);
                let args = args.iter().map(|(_, n)| n.as_rust_order(&reg));
                code!(
                    rust,
                    "pub type {} = fn({}) -> {};"
                    @ name.reg(&reg), Separated::args(args).reg(&reg), ret.reg(&reg)
                )?;
            }
            ToplevelBody::Struct { union, members } => {
                let keyword = match union {
                    true => "union",
                    false => "struct",
                };
                let resolved_members = resolve_bitfield_members(&members, &reg);
                let members = resolved_members.iter().map(|(name, ty)| {
                    // FIXME this allocates strings, also its sooo ugly
                    let name = match name {
                        SpurOrString::String(s) => s.to_owned(),
                        SpurOrString::Spur(s) => match resolve_ident(&*reg.resolve(s), &reg) {
                            SpurOrString::String(s) => s,
                            SpurOrString::Spur(s) => reg.resolve(&s).to_owned(),
                        },
                    };
                    let ty = ty.as_rust_order(&reg);
                    format!("{}: {}", name, ty.reg(&reg))
                });
                code!(
                    rust,
                    "pub {} {} {{"
                    "    {}"
                    "}}"
                    @ keyword, name.reg(&reg), Separated::members(members)
                )?;
            }
            ToplevelBody::Constant { ty, val } => {
                code!(
                    rust,
                    "pub const {}: {} = {};"
                    @ name.reg(&reg), ty.reg(&reg), val.reg(&reg)
                )?;
            }
            ToplevelBody::Enum { members } => {
                let members = members.iter().map(|(member_name, val)| {
                    let name = make_enum_member_rusty(name, *member_name, false, &reg);
                    // FIXME this allocates strings
                    let name = match resolve_ident(&name, &reg) {
                        SpurOrString::String(s) => s,
                        SpurOrString::Spur(s) => reg.resolve(&s).to_owned(),
                    };
                    let val = match val {
                        EnumValue::Bitpos(pos) => format!("{} = 1 << {}", name, pos),
                        EnumValue::Value(val) => format!("{} = {}", name, val),
                        EnumValue::Alias(alias) => format!("{} = Self::{}", name, alias.reg(&reg)),
                    };
                    val
                });
                code!(
                    rust,
                    "pub enum {} {{"
                    "    {}"
                    "}}"
                    @ name.reg(&reg), Separated::members(members)
                )?;
            }
            ToplevelBody::BitmaskBits { members } => {
                // when vulkan has a Bitmask that is reserved for future use and thus has no actual flag values, there are no BitmaskBits defined and the spec omits Bitmask dependency
                // however of course there are exceptions such as VkSemaphoreCreateFlagBits which is not declared as a dependency but is actually an item :(
                let bitmask_name = match bitmask_pairing.get(&name) {
                    Some(n) => n,
                    None => continue,
                };
                // skip generating empty impl blocks
                if members.is_empty() {
                    continue;
                }

                let ty = get_concrete_type(*bitmask_name, &reg);
                let members = members.iter().map(|(member_name, val)| {
                    let name = make_enum_member_rusty(name, *member_name, true, &reg);
                    let val = match val {
                        EnumValue::Bitpos(pos) => {
                            format!("const {}: {} = 1 << {}", name, ty.reg(&reg), pos)
                        }
                        EnumValue::Value(val) => {
                            format!("const {}: {} = {}", name, ty.reg(&reg), val)
                        }
                        EnumValue::Alias(alias) => format!(
                            "const {}: {} = Self::{}",
                            name,
                            ty.reg(&reg),
                            alias.reg(&reg)
                        ),
                    };
                    val
                });
                code!(
                    rust,
                    "impl {} {{"
                    "    {}"
                    "}}"
                    @ bitmask_name.reg(&reg), Separated::members(members)
                )?;
            }
            ToplevelBody::Command {
                return_type,
                params,
            } => {}
        }
    }

    for header in include_headers {
        code!(
            c,
            "#include \"{}\"\n"
            @ header.reg(&reg)
        )?;
    }

    Ok(())
}

// this whole functions sucks
fn resolve_ident<'a>(str: &str, reg: &Registry) -> SpurOrString {
    match str {
        // TODO consider using kind
        "type" => SpurOrString::String("type_".to_owned()),
        _ => {
            if str.chars().next().unwrap().is_ascii_digit() {
                // TODO do something better, perhaps switch the digit with the next char and then fallback to this
                let mut string = "_".to_owned();
                string += str;
                SpurOrString::String(string)
            } else if let Some(spur) = reg.get(str) {
                SpurOrString::Spur(spur)
            } else {
                SpurOrString::String(str.to_owned())
            }
        }
    }
}

// resolve_bitfield_members can create new identifiers and borrowchecker doesn't like it that
// it would need mutable acess to Registry to intern new strings, FIXME
enum SpurOrString {
    String(String),
    Spur(Spur),
}

fn resolve_bitfield_members(
    members: &[(Spur, TypeDecl)],
    reg: &Registry,
) -> Vec<(SpurOrString, TypeDecl)> {
    let mut resolved = Vec::new();
    let mut last_ty: Option<Vec<TypeToken>> = None;
    let mut current_bits = 0;
    let mut max_bits = 0;
    let mut merged_members: Vec<Spur> = Vec::new();

    for (name, ty) in members {
        // the type matches and it is a bitfield
        if Some(&ty.tokens) == last_ty.as_ref() && ty.bitfield_len.is_some() {
            let bits = ty.bitfield_len.unwrap();
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
        if let Some(tokens) = last_ty.take() {
            assert!(!merged_members.is_empty());

            // TODO consider some better naming rather than just concatenating everything
            let name = if merged_members.len() == 1 {
                SpurOrString::Spur(merged_members[0])
            } else {
                let mut concat = reg.resolve(&merged_members[0]).to_owned();
                for member in &merged_members[1..] {
                    concat += "_";
                    concat += &*reg.resolve(&member);
                }
                SpurOrString::String(concat)
            };
            resolved.push((
                name,
                TypeDecl {
                    tokens,
                    array_len: None,
                    bitfield_len: Some(current_bits),
                },
            ));
            merged_members.clear();
        }

        // start accumulating a new type, if it isn't a bitfield, we add it to the resolved vec straight away,
        // since last_ty is still None, the next member that comes skips both of the block above and can either
        // start accumulating because it is a bitfield or is again just passed through to resolved
        if let Some(bits) = ty.bitfield_len {
            let type_bits = match ty.tokens[0] {
                TypeToken::Ident(ty) => match &*reg.resolve(&get_concrete_type(ty, reg)) {
                    "uint32_t" | "int32_t" | "VkFlags" => 32,
                    "uint64_t" | "int64_t" | "VkFlags64" => 64,
                    other => todo!("Add another type ('{}') to this match", other),
                },
                // microsoft (https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170) says that only primitive types
                // can be bitfields, in practice this means that the type tokens will be just an ident, also in the spec only one?
                // type is ever used for bitfield so here will be a little hardcoded lookup from type name to bit width,
                // we don't really ever have good quality information about type layout as we expect bindgen to do it for us
                _ => unreachable!(),
            };
            assert!(bits <= type_bits);
            max_bits = type_bits;
            current_bits = bits;
            last_ty = Some(ty.tokens.clone());
            merged_members.push(*name);
        } else {
            resolved.push((SpurOrString::Spur(*name), ty.clone()));
        }
    }

    resolved
}

// it seems that resolve is my new favorite word
// jumps through as many aliases is needed and returns the concrete toplevel item,
// in cases where the provided identifier is not and alias it is simply returned back
fn resolve_alias(alias: Spur, kind: ToplevelKind, registry: &Registry) -> &Toplevel {
    let mut alias = alias;
    loop {
        // what if the alias is to something that is added by bindgen and is never in our registry?
        let &(index, ty) = registry
            .item_map
            .get(&alias)
            .expect("Alias is dangling, see comment.");
        // assert that no weird things are going on as other interface items can't alias this way
        assert!(ty == ItemKind::Toplevel);

        let top = &registry.toplevel[index as usize];
        match top.1 {
            ToplevelBody::Alias {
                alias_of,
                kind: alias_kind,
            } => {
                assert!(kind == alias_kind);
                alias = alias_of;
            }
            _ => return top,
        }
    }
}

// type is roughly synonymous with a toplevel item, the difference between this and resolve_alias
// is that this also jumps through things preserving the type layout, such as handles or constants
// this will stop at typedefs as those are handled by bindgen and thus don't have information about
fn get_concrete_type(toplevel: Spur, reg: &Registry) -> Spur {
    let mut toplevel = toplevel;
    loop {
        // what if the alias is to something that is added by bindgen and is never in our registry?
        // bindgen will result in more common types like uint32_t or VkFlags that can be matched by the caller
        let &(index, ty) = match reg.item_map.get(&toplevel) {
            Some(some) => some,
            None => return toplevel,
        };
        // assert that no weird things are going on as other interface items can't alias this way
        if ty != ItemKind::Toplevel {
            let b = &*reg.resolve(&toplevel);
            let a = "s";
        }

        let top = &reg.toplevel[index as usize];
        match &top.1 {
            ToplevelBody::Alias { alias_of, kind: alias_kind } => toplevel = *alias_of,
            ToplevelBody::Bitmask { ty, bits_enum } => toplevel = *ty,
            ToplevelBody::Handle { object_type, dispatchable } => toplevel = reg.get("uint64_t").unwrap(),
            ToplevelBody::Constant { ty, val } => toplevel = *ty,
            ToplevelBody::Included { .. } |
            ToplevelBody::Basetype { .. } |
            ToplevelBody::Struct { .. } |
            ToplevelBody::Funcpointer { .. } => return toplevel,
            ToplevelBody::Command { .. } => unreachable!(),
            // even though it's called bits, it's just an enum with different semantics
            ToplevelBody::BitmaskBits { .. } |
            ToplevelBody::Enum { .. } => unreachable!("This doesn't really make sense as in vulkan 'enum's only have integer values but no types."),
        }
    }
}

// enum VkPresentModeKHR {
//     VK_PRESENT_MODE_IMMEDIATE_KHR = 0, -> Immediate = 0,
//     ..
// }
//
// this will fuzzy match on the member name and strip the enum name and the extension tag boilerplate
// we also have this beauty, so we will have to skip any "Flags":
//
// impl VkDebugReportFlagsEXT {
//     const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkFlags = 1 << 0;
//     ..
// }
fn make_enum_member_rusty(
    enum_name: Spur,
    member_name: Spur,
    constant_syntax: bool,
    reg: &Registry,
) -> String {
    // this function originally allocated vecs and such but with my unhealthy tendencies it now does everything it can inplace
    // help

    // gets the end position (index of the element after the last one we want)
    // of the current chunk, chunks consist of substrings starting with a uppercase letter
    // and ending with a lowercase letter or at the end of the whole string
    fn get_next_enum_chunk(str: &str, start: usize) -> usize {
        let mut prev_lowercase = false;
        for (i, c) in str[start..].chars().enumerate() {
            let cur_lowercase = c.is_ascii_lowercase();
            if prev_lowercase == true && cur_lowercase == false {
                return start + i;
            }
            prev_lowercase = cur_lowercase;
        }
        return str.len();
    }
    // chunks are separated by underscores, we strip the underscore lower down
    fn get_next_member_chunk(str: &str, start: usize) -> usize {
        str.get((start + 1)..)
            .and_then(|s| s.chars().position(|c| c == '_').map(|p| start + p + 1))
            .unwrap_or(str.len())
    }

    let enum_str = &*reg.resolve(&enum_name);
    let member_str = &*reg.resolve(&member_name);

    let mut enum_start = 0;
    let mut enum_end = get_next_enum_chunk(enum_str, 0);

    let mut member_start = 0;
    let mut member_end = get_next_member_chunk(member_str, 0);

    let mut out = String::new();
    while member_start < member_str.len() {
        let estr = &enum_str[enum_start..enum_end];
        let mut mstr = &member_str[member_start..member_end];

        // there are underscores between chunks, we don't want to compare them
        if &mstr[0..1] == "_" {
            mstr = &mstr[1..];
        }

        // the two strings case-insetively match
        let same = if estr.len() == mstr.len() {
            let mut e = estr.chars().map(|c| c.to_ascii_lowercase());
            let mut m = mstr.chars().map(|c| c.to_ascii_lowercase());
            e.eq(m)
        } else {
            false
        };
        // we skip any Flags substrings, see above
        let skip_flags = estr == "Flags";

        // we skip Flags as they are part of the enum but not the boilerplate
        if estr == "Flags" {
            enum_start = enum_end;
            enum_end = get_next_enum_chunk(enum_str, enum_start);
        // both are the same, that means they are part of the boilerplate that we want to strip
        } else if same {
            enum_start = enum_end;
            enum_end = get_next_enum_chunk(enum_str, enum_start);

            member_start = member_end;
            member_end = get_next_member_chunk(member_str, member_start);
        // we can safely say that this identifier is actually the relevant bit from the original string
        } else {
            if constant_syntax && !out.is_empty() {
                out.push('_');
            }
            let start = out.len();
            out += mstr;
            if constant_syntax {
                out[start..].make_ascii_uppercase();
            } else {
                out[(start + 1)..].make_ascii_lowercase();
            }

            member_start = member_end;
            member_end = get_next_member_chunk(member_str, member_start);
        }
    }
    out
}

#[test]
fn test_enum_rustify() {
    let mut reg = Registry::new();

    let enum_name = reg.get_or_intern("VkDebugReportFlagsEXT");
    let member_name = reg.get_or_intern("VK_DEBUG_REPORT_INFORMATION_BIT_EXT");

    let expect = "INFORMATION_BIT";
    let out = make_enum_member_rusty(enum_name, member_name, true, &reg);
    assert_eq!(out, expect);

    let expect = "InformationBit";
    let out = make_enum_member_rusty(enum_name, member_name, false, &reg);
    assert_eq!(out, expect);

    let enum_name = reg.get_or_intern("VkTestLongerThingEXT");
    let member_name = reg.get_or_intern("VK_TEST_LONGER_THING_HUZZAH_CRABS_EXT");

    let expect = "HUZZAH_CRABS";
    let out = make_enum_member_rusty(enum_name, member_name, true, &reg);
    assert_eq!(out, expect);

    let expect = "HuzzahCrabs";
    let out = make_enum_member_rusty(enum_name, member_name, false, &reg);
    assert_eq!(out, expect);
}