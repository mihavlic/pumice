use crate::{
    format_utils::{section_get_path, FormatWriter, Pathed, Separated, WriteWriteAdapter},
    ownership::resolve_ownership,
    workarounds::apply_workarounds,
};

use derives::DeriveData;
use fs_utils::copy_dir_recursive;
use generator_lib::{
    configuration::GenConfig,
    interner::{Intern, UniqueStr},
    smallvec::SmallVec,
    type_declaration::{TyToken, Type},
    CommandParameter, ConstantValue, EnumValue, Registry, StructMember, Symbol, SymbolBody,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Write},
    fs::{File, OpenOptions},
    io::BufWriter,
    ops::Deref,
    path::{Path, PathBuf},
};

mod derives;
mod format_utils;
mod fs_utils;
mod ownership;
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

macro_rules! common_strings {
    ($($string:ident),+) => {
        pub struct CommonStrings {
            $(
                pub $string: UniqueStr,
            )+
        }
        impl CommonStrings {
            fn new(int: &generator_lib::interner::Interner) -> Self {
                Self {
                    $(
                        $string: stringify!($string).intern(int),
                    )+
                }
            }
        }
    };
}

macro_rules! common_types {
    ($($name:ident: $string:expr),+) => {
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
    void, int, char, float, double,
    size_t, uint8_t, uint16_t, uint32_t, uint64_t, int8_t, int16_t, int32_t, int64_t,
    usize, u8, u16, u32, u64, i8, i16, i32, i64,
    vk_platform, disabled
}

common_types! {
    cstring:  "const char *",
    void:     "void",
    void_ptr: "void *"
}

pub struct Context {
    pub reg: Registry,
    pub symbol_ownership: Vec<u32>,
    pub sections: Vec<Section>,
    // previously we had sorted the sections vector and then binary searched thta
    // however the ownership algorithm is order sensitive and this may potentially
    // cause issues later, instead we now have a hashmap
    pub section_map: HashMap<UniqueStr, u32>,
    pub strings: CommonStrings,
    pub types: CommonTypes,
}

impl Context {
    pub fn new(reg: Registry) -> Self {
        let sections_len = reg.features.len() + reg.extensions.len();

        let mut s = Self {
            symbol_ownership: vec![INVALID_SECTION; reg.symbols.len()],
            sections: Vec::with_capacity(sections_len),
            section_map: HashMap::with_capacity(sections_len),
            strings: CommonStrings::new(&reg),
            types: CommonTypes::new(&reg),
            reg,
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

            s.section_map.extend(
                s.sections
                    .iter()
                    .enumerate()
                    .map(|(i, e)| (e.name, i as u32)),
            );
        }

        apply_workarounds(&mut s);
        resolve_ownership(&mut s);

        s
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
        *self.section_map.get(&name).unwrap()
    }
    pub fn find_section(&self, name: UniqueStr) -> &Section {
        &self.sections[self.find_section_idx(name) as usize]
    }
    pub fn remove_symbol(&mut self, index: u32) {
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
    ctx: &mut Context,
    glue: &dyn AsRef<Path>,
    out: &dyn AsRef<Path>,
    conf: &GenConfig,
) -> Result<(), Box<dyn Error>> {
    apply_renames(&ctx);

    let out_dir = out.as_ref().to_path_buf();
    let mut cur_dir = PathBuf::new();

    std::fs::create_dir_all(out_dir.join("src/extensions"))?;

    copy_dir_recursive(glue, out)?;

    {
        use std::io::Write;

        let mut lib = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .append(true)
                .open(out_dir.join("src/lib.rs"))?,
        );
        let mut exts = BufWriter::new(File::create(out_dir.join("src/extensions/mod.rs"))?);

        // the formatter converts \r into un-ignorable newlines
        write!(&mut lib, "\rpub mod extensions;\r\r")?;

        let mut features = ctx
            .reg
            .features
            .iter()
            .filter(|f| conf.is_feature_used(f.name))
            .map(|f| f.name)
            .collect::<Vec<_>>();

        features.sort_by(|a, b| a.resolve().cmp(b.resolve()));

        let mut extensions = ctx
            .reg
            .extensions
            .iter()
            .filter(|e| conf.is_extension_used(e.name))
            .map(|e| e.name)
            .collect::<Vec<_>>();

        extensions.sort_by(|a, b| a.resolve().cmp(b.resolve()));

        for feature in features {
            writeln!(&mut lib, "pub mod {};", feature)?;
        }

        for extension in extensions {
            writeln!(&mut exts, "pub mod {};", extension)?;
        }
    }

    // (item index, section index)
    let mut items = Vec::new();
    for (i, &Symbol(name, _)) in ctx.reg.symbols.iter().enumerate() {
        if let Some(section_idx) = ctx.get_item_section_idx(name) {
            let section = ctx.get_section(section_idx).unwrap();

            let used = match section.kind {
                SectionKind::Feature(_) => conf.is_feature_used(section.name),
                SectionKind::Extension(_) => conf.is_extension_used(section.name),
            };

            if used {
                items.push((i, section_idx));
            }
        }
    }

    items.sort_by_key(|i| i.1);

    let mut derives = DeriveData::new(&ctx);

    let mut prev_section = INVALID_SECTION;
    let mut section_writer = None;

    for &(index, section) in &items {
        if section != prev_section {
            let section = ctx.get_section(section).unwrap();

            cur_dir.clone_from(&out_dir);
            cur_dir.push("src");
            cur_dir.extend(section_get_path(&section));
            cur_dir.push(section.name.resolve());
            cur_dir.set_extension("rs");

            let file = File::create(&cur_dir)?;

            let mut writer = FormatWriter::new(WriteWriteAdapter(BufWriter::new(file)));

            code2!(
                &mut writer,
                "use crate::{{char, cstr, double, float, int, void}};\n"
            )?;

            section_writer = Some(writer);
        }
        prev_section = section;

        let writer = section_writer.as_mut().unwrap();
        let Symbol(name, body) = &ctx.reg.symbols[index];

        write_item(*name, body, writer, &mut derives, section, &ctx)?;
    }

    Ok(())
}

fn write_item<W: Write>(
    name: UniqueStr,
    body: &SymbolBody,
    writer: &mut FormatWriter<W>,
    derives: &mut DeriveData,
    section: u32,
    ctx: &Context,
) -> std::fmt::Result {
    macro_rules! path {
        ($var:expr) => {
            Pathed($var, &ctx, section)
        };
    }

    macro_rules! select {
        ($cond:expr, $true:expr, $false:expr) => {
            if $cond {
                $true
            } else {
                $false
            }
        };
    }

    match body {
        &SymbolBody::Alias(of) => {
            if !is_std_type(of, &ctx) {
                let target = resolve_alias(of, &ctx);
                match target.1 {
                    SymbolBody::Alias { .. } | SymbolBody::Define { .. } => {
                        unreachable!();
                    }
                    SymbolBody::Constant { .. } => {
                        let ty = get_underlying_type(target.0, &ctx);
                        code2!(
                            writer,
                            "pub const {}: {} = {};"
                            @ name, path!(&ty), path!(target.0)
                        )?;
                        return Ok(());
                    }
                    SymbolBody::Command { .. } => {
                        code2!(
                            writer,
                            "// TODO alias of command '{}'"
                            @ name
                        )?;
                        return Ok(());
                    }
                    _ => {}
                }
            };

            code2!(
                writer,
                "pub type {} = {};"
                @ name, path!(of)
            )?;
        }
        SymbolBody::Redeclaration(ty) => {
            code2!(
                writer,
                "pub type {} = {};"
                @ name, path!(ty)
            )?;
        }
        // there is nothing to do with defines in rust, just skip them
        SymbolBody::Define { .. } => {}
        SymbolBody::Included { .. } => {
            unreachable!("[{}]", name);
        }
        SymbolBody::Basetype { .. } => {
            unreachable!("[{}] Cannot process C preprocessor code, this type should be manually replaced in a workaround.", name);
        }
        SymbolBody::Handle { dispatchable, .. } => {
            let default = if *dispatchable { "" } else { ", Default" };
            code2!(
                writer,
                "#[repr(transparent)]"
                "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash{})]"
                "pub struct {}(u64);"
                @ default, name
            )?;
        }
        SymbolBody::Funcpointer {
            ret: return_type,
            args,
        } => {
            let args =
                Separated::args(args.iter(), |(_, ty), f| convert_type_in_fun_ctx(ty).fmt(f));
            code2!(
                writer,
                "pub type {} = unsafe extern \"system\" fn({}) -> {};"
                @ name, args, path!(return_type)
            )?;
        }
        SymbolBody::Struct { union, members } => {
            let keyword = match union {
                true => "union",
                false => "struct",
            };
            let members = merge_bitfield_members(&members, &ctx);
            let members = Separated::members(members.iter(), |&(name, ty), f| {
                format_args!("{}: {}", name, path!(ty)).fmt(f)
            });
            code2!(
                writer,
                "#[derive(Clone{}{})]"
                "#[repr(C)]"
                "pub {} {} {{"
                "    {}"
                "}}"
                @
                select!(derives.is_copy(name, ctx), ", Copy", ""),
                select!(derives.is_eq(name, ctx), ", PartialEq, Eq, Hash", ""),
                keyword, name, members
            )?;
        }
        SymbolBody::Constant { val, .. } => {
            let ty = get_underlying_type(name, &ctx);

            write!(writer, "pub const {}: {} = ", name, path!(&ty))?;

            match val {
                ConstantValue::Literal(i) => writeln!(writer, "{};", i),
                ConstantValue::Expression(e) => writeln!(writer, "{};", e),
                ConstantValue::Symbol(s) => writeln!(writer, "{};", path!(*s)),
                ConstantValue::String(s) => {
                    writeln!(writer, "cstr!(\"{}\");", s)
                }
            }?;
        }
        SymbolBody::Enum { ty, members, .. } => {
            let ty = get_underlying_type(*ty, &ctx);

            code2!(
                writer,
                "#[derive(Clone, Copy, PartialEq{}{})]"
                "pub struct {}({});"
                @
                select!(derives.is_eq(name, ctx), ", Eq, Hash", ""),
                select!(derives.is_default(name, ctx), ", Default", ""),
                name, path!(&ty)
            )?;

            // skip generating empty impl blocks
            if members.is_empty() {
                return Ok(());
            }

            let members = Separated::statements(members.iter(), |(name, val), f| {
                let name = name;
                match val {
                    EnumValue::Bitpos(pos) => {
                        format_args!("pub const {}: Self = Self(1 << {})", name, pos).fmt(f)
                    }
                    EnumValue::Value(val) => {
                        format_args!("pub const {}: Self = Self({})", name, val).fmt(f)
                    }
                    EnumValue::Alias(alias) => {
                        format_args!("pub const {}: Self = Self::{}", name, alias).fmt(f)
                    }
                }
            });

            code2!(
                writer,
                "impl {} {{"
                "    {}"
                "}}"
                @ name, members
            )?;
        }
        SymbolBody::Command {
            return_type,
            params,
        } => {
            let return_type = convert_type_in_fun_ctx(return_type);

            let args = Separated::args(params.iter(), |param, f| {
                format_args!(
                    "{}: {}",
                    param.name,
                    path!(&convert_type_in_fun_ctx(&param.ty))
                )
                .fmt(f)
            });

            // uhh ... we're not allocating a string anymore though
            let mut _tmp = None;
            let (ret1, ret2): (&str, &dyn Display) = loop {
                if let Some(ty) = return_type.try_only_basetype() {
                    if ty == ctx.strings.void {
                        break (&"", &"");
                    }
                }
                _tmp = Some(path!(&return_type));
                break (&" -> ", _tmp.as_ref().unwrap());
            };

            code2!(
                writer,
                "pub fn {}({}){}{} {{ todo!() }}"
                @ name, args, ret1, ret2
            )?;
        }
    }

    Ok(())
}

// whether the type is provided from the rust standard library and as such has no entry in the Registry
pub fn is_std_type(ty: UniqueStr, ctx: &Context) -> bool {
    let s = &ctx.strings;
    switch!(ty;
        s.void, s.int, s.char, s.float, s.double,
        s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => true;
        @ false
    )
}

fn apply_renames(ctx: &Context) {
    let renames = &[
        // rust-native integer types
        ("uint8_t", "u8"),
        ("uint16_t", "u16"),
        ("uint32_t", "u32"),
        ("uint64_t", "u64"),
        ("int8_t", "i8"),
        ("int16_t", "i16"),
        ("int32_t", "i32"),
        ("int64_t", "i64"),
        ("size_t", "usize"),
        // avoid conflicts
        ("type", "kind"),
    ];

    for &(from, to) in renames {
        from.intern(ctx).rename(to.intern(ctx));
    }

    let mut buf = String::new();

    for feature in &ctx.reg.features {
        buf.clear();
        buf.push_str("vk");
        buf.push_str(feature.number.resolve());
        buf.retain(|c| c != '.');

        feature.name.rename(buf.intern(ctx));
    }

    for extension in &ctx.reg.extensions {
        let prefixes = &["VK_", "vulkan_video_codec_", "vulkan_video_codecs_"];

        let name = extension.name.resolve();

        let mut stripped = None;
        for &prefix in prefixes {
            if let Some(strip) = name.strip_prefix(prefix) {
                stripped = Some(strip);
                break;
            }
        }

        buf.clear();
        buf.push_str(stripped.unwrap());
        buf.make_ascii_lowercase();

        extension.name.rename(buf.intern(ctx));
    }

    'outer: for Symbol(name, _) in &ctx.reg.symbols {
        let str = name.resolve();
        if str.len() >= 3 {
            let amount = loop {
                if str.starts_with("VK_") {
                    break 3;
                }

                let mut chars = ['\0'; 3];
                for (i, c) in str.chars().take(3).enumerate() {
                    chars[i] = c;
                }

                if (chars[0] == 'V' || chars[0] == 'v')
                    && chars[1] == 'k'
                    && chars[2].is_ascii_uppercase()
                {
                    break 2;
                }

                continue 'outer;
            };

            name.rename_trim_prefix(amount);
        }
    }

    for Symbol(name, body) in &ctx.reg.symbols {
        match body {
            SymbolBody::Enum { members, .. } => {
                for &(member_name, ..) in members {
                    make_enum_member_rusty(*name, member_name, true, &mut buf);
                    member_name.rename(buf.intern(&ctx));
                }
            }
            SymbolBody::Struct { members, .. } => {
                for &StructMember { name, .. } in members {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
            }
            SymbolBody::Funcpointer { args, .. } => {
                for &(name, ..) in args {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
                name.rename(name.resolve().strip_prefix("PFN_vk").unwrap().intern(ctx));
            }
            SymbolBody::Command { params, .. } => {
                for &CommandParameter { name, .. } in params {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
                camel_to_snake(name.resolve(), &mut buf);
                name.rename(buf.intern(&ctx));
            }
            _ => {}
        }
    }
}

// essentially allows matching against runtime values
macro_rules! switch {
    ($what:expr; $( $($e:expr),+ => $to:expr;)+ @ $esle:expr) => {
        $(
            if $($what == $e) ||+  {
                $to
            } else
        )+
        {
            $esle
        }
    };
}
pub(crate) use switch;

fn merge_bitfield_members<'a>(
    members: &'a [StructMember],
    ctx: &Context,
) -> Vec<(UniqueStr, &'a Type)> {
    let mut resolved = Vec::new();
    let mut last_ty: Option<&Type> = None;
    let mut current_bits = 0;
    let mut max_bits = 0;
    let mut merged_members: Vec<UniqueStr> = Vec::new();

    for member in members {
        let StructMember { name, ty, bitfield } = member;

        // the type matches and it is a bitfield
        if Some(ty) == last_ty && bitfield.is_some() {
            let bits = bitfield.unwrap();
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
        if let Some(ty) = last_ty.take() {
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
                concat.intern(ctx)
            };

            resolved.push((name, ty));
            merged_members.clear();
        }

        // start accumulating a new type, if it isn't a bitfield, we add it to the resolved vec straight away,
        // since last_ty is still None, the next member that comes skips both of the block above and can either
        // start accumulating because it is a bitfield or is again just passed through to resolved
        if let Some(bits) = bitfield {
            // microsoft (https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170) says that only primitive types
            // can be bitfields, in practice this means that the type tokens will be just an ident, also in the spec only one?
            // type is ever used for bitfield so here will be a little hardcoded lookup from type name to bit width,
            // we don't really ever have good quality information about type layout as we expect bindgen to do it for us
            let basetype = ty
                .try_only_basetype()
                .expect("Only a base raw integer can be a bitfield.");

            let s = &ctx.strings;
            let underlying = get_underlying_type(basetype, ctx)
                .try_only_basetype()
                .unwrap();
            let type_bits = switch!(underlying;
                s.uint8_t,  s.int8_t => 8;
                s.uint16_t, s.int16_t => 16;
                s.uint32_t, s.int32_t => 32;
                s.uint64_t, s.int64_t => 64;
                @ unimplemented!("Don't know the bit-width of '{}'", underlying)
            );

            assert!(*bits <= type_bits);

            max_bits = type_bits;
            current_bits = *bits;
            last_ty = Some(ty);
            merged_members.push(*name);
        } else {
            resolved.push((*name, ty));
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

// Get the underlying type of a symbol.
// The difference between this and `resolve_alias()` is that this also jumps through "transparent" symbols, such as handles or constants.
fn get_underlying_type(name: UniqueStr, ctx: &Context) -> Type {
    let mut symbol = name;
    loop {
        if is_std_type(symbol, &ctx) {
            return Type::from_only_basetype(symbol);
        }

        let top = ctx.find_symbol(symbol).unwrap();
        match top {
            SymbolBody::Alias(of) => symbol = *of,
            SymbolBody::Enum { ty, .. } => symbol = *ty,
            SymbolBody::Handle { .. } => symbol = ctx.strings.uint64_t, // the underlying type of all handles is this
            SymbolBody::Constant { ty, val } => {
                if let Some(ty) = ty {
                    symbol = *ty;
                } else {
                    match val {
                        ConstantValue::Literal(_) | ConstantValue::Expression(_) => {
                            return Type::from_only_basetype(ctx.strings.uint32_t)
                        }
                        ConstantValue::Symbol(s) => symbol = *s,
                        // FIXME avoid allocating?
                        // this function returns either just a basetype or a cstrings we could
                        // return a &Type of the cstring but then we can't return the quite
                        // arbitrary basetypes which have to be constructed in this scope this could
                        // be solved by providing the function with a mutable reference to some
                        // scratch but would be quite ugly
                        ConstantValue::String(_) => return ctx.types.cstring.clone(),
                    }
                }
            }
            SymbolBody::Redeclaration(_)
            | SymbolBody::Basetype { .. }
            | SymbolBody::Included { .. }
            | SymbolBody::Struct { .. }
            | SymbolBody::Funcpointer { .. } => return Type::from_only_basetype(symbol),
            SymbolBody::Command { .. } | SymbolBody::Define { .. } => unreachable!(),
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

fn camel_to_snake(str: &str, out: &mut String) {
    out.clear();

    let mut iter = CamelCaseSplit::new(str);
    out.push_str(iter.next().unwrap());
    for next in iter {
        out.push('_');
        out.push_str(next);
    }
    out.make_ascii_lowercase();
}

// this will fuzzy match on the member name and strip the enum name and the extension tag boilerplate
//  VkDebugReportFlagsEXT -> Vk Debug Report Flags EXT
//  VK_DEBUG_REPORT_INFORMATION_BIT_EXT -> VK DEBUG REPORT INFORMATION BIT EXT
// => INFORMATION BIT
fn make_enum_member_rusty(
    enum_name: UniqueStr,
    member_name: UniqueStr,
    constant_syntax: bool,
    out: &mut String,
) {
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

    out.clear();

    // workaround for identifiers starting with a digit, see above
    let mut prev_char = None;

    let enum_str = enum_name.resolve_original();
    let member_str = member_name.resolve_original();

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
            *out += mstr;

            // currently we pushed into out a string that is all upeercase due to being derived from the member string
            // which is following constant syntax, now if we don't want to output constant syntax we make all but the
            // first letter that we just added lowercase
            if !constant_syntax {
                out[(len + 1)..].make_ascii_lowercase();
            }
        }

        prev_char = Some(start_char);
    }
}

// Since C is such a lovely language, some types have different semantics when they are used as a function argument.
// For example, consider:
//   void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4]) {}
// the `const float [4]` actually means (in rust) `*const [float; 4]`
// this means that types in function arguments must be specially handled
fn convert_type_in_fun_ctx(ty: &Type) -> Type {
    // arr 4, const, float
    let mut out = SmallVec::new();
    let mut tokens = ty.0.iter().peekable();

    while let Some(&token) = tokens.next() {
        match token {
            TyToken::Array(_) => {
                out.push(TyToken::Ptr);
                // if we have a situation like
                //   `const char[4]`
                //   Arr(4), Const, Ident(Char)
                // we would get
                //   Ptr, Arr(4), Const, Ident(Char)
                // yet we want
                //   Ptr, Const, Arr(4), Ident(Char)
                // which is obviously invalid, so if we see a Const when
                // emitting a pointer, we eat it from the Arr and re-emit it
                // after the pointer, after which the actual array is emitted
                if let Some(TyToken::Const) = tokens.peek() {
                    out.push(TyToken::Const);
                    tokens.next();
                }
            }
            _ => {}
        }
        out.push(token);
    }

    Type(out)
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

    let mut buf = String::new();
    for &(enum_name, member_name, const_expect, normal_expect) in data {
        let enum_name = enum_name.intern(&reg);
        let member_name = member_name.intern(&reg);

        make_enum_member_rusty(enum_name, member_name, true, &mut buf);
        assert_eq!(const_expect, buf);

        make_enum_member_rusty(enum_name, member_name, false, &mut buf);
        assert_eq!(normal_expect, buf);
    }
}
