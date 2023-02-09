#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkTextureLODGatherFormatPropertiesAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html)
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub supports_texture_gather_lodbias_amd: crate::vk10::Bool32,
}
impl Default for TextureLODGatherFormatPropertiesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            supports_texture_gather_lodbias_amd: Default::default(),
        }
    }
}
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
pub const AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_texture_gather_bias_lod"
);
