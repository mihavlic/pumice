#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoKHR.html)
pub type ExternalMemoryImageCreateInfoKHR = crate::vk11::ExternalMemoryImageCreateInfo;
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfoKHR.html)
pub type ExternalMemoryBufferCreateInfoKHR = crate::vk11::ExternalMemoryBufferCreateInfo;
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoKHR.html)
pub type ExportMemoryAllocateInfoKHR = crate::vk11::ExportMemoryAllocateInfo;
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = crate::vk11::QUEUE_FAMILY_EXTERNAL;
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_memory"
);
