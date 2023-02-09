#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelineProtectedAccessFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineProtectedAccessFeaturesEXT.html)
pub struct PhysicalDevicePipelineProtectedAccessFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_protected_access: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePipelineProtectedAccessFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            pipeline_protected_access: Default::default(),
        }
    }
}
pub const EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: u32 = 1;
pub const EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_pipeline_protected_access"
);
