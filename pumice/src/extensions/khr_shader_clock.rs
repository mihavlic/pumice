#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html)
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_subgroup_clock: crate::vk10::Bool32,
    pub shader_device_clock: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderClockFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            shader_subgroup_clock: Default::default(),
            shader_device_clock: Default::default(),
        }
    }
}
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_shader_clock"
);
