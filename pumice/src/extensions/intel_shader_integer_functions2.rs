#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_integer_functions_2: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL,
            p_next: std::ptr::null_mut(),
            shader_integer_functions_2: Default::default(),
        }
    }
}
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_INTEL_shader_integer_functions2"
);
