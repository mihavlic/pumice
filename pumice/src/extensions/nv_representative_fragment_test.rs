#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub representative_fragment_test: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            representative_fragment_test: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html)
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub representative_fragment_test_enable: crate::vk10::Bool32,
}
impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            representative_fragment_test_enable: Default::default(),
        }
    }
}
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_representative_fragment_test"
);
