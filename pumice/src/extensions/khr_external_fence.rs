#[doc(alias = "VkFenceImportFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagsKHR.html)
pub type FenceImportFlagsKHR = crate::vk11::FenceImportFlags;
#[doc(alias = "VkExportFenceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfoKHR.html)
pub type ExportFenceCreateInfoKHR = crate::vk11::ExportFenceCreateInfo;
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_FENCE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_fence"
);
