use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::parse_type_decl,
    FeatureExtensionItem, InterfaceItem, Symbol, SymbolBody,
};

use crate::Context;

enum Workaround {
    Remove,
    Replace(SymbolBody),
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
        Workaround::Replace(SymbolBody::Redeclaration(parse_type_decl(arg, ctx).1))
    };

    let ownership = |arg: &str| -> Workaround {
        Workaround::SetOwnership(
            ctx.find_section_idx(arg.intern(ctx))
                .unwrap_or_else(|| panic!("No such section '{}'", arg)),
        )
    };

    #[rustfmt::skip]
    let mut workarounds = [
        // vk_platform is in <require> tags??
        (Workaround::Remove, "vk_platform"),
        // for some reason the video.xml extensions have requires that contain headers, which are not symbols!
        (Workaround::Remove, "vk_video/vulkan_video_codecs_common.h"),
        (Workaround::Remove, "vk_video/vulkan_video_codec_h264std.h"),
        (Workaround::Remove, "vk_video/vulkan_video_codec_h265std.h"),
        // constants that require evaluating preprocessor macros to be valid and frankly I don't care about version numbers
        // FIXME start evaluating preprocessor macros
        (Workaround::Remove, "VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION"),
        (Workaround::Remove, "VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION"),
        (Workaround::Remove, "VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_SPEC_VERSION"),
        (Workaround::Remove, "VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION"),
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
        // base types that are incluided from the cursed `vk_platform`, they are removed and then have a special case in the path resolution function
        (Workaround::Remove, "void"),
        (Workaround::Remove, "char"),
        (Workaround::Remove, "float"),
        (Workaround::Remove, "double"),
        (Workaround::Remove, "uint8_t"),
        (Workaround::Remove, "uint16_t"),
        (Workaround::Remove, "uint32_t"),
        (Workaround::Remove, "uint64_t"),
        (Workaround::Remove, "int8_t"),
        (Workaround::Remove, "int16_t"),
        (Workaround::Remove, "int32_t"),
        (Workaround::Remove, "int64_t"),
        (Workaround::Remove, "size_t"),
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
        // manually add ownership that is missing
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
        // Objective C stuff?
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

    workarounds.sort_by_key(|s| s.0);

    let mut i = 0;
    'outer: while i < ctx.reg.symbols.len() {
        let Symbol(name, body) = &mut ctx.reg.symbols[i];

        for (_, method) in sorted_get_all(*name, &workarounds).unwrap_or(&[]) {
            match method {
                Workaround::Remove => {
                    ctx.remove_symbol(i as u32);
                    continue 'outer;
                }
                Workaround::Replace(with) => {
                    // FIXME technically the clone here is unneccessary as each workaround will be applied only once
                    *body = with.clone();
                }
                Workaround::SetOwnership(idx) => {
                    ctx.symbol_ownership[i] = *idx;
                }
            }
        }

        match body {
            SymbolBody::Enum { members, .. } => prune_leaf_vec(members, |(s, _)| *s, &workarounds),
            _ => {}
        }

        i += 1;
    }

    for feature in &mut ctx.reg.features {
        prune_feature_extension(&mut feature.children, &workarounds);
    }

    for ext in &mut ctx.reg.extensions {
        prune_feature_extension(&mut ext.children, &workarounds);
    }
}

fn prune_feature_extension(
    children: &mut Vec<FeatureExtensionItem>,
    workarounds: &Vec<(UniqueStr, Workaround)>,
) {
    for child in children {
        match child {
            FeatureExtensionItem::Require { items, .. } => prune_leaf_vec(
                items,
                |item| match item {
                    InterfaceItem::Simple { name, .. } => *name,
                    InterfaceItem::Extend { name, .. } => *name,
                },
                workarounds,
            ),
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
