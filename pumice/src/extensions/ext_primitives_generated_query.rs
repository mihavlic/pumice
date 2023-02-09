#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT.html)
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub primitives_generated_query: crate::vk10::Bool32,
    pub primitives_generated_query_with_rasterizer_discard: crate::vk10::Bool32,
    pub primitives_generated_query_with_non_zero_streams: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            primitives_generated_query: Default::default(),
            primitives_generated_query_with_rasterizer_discard: Default::default(),
            primitives_generated_query_with_non_zero_streams: Default::default(),
        }
    }
}
pub const EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION: u32 = 1;
pub const EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_primitives_generated_query"
);
