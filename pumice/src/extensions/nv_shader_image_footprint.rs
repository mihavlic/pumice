#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html)
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_footprint: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            image_footprint: Default::default(),
        }
    }
}
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
pub const NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_shader_image_footprint"
);
