#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html)
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html)
pub type QueueFamilyGlobalPriorityPropertiesEXT = crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
pub const MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = crate::extensions::khr_global_priority::MAX_GLOBAL_PRIORITY_SIZE_KHR;
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
pub const EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_global_priority_query"
);
