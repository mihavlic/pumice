#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html)
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_shader_sample_interlock: crate::vk10::Bool32,
    pub fragment_shader_pixel_interlock: crate::vk10::Bool32,
    pub fragment_shader_shading_rate_interlock: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_shader_sample_interlock: Default::default(),
            fragment_shader_pixel_interlock: Default::default(),
            fragment_shader_shading_rate_interlock: Default::default(),
        }
    }
}
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
pub const EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_fragment_shader_interlock"
);
