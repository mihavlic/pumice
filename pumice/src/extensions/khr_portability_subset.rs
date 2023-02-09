#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html)
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub constant_alpha_color_blend_factors: crate::vk10::Bool32,
    pub events: crate::vk10::Bool32,
    pub image_view_format_reinterpretation: crate::vk10::Bool32,
    pub image_view_format_swizzle: crate::vk10::Bool32,
    pub image_view_2_don_3_dimage: crate::vk10::Bool32,
    pub multisample_array_image: crate::vk10::Bool32,
    pub mutable_comparison_samplers: crate::vk10::Bool32,
    pub point_polygons: crate::vk10::Bool32,
    pub sampler_mip_lod_bias: crate::vk10::Bool32,
    pub separate_stencil_mask_ref: crate::vk10::Bool32,
    pub shader_sample_rate_interpolation_functions: crate::vk10::Bool32,
    pub tessellation_isolines: crate::vk10::Bool32,
    pub tessellation_point_mode: crate::vk10::Bool32,
    pub triangle_fans: crate::vk10::Bool32,
    pub vertex_attribute_access_beyond_stride: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            constant_alpha_color_blend_factors: Default::default(),
            events: Default::default(),
            image_view_format_reinterpretation: Default::default(),
            image_view_format_swizzle: Default::default(),
            image_view_2_don_3_dimage: Default::default(),
            multisample_array_image: Default::default(),
            mutable_comparison_samplers: Default::default(),
            point_polygons: Default::default(),
            sampler_mip_lod_bias: Default::default(),
            separate_stencil_mask_ref: Default::default(),
            shader_sample_rate_interpolation_functions: Default::default(),
            tessellation_isolines: Default::default(),
            tessellation_point_mode: Default::default(),
            triangle_fans: Default::default(),
            vertex_attribute_access_beyond_stride: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html)
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_vertex_input_binding_stride_alignment: u32,
}
impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            min_vertex_input_binding_stride_alignment: Default::default(),
        }
    }
}
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
pub const KHR_PORTABILITY_SUBSET_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_portability_subset"
);
