#[doc(alias = "VkAccessFlags2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlags2KHR.html)
pub type AccessFlags2KHR = crate::vk13::AccessFlags2;
#[doc(alias = "VkPipelineStageFlags2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlags2KHR.html)
pub type PipelineStageFlags2KHR = crate::vk13::PipelineStageFlags2;
#[doc(alias = "VkSubmitFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagsKHR.html)
pub type SubmitFlagsKHR = crate::vk13::SubmitFlags;
#[doc(alias = "VkMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2KHR.html)
pub type MemoryBarrier2KHR = crate::vk13::MemoryBarrier2;
#[doc(alias = "VkImageMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2KHR.html)
pub type ImageMemoryBarrier2KHR = crate::vk13::ImageMemoryBarrier2;
#[doc(alias = "VkBufferMemoryBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2KHR.html)
pub type BufferMemoryBarrier2KHR = crate::vk13::BufferMemoryBarrier2;
#[doc(alias = "VkDependencyInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfoKHR.html)
pub type DependencyInfoKHR = crate::vk13::DependencyInfo;
#[doc(alias = "VkSemaphoreSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfoKHR.html)
pub type SemaphoreSubmitInfoKHR = crate::vk13::SemaphoreSubmitInfo;
#[doc(alias = "VkCommandBufferSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfoKHR.html)
pub type CommandBufferSubmitInfoKHR = crate::vk13::CommandBufferSubmitInfo;
#[doc(alias = "VkSubmitInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2KHR.html)
pub type SubmitInfo2KHR = crate::vk13::SubmitInfo2;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html)
pub struct QueueFamilyCheckpointProperties2NV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub checkpoint_execution_stage_mask: crate::vk13::PipelineStageFlags2,
}
impl Default for QueueFamilyCheckpointProperties2NV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCheckpointData2NV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html)
pub struct CheckpointData2NV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub stage: crate::vk13::PipelineStageFlags2,
    pub p_checkpoint_marker: *mut std::os::raw::c_void,
}
impl Default for CheckpointData2NV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CHECKPOINT_DATA_2_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSynchronization2FeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2FeaturesKHR.html)
pub type PhysicalDeviceSynchronization2FeaturesKHR = crate::vk13::PhysicalDeviceSynchronization2Features;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetEvent2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
pub unsafe fn cmd_set_event_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    p_dependency_info: *const crate::vk13::DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_event_2
        .unwrap())(command_buffer, event, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
    pub unsafe fn cmd_set_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        dependency_info: &crate::vk13::DependencyInfo,
    ) {
        let cmd_set_event_2_khr = (*self.table).cmd_set_event_2_khr.unwrap();
        cmd_set_event_2_khr(command_buffer, event, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResetEvent2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
pub unsafe fn cmd_reset_event_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    stage_mask: crate::vk13::PipelineStageFlags2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_reset_event_2
        .unwrap())(command_buffer, event, stage_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
    pub unsafe fn cmd_reset_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: Option<crate::vk13::PipelineStageFlags2>,
    ) {
        let cmd_reset_event_2_khr = (*self.table).cmd_reset_event_2_khr.unwrap();
        cmd_reset_event_2_khr(
            command_buffer,
            event,
            match stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWaitEvents2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
pub unsafe fn cmd_wait_events_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    event_count: u32,
    p_events: *const crate::vk10::Event,
    p_dependency_infos: *const crate::vk13::DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_wait_events_2
        .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
    pub unsafe fn cmd_wait_events_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        events: &[crate::vk10::Event],
        dependency_infos: &[crate::vk13::DependencyInfo],
    ) {
        let cmd_wait_events_2_khr = (*self.table).cmd_wait_events_2_khr.unwrap();
        let event_count = events.len().min(dependency_infos.len());
        cmd_wait_events_2_khr(
            command_buffer,
            event_count as _,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
pub unsafe fn cmd_pipeline_barrier_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_dependency_info: *const crate::vk13::DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_pipeline_barrier_2
        .unwrap())(command_buffer, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
    pub unsafe fn cmd_pipeline_barrier_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dependency_info: &crate::vk13::DependencyInfo,
    ) {
        let cmd_pipeline_barrier_2_khr = (*self.table)
            .cmd_pipeline_barrier_2_khr
            .unwrap();
        cmd_pipeline_barrier_2_khr(command_buffer, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueSubmit2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
pub unsafe fn queue_submit_2_khr(
    queue: crate::vk10::Queue,
    submit_count: u32,
    p_submits: *const crate::vk13::SubmitInfo2,
    fence: crate::vk10::Fence,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_submit_2
        .unwrap())(queue, submit_count, p_submits, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueSubmit2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
    pub unsafe fn queue_submit_2_khr(
        &self,
        queue: crate::vk10::Queue,
        submits: &[crate::vk13::SubmitInfo2],
        fence: crate::vk10::Fence,
    ) -> crate::VulkanResult<()> {
        let queue_submit_2_khr = (*self.table).queue_submit_2_khr.unwrap();
        let submit_count = submits.len();
        let result = queue_submit_2_khr(
            queue,
            submit_count as _,
            submits.as_ptr(),
            fence,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
pub unsafe fn cmd_write_timestamp_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    stage: crate::vk13::PipelineStageFlags2,
    query_pool: crate::vk10::QueryPool,
    query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_timestamp_2
        .unwrap())(command_buffer, stage, query_pool, query)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
    pub unsafe fn cmd_write_timestamp_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: Option<crate::vk13::PipelineStageFlags2>,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        let cmd_write_timestamp_2_khr = (*self.table).cmd_write_timestamp_2_khr.unwrap();
        cmd_write_timestamp_2_khr(
            command_buffer,
            match stage {
                Some(v) => v,
                None => Default::default(),
            },
            query_pool,
            query as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html)
pub unsafe fn cmd_write_buffer_marker_2_amd(
    command_buffer: crate::vk10::CommandBuffer,
    stage: crate::vk13::PipelineStageFlags2,
    dst_buffer: crate::vk10::Buffer,
    dst_offset: crate::vk10::DeviceSize,
    marker: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_buffer_marker_2_amd
        .unwrap())(command_buffer, stage, dst_buffer, dst_offset, marker)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html)
    pub unsafe fn cmd_write_buffer_marker_2_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: Option<crate::vk13::PipelineStageFlags2>,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        marker: u32,
    ) {
        let cmd_write_buffer_marker_2_amd = (*self.table)
            .cmd_write_buffer_marker_2_amd
            .unwrap();
        cmd_write_buffer_marker_2_amd(
            command_buffer,
            match stage {
                Some(v) => v,
                None => Default::default(),
            },
            dst_buffer,
            dst_offset as _,
            marker as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetQueueCheckpointData2NV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html)
pub unsafe fn get_queue_checkpoint_data_2_nv(
    queue: crate::vk10::Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_queue_checkpoint_data_2_nv
        .unwrap())(queue, p_checkpoint_data_count, p_checkpoint_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html)
    pub unsafe fn get_queue_checkpoint_data_2_nv(
        &self,
        queue: crate::vk10::Queue,
        checkpoint_data_count: Option<u32>,
        mut checkpoint_data_callback: impl FnMut(&mut Vec<CheckpointData2NV>),
    ) -> Vec<CheckpointData2NV> {
        let get_queue_checkpoint_data_2_nv = (*self.table)
            .get_queue_checkpoint_data_2_nv
            .unwrap();
        let mut checkpoint_data_count = match checkpoint_data_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_queue_checkpoint_data_2_nv(queue, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut checkpoint_data = vec![
            Default::default(); checkpoint_data_count as usize
        ];
        checkpoint_data_callback(&mut checkpoint_data);
        get_queue_checkpoint_data_2_nv(
            queue,
            &mut checkpoint_data_count,
            checkpoint_data.as_mut_ptr(),
        );
        checkpoint_data
    }
}
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_synchronization2"
);
