#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html)
pub struct PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_early_and_late_fragment_tests: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
            p_next: std::ptr::null_mut(),
            shader_early_and_late_fragment_tests: Default::default(),
        }
    }
}
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION: u32 = 1;
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_shader_early_and_late_fragment_tests"
);
