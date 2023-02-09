#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html)
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_image_int_64_atomics: crate::vk10::Bool32,
    pub sparse_image_int_64_atomics: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_image_int_64_atomics: Default::default(),
            sparse_image_int_64_atomics: Default::default(),
        }
    }
}
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_shader_image_atomic_int64"
);
