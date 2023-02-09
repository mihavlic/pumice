#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html)
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub compute_derivative_group_quads: crate::vk10::Bool32,
    pub compute_derivative_group_linear: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            compute_derivative_group_quads: Default::default(),
            compute_derivative_group_linear: Default::default(),
        }
    }
}
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_compute_shader_derivatives"
);
