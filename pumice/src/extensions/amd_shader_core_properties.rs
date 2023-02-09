#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderCorePropertiesAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html)
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}
impl Default for PhysicalDeviceShaderCorePropertiesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
            p_next: std::ptr::null_mut(),
            shader_engine_count: Default::default(),
            shader_arrays_per_engine_count: Default::default(),
            compute_units_per_shader_array: Default::default(),
            simd_per_compute_unit: Default::default(),
            wavefronts_per_simd: Default::default(),
            wavefront_size: Default::default(),
            sgprs_per_simd: Default::default(),
            min_sgpr_allocation: Default::default(),
            max_sgpr_allocation: Default::default(),
            sgpr_allocation_granularity: Default::default(),
            vgprs_per_simd: Default::default(),
            min_vgpr_allocation: Default::default(),
            max_vgpr_allocation: Default::default(),
            vgpr_allocation_granularity: Default::default(),
        }
    }
}
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_shader_core_properties"
);
