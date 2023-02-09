#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html)
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format_rgba_10x_6_without_ycb_cr_sampler: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            format_rgba_10x_6_without_ycb_cr_sampler: Default::default(),
        }
    }
}
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
pub const EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_rgba10x6_formats"
);
