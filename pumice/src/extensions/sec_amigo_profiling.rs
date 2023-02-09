#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceAmigoProfilingFeaturesSEC")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html)
pub struct PhysicalDeviceAmigoProfilingFeaturesSEC {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub amigo_profiling: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceAmigoProfilingFeaturesSEC {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC,
            p_next: std::ptr::null_mut(),
            amigo_profiling: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAmigoProfilingSubmitInfoSEC")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAmigoProfilingSubmitInfoSEC.html)
pub struct AmigoProfilingSubmitInfoSEC {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub first_draw_timestamp: u64,
    pub swap_buffer_timestamp: u64,
}
impl Default for AmigoProfilingSubmitInfoSEC {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::AMIGO_PROFILING_SUBMIT_INFO_SEC,
            p_next: std::ptr::null(),
            first_draw_timestamp: Default::default(),
            swap_buffer_timestamp: Default::default(),
        }
    }
}
pub const SEC_AMIGO_PROFILING_SPEC_VERSION: u32 = 1;
pub const SEC_AMIGO_PROFILING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_SEC_amigo_profiling"
);
