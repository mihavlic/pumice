use generator_lib::{Intern, Toplevel, ToplevelBody};
use lasso::Spur;

use crate::Context;

enum Workaround {
    Remove,
    Replace(ToplevelBody),
    SetOwnership(u32),
}

fn sorted_get_all(name: Spur, buf: &[(Spur, Workaround)]) -> Option<&[(Spur, Workaround)]> {
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

    let mut end = index;
    while index <= buf.len() {
        end += 1;
        if buf[end].0 != name {
            break;
        }
    }

    Some(&buf[start..end])
}

pub fn apply_workarounds(ctx: &mut Context) {
    let Context {
        reg,
        item_ownership,
        sections,
    } = ctx;

    #[rustfmt::skip]
    let mut workarounds = [
        // all the different identifiers commented as typos in vk.xml 
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
        (Workaround::Remove, "VK_GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME"),
    ].into_iter().map(|(method, name)| (name.intern(reg), method)).collect::<Vec<_>>();

    workarounds.sort_by_key(|s| s.0);

    let mut i = 0;
    'outer: while i < reg.toplevel.len() {
        let Toplevel(name, body) = &mut reg.toplevel[i];

        for (_, method) in sorted_get_all(*name, &workarounds).unwrap_or(&[]) {
            match method {
                Workaround::Remove => {
                    let name = *name; // borrowchecker things
                    reg.toplevel.remove(i);
                    reg.item_map.remove(&name).unwrap();
                    item_ownership.remove(i);

                    // need to adjust all the following indexes in the item_map because we've just deleted an element
                    for Toplevel(name, ..) in &reg.toplevel[i..] {
                        reg.item_map.get_mut(name).unwrap().0 -= 1;
                    }

                    continue 'outer;
                }
                Workaround::Replace(with) => {
                    // technically the clone here is unneccessary as each workaround will be applied only once
                    *body = with.clone();
                }
                Workaround::SetOwnership(idx) => {
                    item_ownership[i] = *idx;
                }
            }
        }

        match body {
            ToplevelBody::Enum { members } => prune_vec(members, &workarounds),
            ToplevelBody::BitmaskBits { members } => prune_vec(members, &workarounds),
            _ => {}
        }

        i += 1;
    }
}

fn prune_vec<T>(vec: &mut Vec<(Spur, T)>, workarounds: &Vec<(Spur, Workaround)>) {
    let mut i = 0;
    while i < vec.len() {
        if let Ok(j) = workarounds.binary_search_by_key(&vec[i].0, |s| s.0) {
            match workarounds[j].1 {
                Workaround::Remove => {
                    vec.remove(i);
                    continue;
                }
                _ => unreachable!(),
            }
        }
        i += 1;
    }
}
