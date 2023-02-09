#[doc(alias = "VkPipelineRasterizationConservativeStateCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    PipelineRasterizationConservativeStateCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceConservativeRasterizationPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html)
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub primitive_overestimation_size: std::os::raw::c_float,
    pub max_extra_primitive_overestimation_size: std::os::raw::c_float,
    pub extra_primitive_overestimation_size_granularity: std::os::raw::c_float,
    pub primitive_underestimation: crate::vk10::Bool32,
    pub conservative_point_and_line_rasterization: crate::vk10::Bool32,
    pub degenerate_triangles_rasterized: crate::vk10::Bool32,
    pub degenerate_lines_rasterized: crate::vk10::Bool32,
    pub fully_covered_fragment_shader_input_variable: crate::vk10::Bool32,
    pub conservative_rasterization_post_depth_coverage: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            primitive_overestimation_size: Default::default(),
            max_extra_primitive_overestimation_size: Default::default(),
            extra_primitive_overestimation_size_granularity: Default::default(),
            primitive_underestimation: Default::default(),
            conservative_point_and_line_rasterization: Default::default(),
            degenerate_triangles_rasterized: Default::default(),
            degenerate_lines_rasterized: Default::default(),
            fully_covered_fragment_shader_input_variable: Default::default(),
            conservative_rasterization_post_depth_coverage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html)
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: std::os::raw::c_float,
}
impl Default for PipelineRasterizationConservativeStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            conservative_rasterization_mode: Default::default(),
            extra_primitive_overestimation_size: Default::default(),
        }
    }
}
#[doc(alias = "VkConservativeRasterizationModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ConservativeRasterizationModeEXT(pub i32);
impl ConservativeRasterizationModeEXT {
    pub const DISABLED: Self = Self(0);
    pub const OVERESTIMATE: Self = Self(1);
    pub const UNDERESTIMATE: Self = Self(2);
}
crate::enum_impl! {
    ConservativeRasterizationModeEXT : i32, DISABLED, OVERESTIMATE, UNDERESTIMATE
}
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
pub const EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_conservative_rasterization"
);
