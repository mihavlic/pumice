#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html)
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ycbcr_2plane_444_formats: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            ycbcr_2plane_444_formats: Default::default(),
        }
    }
}
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
pub const EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_ycbcr_2plane_444_formats"
);
