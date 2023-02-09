#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html)
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_subgroup_uniform_control_flow: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            shader_subgroup_uniform_control_flow: Default::default(),
        }
    }
}
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_shader_subgroup_uniform_control_flow"
);
