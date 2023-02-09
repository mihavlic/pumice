#[doc(alias = "VkSemaphoreImportFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagsKHR.html)
pub type SemaphoreImportFlagsKHR = crate::vk11::SemaphoreImportFlags;
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfoKHR.html)
pub type ExportSemaphoreCreateInfoKHR = crate::vk11::ExportSemaphoreCreateInfo;
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_semaphore"
);
