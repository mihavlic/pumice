#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html)
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_lod: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            min_lod: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html)
pub struct ImageViewMinLodCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub min_lod: std::os::raw::c_float,
}
impl Default for ImageViewMinLodCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            min_lod: Default::default(),
        }
    }
}
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
pub const EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_image_view_min_lod"
);
