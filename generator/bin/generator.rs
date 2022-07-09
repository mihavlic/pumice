use crate::{
    format_utils::{FormatWriter, Separated, WriteWriteAdapter},
    ownership::resolve_ownership,
    workarounds::apply_workarounds,
};
use generator_lib::{
    process_registry,
    type_declaration::{TypeDecl, TypeToken},
    EnumValue, FeatureExtensionItem, InterfaceItem, Intern, Interner, ItemIdx, ItemKind, Registry,
    Resolve, Toplevel, ToplevelBody, ToplevelKind,
};
use lasso::{Rodeo, Spur};
use registry_format::InternerWrap;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Display, Write},
    fs::File,
    io::BufWriter,
    ops::Deref,
    path::{Path, PathBuf},
};

mod format_utils;
mod ownership;
mod registry_format;
mod workarounds;

pub enum SectionKind {
    // this makes for redundant information as the header path is also used for the name, however this doesn't increase the size of the enum and makes the design more consistent
    Header(Spur),
    Feature(u32),
    Extension(u32),
}

pub const INVALID_SECTION: u32 = u32::MAX;

pub struct Section {
    pub name: Spur,
    pub kind: SectionKind,
}

pub struct Context<'a> {
    pub(crate) reg: Registry<'a>,
    pub(crate) item_ownership: Vec<u32>,
    pub(crate) sections: Vec<Section>,
}

impl<'a> Context<'a> {
    fn new(vk_xml: Registry, video_xml: Registry) -> Self {
        // assert!(video_xml.)
        todo!()
    }
    fn get_section(&self, index: u32) -> Option<&Section> {
        if index == INVALID_SECTION {
            None
        } else {
            Some(&self.sections[index as usize])
        }
    }
    fn get_section_idx(&self, name: Spur) -> u32 {
        self.sections
            .binary_search_by_key(&name, |s| s.name)
            .unwrap() as u32
    }
}

impl<'a> Deref for Context<'a> {
    type Target = Interner;

