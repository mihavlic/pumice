#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html)
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shared_present_supported_usage_flags: crate::vk10::ImageUsageFlags,
}
impl Default for SharedPresentSurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            shared_present_supported_usage_flags: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSwapchainStatusKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html)
pub unsafe fn get_swapchain_status_khr(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_swapchain_status_khr
        .unwrap())(device, swapchain)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html)
    pub unsafe fn get_swapchain_status_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let get_swapchain_status_khr = (*self.table).get_swapchain_status_khr.unwrap();
        let result = get_swapchain_status_khr(self.handle, swapchain);
        crate::new_result(result, result)
    }
}
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_shared_presentable_image"
);
