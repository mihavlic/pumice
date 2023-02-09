#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html)
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub components: crate::vk10::ComponentMapping,
    pub srgb: crate::vk10::Bool32,
}
impl Default for SamplerBorderColorComponentMappingCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            components: Default::default(),
            srgb: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html)
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub border_color_swizzle: crate::vk10::Bool32,
    pub border_color_swizzle_from_image: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            border_color_swizzle: Default::default(),
            border_color_swizzle_from_image: Default::default(),
        }
    }
}
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_border_color_swizzle"
);
