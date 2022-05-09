#![allow(unused)]
use format_utils::{RegistryWrap, Separated};
use generator_lib::{
    process_registry,
    type_declaration::{TypeDecl, TypeToken},
    EnumValue, InterfaceItem, Intern, ItemKind, Registry, Toplevel, ToplevelBody, ToplevelKind,
};
use lasso::{Rodeo, Spur};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

mod format_utils;

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

    // todo filter items by maximum api version, extensions, and all the other <requires> stuff
    // esentially crawl through all the possible definition places and reject those that don't match the hard limits
    // and refcount all the global items, then emit additional things like when apis can add constants and annotate
    // them with the accumulated required properties, for example a constant in VK 1.1 would get a requirement api=1.1
    // this should be reconsidered though as it would be nice to keep these in different files and would allow us to reject
    // api definitions at a rough granuality first without shoving it all into the compiler

    let mut include_headers = Vec::new();
    let mut c = BufWriter::new(bindgen_file);
    let mut rust = BufWriter::new(rust_file);

    fn batch_rename(reg: &Registry, renames: &[(&str, &str)]) {
        for (original, rename) in renames {
            reg.add_rename_with(original.intern(&reg), || rename.intern(&reg));
        }
    }

    // TODO this is perhaps not ideal
    batch_rename(
        &reg,
        &[
            ("uint8_t", "u8"),
            ("uint16_t", "u16"),
            ("uint32_t", "u32"),
            ("uint64_t", "u64"),
            ("int8_t", "i8"),
            ("int16_t", "i16"),
            ("int32_t", "i32"),
            ("int64_t", "i64"),
        ],
    );

    code!(
        rust,
        "pub type void = std::ffi::c_void;"
        "pub type size_t = std::ffi::c_size_t;"
        "pub type int = std::ffi::c_int;"
        "pub type uint = std::ffi::c_uint;"
        "pub type float = std::ffi::c_float;"
        "pub type double = std::ffi::c_double;"
        @
    )?;

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
                    // the rest of vulkan still refers to the *Bits structs even though we don't emit them
                    reg.add_rename(bits_enum, name);
                }
            }
            _ => {}
        }
    }

    for item in &reg.toplevel {
        let name = item.0;
        match &item.1 {
            ToplevelBody::Enum { members } => {
                for (member_name, val) in members.iter() {
                    reg.add_rename_with(*member_name, || {
                        make_enum_member_rusty(name, *member_name, false, &reg).intern(&reg)
                    });
                }
            }
            ToplevelBody::BitmaskBits { members } => {
                for (member_name, val) in members.iter() {
                    reg.add_rename_with(*member_name, || {
                        make_enum_member_rusty(name, *member_name, true, &reg).intern(&reg)
                    });
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
                // TODO when we're actually generating semantically valid rust code add #repr(transparent)
                code!(
                    rust,
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
                        SpurOrString::Spur(s) => match resolve_ident(s, &reg) {
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
            // TODO what to do with this thing? The make_rusty.. function obviously fails at this
            // Toplevel(
            //     "VkColorSpaceKHR",
            //     Enum {
            //         members: [
            //             (
            //                 "VK_COLOR_SPACE_SRGB_NONLINEAR_KHR",
            //                 Value(
            //                     0,
            //                 ),
            //             ),
            //             (
            //                 "VK_COLORSPACE_SRGB_NONLINEAR_KHR",
            //                 Alias(
            //                     "VK_COLOR_SPACE_SRGB_NONLINEAR_KHR",
            //                 ),
            //             ),
            //         ],
            //     },
            // )
            ToplevelBody::Enum { members } => {
                let members = members.iter().map(|(member_name, val)| {
                    // FIXME this allocates strings
                    let name = match resolve_ident(&member_name, &reg) {
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
                // skip generating empty impl blocks
                if members.is_empty() {
                    continue;
                }

                // when vulkan has a Bitmask that is reserved for future use and thus has no actual flag values, there are no BitmaskBits defined and the spec omits Bitmask dependency
                // however of course there are exceptions such as VkSemaphoreCreateFlagBits which is not declared as a dependency but is actually an item :(
                let bitmask_name = match bitmask_pairing.get(&name) {
                    Some(n) => n,
                    None => continue,
                };

                let ty = get_concrete_type(*bitmask_name, &reg);
                let bits = members.iter().map(|(member_name, val)| {
                    let ty = ty.reg(&reg);
                    let name = member_name.reg(&reg);

                    let val = match val {
                        EnumValue::Bitpos(pos) => {
                            format!("const {}: {} = 1 << {}", name, ty, pos)
                        }
                        EnumValue::Value(val) => {
                            format!("const {}: {} = {}", name, ty, val)
                        }
                        EnumValue::Alias(alias) => {
                            format!("const {}: {} = Self::{}", name, ty, alias.reg(&reg))
                        }
                    };
                    val
                });
                code!(
                    rust,
                    "impl {} {{"
                    "    {}"
                    "}}"
                    @ bitmask_name.reg(&reg), Separated::statements(bits)
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
            "#include \"{}\""
            @ header.reg(&reg)
        )?;
    }

    Ok(())
}

// this whole functions sucks
fn resolve_ident<'a>(spur: &Spur, reg: &Registry) -> SpurOrString {
    let str = reg.resolve(spur);
    match &*str {
        // TODO consider using kind
        "type" => SpurOrString::String("type_".to_owned()),
        _ => {
            if str.chars().next().unwrap().is_ascii_digit() {
                // TODO do something better, perhaps switch the digit with the next char and then fallback to this
                let mut string = "_".to_owned();
                string += &*str;
                SpurOrString::String(string)
            } else if let Some(spur) = reg.get(&*str) {
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
                // TODO match against spurs made from &'static str
                TypeToken::Ident(ty) => match &*reg.resolve(&get_concrete_type(ty, reg)) {
                    "uint8_t" | "int8_t" | "u8" | "i8" => 8,
                    "uint16_t" | "int16_t" | "u16" | "i16" => 16,
                    "uint32_t" | "int32_t" | "VkFlags" | "u32" | "i32" => 32,
                    "uint64_t" | "int64_t" | "VkFlags64" | "u64" | "i64" => 64,
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
    // there are occasionally numbers as well, right now if the number is preceded by an uppercase number we treat is as a lowercase, otherwise it's uppercase

    // VkVideoEncodeH265CapabilityFlagsEXT
    // VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT

    // VkFormatFeatureFlags2
    // VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT

    // gets the end position (index of the element after the last one we want)
    // of the current chunk, chunks consist of substrings starting with a uppercase letter
    // and ending with a lowercase letter or at the end of the whole string
    fn get_next_enum_chunk(str: &str, start: usize) -> usize {
        let mut prev = 'A'; // any uppercase letter will do
        for (i, c) in str[start..].chars().enumerate() {
            // just match all the different situations where we want to end a chunk
            // Hah|A, Hah|42, 42|Aha
            if (prev.is_ascii_lowercase() && c.is_ascii_uppercase())
                || (prev.is_ascii_lowercase() && c.is_ascii_digit())
                || (prev.is_ascii_digit() && c.is_ascii_uppercase())
            {
                return start + i;
            }
            prev = c;
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

    let data = &[
        (
            "VkDebugReportFlagsEXT",
            "VK_DEBUG_REPORT_INFORMATION_BIT_EXT",
            "INFORMATION_BIT",
            "InformationBit",
        ),
        (
            "VkTestLongerThingEXT",
            "VK_TEST_LONGER_THING_HUZZAH_CRABS_EXT",
            "HUZZAH_CRABS",
            "HuzzahCrabs",
        ),
        (
            "VkFormatFeatureFlags2",
            "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT",
            "SAMPLED_IMAGE_BIT",
            "SampledImageBit",
        ),
        (
            "VkVideoEncodeH265CapabilityFlagsEXT",
            "VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT",
            "SEPARATE_COLOUR_PLANE_BIT",
            "SeparateColourPlaneBit",
        ),
    ];

    for (enum_name, member_name, const_expect, normal_expect) in data {
        let enum_name = enum_name.intern(&reg);
        let member_name = member_name.intern(&reg);

        let const_syntax = make_enum_member_rusty(enum_name, member_name, true, &reg);
        assert_eq!(const_expect, &const_syntax);

        let normal_syntax = make_enum_member_rusty(enum_name, member_name, false, &reg);
        assert_eq!(normal_expect, &normal_syntax);
    }
}
