#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkShaderResourceUsageAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderResourceUsageAMD.html)
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}
impl Default for ShaderResourceUsageAMD {
    fn default() -> Self {
        Self {
            num_used_vgprs: Default::default(),
            num_used_sgprs: Default::default(),
            lds_size_per_local_work_group: Default::default(),
            lds_usage_size_in_bytes: Default::default(),
            scratch_mem_usage_in_bytes: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkShaderStatisticsInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStatisticsInfoAMD.html)
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: crate::vk10::ShaderStageFlags,
    pub resource_usage: ShaderResourceUsageAMD,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3],
}
impl Default for ShaderStatisticsInfoAMD {
    fn default() -> Self {
        Self {
            shader_stage_mask: Default::default(),
            resource_usage: Default::default(),
            num_physical_vgprs: Default::default(),
            num_physical_sgprs: Default::default(),
            num_available_vgprs: Default::default(),
            num_available_sgprs: Default::default(),
            compute_work_group_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkShaderInfoTypeAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderInfoTypeAMD(pub i32);
impl ShaderInfoTypeAMD {
    pub const STATISTICS: Self = Self(0);
    pub const BINARY: Self = Self(1);
    pub const DISASSEMBLY: Self = Self(2);
}
crate::enum_impl! {
    ShaderInfoTypeAMD : i32, STATISTICS, BINARY, DISASSEMBLY
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetShaderInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html)
pub unsafe fn get_shader_info_amd(
    device: crate::vk10::Device,
    pipeline: crate::vk10::Pipeline,
    shader_stage: crate::vk10::ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut std::os::raw::c_void,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_shader_info_amd
        .unwrap())(device, pipeline, shader_stage, info_type, p_info_size, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetShaderInfoAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html)
    pub unsafe fn get_shader_info_amd(
        &self,
        pipeline: crate::vk10::Pipeline,
        shader_stage: crate::vk10::ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info_size: *mut usize,
        info: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let get_shader_info_amd = (*self.table).get_shader_info_amd.unwrap();
        let result = get_shader_info_amd(
            self.handle,
            pipeline,
            shader_stage as _,
            info_type as _,
            info_size,
            info,
        );
        crate::new_result(result, result)
    }
}
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
pub const AMD_SHADER_INFO_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_shader_info"
);
