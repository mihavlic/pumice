#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html)
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub primitive_topology_list_restart: crate::vk10::Bool32,
    pub primitive_topology_patch_list_restart: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            primitive_topology_list_restart: Default::default(),
            primitive_topology_patch_list_restart: Default::default(),
        }
    }
}
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_primitive_topology_list_restart"
);
