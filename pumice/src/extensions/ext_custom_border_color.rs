#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html)
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub custom_border_color: crate::vk10::ClearColorValue,
    pub format: crate::vk10::Format,
}
impl Default for SamplerCustomBorderColorCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            custom_border_color: Default::default(),
            format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html)
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_custom_border_color_samplers: u32,
}
impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_custom_border_color_samplers: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html)
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub custom_border_colors: crate::vk10::Bool32,
    pub custom_border_color_without_format: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            custom_border_colors: Default::default(),
            custom_border_color_without_format: Default::default(),
        }
    }
}
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_custom_border_color"
);
