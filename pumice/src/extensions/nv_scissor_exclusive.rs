#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html)
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub exclusive_scissor: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            exclusive_scissor: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html)
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const crate::vk10::Rect2D,
}
impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            exclusive_scissor_count: Default::default(),
            p_exclusive_scissors: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
pub unsafe fn cmd_set_exclusive_scissor_nv(
    command_buffer: crate::vk10::CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const crate::vk10::Rect2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_exclusive_scissor_nv
        .unwrap())(
        command_buffer,
        first_exclusive_scissor,
        exclusive_scissor_count,
        p_exclusive_scissors,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[crate::vk10::Rect2D],
    ) {
        let cmd_set_exclusive_scissor_nv = (*self.table)
            .cmd_set_exclusive_scissor_nv
            .unwrap();
        let exclusive_scissor_count = exclusive_scissors.len();
        cmd_set_exclusive_scissor_nv(
            command_buffer,
            first_exclusive_scissor as _,
            exclusive_scissor_count as _,
            exclusive_scissors.as_ptr(),
        );
    }
}
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_scissor_exclusive"
);
