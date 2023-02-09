#[doc(alias = "VkDriverIdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverIdKHR.html)
pub type DriverIdKHR = crate::vk12::DriverId;
#[doc(alias = "VkConformanceVersionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConformanceVersionKHR.html)
pub type ConformanceVersionKHR = crate::vk12::ConformanceVersion;
#[doc(alias = "VkPhysicalDeviceDriverPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverPropertiesKHR.html)
pub type PhysicalDeviceDriverPropertiesKHR = crate::vk12::PhysicalDeviceDriverProperties;
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = crate::vk12::MAX_DRIVER_NAME_SIZE;
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = crate::vk12::MAX_DRIVER_INFO_SIZE;
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_driver_properties"
);
