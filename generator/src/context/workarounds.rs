use std::fmt::Write;

use super::Context;
use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::parse_type_decl,
    ConstantValue, FeatureExtensionItem, InterfaceItem, RedeclarationMethod, Symbol, SymbolBody,
};

enum Workaround {
    Remove,
    Replace(SymbolBody),
    ReplaceRequireExtendValue(ConstantValue),
    SetOwnership(u32),
}

fn sorted_get_all(
    name: UniqueStr,
    buf: &[(UniqueStr, Workaround)],
) -> Option<&[(UniqueStr, Workaround)]> {
    let index = buf.binary_search_by_key(&name, |s| s.0).ok()?;

    let mut start = index;
    while start > 0 {
        let below = start - 1;
        if buf[below].0 != name {
            break;
        } else {
            start = below;
        }
    }

    let mut end = index + 1;
    while end < buf.len() {
        if buf[end].0 != name {
            break;
        }
        end += 1;
    }

    Some(&buf[start..end])
}

pub fn apply_workarounds(ctx: &mut Context) {
    let alias =
        |arg: &str| -> Workaround { Workaround::Replace(SymbolBody::Alias(arg.intern(ctx))) };

    let redeclare = |arg: &str| -> Workaround {
        Workaround::Replace(SymbolBody::Redeclaration(RedeclarationMethod::Type(
            parse_type_decl(arg, ctx).1,
        )))
    };

    let redeclare_custom = |arg: fn(&mut dyn Write) -> std::fmt::Result| -> Workaround {
        Workaround::Replace(SymbolBody::Redeclaration(RedeclarationMethod::Custom(arg)))
    };

    let ownership = |arg: &str| -> Workaround {
        let name = arg.intern(ctx);
        Workaround::SetOwnership(
            ctx.get_section_idx(ctx.strings.pumice..name)
                .unwrap_or_else(|| panic!("No such section '{}'", arg)),
        )
    };

    let alias_extend = |arg: &str| -> Workaround {
        Workaround::ReplaceRequireExtendValue(ConstantValue::Symbol(arg.intern(ctx)))
    };

    #[rustfmt::skip]
    let mut workarounds = [
        // useless macros that break our useless macro hacks
        (Workaround::Remove, "VK_DEFINE_HANDLE"),
        (Workaround::Remove, "VK_USE_64_BIT_PTR_DEFINES"),
        (Workaround::Remove, "VK_NULL_HANDLE"),
        (Workaround::Remove, "VK_DEFINE_NON_DISPATCHABLE_HANDLE"),
        // deprecated version packing macros
        (Workaround::Remove, "VK_MAKE_VERSION"),
        (Workaround::Remove, "VK_VERSION_MAJOR"),
        (Workaround::Remove, "VK_VERSION_MINOR"),
        (Workaround::Remove, "VK_VERSION_PATCH"),
        (Workaround::Remove, "VK_API_VERSION"),
        // manually reimplemented version macros, it seems that erupt can do this arbitrarily but this is significantly less effort 
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {{
                        (variant << 29) | (major << 22) | (minor << 12) | patch
                    }}"
                )
            ),
            "VK_MAKE_API_VERSION"
        ),
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn api_version_variant(version: u32) -> u32 {{
                        version >> 29
                    }}"
                )
            ),
            "VK_API_VERSION_VARIANT"
        ),
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn api_version_major(version: u32) -> u32 {{
                        (version >> 22) & 0x7f
                    }}"
                )
            ),
            "VK_API_VERSION_MAJOR"
        ),
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn api_version_minor(version: u32) -> u32 {{
                        (version >> 12) & 0x3ff
                    }}"
                )
            ),
            "VK_API_VERSION_MINOR"
        ),
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn api_version_patch(version: u32) -> u32 {{
                        version & 0xfff
                    }}"
                )
            ),
            "VK_API_VERSION_PATCH"
        ),
        (
            redeclare_custom(|w| 
                write!(w,
                    "pub const fn make_video_std_version(major: u32, minor: u32, patch: u32) -> u32 {{
                        (major << 22) | (minor << 12) | patch
                    }}"
                )
            ),
            "VK_MAKE_VIDEO_STD_VERSION"
        ),
        // vk_platform is in <require> tags??
        (Workaround::Remove, "vk_platform"),
        // for some reason the video.xml extensions have requires that contain headers, which are not symbols!
        (Workaround::Remove, "vk_video/vulkan_video_codecs_common.h"),
        (Workaround::Remove, "vk_video/vulkan_video_codec_h264std.h"),
        (Workaround::Remove, "vk_video/vulkan_video_codec_h265std.h"),
        // VK_PIPELINE_CREATE_DISPATCH_BASE is a typo without the BIT, yet VK_PIPELINE_CREATE_DISPATCH_BASE_KHR is derived off it
        (Workaround::Remove, "VK_PIPELINE_CREATE_DISPATCH_BASE"),
        (alias_extend("VK_PIPELINE_CREATE_DISPATCH_BASE_BIT"), "VK_PIPELINE_CREATE_DISPATCH_BASE_KHR"),
        // symbols commented as typos in vk.xml, they are messy and at worst break our renaming schemes 
        (Workaround::Remove, "VK_STENCIL_FRONT_AND_BACK"),
        (Workaround::Remove, "VK_COLORSPACE_SRGB_NONLINEAR_KHR"),
        (Workaround::Remove, "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT"),
        (Workaround::Remove, "VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT"),
        (Workaround::Remove, "VK_SURFACE_COUNTER_VBLANK_EXT"),
        (Workaround::Remove, "VK_QUERY_SCOPE_COMMAND_BUFFER_KHR"),
        (Workaround::Remove, "VK_QUERY_SCOPE_RENDER_PASS_KHR"),
        (Workaround::Remove, "VK_QUERY_SCOPE_COMMAND_KHR"),
        (Workaround::Remove, "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR"),
        (Workaround::Remove, "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR"),
        (Workaround::Remove, "VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT"),
        (Workaround::Remove, "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
        (Workaround::Remove, "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE1_SPEC_VERSION"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE1_EXTENSION_NAME"),
        (Workaround::Remove, "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT"),
        (Workaround::Remove, "VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION"),
        (Workaround::Remove, "VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME"),
        (Workaround::Remove, "VK_COLOR_SPACE_DCI_P3_LINEAR_EXT"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE2_SPEC_VERSION"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE2_EXTENSION_NAME"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE3_SPEC_VERSION"),
        (Workaround::Remove, "VK_KHR_MAINTENANCE3_EXTENSION_NAME"),
        (Workaround::Remove, "VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL"),
        (Workaround::Remove, "VK_GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION"),
        // hardcoded types that are normally included from headers
        // from https://github.com/Friz64/erupt/blob/9bce30f1a1239d0198258abc60473e3c9f1d6f8a/generator/src/declaration.rs#L100
        (alias("void"),             "Display"),
        (alias("uint64_t"),         "VisualID"),
        (alias("uint64_t"),         "Window"),
        (alias("uint64_t"),         "RROutput"),
        (alias("void"),             "xcb_connection_t"),
        (alias("uint32_t"),         "xcb_visualid_t"),
        (alias("uint32_t"),         "xcb_window_t"),
        (alias("void"),             "wl_display"),
        (alias("void"),             "wl_surface"),
        (alias("void"),             "SECURITY_ATTRIBUTES"),
        (alias("uint32_t"),         "DWORD"),
        (redeclare("void *"),       "HINSTANCE"),
        (redeclare("void *"),       "HWND"),
        (redeclare("void *"),       "HMONITOR"),
        (redeclare("void *"),       "HANDLE"),
        (redeclare("const uint16_t*"), "LPCWSTR"),
        (redeclare("void *"),       "zx_handle_t"),
        (alias("uint32_t"),         "GgpStreamDescriptor"),
        (alias("uint64_t"),         "GgpFrameToken"),
        (alias("void"),             "IDirectFB"),
        (alias("void"),             "IDirectFBSurface"),
        (alias("void"),             "_screen_context"),
        (alias("void"),             "_screen_window"),
        // manually add ownership that is missing / symbols assumed to be imported from headers 
        (ownership("VK_KHR_xcb_surface"), "Display"),
        (ownership("VK_KHR_xcb_surface"), "VisualID"),
        (ownership("VK_KHR_xcb_surface"), "Window"),
        (ownership("VK_KHR_xcb_surface"), "RROutput"),
        (ownership("VK_KHR_xcb_surface"), "xcb_connection_t"),
        (ownership("VK_KHR_xcb_surface"), "xcb_visualid_t"),
        (ownership("VK_KHR_xcb_surface"), "xcb_window_t"),
        (ownership("VK_KHR_wayland_surface"), "wl_display"),
        (ownership("VK_KHR_wayland_surface"), "wl_surface"),
        (ownership("VK_KHR_win32_surface"), "HINSTANCE"),
        (ownership("VK_KHR_win32_surface"), "HWND"),
        (ownership("VK_KHR_win32_surface"), "HMONITOR"),
        (ownership("VK_KHR_win32_surface"), "HANDLE"),
        (ownership("VK_KHR_win32_surface"), "SECURITY_ATTRIBUTES"),
        (ownership("VK_KHR_win32_surface"), "DWORD"),
        (ownership("VK_KHR_win32_surface"), "LPCWSTR"),
        (ownership("VK_EXT_directfb_surface"), "IDirectFB"),
        (ownership("VK_EXT_directfb_surface"), "IDirectFBSurface"),
        (ownership("VK_FUCHSIA_imagepipe_surface"), "zx_handle_t"),
        (ownership("VK_GGP_stream_descriptor_surface"), "GgpStreamDescriptor"),
        (ownership("VK_GGP_stream_descriptor_surface"), "GgpFrameToken"),
        (ownership("VK_QNX_screen_surface"), "_screen_context"),
        (ownership("VK_QNX_screen_surface"), "_screen_window"),
        // apple stuff which is weird due to objective C
        (alias("void"),             "ANativeWindow"),
        (alias("void"),             "AHardwareBuffer"),
        (alias("void"),             "CAMetalLayer"),
        (redeclare("void *"),       "MTLDevice_id"),
        (redeclare("void *"),       "MTLCommandQueue_id"),
        (redeclare("void *"),       "MTLBuffer_id"),
        (redeclare("void *"),       "MTLTexture_id"),
        (redeclare("void *"),       "MTLSharedEvent_id"),
        (redeclare("void *"),       "IOSurfaceRef"),
    ].into_iter().map(|(method, name)| (name.intern(ctx), method)).collect::<Vec<_>>();

    // base types that are included from the cursed `vk_platform`, they are removed and then have a special case in the path resolution function
    // the meaning of basetype is changed from that of the registry to mean a primitive type that cannot be decomposed rather than a weird edge case
    let basetypes = [
        "void", "int", "char", "float", "double", "bool", "uint8_t", "uint16_t", "uint32_t",
        "uint64_t", "int8_t", "int16_t", "int32_t", "int64_t", "size_t",
    ];

    {
        let vk10 = ctx
            .get_section_idx(ctx.strings.pumice..ctx.strings.VK_VERSION_1_0)
            .unwrap();

        for base in basetypes {
            let name = base.intern(ctx);

            let body = SymbolBody::Basetype {
                code: "__implementation_detail".to_owned(),
            };

            if ctx.get_symbol_index(name).is_some() {
                workarounds.push((name, Workaround::Replace(body)));
            } else {
                ctx.reg.add_symbol(name, body);
            }

            workarounds.push((name, Workaround::SetOwnership(vk10)));
        }
    }

    workarounds.sort_by_key(|s| s.0);

    let mut i = 0;
    'outer: while i < ctx.reg.symbols.len() {
        let Symbol(name, body) = &mut ctx.reg.symbols[i];

        for (_, method) in sorted_get_all(*name, &workarounds).unwrap_or(&[]) {
            match method {
                Workaround::Remove => {
                    // borrowchk tricks
                    let name = *name;
                    ctx.remove_symbol(name);
                    continue 'outer;
                }
                Workaround::Replace(with) => {
                    // FIXME technically the clone here is unneccessary as each workaround will be applied only once
                    *body = with.clone();
                }
                Workaround::SetOwnership(section_idx) => {
                    ctx.symbol_ownership.insert(*name, *section_idx);
                }
                Workaround::ReplaceRequireExtendValue(_) => {}
            }
        }

        match body {
            SymbolBody::Enum { members, .. } => prune_leaf_vec(members, |(s, _)| *s, &workarounds),
            SymbolBody::Alias(_) => {
                // QUIRK when aliasing promoted variants to preserve backwards compatibility,
                // both the *Flags and *FlagBits are aliased, since we remove *FlagBits
                // we also need to remove this alias
                if name.resolve().contains("FlagBits") {
                    // borrowchk tricks
                    let name = *name;
                    ctx.remove_symbol(name);
                    // mark this symbol as removed so that resolve_ownership can ignore it
                    name.rename(ctx.strings.__RESERVED_INVALID_PLACEHOLDER);
                    continue 'outer;
                }
            }
            _ => {}
        }

        i += 1;
    }

    for feature in &mut ctx.reg.features {
        prune_feature_extension(&mut feature.children, &workarounds);
    }

    for ext in &mut ctx.reg.extensions {
        if let Ok(j) = workarounds.binary_search_by_key(&ext.name, |s| s.0) {
            match &workarounds[j].1 {
                Workaround::Remove => {
                    // extension have by now been turned into section in a specific order
                    // thus we can't just remove it, best we can do is disable it
                    // todo this is not ideal
                    let supported = &mut ext.supported;
                    supported.clear();
                    supported.push(ctx.strings.disabled);
                    continue;
                }
                _ => {}
            }
        }
        prune_feature_extension(&mut ext.children, &workarounds);
    }
}

