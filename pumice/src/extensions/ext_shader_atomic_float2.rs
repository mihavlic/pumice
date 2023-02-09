#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html)
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_buffer_float_16_atomics: crate::vk10::Bool32,
    pub shader_buffer_float_16_atomic_add: crate::vk10::Bool32,
    pub shader_buffer_float_16_atomic_min_max: crate::vk10::Bool32,
    pub shader_buffer_float_32_atomic_min_max: crate::vk10::Bool32,
    pub shader_buffer_float_64_atomic_min_max: crate::vk10::Bool32,
    pub shader_shared_float_16_atomics: crate::vk10::Bool32,
    pub shader_shared_float_16_atomic_add: crate::vk10::Bool32,
    pub shader_shared_float_16_atomic_min_max: crate::vk10::Bool32,
    pub shader_shared_float_32_atomic_min_max: crate::vk10::Bool32,
    pub shader_shared_float_64_atomic_min_max: crate::vk10::Bool32,
    pub shader_image_float_32_atomic_min_max: crate::vk10::Bool32,
    pub sparse_image_float_32_atomic_min_max: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_buffer_float_16_atomics: Default::default(),
            shader_buffer_float_16_atomic_add: Default::default(),
            shader_buffer_float_16_atomic_min_max: Default::default(),
            shader_buffer_float_32_atomic_min_max: Default::default(),
            shader_buffer_float_64_atomic_min_max: Default::default(),
            shader_shared_float_16_atomics: Default::default(),
            shader_shared_float_16_atomic_add: Default::default(),
            shader_shared_float_16_atomic_min_max: Default::default(),
            shader_shared_float_32_atomic_min_max: Default::default(),
            shader_shared_float_64_atomic_min_max: Default::default(),
            shader_image_float_32_atomic_min_max: Default::default(),
            sparse_image_float_32_atomic_min_max: Default::default(),
        }
    }
}
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
pub const EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_shader_atomic_float2"
);
