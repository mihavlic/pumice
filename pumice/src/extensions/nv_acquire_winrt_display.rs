#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireWinrtDisplayNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html)
pub unsafe fn acquire_winrt_display_nv(
    physical_device: crate::vk10::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .acquire_winrt_display_nv
        .unwrap())(physical_device, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html)
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::VulkanResult<()> {
        let acquire_winrt_display_nv = (*self.table).acquire_winrt_display_nv.unwrap();
        let result = acquire_winrt_display_nv(physical_device, display);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetWinrtDisplayNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html)
pub unsafe fn get_winrt_display_nv(
    physical_device: crate::vk10::PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_winrt_display_nv
        .unwrap())(physical_device, device_relative_id, p_display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetWinrtDisplayNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html)
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        device_relative_id: u32,
    ) -> crate::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let get_winrt_display_nv = (*self.table).get_winrt_display_nv.unwrap();
        let mut display = Default::default();
        let result = get_winrt_display_nv(
            physical_device,
            device_relative_id as _,
            &mut display,
        );
        crate::new_result(display, result)
    }
}
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
pub const NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_acquire_winrt_display"
);
