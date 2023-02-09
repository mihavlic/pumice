#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html)
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_density_map_deferred: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_density_map_deferred: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2PropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html)
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subsampled_loads: crate::vk10::Bool32,
    pub subsampled_coarse_reconstruction_early_access: crate::vk10::Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
}
impl Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            subsampled_loads: Default::default(),
            subsampled_coarse_reconstruction_early_access: Default::default(),
            max_subsampled_array_layers: Default::default(),
            max_descriptor_set_subsampled_samplers: Default::default(),
        }
    }
}
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_fragment_density_map2"
);
