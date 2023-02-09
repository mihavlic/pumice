#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html)
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pageable_device_local_memory: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            pageable_device_local_memory: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html)
pub unsafe fn set_device_memory_priority_ext(
    device: crate::vk10::Device,
    memory: crate::vk10::DeviceMemory,
    priority: std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_device_memory_priority_ext
        .unwrap())(device, memory, priority)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html)
    pub unsafe fn set_device_memory_priority_ext(
        &self,
        memory: crate::vk10::DeviceMemory,
        priority: std::os::raw::c_float,
    ) {
        let set_device_memory_priority_ext = (*self.table)
            .set_device_memory_priority_ext
            .unwrap();
        set_device_memory_priority_ext(self.handle, memory, priority as _);
    }
}
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_pageable_device_local_memory"
);
