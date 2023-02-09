#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html)
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub provoking_vertex_last: crate::vk10::Bool32,
    pub transform_feedback_preserves_provoking_vertex: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceProvokingVertexFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            provoking_vertex_last: Default::default(),
            transform_feedback_preserves_provoking_vertex: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html)
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub provoking_vertex_mode_per_pipeline: crate::vk10::Bool32,
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceProvokingVertexPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            provoking_vertex_mode_per_pipeline: Default::default(),
            transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html)
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}
impl Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            provoking_vertex_mode: Default::default(),
        }
    }
}
#[doc(alias = "VkProvokingVertexModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ProvokingVertexModeEXT(pub i32);
impl ProvokingVertexModeEXT {
    pub const FIRST_VERTEX: Self = Self(0);
    pub const LAST_VERTEX: Self = Self(1);
}
crate::enum_impl! {
    ProvokingVertexModeEXT : i32, FIRST_VERTEX, LAST_VERTEX
}
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_provoking_vertex"
);
