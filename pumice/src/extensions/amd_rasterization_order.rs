#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationStateRasterizationOrderAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html)
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub rasterization_order: RasterizationOrderAMD,
}
impl Default for PipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            p_next: std::ptr::null(),
            rasterization_order: Default::default(),
        }
    }
}
#[doc(alias = "VkRasterizationOrderAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RasterizationOrderAMD(pub i32);
impl RasterizationOrderAMD {
    pub const STRICT: Self = Self(0);
    pub const RELAXED: Self = Self(1);
}
crate::enum_impl! {
    RasterizationOrderAMD : i32, STRICT, RELAXED
}
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
pub const AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_rasterization_order"
);
