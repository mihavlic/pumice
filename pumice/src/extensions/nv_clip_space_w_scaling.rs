#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkViewportWScalingNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html)
pub struct ViewportWScalingNV {
    pub xcoeff: std::os::raw::c_float,
    pub ycoeff: std::os::raw::c_float,
}
impl Default for ViewportWScalingNV {
    fn default() -> Self {
        Self {
            xcoeff: Default::default(),
            ycoeff: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html)
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub viewport_wscaling_enable: crate::vk10::Bool32,
    pub viewport_count: u32,
    pub p_viewport_wscalings: *const ViewportWScalingNV,
}
impl Default for PipelineViewportWScalingStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            viewport_wscaling_enable: Default::default(),
            viewport_count: Default::default(),
            p_viewport_wscalings: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportWScalingNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html)
pub unsafe fn cmd_set_viewport_wscaling_nv(
    command_buffer: crate::vk10::CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_wscalings: *const ViewportWScalingNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_wscaling_nv
        .unwrap())(command_buffer, first_viewport, viewport_count, p_viewport_wscalings)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html)
    pub unsafe fn cmd_set_viewport_wscaling_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_wscalings: &[ViewportWScalingNV],
    ) {
        let cmd_set_viewport_wscaling_nv = (*self.table)
            .cmd_set_viewport_wscaling_nv
            .unwrap();
        let viewport_count = viewport_wscalings.len();
        cmd_set_viewport_wscaling_nv(
            command_buffer,
            first_viewport as _,
            viewport_count as _,
            viewport_wscalings.as_ptr(),
        );
    }
}
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_clip_space_w_scaling"
);
