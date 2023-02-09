#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT.html)
pub struct PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub non_seamless_cube_map: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            non_seamless_cube_map: Default::default(),
        }
    }
}
pub const EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION: u32 = 1;
pub const EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_non_seamless_cube_map"
);
