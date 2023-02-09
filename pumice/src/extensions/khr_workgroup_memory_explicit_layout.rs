#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html)
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub workgroup_memory_explicit_layout: crate::vk10::Bool32,
    pub workgroup_memory_explicit_layout_scalar_block_layout: crate::vk10::Bool32,
    pub workgroup_memory_explicit_layout_8_bit_access: crate::vk10::Bool32,
    pub workgroup_memory_explicit_layout_16_bit_access: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            workgroup_memory_explicit_layout: Default::default(),
            workgroup_memory_explicit_layout_scalar_block_layout: Default::default(),
            workgroup_memory_explicit_layout_8_bit_access: Default::default(),
            workgroup_memory_explicit_layout_16_bit_access: Default::default(),
        }
    }
}
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_workgroup_memory_explicit_layout"
);
