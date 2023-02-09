#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html)
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub linear_color_attachment: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            linear_color_attachment: Default::default(),
        }
    }
}
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
pub const NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_linear_color_attachment"
);
