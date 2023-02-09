#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html)
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub robust_buffer_access_2: crate::vk10::Bool32,
    pub robust_image_access_2: crate::vk10::Bool32,
    pub null_descriptor: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRobustness2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            robust_buffer_access_2: Default::default(),
            robust_image_access_2: Default::default(),
            null_descriptor: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html)
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub robust_storage_buffer_access_size_alignment: crate::vk10::DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceRobustness2PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            robust_storage_buffer_access_size_alignment: Default::default(),
            robust_uniform_buffer_access_size_alignment: Default::default(),
        }
    }
}
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_robustness2"
);
