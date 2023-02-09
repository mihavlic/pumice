#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html)
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub heap_budget: [crate::vk10::DeviceSize; crate::vk10::MAX_MEMORY_HEAPS as usize],
    pub heap_usage: [crate::vk10::DeviceSize; crate::vk10::MAX_MEMORY_HEAPS as usize],
}
impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            heap_budget: unsafe { std::mem::zeroed() },
            heap_usage: unsafe { std::mem::zeroed() },
        }
    }
}
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_memory_budget"
);
