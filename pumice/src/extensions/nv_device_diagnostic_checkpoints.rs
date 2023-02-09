#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html)
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub checkpoint_execution_stage_mask: crate::vk10::PipelineStageFlags,
}
impl Default for QueueFamilyCheckpointPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCheckpointDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointDataNV.html)
pub struct CheckpointDataNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub stage: crate::vk10::PipelineStageFlags,
    pub p_checkpoint_marker: *mut std::os::raw::c_void,
}
impl Default for CheckpointDataNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CHECKPOINT_DATA_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCheckpointNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html)
pub unsafe fn cmd_set_checkpoint_nv(
    command_buffer: crate::vk10::CommandBuffer,
    p_checkpoint_marker: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_checkpoint_nv
        .unwrap())(command_buffer, p_checkpoint_marker)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCheckpointNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html)
    pub unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        checkpoint_marker: *const std::os::raw::c_void,
    ) {
        let cmd_set_checkpoint_nv = (*self.table).cmd_set_checkpoint_nv.unwrap();
        cmd_set_checkpoint_nv(command_buffer, checkpoint_marker);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetQueueCheckpointDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html)
pub unsafe fn get_queue_checkpoint_data_nv(
    queue: crate::vk10::Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_queue_checkpoint_data_nv
        .unwrap())(queue, p_checkpoint_data_count, p_checkpoint_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html)
    pub unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: crate::vk10::Queue,
        checkpoint_data_count: Option<u32>,
        mut checkpoint_data_callback: impl FnMut(&mut Vec<CheckpointDataNV>),
    ) -> Vec<CheckpointDataNV> {
        let get_queue_checkpoint_data_nv = (*self.table)
            .get_queue_checkpoint_data_nv
            .unwrap();
        let mut checkpoint_data_count = match checkpoint_data_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_queue_checkpoint_data_nv(queue, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut checkpoint_data = vec![
            Default::default(); checkpoint_data_count as usize
        ];
        checkpoint_data_callback(&mut checkpoint_data);
        get_queue_checkpoint_data_nv(
            queue,
            &mut checkpoint_data_count,
            checkpoint_data.as_mut_ptr(),
        );
        checkpoint_data
    }
}
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_device_diagnostic_checkpoints"
);
