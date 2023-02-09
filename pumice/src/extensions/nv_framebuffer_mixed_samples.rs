#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCoverageModulationStateCreateFlagsNV(pub u32);
crate::bitflags_impl! {
    PipelineCoverageModulationStateCreateFlagsNV : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html)
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: crate::vk10::Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const std::os::raw::c_float,
}
impl Default for PipelineCoverageModulationStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_modulation_mode: Default::default(),
            coverage_modulation_table_enable: Default::default(),
            coverage_modulation_table_count: Default::default(),
            p_coverage_modulation_table: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkCoverageModulationModeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoverageModulationModeNV(pub i32);
impl CoverageModulationModeNV {
    pub const NONE: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const ALPHA: Self = Self(2);
    pub const RGBA: Self = Self(3);
}
crate::enum_impl! {
    CoverageModulationModeNV : i32, NONE, RGB, ALPHA, RGBA
}
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_framebuffer_mixed_samples"
);
