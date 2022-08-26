use std::collections::HashMap;
use std::fmt::Write;

use super::{is_std_type, resolve_alias, AddedVariants};
use crate::{context::Context, switch};
use generator_lib::{
    interner::{Intern, UniqueStr},
    Declaration, Symbol, SymbolBody,
};

pub fn apply_renames(added_variants: &HashMap<UniqueStr, Vec<AddedVariants>>, ctx: &Context) {
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
        // ffi types
        ("void", "c_void"),
        ("int", "c_int"),
        ("char", "c_char"),
        ("float", "c_float"),
        ("double", "c_double"),
        // avoid conflicts
        ("type", "kind"),
        // the normal renaming makes it into "common" which is confusing
        ("vulkan_video_codecs_common", "video_common"),
    ];

    for &(from, to) in renames {
        from.intern(ctx).rename(to.intern(ctx));
    }

    let mut buf = String::new();

    for feature in &ctx.reg.features {
        buf.clear();
        write!(buf, "vk{}{}", feature.major, feature.minor).unwrap();
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
        buf.push_str(stripped.unwrap_or(name));
        buf.make_ascii_lowercase();

        extension.name.rename(buf.intern(ctx));
    }

    'outer: for &(i, _) in &ctx.symbols {
        let Symbol(name, _) = &ctx.reg.symbols[i];

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

    // collect all enums that alias a given other enum
    let mut enum_aliases: HashMap<UniqueStr, Vec<UniqueStr>> = HashMap::new();
    for &(i, _) in &ctx.symbols {
        let &Symbol(name, ref body) = &ctx.reg.symbols[i];

        match body {
            &SymbolBody::Alias(of) => {
                if !is_std_type(of, &ctx) {
                    let (target_name, body) = resolve_alias(of, &ctx);
                    match body {
                        SymbolBody::Alias { .. } => {
                            enum_aliases.entry(target_name).or_default().push(name);
                        }
                        _ => {}
                    }
                };
            }
            _ => {}
        }
    }

    for &(i, _) in &ctx.symbols {
        let &Symbol(name, ref body) = &ctx.reg.symbols[i];

        match body {
            &SymbolBody::Alias(of) => {
                if !is_std_type(of, &ctx) {
                    let (_, body) = resolve_alias(of, &ctx);
                    match body {
                        SymbolBody::Command { .. } => {
                            camel_to_snake(name.resolve(), &mut buf);
                            name.rename(buf.intern(&ctx));
                        }
                        _ => {}
                    }
                };
            }
            SymbolBody::Enum { members, .. } => {
                let aliases = enum_aliases.get(&name);
                let prefixes = aliases
                    .into_iter()
                    .flatten()
                    .map(|s| CamelCaseSplit::new(s.resolve()).last().unwrap())
                    .filter(|&s| is_tag_name(s))
                    .collect::<Vec<_>>();

                for &(member_name, ..) in members {
                    make_enum_member_rusty(
                        name.resolve_original(),
                        member_name.resolve_original(),
                        true,
                        &mut buf,
                    );

                    let mut stripped = None;
                    for &prefix in &prefixes {
                        if let Some(strip) = buf.strip_suffix(prefix) {
                            stripped = Some(strip);
                            break;
                        }
                    }

                    let result = stripped.unwrap_or(&buf);
                    member_name.rename(result.intern(&ctx));
                }
            }
            SymbolBody::Struct { members, .. } => {
                for &Declaration { name, .. } in members {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
            }
            SymbolBody::Funcpointer { args, .. } => {
                for &(name, ..) in args {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
                buf.clear();
                let strip = name.resolve().strip_prefix("PFN_vk").unwrap();
                write!(buf, "Pfn{}", strip).unwrap();
                name.rename(buf.intern(ctx));
            }
            SymbolBody::Command { params, .. } => {
                for &Declaration { name, .. } in params {
                    camel_to_snake(name.resolve(), &mut buf);
                    name.rename(buf.intern(&ctx));
                }
                camel_to_snake(name.resolve(), &mut buf);
                name.rename(buf.intern(&ctx));
            }
            _ => {}
        }
    }

    for (enum_name, added) in added_variants {
        for variants in added {
            if !variants.applicable {
                continue;
            }

            // can't do this, VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR creates ambiguity
            // // for example VK_KHR_push_descriptor
            // let tag = variants.source_extension.resolve_original().split('_').nth(1).unwrap();
            // let tag = if is_tag_name(tag) {
            //     Some(tag)
            // } else {
            //     None
            // };

            for &(name, _) in &variants.variants {
                make_enum_member_rusty(
                    enum_name.resolve_original(),
                    name.resolve_original(),
                    true,
                    &mut buf,
                );

                // let result = tag.and_then(|t| buf.strip_suffix(t).map(|s| s.trim_end_matches('_')) ).unwrap_or(&buf);
                name.rename(buf.intern(&ctx));
            }
        }
    }
}

pub fn is_tag_name(s: &str) -> bool {
    switch!(
        s;
        "KHR", "EXT", "NV", "AMD", "VALVE", "IMG", "AMDX", "ARM", "FSL", "BRCM", "NXP", "NVX", "VIV", "VSI", "KDAB", "ANDROID", "CHROMIUM",
        "FUCHSIA", "GGP", "GOOGLE", "QCOM", "LUNARG", "NZXT", "SAMSUNG", "SEC", "TIZEN", "RENDERDOC", "NN", "MVK",
        "KHX", "MESA", "INTEL", "HUAWEI", "QNX", "JUICE", "FB", "RASTERGRID" => true;
        @ false
    )
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

pub fn camel_to_snake(str: &str, out: &mut String) {
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
pub fn make_enum_member_rusty(
    enum_name: &str,
    member_name: &str,
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

    // we skip "Flags" as they are part of the enum boilerplate but don't occur in the member, see above
    let mut enum_chunks = CamelCaseSplit::new(enum_name)
        .filter(|s| *s != "Flags")
        .peekable();

    let mut member_chunks = member_name.split('_').filter(|&s| s != "BIT");

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

#[test]
fn test_enum_rustify() {
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
        (
            "VkStructureType",
            "VK_STRUCTURE_TYPE_APPLICATION_INFO",
            "APPLICATION_INFO",
            "ApplicationInfo",
        ),
    ];

    let mut buf = String::new();
    for &(enum_name, member_name, const_expect, normal_expect) in data {
        make_enum_member_rusty(enum_name, member_name, true, &mut buf);
        assert_eq!(const_expect, buf);

        make_enum_member_rusty(enum_name, member_name, false, &mut buf);
        assert_eq!(normal_expect, buf);
    }
}
