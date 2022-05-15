use std::collections::HashSet;

use generator_lib::{Intern, Registry};

#[rustfmt::skip]
pub fn apply_workarounds(reg: &mut Registry) {
    // all the different identifiers commented as typos in vk.xml
    let remove = &[
        "VK_STENCIL_FRONT_AND_BACK",
        "VK_COLORSPACE_SRGB_NONLINEAR_KHR",
        "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT",
        "VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT",
        "VK_SURFACE_COUNTER_VBLANK_EXT",
        "VK_QUERY_SCOPE_COMMAND_BUFFER_KHR",
        "VK_QUERY_SCOPE_RENDER_PASS_KHR",
        "VK_QUERY_SCOPE_COMMAND_KHR",
        "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR",
        "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR",
        "VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT",
        "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR",
        "VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT",
        "VK_KHR_MAINTENANCE1_SPEC_VERSION",
        "VK_KHR_MAINTENANCE1_EXTENSION_NAME",
        "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT",
        "VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION",
        "VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME",
        "VK_COLOR_SPACE_DCI_P3_LINEAR_EXT",
        "VK_KHR_MAINTENANCE2_SPEC_VERSION",
        "VK_KHR_MAINTENANCE2_EXTENSION_NAME",
        "VK_KHR_MAINTENANCE3_SPEC_VERSION",
        "VK_KHR_MAINTENANCE3_EXTENSION_NAME",
        "VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL",
        "VK_GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION",
        "VK_GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME",
    ];

    let mut set = HashSet::new();
    for name in remove {
        let ever_false = set.insert(name.intern(&reg));
        assert!(ever_false == true, "Duplicate: {}", name);
    }

    // muah
    macro_rules! vec_remove {
        ($name:expr, $index:ident, $vec:expr) => {
            if set.contains(&$name) {
                $vec.remove($index);
                continue;
            }
            let tmp = $index;
            $index += 1;
            // shadow the index because code after this may need the one before incrementing
            let $index = tmp;
        }
    }

    macro_rules! leaf_vec_remove {
        ($vec:expr, $index:tt) => {
            let mut leaf_i = 0;
            #[allow(unused)]
            while leaf_i < $vec.len() {
                let leaf_name = &$vec[leaf_i].$index;
                vec_remove!(leaf_name, leaf_i, $vec);
            }
        }
    }

    let mut name_i = 0;
    while name_i < reg.toplevel.len() {
        vec_remove!(reg.toplevel[name_i].0, name_i, reg.toplevel);
        
        match &mut reg.toplevel[name_i].1 {
            generator_lib::ToplevelBody::Enum { members } => {leaf_vec_remove!(members, 0);}
            generator_lib::ToplevelBody::BitmaskBits { members } => {leaf_vec_remove!(members, 0);}
            _ => {}
        }
    }
}
