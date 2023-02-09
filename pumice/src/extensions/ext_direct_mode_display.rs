#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkReleaseDisplayEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html)
pub unsafe fn release_display_ext(
    physical_device: crate::vk10::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .release_display_ext
        .unwrap())(physical_device, display)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkReleaseDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html)
    pub unsafe fn release_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::VulkanResult<()> {
        let release_display_ext = (*self.table).release_display_ext.unwrap();
        let result = release_display_ext(physical_device, display);
        crate::new_result((), result)
    }
}
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_direct_mode_display"
);
