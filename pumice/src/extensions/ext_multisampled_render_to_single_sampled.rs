#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT.html)
pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub multisampled_render_to_single_sampled: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            multisampled_render_to_single_sampled: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassResolvePerformanceQueryEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassResolvePerformanceQueryEXT.html)
pub struct SubpassResolvePerformanceQueryEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub optimal: crate::vk10::Bool32,
}
impl Default for SubpassResolvePerformanceQueryEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT,
            p_next: std::ptr::null_mut(),
            optimal: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMultisampledRenderToSingleSampledInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultisampledRenderToSingleSampledInfoEXT.html)
pub struct MultisampledRenderToSingleSampledInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub multisampled_render_to_single_sampled_enable: crate::vk10::Bool32,
    pub rasterization_samples: crate::vk10::SampleCountFlags,
}
impl Default for MultisampledRenderToSingleSampledInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT,
            p_next: std::ptr::null(),
            multisampled_render_to_single_sampled_enable: Default::default(),
            rasterization_samples: Default::default(),
        }
    }
}
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION: u32 = 1;
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_multisampled_render_to_single_sampled"
);
