#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html)
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
impl Default for PipelineCompilerControlCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
#[doc(alias = "VkPipelineCompilerControlFlagsAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagsAMD.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagsAMD(pub u32);
crate::bitflags_impl! {
    PipelineCompilerControlFlagsAMD : u32, 0x0,
}
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_pipeline_compiler_control"
);
