#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub texel_buffer_alignment: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: Default::default(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html)
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT = crate::vk13::PhysicalDeviceTexelBufferAlignmentProperties;
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
pub const EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_texel_buffer_alignment"
);
