#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html)
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format_a4_r4_g4_b4: crate::vk10::Bool32,
    pub format_a4_b4_g4_r4: crate::vk10::Bool32,
}
impl Default for PhysicalDevice4444FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            format_a4_r4_g4_b4: Default::default(),
            format_a4_b4_g4_r4: Default::default(),
        }
    }
}
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
pub const EXT_4444_FORMATS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_4444_formats"
);
