#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPresentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html)
pub struct DisplayPresentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_rect: crate::vk10::Rect2D,
    pub dst_rect: crate::vk10::Rect2D,
    pub persistent: crate::vk10::Bool32,
}
impl Default for DisplayPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            src_rect: Default::default(),
            dst_rect: Default::default(),
            persistent: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
pub unsafe fn create_shared_swapchains_khr(
    device: crate::vk10::Device,
    swapchain_count: u32,
    p_create_infos: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_swapchains: *mut crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_shared_swapchains_khr
        .unwrap())(device, swapchain_count, p_create_infos, p_allocator, p_swapchains)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[crate::extensions::khr_swapchain::SwapchainCreateInfoKHR],
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<Vec<crate::extensions::khr_swapchain::SwapchainKHR>> {
        let create_shared_swapchains_khr = (*self.table)
            .create_shared_swapchains_khr
            .unwrap();
        let swapchain_count = create_infos.len();
        let mut swapchains = vec![Default::default(); swapchain_count as usize];
        let result = create_shared_swapchains_khr(
            self.handle,
            swapchain_count as _,
            create_infos.as_ptr(),
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            swapchains.as_mut_ptr(),
        );
        crate::new_result(swapchains, result)
    }
}
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_display_swapchain"
);