    fn deref(&self) -> &Self::Target {
        &self.reg.interner
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let interner = Interner::new();
    let mut vk_xml = process_registry(
        &std::fs::read_to_string("/home/eg/Downloads/vk.xml")?,
        &interner,
    );
    apply_workarounds(&mut vk_xml);

    let mut selected_sections = vec![
        "VK_VERSION_1_0".intern(&vk_xml),
        "VK_KHR_swapchain".intern(&vk_xml),
    ];

    // let mut i = 0;
    // while i < selected_sections.len() {
    //     let section_name = selected_sections[i];
    //     let section = sections.iter().find(|s| s.name == section_name).unwrap();
    //     foreach_dependency(
    //         section,
    //         |s| {
    //             if !selected_sections.contains(&s.name) {
    //                 selected_sections.push(s.name);
    //             }
    //         },
    //         reg,
    //     );
    //     i += 1;
    // }

    // todo filter items by maximum api version, extensions, and all the other <requires> stuff
    // esentially crawl through all the possible definition places and reject those that don't match the hard limits
    // and refcount all the global items, then emit additional things like when apis can add constants and annotate
    // them with the accumulated required properties, for example a constant in VK 1.1 would get a requirement api=1.1
    // this should be reconsidered though as it would be nice to keep these in different files and would allow us to reject
    // api definitions at a rough granuality first without shoving it all into the compiler

    // let own = resolve_ownership(&vk_xml, &mut selected_sections);
    apply_renames(&vk_xml);

    // for (i, section) in own.items.iter().enumerate() {
    //     let name = vk_xml.toplevel[i].0;
    //     let section = match *section {
    //         INVALID_SECTION => "INVALID".intern(&vk_xml),
    //         other => own.sections[other as usize].name,
    //     };

    //     println!("{}: {}", name.resolve(&vk_xml), section.resolve(&vk_xml))
    // }

    println!(
        "{:?}",
        selected_sections
            .iter()
            .map(|s| s.resolve(&vk_xml))
            .collect::<Vec<_>>()
    );

    // in vulkan bitmasks are implemented as a typedef of an integer that serves as the actual object of the bitmask
    // and a distinct enum that hold the various bitflags, with rust we want to have only the integer with the flags as associated values
    // this needs to know which Bitmask a BitmaskBits specifies the bits for
    let mut bitmask_pairing = HashMap::new();

    for item in &vk_xml.toplevel {
        let name = item.0;
        match item.1 {
            ToplevelBody::Bitmask { bits_enum, .. } => {
                if let Some(bits_enum) = bits_enum {
                    bitmask_pairing.insert(bits_enum, name);
                }
            }
            _ => {}
        }
    }

    selected_sections.sort_unstable();

    // (item index, section index)
    let mut items = Vec::with_capacity(vk_xml.toplevel.len());
    for (i, item) in vk_xml.toplevel.iter().enumerate() {
        if let Some(section) = own.get_ownership(&item.0, &vk_xml) {
            let name = own.get_section(section).name;
            if selected_sections.binary_search(&name).is_ok() {
                items.push((i, section));
            }
        }
    }

    // we rely on this sort being stable, ie preserves the relative order of equal elements
    items.sort_by_key(|i| i.1);

    let out_dir = PathBuf::from(std::env::args_os().nth(1).unwrap());
    let mut prev_section = INVALID_SECTION;
    let mut section_writer = None;

    for &(item, section) in &items {
        if section != prev_section {
            // TODO process the names to be more rusty since right now they are literally the names of vulkan extensions
            let file_name = own.get_section(section).name.resolve(&vk_xml);
            let file = File::create(out_dir.join(&*file_name))?;
            section_writer = Some(FormatWriter::new(WriteWriteAdapter(BufWriter::new(file))));
        }
        prev_section = section;

        let writer = section_writer.as_mut().unwrap();

        let item = &vk_xml.toplevel[item];
        let name = item.0.int(&vk_xml);
        match &item.1 {
            ToplevelBody::Alias { alias_of, kind } => match kind {
                ToplevelKind::Command => {
                    code2!(
                        writer,
                        "// TODO alias of command '{}'"
                        @ name
                    )?;
                }
                ToplevelKind::Constant => {
                    let item = resolve_alias(*alias_of, ToplevelKind::Constant, &vk_xml);
                    let (ty, _val) = match item.1 {
                        ToplevelBody::Constant { ty, val } => (ty, val),
                        _ => unreachable!(),
                    };

                    code2!(
                        writer,
                        "pub const {}: {} = {};"
                        @ name, ty.int(&vk_xml), alias_of.int(&vk_xml)
                    )?;
                }
                _ => {
                    code2!(
                        writer,
                        "pub type {} = {};"
                        @ name, alias_of.int(&vk_xml)
                    )?;
                }
            },
            ToplevelBody::Included { header } => {}
            // there is nothing to do with defines in rust, just skip them
            ToplevelBody::Define { .. } => {}
            // this is most often just a typedef
            // TODO parse the typedef (and replace it with an alias) so that we have more complete information
            ToplevelBody::Basetype { ty, code } => {
                // let ty = ty.expect(
                //     "Previous passes should correct all basetype definitions with missing type.",
                // );
                let ty = ty.unwrap_or_else(|| "TODO".intern(&vk_xml));
                code2!(
                    writer,
                    "pub type {} = {};"
                    @ name, ty.int(&vk_xml)
                )?;
            }
            ToplevelBody::Bitmask { ty, .. } => {
                // TODO when we're actually generating semantically valid rust code add #repr(transparent)
                code2!(
                    writer,
                    "pub struct {}(pub {});"
                    @ name, ty.int(&vk_xml)
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
                code2!(
                    writer,
                    "/// {}{}"
                    "#[repr(transparent)]"
                    "pub struct {}(u64);"
                    @ dispatchable, object_type.int(&vk_xml), name
                )?;
            }
            ToplevelBody::Funcpointer { return_type, args } => {
                let ret = return_type.as_rust_order(&vk_xml);
                let args = Separated::args(args.iter(), |(_, ty), f| {
                    ty.as_rust_order(&vk_xml).int(&vk_xml).fmt(f)
                });
                code2!(
                    writer,
                    "pub type {} = fn({}) -> {};"
                    @ name, args, ret.int(&vk_xml)
                )?;
            }
            ToplevelBody::Struct { union, members } => {
                let keyword = match union {
                    true => "union",
                    false => "struct",
                };
                let members = merge_bitfield_members(&members, &vk_xml);
                let members = Separated::members(members.iter(), |(name, ty), f| {
                    let ty = ty.as_rust_order(&vk_xml);
                    format_args!("{}: {}", name.int(&vk_xml), ty.int(&vk_xml)).fmt(f)
                });
                code2!(
                    writer,
                    "pub {} {} {{"
                    "    {}"
                    "}}"
                    @ keyword, name, members
                )?;
            }
            ToplevelBody::Constant { ty, val } => {
                code2!(
                    writer,
                    "pub const {}: {} = {};"
                    @ name, ty.int(&vk_xml), val.int(&vk_xml)
                )?;
            }
            ToplevelBody::Enum { members } => {
                let members = Separated::members(members.iter(), |(name, val), f| {
                    let name = name.int(&vk_xml);
                    match val {
                        EnumValue::Bitpos(pos) => format_args!("{} = 1 << {}", name, *pos).fmt(f),
                        EnumValue::Value(val) => format_args!("{} = {}", name, *val).fmt(f),
                        EnumValue::Alias(alias) => {
                            format_args!("{} = Self::{}", name, (*alias).int(&vk_xml)).fmt(f)
                        }
                    }
                });
                code2!(
                    writer,
                    "pub enum {} {{"
                    "    {}"
                    "}}"
                    @ name, members
                )?;
            }
            ToplevelBody::BitmaskBits { members } => {
                // skip generating empty impl blocks
                if members.is_empty() {
                    continue;
                }

                // when vulkan has a Bitmask that is reserved for future use and thus has no actual flag values, there are no BitmaskBits defined and the spec omits Bitmask dependency
                // however of course there are exceptions such as VkSemaphoreCreateFlagBits which is not declared as a dependency but is actually an item :(
                let bitmask_name = match bitmask_pairing.get(&item.0) {
                    Some(n) => n,
                    None => continue,
                };

                let ty = get_concrete_type(*bitmask_name, &vk_xml).int(&vk_xml);
                let bits = Separated::statements(members.iter(), |(name, val), f| {
                    let name = name.int(&vk_xml);
                    match val {
                        EnumValue::Bitpos(pos) => {
                            format_args!("const {}: {} = 1 << {}", name, ty, pos).fmt(f)
                        }
                        EnumValue::Value(val) => {
                            format_args!("const {}: {} = {}", name, ty, val).fmt(f)
                        }
                        EnumValue::Alias(alias) => {
                            format_args!("const {}: {} = Self::{}", name, ty, alias.int(&vk_xml))
                                .fmt(f)
                        }
                    }
                });

                code2!(
                    writer,
                    "impl {} {{"
                    "    {}"
                    "}}"
                    @ bitmask_name.int(&vk_xml), bits
                )?;
            }
            ToplevelBody::Command {
                return_type,
                params,
            } => {
                let mut return_str = None;
                // function just returns 'void'
                if return_type.tokens.len() == 1 {
                    if let TypeToken::Ident(ty) = &return_type.tokens[0] {
                        if &*vk_xml.resolve(ty) == "void" {
                            return_str = Some("".to_string());
                        }
                    }
                }
                // the function has an actual return type
                if return_str.is_none() {
                    return_str = Some(format!(
                        " -> {}",
                        return_type.as_rust_order(&vk_xml).int(&vk_xml)
                    ));
                }

                let args = Separated::args(params.iter(), |param, f| {
                    format_args!(
                        "{}: {}",
                        param.name.int(&vk_xml),
                        param.ty.as_rust_order(&vk_xml).int(&vk_xml)
                    )
                    .fmt(f)
                });

                code2!(
                    writer,
                    "pub fn {}({}){} {{}}"
                    @ name, args, return_str.unwrap()
                )?;
            }
        }
    }

    // code2!(
    //     rust,
    //     "pub type void = std::ffi::c_void;"
    //     "pub type size_t = std::ffi::c_size_t;"
    //     "pub type int = std::ffi::c_int;"
    //     "pub type uint = std::ffi::c_uint;"
    //     "pub type float = std::ffi::c_float;"
    //     "pub type double = std::ffi::c_double;"
    //     @
    // )?

    // for header in include_headers {
    //     code2!(
    //         c,
    //         "#include \"{}\""
    //         @ header.reg(&reg)
    //     )?;
    // }

    Ok(())
}

fn apply_renames(reg: &Registry) {
    let renames = &[
        ("type", "kind"),
        ("uint8_t", "u8"),
        ("uint16_t", "u16"),
        ("uint32_t", "u32"),
        ("uint64_t", "u64"),
        ("int8_t", "i8"),
        ("int16_t", "i16"),
        ("int32_t", "i32"),
        ("int64_t", "i64"),
    ];

    for (original, rename) in renames {
        reg.add_rename_with(original.intern(reg), || rename.intern(reg));
    }

    for item in &reg.toplevel {
        let name = item.0;
        match item.1 {
            ToplevelBody::Bitmask { bits_enum, .. } => {
                if let Some(bits_enum) = bits_enum {
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
                for (member_name, _val) in members.iter() {
                    reg.add_rename_with(*member_name, || {
                        make_enum_member_rusty(name, *member_name, false, &reg).intern(&reg)
                    });
                }
            }
            ToplevelBody::BitmaskBits { members } => {
                for (member_name, _val) in members.iter() {
                    reg.add_rename_with(*member_name, || {
                        make_enum_member_rusty(name, *member_name, true, &reg).intern(&reg)
                    });
                }
            }
            _ => {}
        }
    }
}

// fn write_item<W: std::fmt::Write>(item: &Toplevel, writer: &mut FormatWriter<W>, bitmask_pairing: &HashMap<Spur, Spur>, included_headers: &mut Vec<Spur>, reg: &Registry) -> std::fmt::Result {

//     Ok(())
// }

fn merge_bitfield_members(members: &[(Spur, TypeDecl)], reg: &Registry) -> Vec<(Spur, TypeDecl)> {
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
                merged_members[0]
            } else {
                let mut concat = reg.resolve(&merged_members[0]).to_owned();
                for member in &merged_members[1..] {
                    concat += "_";
                    concat += &*reg.resolve(&member);
                }
                concat.intern(&reg)
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
            resolved.push((*name, ty.clone()));
        }
    }

    resolved
}

// jumps through as many aliases (Toplevel::Alias) is needed and returns the concrete toplevel item,
// in cases where the provided identifier is not an alias it is simply returned back
fn resolve_alias<'a>(alias: Spur, kind: ToplevelKind, reg: &'a Registry<'_>) -> &'a Toplevel {
    let mut alias = alias;
    loop {
        // FIXME what if the alias is to something that is added by bindgen and is never in our registry?
        let &(index, ty) = reg
            .item_map
            .get(&alias)
            .expect("Alias is dangling, see comment.");
        // assert that no weird things are going on as other interface items can't alias this way
        assert!(ty == ItemKind::Toplevel);

        let top = &reg.toplevel[index as usize];
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

// get the underlying type of a toplevel item (type is roughly synonymous with a toplevel item),
// the difference between this and resolve_alias is that this also jumps through things preserving
// the type layout, such as handles or constants this will stop at typedefs as those are
// handled by bindgen and thus we don't have information about
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
        assert!(ty == ItemKind::Toplevel);

        let top = &reg.toplevel[index as usize];
        match &top.1 {
            ToplevelBody::Alias { alias_of, .. } => toplevel = *alias_of,
            ToplevelBody::Bitmask { ty, .. } => toplevel = *ty,
            ToplevelBody::Handle { .. } => toplevel = "uint64_t".intern(&reg), // the underlying type of all handles is this 
            ToplevelBody::Constant { ty, .. } => toplevel = *ty,
            ToplevelBody::Basetype { ty, .. } => {
                // TODO is this what we want? It was introduced to change the types in bitset members from 'VkFlags' to 'u32'
                // it is doubtful whether it makes any difference  
                if let Some(ty) = ty {
                    toplevel = *ty;
                } else {
                    return toplevel;
                }
            },
            ToplevelBody::Included { .. } |
            ToplevelBody::Struct { .. } |
            ToplevelBody::Funcpointer { .. } => return toplevel,
            ToplevelBody::Command { .. } | ToplevelBody::Define { .. } => unreachable!(),
            // even though it's called bits, it's just an enum with different semantics
            ToplevelBody::BitmaskBits { .. } |
            ToplevelBody::Enum { .. } => unreachable!("This doesn't really make sense as in vulkan 'enum's only have integer values but no types."),
        }
    }
}

pub struct CamelCaseSplit<'a> {
    str: &'a str,
}

