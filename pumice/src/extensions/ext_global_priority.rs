#[doc(alias = "VkQueueGlobalPriorityEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityEXT.html)
pub type QueueGlobalPriorityEXT = crate::extensions::khr_global_priority::QueueGlobalPriorityKHR;
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html)
pub type DeviceQueueGlobalPriorityCreateInfoEXT = crate::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR;
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
pub const EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_global_priority"
);
