#[doc(alias = "VkPipelineRasterizationStateStreamCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationStateStreamCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    PipelineRasterizationStateStreamCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html)
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub transform_feedback: crate::vk10::Bool32,
    pub geometry_streams: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            transform_feedback: Default::default(),
            geometry_streams: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html)
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: crate::vk10::DeviceSize,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: crate::vk10::Bool32,
    pub transform_feedback_streams_lines_triangles: crate::vk10::Bool32,
    pub transform_feedback_rasterization_stream_select: crate::vk10::Bool32,
    pub transform_feedback_draw: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_transform_feedback_streams: Default::default(),
            max_transform_feedback_buffers: Default::default(),
            max_transform_feedback_buffer_size: Default::default(),
            max_transform_feedback_stream_data_size: Default::default(),
            max_transform_feedback_buffer_data_size: Default::default(),
            max_transform_feedback_buffer_data_stride: Default::default(),
            transform_feedback_queries: Default::default(),
            transform_feedback_streams_lines_triangles: Default::default(),
            transform_feedback_rasterization_stream_select: Default::default(),
            transform_feedback_draw: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html)
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterization_stream: u32,
}
impl Default for PipelineRasterizationStateStreamCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rasterization_stream: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk10::Buffer,
    p_offsets: *const crate::vk10::DeviceSize,
    p_sizes: *const crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_transform_feedback_buffers_ext
        .unwrap())(
        command_buffer,
        first_binding,
        binding_count,
        p_buffers,
        p_offsets,
        p_sizes,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk10::Buffer],
        offsets: &[crate::vk10::DeviceSize],
        sizes: &[crate::vk10::DeviceSize],
    ) {
        let cmd_bind_transform_feedback_buffers_ext = (*self.table)
            .cmd_bind_transform_feedback_buffers_ext
            .unwrap();
        let binding_count = buffers.len().min(offsets.len()).min(sizes.len());
        cmd_bind_transform_feedback_buffers_ext(
            command_buffer,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr(),
            offsets.as_ptr(),
            sizes.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
pub unsafe fn cmd_begin_transform_feedback_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const crate::vk10::Buffer,
    p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_transform_feedback_ext
        .unwrap())(
        command_buffer,
        first_counter_buffer,
        counter_buffer_count,
        p_counter_buffers,
        p_counter_buffer_offsets,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[crate::vk10::Buffer],
        counter_buffer_offsets: &[crate::vk10::DeviceSize],
    ) {
        let cmd_begin_transform_feedback_ext = (*self.table)
            .cmd_begin_transform_feedback_ext
            .unwrap();
        let counter_buffer_count = counter_buffers
            .len()
            .min(counter_buffer_offsets.len());
        cmd_begin_transform_feedback_ext(
            command_buffer,
            first_counter_buffer as _,
            counter_buffer_count as _,
            counter_buffers.as_ptr(),
            counter_buffer_offsets.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
pub unsafe fn cmd_end_transform_feedback_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const crate::vk10::Buffer,
    p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_transform_feedback_ext
        .unwrap())(
        command_buffer,
        first_counter_buffer,
        counter_buffer_count,
        p_counter_buffers,
        p_counter_buffer_offsets,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[crate::vk10::Buffer],
        counter_buffer_offsets: &[crate::vk10::DeviceSize],
    ) {
        let cmd_end_transform_feedback_ext = (*self.table)
            .cmd_end_transform_feedback_ext
            .unwrap();
        let counter_buffer_count = counter_buffers
            .len()
            .min(counter_buffer_offsets.len());
        cmd_end_transform_feedback_ext(
            command_buffer,
            first_counter_buffer as _,
            counter_buffer_count as _,
            counter_buffers.as_ptr(),
            counter_buffer_offsets.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
pub unsafe fn cmd_begin_query_indexed_ext(
    command_buffer: crate::vk10::CommandBuffer,
    query_pool: crate::vk10::QueryPool,
    query: u32,
    flags: crate::vk10::QueryControlFlags,
    index: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_query_indexed_ext
        .unwrap())(command_buffer, query_pool, query, flags, index)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        flags: Option<crate::vk10::QueryControlFlags>,
        index: u32,
    ) {
        let cmd_begin_query_indexed_ext = (*self.table)
            .cmd_begin_query_indexed_ext
            .unwrap();
        cmd_begin_query_indexed_ext(
            command_buffer,
            query_pool,
            query as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            index as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
pub unsafe fn cmd_end_query_indexed_ext(
    command_buffer: crate::vk10::CommandBuffer,
    query_pool: crate::vk10::QueryPool,
    query: u32,
    index: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_query_indexed_ext
        .unwrap())(command_buffer, query_pool, query, index)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        index: u32,
    ) {
        let cmd_end_query_indexed_ext = (*self.table).cmd_end_query_indexed_ext.unwrap();
        cmd_end_query_indexed_ext(command_buffer, query_pool, query as _, index as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
pub unsafe fn cmd_draw_indirect_byte_count_ext(
    command_buffer: crate::vk10::CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: crate::vk10::Buffer,
    counter_buffer_offset: crate::vk10::DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indirect_byte_count_ext
        .unwrap())(
        command_buffer,
        instance_count,
        first_instance,
        counter_buffer,
        counter_buffer_offset,
        counter_offset,
        vertex_stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: crate::vk10::Buffer,
        counter_buffer_offset: crate::vk10::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        let cmd_draw_indirect_byte_count_ext = (*self.table)
            .cmd_draw_indirect_byte_count_ext
            .unwrap();
        cmd_draw_indirect_byte_count_ext(
            command_buffer,
            instance_count as _,
            first_instance as _,
            counter_buffer,
            counter_buffer_offset as _,
            counter_offset as _,
            vertex_stride as _,
        );
    }
}
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
pub const EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_transform_feedback"
);
