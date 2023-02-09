#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImage2DViewOf3DFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImage2DViewOf3DFeaturesEXT.html)
pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_2_dview_of_3_d: crate::vk10::Bool32,
    pub sampler_2_dview_of_3_d: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImage2DViewOf3DFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            image_2_dview_of_3_d: Default::default(),
            sampler_2_dview_of_3_d: Default::default(),
        }
    }
}
pub const EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION: u32 = 1;
pub const EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_image_2d_view_of_3d"
);
