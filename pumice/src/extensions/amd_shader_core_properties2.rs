#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html)
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}
impl Default for PhysicalDeviceShaderCoreProperties2AMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
            p_next: std::ptr::null_mut(),
            shader_core_features: Default::default(),
            active_compute_unit_count: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagsAMD(pub u32);
crate::bitflags_impl! {
    ShaderCorePropertiesFlagsAMD : u32, 0x0,
}
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_shader_core_properties2"
);
