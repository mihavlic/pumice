#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html)
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}
impl Default for RefreshCycleDurationGOOGLE {
    fn default() -> Self {
        Self {
            refresh_duration: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPastPresentationTimingGOOGLE.html)
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}
impl Default for PastPresentationTimingGOOGLE {
    fn default() -> Self {
        Self {
            present_id: Default::default(),
            desired_present_time: Default::default(),
            actual_present_time: Default::default(),
            earliest_present_time: Default::default(),
            present_margin: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html)
pub struct PresentTimesInfoGOOGLE {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain_count: u32,
    pub p_times: *const PresentTimeGOOGLE,
}
impl Default for PresentTimesInfoGOOGLE {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRESENT_TIMES_INFO_GOOGLE,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_times: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPresentTimeGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html)
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}
impl Default for PresentTimeGOOGLE {
    fn default() -> Self {
        Self {
            present_id: Default::default(),
            desired_present_time: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
pub unsafe fn get_refresh_cycle_duration_google(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_refresh_cycle_duration_google
        .unwrap())(device, swapchain, p_display_timing_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::VulkanResult<RefreshCycleDurationGOOGLE> {
        let get_refresh_cycle_duration_google = (*self.table)
            .get_refresh_cycle_duration_google
            .unwrap();
        let mut display_timing_properties = Default::default();
        let result = get_refresh_cycle_duration_google(
            self.handle,
            swapchain,
            &mut display_timing_properties,
        );
        crate::new_result(display_timing_properties, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
pub unsafe fn get_past_presentation_timing_google(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    p_presentation_timing_count: *mut u32,
    p_presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_past_presentation_timing_google
        .unwrap())(
        device,
        swapchain,
        p_presentation_timing_count,
        p_presentation_timings,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        presentation_timing_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<PastPresentationTimingGOOGLE>, crate::vk10::Result)> {
        let get_past_presentation_timing_google = (*self.table)
            .get_past_presentation_timing_google
            .unwrap();
        let mut presentation_timing_count = match presentation_timing_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_past_presentation_timing_google(
                    self.handle,
                    swapchain,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut presentation_timings = vec![
            Default::default(); presentation_timing_count as usize
        ];
        let result = get_past_presentation_timing_google(
            self.handle,
            swapchain,
            &mut presentation_timing_count,
            presentation_timings.as_mut_ptr(),
        );
        crate::new_result((presentation_timings, result), result)
    }
}
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_GOOGLE_display_timing"
);