fn prune_feature_extension(
    children: &mut Vec<FeatureExtensionItem>,
    workarounds: &Vec<(UniqueStr, Workaround)>,
) {
    for child in children {
        match child {
            FeatureExtensionItem::Require { items, .. } => {
                let mut i = 0;
                while i < items.len() {
                    let name = match &items[i] {
                        InterfaceItem::Simple { name, .. } => *name,
                        InterfaceItem::Extend { name, .. } => *name,
                    };

                    if let Ok(j) = workarounds.binary_search_by_key(&name, |s| s.0) {
                        match &workarounds[j].1 {
                            Workaround::Remove => {
                                items.remove(i);
                                continue;
                            }
                            Workaround::ReplaceRequireExtendValue(val) => {
                                if let InterfaceItem::Extend { value, .. } = &mut items[i] {
                                    *value = val.clone();
                                } else {
                                    unreachable!()
                                }
                            }
                            _ => {}
                        }
                    }
                    i += 1;
                }
            }
            _ => {}
        }
    }
}

fn prune_leaf_vec<T, F: Fn(&T) -> UniqueStr>(
    vec: &mut Vec<T>,
    fun: F,
    workarounds: &Vec<(UniqueStr, Workaround)>,
) {
    let mut i = 0;
    while i < vec.len() {
        let name = fun(&vec[i]);
        if let Ok(j) = workarounds.binary_search_by_key(&name, |s| s.0) {
            match workarounds[j].1 {
                Workaround::Remove => {
                    vec.remove(i);
                    continue;
                }
                _ => {}
            }
        }
        i += 1;
    }
}
