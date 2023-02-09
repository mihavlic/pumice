#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html)
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_buffer_float_32_atomics: crate::vk10::Bool32,
    pub shader_buffer_float_32_atomic_add: crate::vk10::Bool32,
    pub shader_buffer_float_64_atomics: crate::vk10::Bool32,
    pub shader_buffer_float_64_atomic_add: crate::vk10::Bool32,
    pub shader_shared_float_32_atomics: crate::vk10::Bool32,
    pub shader_shared_float_32_atomic_add: crate::vk10::Bool32,
    pub shader_shared_float_64_atomics: crate::vk10::Bool32,
    pub shader_shared_float_64_atomic_add: crate::vk10::Bool32,
    pub shader_image_float_32_atomics: crate::vk10::Bool32,
    pub shader_image_float_32_atomic_add: crate::vk10::Bool32,
    pub sparse_image_float_32_atomics: crate::vk10::Bool32,
    pub sparse_image_float_32_atomic_add: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_buffer_float_32_atomics: Default::default(),
            shader_buffer_float_32_atomic_add: Default::default(),
            shader_buffer_float_64_atomics: Default::default(),
            shader_buffer_float_64_atomic_add: Default::default(),
            shader_shared_float_32_atomics: Default::default(),
            shader_shared_float_32_atomic_add: Default::default(),
            shader_shared_float_64_atomics: Default::default(),
            shader_shared_float_64_atomic_add: Default::default(),
            shader_image_float_32_atomics: Default::default(),
            shader_image_float_32_atomic_add: Default::default(),
            sparse_image_float_32_atomics: Default::default(),
            sparse_image_float_32_atomic_add: Default::default(),
        }
    }
}
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
pub const EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_shader_atomic_float"
);
