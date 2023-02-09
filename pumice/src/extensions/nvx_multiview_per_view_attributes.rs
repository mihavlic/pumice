#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub per_view_position_all_components: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            per_view_position_all_components: Default::default(),
        }
    }
}
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NVX_multiview_per_view_attributes"
);
