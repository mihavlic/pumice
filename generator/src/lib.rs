use crate::{
    format_utils::{FormatWriter, Separated, WriteWriteAdapter},
    ownership::{add_dependent_sections, resolve_ownership},
    workarounds::apply_workarounds,
};
use generator_lib::{
    interner::{Intern, UniqueStr},
    process_registry_xml,
    type_declaration::{CDecl, Decl, TypeToken},
    EnumValue, Registry, Symbol, SymbolBody,
};
use registry_format::Pathed;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Write},
    fs::File,
    io::BufWriter,
    ops::Deref,
    path::Path,
};

mod format_utils;
mod ownership;
mod registry_format;
mod workarounds;

pub enum SectionKind {
    Feature(u32),
    Extension(u32),
}

pub const INVALID_SECTION: u32 = u32::MAX;

pub struct Section {
    pub name: UniqueStr,
    pub kind: SectionKind,
}

pub struct Context {
    pub(crate) reg: Registry,
    pub(crate) symbol_ownership: Vec<u32>,
    pub(crate) sections: Vec<Section>,
}

impl Context {
    pub fn new(reg: Registry) -> Self {
        let mut s = Self {
            symbol_ownership: vec![INVALID_SECTION; reg.symbols.len()],
            reg,
            sections: Vec::new(),
        };

        {
            for (i, feature) in s.reg.features.iter().enumerate() {
                s.sections.push(Section {
                    name: feature.name,
                    kind: SectionKind::Feature(i as u32),
                });
            }

            for (i, extension) in s.reg.extensions.iter().enumerate() {
                s.sections.push(Section {
                    name: extension.name,
                    kind: SectionKind::Extension(i as u32),
                });
            }

            s.sections.sort_by_key(|s| s.name);
        }

        apply_workarounds(&mut s);
        resolve_ownership(&mut s);
        s
    }
    pub fn reg(&self) -> &Registry {
        &self.reg
    }
    pub fn item_ownership(&self) -> &[u32] {
        &self.symbol_ownership
    }
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }
    pub fn get_item_section_idx(&self, name: UniqueStr) -> Option<u32> {
        let index = self.get_symbol_index(name)?;
        let section = self.symbol_ownership[index as usize];

        if section == INVALID_SECTION {
            None
        } else {
            Some(section)
        }
    }
    pub fn get_section(&self, index: u32) -> Option<&Section> {
        if index == INVALID_SECTION {
            None
        } else {
            Some(&self.sections[index as usize])
        }
    }
    pub fn find_section_idx(&self, name: UniqueStr) -> u32 {
        self.sections
            .binary_search_by_key(&name, |s| s.name)
            .unwrap() as u32
    }
    fn find_section(&self, name: UniqueStr) -> &Section {
        &self.sections[self.find_section_idx(name) as usize]
    }
    fn remove_symbol(&mut self, index: u32) {
        self.reg.remove_symbol(index);
        self.symbol_ownership.remove(index as usize);
    }
}

impl Deref for Context {
    type Target = Registry;

    fn deref(&self) -> &Self::Target {
        &self.reg
    }
}

