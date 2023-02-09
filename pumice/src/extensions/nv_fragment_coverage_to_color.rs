#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCoverageToColorStateCreateFlagsNV(pub u32);
crate::bitflags_impl! {
    PipelineCoverageToColorStateCreateFlagsNV : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html)
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: crate::vk10::Bool32,
    pub coverage_to_color_location: u32,
}
impl Default for PipelineCoverageToColorStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_to_color_enable: Default::default(),
            coverage_to_color_location: Default::default(),
        }
    }
}
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_fragment_coverage_to_color"
);
