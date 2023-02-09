#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html)
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_coherent_memory: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            p_next: std::ptr::null_mut(),
            device_coherent_memory: Default::default(),
        }
    }
}
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
pub const AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_device_coherent_memory"
);