pub fn write_bindings(
    vk_xml: &str,
    video_xml: &str,
    out: &dyn AsRef<Path>,
    sections: &[&str],
) -> Result<(), Box<dyn Error>> {
    let mut reg = Registry::new();

    process_registry_xml(&mut reg, vk_xml);
    process_registry_xml(&mut reg, video_xml);

    let ctx = Context::new(reg);

    apply_renames(&ctx);

    let mut selected_sections = Vec::new();
    if sections.contains(&"@all") {
        let disabled = "disabled".intern(&ctx);
        selected_sections.extend(ctx.reg.features.iter().map(|a| a.name));
        selected_sections.extend(
            ctx.reg
                .extensions
                .iter()
                .filter(|a| a.supported.get(0) != Some(&disabled))
                .map(|a| a.name),
        );
    } else {
        selected_sections.extend(sections.iter().map(|str| str.intern(&ctx)));
        add_dependent_sections(&mut selected_sections, &ctx);
    }

    // in vulkan bitmasks are implemented as a typedef of an integer that serves as the actual object of the bitmask
    // and a distinct enum that hold the various bitflags, with rust we want to have only the integer with the flags as associated values
    // this needs to know which Bitmask a BitmaskBits specifies the bits for
    let mut bitmask_pairing = HashMap::new();

    for Symbol(name, body) in &ctx.reg.symbols {
        match body {
            SymbolBody::Bitmask { bits_enum, .. } => {
                if let Some(bits_enum) = bits_enum {
                    bitmask_pairing.insert(*bits_enum, *name);
                }
            }
            _ => {}
        }
    }

    selected_sections.sort_unstable();

    // (item index, section index)
    let mut items = Vec::new();
    for (i, &Symbol(name, _)) in ctx.reg.symbols.iter().enumerate() {
        if let Some(section) = ctx.get_item_section_idx(name) {
            let name = ctx.get_section(section).unwrap().name;
            if selected_sections.binary_search(&name).is_ok() {
                items.push((i, section));
            }
        }
    }

    // we rely on this sort being stable, ie preserves the relative order of equal elements
    items.sort_by_key(|i| i.1);

    let out_dir = out.as_ref().to_path_buf();
    let mut prev_section = INVALID_SECTION;
    let mut section_writer = None;

    {
        let file = File::create(out_dir.join("platform.rs"))?;
        let mut write = FormatWriter::new(WriteWriteAdapter(BufWriter::new(file)));
        code2!(
            &mut write,
            "// aliases providing definitions for some types from vk_platform.h"
            "pub type void = std::ffi::c_void;"
            "pub type char = std::ffi::c_char;"
            "pub type float = std::ffi::c_float;"
            "pub type double = std::ffi::c_double;"
            "pub type size_t = std::ffi::c_size_t;"
            "pub type int = std::ffi::c_int;"
            @
        )?;
    }

    let mut cur_section = 0;

    for &(item, section) in &items {
        if section != prev_section {
            // TODO process the names to be more rusty since right now they are literally the names of vulkan extensions
            let file_name = ctx.get_section(section).unwrap().name.resolve();
            let file = File::create(out_dir.join(&*file_name).with_extension("rs"))?;
            let writer = FormatWriter::new(WriteWriteAdapter(BufWriter::new(file)));

            section_writer = Some(writer);
            section_writer
                .as_mut()
                .unwrap()
                .write_str("use crate::platform::*;\n")?;

            cur_section = section;
        }
        prev_section = section;

        let writer = section_writer.as_mut().unwrap();

        let Symbol(name, body) = &ctx.reg.symbols[item];
        match body {
            &SymbolBody::Alias(of) => {
                if !is_std_type(of) {
                    let target = resolve_alias(of, &ctx.reg);
                    match target.1 {
                        SymbolBody::BitmaskBits { .. } => continue,
                        SymbolBody::Alias { .. } | SymbolBody::Define { .. } => {
                            unreachable!();
                        }
                        SymbolBody::Constant { ty, .. } => {
                            code2!(
                                writer,
                                "pub const {}: {} = {};"
                                @ name, ty, Pathed(target.0, &ctx, cur_section)
                            )?;
                            continue;
                        }
                        SymbolBody::Command { .. } => {
                            code2!(
                                writer,
                                "// TODO alias of command '{}'"
                                @ name
                            )?;
                            continue;
                        }
                        _ => {}
                    }
                };

                code2!(
                    writer,
                    "pub type {} = {};"
                    @ name, Pathed(of, &ctx, cur_section)
                )?;
            }
            SymbolBody::Redeclaration(ty) => {
                code2!(
                    writer,
                    "pub type {} = {};"
                    @ name, ty
                )?;
            }
            // there is nothing to do with defines in rust, just skip them
            SymbolBody::Define { .. } => {}
            SymbolBody::Included { .. } => {
                // verbose error is unnecessary as this is caught in ownership.rs
                unreachable!("[{}]", name);
            }
            SymbolBody::Basetype { .. } => {
                unreachable!("[{}] Cannot process C preprocessor code, this type should be manually replaced in a workaround.", name);
            }
            SymbolBody::Bitmask { ty, .. } => {
                // TODO when we're actually generating semantically valid rust code add #repr(transparent)
                code2!(
                    writer,
                    "pub struct {}(pub {});"
                    @ name, ty
                )?;
            }
            SymbolBody::Handle {
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
                    @ dispatchable, object_type, name
                )?;
            }
            SymbolBody::Funcpointer { return_type, args } => {
                let ret = return_type;
                let args = Separated::args(args.iter(), |(_, ty), f| ty.fmt(f));
                code2!(
                    writer,
                    "pub type {} = fn({}) -> {};"
                    @ name, args, ret
                )?;
            }
            SymbolBody::Struct { union, members } => {
                let keyword = match union {
                    true => "union",
                    false => "struct",
                };
                let members = merge_bitfield_members(&members, &ctx);
                let members = Separated::members(members.iter(), |(name, ty), f| {
                    let ty = ty;
                    format_args!("{}: {}", name, ty).fmt(f)
                });
                code2!(
                    writer,
                    "pub {} {} {{"
                    "    {}"
                    "}}"
                    @ keyword, name, members
                )?;
            }
            SymbolBody::Constant { ty, val } => {
                code2!(
                    writer,
                    "pub const {}: {} = {};"
                    @ name, ty, val
                )?;
            }
            SymbolBody::Enum { members } => {
                let members = Separated::members(members.iter(), |(name, val), f| {
                    let name = name;
                    match val {
                        EnumValue::Bitpos(pos) => format_args!("{} = 1 << {}", name, *pos).fmt(f),
                        EnumValue::Value(val) => format_args!("{} = {}", name, val).fmt(f),
                        EnumValue::Alias(alias) => {
                            format_args!("{} = Self::{}", name, alias).fmt(f)
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
            SymbolBody::BitmaskBits { members } => {
                // skip generating empty impl blocks
                if members.is_empty() {
                    continue;
                }

                // when vulkan has a Bitmask that is reserved for future use and thus has no actual flag values, there are no BitmaskBits defined and the spec omits Bitmask dependency
                // however of course there are exceptions such as VkSemaphoreCreateFlagBits which is not Cdeclared as a dependency but is actually an item :(
                let bitmask_name = match bitmask_pairing.get(name) {
                    Some(n) => *n,
                    None => continue,
                };

                let ty = Pathed(get_concrete_type(bitmask_name, &ctx.reg), &ctx, cur_section);
                let bits = Separated::statements(members.iter(), |(name, val), f| {
                    let name = name;
                    match val {
                        EnumValue::Bitpos(pos) => {
                            format_args!("const {}: {} = 1 << {}", name, ty, pos).fmt(f)
                        }
                        EnumValue::Value(val) => {
                            format_args!("const {}: {} = {}", name, ty, val).fmt(f)
                        }
                        EnumValue::Alias(alias) => {
                            format_args!("const {}: {} = Self::{}", name, ty, alias).fmt(f)
                        }
                    }
                });

                code2!(
                    writer,
                    "impl {} {{"
                    "    {}"
                    "}}"
                    @ bitmask_name, bits
                )?;
            }
            SymbolBody::Command {
                return_type,
                params,
            } => {
                let mut return_str = String::new();

                loop {
                    // function just returns 'void'
                    if return_type.0.tokens.len() == 1 {
                        if let TypeToken::Ident(ty) = &return_type.0.tokens[0] {
                            if ty.resolve() == "void" {
                                break;
                            }
                        }
                    }
                    return_str = format!(" -> {}", return_type);
                    break;
                }

                let args = Separated::args(params.iter(), |param, f| {
                    format_args!("{}: {}", param.name, param.ty).fmt(f)
                });

                code2!(
                    writer,
                    "pub fn {}({}){} {{}}"
                    @ name, args, return_str
                )?;
            }
        }
    }

    Ok(())
}

// whether the type is provided from the rust standard library and as such has no entry in the Registry
pub fn is_std_type(ty: UniqueStr) -> bool {
    match &*ty.resolve() {
        "void" | "char" | "float" | "double" | "int8_t" | "uint8_t" | "int16_t" | "uint16_t"
        | "uint32_t" | "int32_t" | "uint64_t" | "int64_t" | "size_t" | "u8" | "u16" | "u32"
        | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" => true,
        _ => false,
    }
}

fn apply_renames(ctx: &Context) {
    for Symbol(name, body) in &ctx.reg.symbols {
        match body {
            SymbolBody::Bitmask { bits_enum, .. } => {
                if let Some(bits_enum) = bits_enum {
                    // the rest of vulkan still refers to the *Bits structs even though we don't emit them
                    bits_enum.rename(name);
                }
            }
            _ => {}
        }
    }

    for Symbol(name, body) in &ctx.reg.symbols {
        match body {
            SymbolBody::Enum { members } => {
                for &(member_name, ..) in members.iter() {
                    let to = make_enum_member_rusty(*name, member_name, false).intern(&ctx);
                    member_name.rename(&to);
                }
            }
            SymbolBody::BitmaskBits { members } => {
                for &(member_name, ..) in members.iter() {
                    let to = make_enum_member_rusty(*name, member_name, true).intern(&ctx);
                    member_name.rename(&to);
                }
            }
            _ => {}
        }
    }
}

fn merge_bitfield_members(
    members: &[(UniqueStr, CDecl)],
    reg: &Registry,
) -> Vec<(UniqueStr, CDecl)> {
    let mut resolved = Vec::new();
    let mut last_ty: Option<Vec<TypeToken>> = None;
    let mut current_bits = 0;
    let mut max_bits = 0;
    let mut merged_members: Vec<UniqueStr> = Vec::new();

    for (name, c_ty) in members {
        let ty = &c_ty.0;
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
                let mut concat = merged_members[0].resolve().to_owned();
                for member in &merged_members[1..] {
                    concat += "_";
                    concat += member.resolve();
                }
                concat.intern(&reg)
            };
            resolved.push((
                name,
                CDecl(Decl {
                    tokens,
                    array_len: None,
                    bitfield_len: Some(current_bits),
                }),
            ));
            merged_members.clear();
        }

        // start accumulating a new type, if it isn't a bitfield, we add it to the resolved vec straight away,
        // since last_ty is still None, the next member that comes skips both of the block above and can either
        // start accumulating because it is a bitfield or is again just passed through to resolved
        if let Some(bits) = ty.bitfield_len {
            let type_bits = match ty.tokens[0] {
                // TODO match against spurs made from &'static str
                TypeToken::Ident(ty) => match ty.resolve() {
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
            resolved.push((*name, c_ty.clone()));
        }
    }

    resolved
}

// jumps through as many aliases (Symbol::Alias) as needed and returns the resulting non-alias symbol,
// in cases where the provided identifier is not an alias it is immediatelly returned back
fn resolve_alias<'a>(alias: UniqueStr, reg: &'a Registry) -> (UniqueStr, &'a SymbolBody) {
    let mut next = alias;
    loop {
        let symbol = reg.find_symbol(next).unwrap();
        match symbol {
            &SymbolBody::Alias(of) => {
                next = of;
            }
            _ => return (next, symbol),
        }
    }
}

// get the underlying type of a symbol, the difference between this and
// resolve_alias is that this also jumps through things preserving the type
// layout, such as handles or constants this will stop at typedefs as those
// are handled by bindgen and thus we don't have information about
fn get_concrete_type(name: UniqueStr, reg: &Registry) -> UniqueStr {
    let mut name = name;
    loop {
        let top = reg.find_symbol(name).unwrap();
        match top {
            SymbolBody::Alias (of) => name = *of,
            SymbolBody::Bitmask { ty, .. } => name = *ty,
            SymbolBody::Handle { .. } => name = "uint64_t".intern(&reg), // the underlying type of all handles is this 
            SymbolBody::Constant { .. } => panic!("FIXME somehow unify symbols and more complex types?"),
            SymbolBody::Redeclaration(_) |
            SymbolBody::Basetype { .. } |
            SymbolBody::Included { .. } |
            SymbolBody::Struct { .. } |
            SymbolBody::Funcpointer { .. } => return name,
            SymbolBody::Command { .. } | SymbolBody::Define { .. } => unreachable!(),
            // even though it's called bits, it's just an enum with different semantics
            SymbolBody::BitmaskBits { .. } |
            SymbolBody::Enum { .. } => unreachable!("This doesn't really make sense as in vulkan 'enum's only have integer values but no types."),
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
    enum_name: UniqueStr,
    member_name: UniqueStr,
    constant_syntax: bool,
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

    // we also can have identifiers that begin with digits when stripped of
    // their boilerplate this is obviously invalid rust and we need to find
    // something to put before it current solution will be to keep track of the
    // starting character of the previous chunk and use that
    //  VkShadingRatePaletteEntryNV
    //  VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV
    //  => E16InvocationsPerPixel

    let mut out = String::new();

    // workaround for identifiers starting with a digit, see above
    let mut prev_char = None;

    let enum_str = enum_name.resolve();
    let member_str = member_name.resolve();

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
    use generator_lib::interner::Interner;

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

        let const_syntax = make_enum_member_rusty(enum_name, member_name, true);
        assert_eq!(const_expect, &const_syntax);

        let normal_syntax = make_enum_member_rusty(enum_name, member_name, false);
        assert_eq!(normal_expect, &normal_syntax);
    }
}
