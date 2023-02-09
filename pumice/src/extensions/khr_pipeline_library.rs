#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html)
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub library_count: u32,
    pub p_libraries: *const crate::vk10::Pipeline,
}
impl Default for PipelineLibraryCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            library_count: Default::default(),
            p_libraries: std::ptr::null(),
        }
    }
}
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_pipeline_library"
);
