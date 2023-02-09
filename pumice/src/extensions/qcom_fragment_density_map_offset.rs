#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html)
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_density_map_offset: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
            p_next: std::ptr::null_mut(),
            fragment_density_map_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html)
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_density_offset_granularity: crate::vk10::Extent2D,
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
            p_next: std::ptr::null_mut(),
            fragment_density_offset_granularity: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassFragmentDensityMapOffsetEndInfoQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html)
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fragment_density_offset_count: u32,
    pub p_fragment_density_offsets: *const crate::vk10::Offset2D,
}
impl Default for SubpassFragmentDensityMapOffsetEndInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
            p_next: std::ptr::null(),
            fragment_density_offset_count: Default::default(),
            p_fragment_density_offsets: std::ptr::null(),
        }
    }
}
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QCOM_fragment_density_map_offset"
);