impl<'a> CamelCaseSplit<'a> {
    fn new(str: &'a str) -> Self {
        Self { str }
    }
}

impl<'a> Iterator for CamelCaseSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.str.is_empty() {
            return None;
        }

        let mut chars = self.str.chars();
        let mut prev = chars.next().unwrap();

        for (i, c) in chars.enumerate() {
            // just match all the different situations where we want to end a "chunk"
            // Aa|A, Aa|42, 42|Aa       * Aa is just an example of identifier starting with a capital letter
            if (prev.is_ascii_lowercase() && c.is_ascii_uppercase())
                || (prev.is_ascii_lowercase() && c.is_ascii_digit())
                || (prev.is_ascii_digit() && c.is_ascii_uppercase())
            {
                let (l, r) = self.str.split_at(i + 1); // +1 because we started iterating after already pulling a character from the iterator
                self.str = r;
                return Some(l);
            }
            prev = c;
        }

        return Some(std::mem::replace(&mut self.str, &""));
    }
}

// this will fuzzy match on the member name and strip the enum name and the extension tag boilerplate
//  VkDebugReportFlagsEXT -> Vk Debug Report Flags EXT
//  VK_DEBUG_REPORT_INFORMATION_BIT_EXT -> VK DEBUG REPORT INFORMATION BIT EXT
// => INFORMATION BIT
fn make_enum_member_rusty(
    enum_name: Spur,
    member_name: Spur,
    constant_syntax: bool,
    int: &Interner,
) -> String {
    // enum VkPresentModeKHR {
    //     VK_PRESENT_MODE_IMMEDIATE_KHR = 0, -> Immediate = 0,
    //     ..
    // }

    // the enum names contain "Flags" while the member does not, this needs to be filtered nevertheless:
    //  impl VkDebugReportFlagsEXT {
    //      const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkFlags = 1 << 0;
    //      ..
    //  }
    //
    //  VkVideoEncodeH265CapabilityFlagsEXT
    //  VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT
    //
    //  VkFormatFeatureFlags2
    //  VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT

    // we also can have identifiers that begin with digits when stripped of their boilerplate
    // this is obviously invalid rust and we need to find something to put before it
    // current solution will be to keep track of the starting character of the previous chunk and use that
    //  VkShadingRatePaletteEntryNV
    //  VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV
    //  => E16InvocationsPerPixel

    let mut out = String::new();

    // workaround for identifiers starting with a digit, see above
    let mut prev_char = None;

    let enum_str = &*int.resolve(&enum_name);
    let member_str = &*int.resolve(&member_name);

    // we skip "Flags" as they are part of the enum boilerplate but don't occur in the member, see above
    let mut enum_chunks = CamelCaseSplit::new(enum_str)
        .filter(|s| *s != "Flags")
        .peekable();

    // let's skip "BIT" as well as it's quite redundant
    let mut member_chunks = member_str.split('_').filter(|s| *s != "BIT");

    while let Some(mstr) = member_chunks.next() {
        let estr = enum_chunks.peek();

        let start_char = mstr.chars().next().unwrap();

        // if estr runs out we just continue processing what's left in member_string
        if let Some(estr) = estr {
            // the strings can never match if their length differs
            if estr.len() == mstr.len() {
                let e = estr.chars().map(|c| c.to_ascii_lowercase());
                let m = mstr.chars().map(|c| c.to_ascii_lowercase());

                // case-insetively compare the strings
                if Iterator::eq(e, m) {
                    enum_chunks.next();
                    prev_char = Some(start_char); // hmmph
                    continue;
                }
            }
        }

        // the chunks differ, that means that mstr is not a part of the boilerplate and is actually relevant
        {
            if out.is_empty() {
                // see above
                if start_char.is_ascii_digit() {
                    out.push(prev_char.unwrap());
                }
            } else if constant_syntax {
                out.push('_');
            }

            let len = out.len();
            out += mstr;

            // currently we pushed into out a string that is all upeercase due to being derived from the member string
            // which is following constant syntax, now if we don't want to output constant syntax we make all but the
            // first letter that we just added lowercase
            if !constant_syntax {
                out[(len + 1)..].make_ascii_lowercase();
            }
        }

        prev_char = Some(start_char);
    }

    out
}

#[test]
fn test_enum_rustify() {
    let reg = Interner::new();

    let data = &[
        (
            "VkDebugReportFlagsEXT",
            "VK_DEBUG_REPORT_INFORMATION_BIT_EXT",
            "INFORMATION",
            "Information",
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
            "SAMPLED_IMAGE",
            "SampledImage",
        ),
        (
            "VkVideoEncodeH265CapabilityFlagsEXT",
            "VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT",
            "SEPARATE_COLOUR_PLANE",
            "SeparateColourPlane",
        ),
        (
            "VkShadingRatePaletteEntryNV",
            "VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV",
            "E16_INVOCATIONS_PER_PIXEL",
            "E16InvocationsPerPixel",
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
