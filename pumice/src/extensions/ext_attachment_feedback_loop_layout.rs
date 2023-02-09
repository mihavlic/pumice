#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT.html)
pub struct PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub attachment_feedback_loop_layout: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            attachment_feedback_loop_layout: Default::default(),
        }
    }
}
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION: u32 = 2;
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_attachment_feedback_loop_layout"
);
