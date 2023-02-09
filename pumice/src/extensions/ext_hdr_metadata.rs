#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkXYColorEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html)
pub struct XYColorEXT {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
}
impl Default for XYColorEXT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkHdrMetadataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html)
pub struct HdrMetadataEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: std::os::raw::c_float,
    pub min_luminance: std::os::raw::c_float,
    pub max_content_light_level: std::os::raw::c_float,
    pub max_frame_average_light_level: std::os::raw::c_float,
}
impl Default for HdrMetadataEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::HDR_METADATA_EXT,
            p_next: std::ptr::null(),
            display_primary_red: Default::default(),
            display_primary_green: Default::default(),
            display_primary_blue: Default::default(),
            white_point: Default::default(),
            max_luminance: Default::default(),
            min_luminance: Default::default(),
            max_content_light_level: Default::default(),
            max_frame_average_light_level: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetHdrMetadataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html)
pub unsafe fn set_hdr_metadata_ext(
    device: crate::vk10::Device,
    swapchain_count: u32,
    p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
    p_metadata: *const HdrMetadataEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_hdr_metadata_ext
        .unwrap())(device, swapchain_count, p_swapchains, p_metadata)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkSetHdrMetadataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html)
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        swapchains: &[crate::extensions::khr_swapchain::SwapchainKHR],
        metadata: &[HdrMetadataEXT],
    ) {
        let set_hdr_metadata_ext = (*self.table).set_hdr_metadata_ext.unwrap();
        let swapchain_count = swapchains.len().min(metadata.len());
        set_hdr_metadata_ext(
            self.handle,
            swapchain_count as _,
            swapchains.as_ptr(),
            metadata.as_ptr(),
        );
    }
}
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
pub const EXT_HDR_METADATA_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_hdr_metadata"
);
