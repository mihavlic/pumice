#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireXlibDisplayEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html)
pub unsafe fn acquire_xlib_display_ext(
    physical_device: crate::vk10::PhysicalDevice,
    dpy: *mut crate::extensions::khr_xcb_surface::Display,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .acquire_xlib_display_ext
        .unwrap())(physical_device, dpy, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html)
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::VulkanResult<()> {
        let acquire_xlib_display_ext = (*self.table).acquire_xlib_display_ext.unwrap();
        let result = acquire_xlib_display_ext(physical_device, dpy, display);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRandROutputDisplayEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html)
pub unsafe fn get_rand_routput_display_ext(
    physical_device: crate::vk10::PhysicalDevice,
    dpy: *mut crate::extensions::khr_xcb_surface::Display,
    rr_output: crate::extensions::khr_xcb_surface::RROutput,
    p_display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_rand_routput_display_ext
        .unwrap())(physical_device, dpy, rr_output, p_display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html)
    pub unsafe fn get_rand_routput_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        rr_output: crate::extensions::khr_xcb_surface::RROutput,
    ) -> crate::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let get_rand_routput_display_ext = (*self.table)
            .get_rand_routput_display_ext
            .unwrap();
        let mut display = Default::default();
        let result = get_rand_routput_display_ext(
            physical_device,
            dpy,
            rr_output as _,
            &mut display,
        );
        crate::new_result(display, result)
    }
}
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_acquire_xlib_display"
);
