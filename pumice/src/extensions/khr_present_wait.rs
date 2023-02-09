#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html)
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_wait: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePresentWaitFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            present_wait: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWaitForPresentKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html)
pub unsafe fn wait_for_present_khr(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .wait_for_present_khr
        .unwrap())(device, swapchain, present_id, timeout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWaitForPresentKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html)
    pub unsafe fn wait_for_present_khr(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let wait_for_present_khr = (*self.table).wait_for_present_khr.unwrap();
        let result = wait_for_present_khr(
            self.handle,
            swapchain,
            present_id as _,
            timeout as _,
        );
        crate::new_result(result, result)
    }
}
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_present_wait"
);
