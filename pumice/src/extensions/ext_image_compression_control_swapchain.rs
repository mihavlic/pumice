#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT.html)
pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_compression_control_swapchain: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            image_compression_control_swapchain: Default::default(),
        }
    }
}
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION: u32 = 1;
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_image_compression_control_swapchain"
);
