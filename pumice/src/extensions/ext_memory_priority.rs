#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html)
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_priority: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            memory_priority: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html)
pub struct MemoryPriorityAllocateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub priority: std::os::raw::c_float,
}
impl Default for MemoryPriorityAllocateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
            p_next: std::ptr::null(),
            priority: Default::default(),
        }
    }
}
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_memory_priority"
);
