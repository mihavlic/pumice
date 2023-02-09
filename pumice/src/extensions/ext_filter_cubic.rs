#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html)
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_view_type: crate::vk10::ImageViewType,
}
impl Default for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
            p_next: std::ptr::null_mut(),
            image_view_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html)
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub filter_cubic: crate::vk10::Bool32,
    pub filter_cubic_minmax: crate::vk10::Bool32,
}
impl Default for FilterCubicImageViewImageFormatPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            filter_cubic: Default::default(),
            filter_cubic_minmax: Default::default(),
        }
    }
}
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_filter_cubic"
);
