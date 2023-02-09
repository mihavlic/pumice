#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html)
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub rasterization_order_color_attachment_access: crate::vk10::Bool32,
    pub rasterization_order_depth_attachment_access: crate::vk10::Bool32,
    pub rasterization_order_stencil_attachment_access: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            rasterization_order_color_attachment_access: Default::default(),
            rasterization_order_depth_attachment_access: Default::default(),
            rasterization_order_stencil_attachment_access: Default::default(),
        }
    }
}
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_rasterization_order_attachment_access"
);
