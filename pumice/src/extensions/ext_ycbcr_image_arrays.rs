#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html)
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ycbcr_image_arrays: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            ycbcr_image_arrays: Default::default(),
        }
    }
}
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_ycbcr_image_arrays"
);
