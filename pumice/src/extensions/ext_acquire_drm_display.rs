#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireDrmDisplayEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html)
pub unsafe fn acquire_drm_display_ext(
    physical_device: crate::vk10::PhysicalDevice,
    drm_fd: i32,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .acquire_drm_display_ext
        .unwrap())(physical_device, drm_fd, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html)
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        drm_fd: i32,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::VulkanResult<()> {
        let acquire_drm_display_ext = (*self.table).acquire_drm_display_ext.unwrap();
        let result = acquire_drm_display_ext(physical_device, drm_fd as _, display);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDrmDisplayEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html)
pub unsafe fn get_drm_display_ext(
    physical_device: crate::vk10::PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_drm_display_ext
        .unwrap())(physical_device, drm_fd, connector_id, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDrmDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html)
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
    ) -> crate::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let get_drm_display_ext = (*self.table).get_drm_display_ext.unwrap();
        let mut display = Default::default();
        let result = get_drm_display_ext(
            physical_device,
            drm_fd as _,
            connector_id as _,
            &mut display,
        );
        crate::new_result(display, result)
    }
}
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
pub const EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_acquire_drm_display"
);
