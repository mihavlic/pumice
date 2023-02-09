#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html)
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub corner_sampled_image: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            corner_sampled_image: Default::default(),
        }
    }
}
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_corner_sampled_image"
);
