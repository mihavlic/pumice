#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewASTCDecodeModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html)
pub struct ImageViewASTCDecodeModeEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub decode_mode: crate::vk10::Format,
}
impl Default for ImageViewASTCDecodeModeEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_ASTC_DECODE_MODE_EXT,
            p_next: std::ptr::null(),
            decode_mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceASTCDecodeFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html)
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub decode_mode_shared_exponent: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceASTCDecodeFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            decode_mode_shared_exponent: Default::default(),
        }
    }
}
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
pub const EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_astc_decode_mode"
);
