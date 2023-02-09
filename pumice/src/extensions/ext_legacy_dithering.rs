#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceLegacyDitheringFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLegacyDitheringFeaturesEXT.html)
pub struct PhysicalDeviceLegacyDitheringFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub legacy_dithering: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceLegacyDitheringFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            legacy_dithering: Default::default(),
        }
    }
}
pub const EXT_LEGACY_DITHERING_SPEC_VERSION: u32 = 1;
pub const EXT_LEGACY_DITHERING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_legacy_dithering"
);
