#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html)
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_smcount: u32,
    pub shader_warps_per_sm: u32,
}
impl Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_smcount: Default::default(),
            shader_warps_per_sm: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html)
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_smbuiltins: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            shader_smbuiltins: Default::default(),
        }
    }
}
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_shader_sm_builtins"
);
