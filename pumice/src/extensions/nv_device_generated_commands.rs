crate::dispatchable_handle!(
    IndirectCommandsLayoutNV, INDIRECT_COMMANDS_LAYOUT_NV, "VkIndirectCommandsLayoutNV",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_generated_commands: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            device_generated_commands: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_graphics_shader_group_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_stream_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_stream_stride: u32,
    pub min_sequences_count_buffer_offset_alignment: u32,
    pub min_sequences_index_buffer_offset_alignment: u32,
    pub min_indirect_commands_buffer_offset_alignment: u32,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_graphics_shader_group_count: Default::default(),
            max_indirect_sequence_count: Default::default(),
            max_indirect_commands_token_count: Default::default(),
            max_indirect_commands_stream_count: Default::default(),
            max_indirect_commands_token_offset: Default::default(),
            max_indirect_commands_stream_stride: Default::default(),
            min_sequences_count_buffer_offset_alignment: Default::default(),
            min_sequences_index_buffer_offset_alignment: Default::default(),
            min_indirect_commands_buffer_offset_alignment: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html)
pub struct GraphicsShaderGroupCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub stage_count: u32,
    pub p_stages: *const crate::vk10::PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const crate::vk10::PipelineVertexInputStateCreateInfo,
    pub p_tessellation_state: *const crate::vk10::PipelineTessellationStateCreateInfo,
}
impl Default for GraphicsShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            p_vertex_input_state: std::ptr::null(),
            p_tessellation_state: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html)
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub group_count: u32,
    pub p_groups: *const GraphicsShaderGroupCreateInfoNV,
    pub pipeline_count: u32,
    pub p_pipelines: *const crate::vk10::Pipeline,
}
impl Default for GraphicsPipelineShaderGroupsCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            pipeline_count: Default::default(),
            p_pipelines: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkBindShaderGroupIndirectCommandNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html)
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}
impl Default for BindShaderGroupIndirectCommandNV {
    fn default() -> Self {
        Self {
            group_index: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkBindIndexBufferIndirectCommandNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html)
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: crate::vk10::DeviceAddress,
    pub size: u32,
    pub index_type: crate::vk10::IndexType,
}
impl Default for BindIndexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: Default::default(),
            index_type: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkBindVertexBufferIndirectCommandNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html)
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: crate::vk10::DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
impl Default for BindVertexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: Default::default(),
            stride: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkSetStateFlagsIndirectCommandNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html)
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}
impl Default for SetStateFlagsIndirectCommandNV {
    fn default() -> Self {
        Self { data: Default::default() }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkIndirectCommandsStreamNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html)
pub struct IndirectCommandsStreamNV {
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
}
impl Default for IndirectCommandsStreamNV {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html)
pub struct IndirectCommandsLayoutTokenNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub token_type: IndirectCommandsTokenTypeNV,
    pub stream: u32,
    pub offset: u32,
    pub vertex_binding_unit: u32,
    pub vertex_dynamic_stride: crate::vk10::Bool32,
    pub pushconstant_pipeline_layout: crate::vk10::PipelineLayout,
    pub pushconstant_shader_stage_flags: crate::vk10::ShaderStageFlags,
    pub pushconstant_offset: u32,
    pub pushconstant_size: u32,
    pub indirect_state_flags: IndirectStateFlagsNV,
    pub index_type_count: u32,
    pub p_index_types: *const crate::vk10::IndexType,
    pub p_index_type_values: *const u32,
}
impl Default for IndirectCommandsLayoutTokenNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
            p_next: std::ptr::null(),
            token_type: Default::default(),
            stream: Default::default(),
            offset: Default::default(),
            vertex_binding_unit: Default::default(),
            vertex_dynamic_stride: Default::default(),
            pushconstant_pipeline_layout: Default::default(),
            pushconstant_shader_stage_flags: Default::default(),
            pushconstant_offset: Default::default(),
            pushconstant_size: Default::default(),
            indirect_state_flags: Default::default(),
            index_type_count: Default::default(),
            p_index_types: std::ptr::null(),
            p_index_type_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html)
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenNV,
    pub stream_count: u32,
    pub p_stream_strides: *const u32,
}
impl Default for IndirectCommandsLayoutCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            token_count: Default::default(),
            p_tokens: std::ptr::null(),
            stream_count: Default::default(),
            p_stream_strides: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeneratedCommandsInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html)
pub struct GeneratedCommandsInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub pipeline: crate::vk10::Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub stream_count: u32,
    pub p_streams: *const IndirectCommandsStreamNV,
    pub sequences_count: u32,
    pub preprocess_buffer: crate::vk10::Buffer,
    pub preprocess_offset: crate::vk10::DeviceSize,
    pub preprocess_size: crate::vk10::DeviceSize,
    pub sequences_count_buffer: crate::vk10::Buffer,
    pub sequences_count_offset: crate::vk10::DeviceSize,
    pub sequences_index_buffer: crate::vk10::Buffer,
    pub sequences_index_offset: crate::vk10::DeviceSize,
}
impl Default for GeneratedCommandsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GENERATED_COMMANDS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            stream_count: Default::default(),
            p_streams: std::ptr::null(),
            sequences_count: Default::default(),
            preprocess_buffer: Default::default(),
            preprocess_offset: Default::default(),
            preprocess_size: Default::default(),
            sequences_count_buffer: Default::default(),
            sequences_count_offset: Default::default(),
            sequences_index_buffer: Default::default(),
            sequences_index_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html)
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub pipeline: crate::vk10::Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub max_sequences_count: u32,
}
impl Default for GeneratedCommandsMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            max_sequences_count: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub u32);
impl IndirectCommandsLayoutUsageFlagsNV {
    pub const EXPLICIT_PREPROCESS: Self = Self(1 << 0);
    pub const INDEXED_SEQUENCES: Self = Self(1 << 1);
    pub const UNORDERED_SEQUENCES: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    IndirectCommandsLayoutUsageFlagsNV : u32, 0x7, EXPLICIT_PREPROCESS,
    INDEXED_SEQUENCES, UNORDERED_SEQUENCES
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IndirectStateFlagsNV(pub u32);
impl IndirectStateFlagsNV {
    pub const FLAG_FRONTFACE: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    IndirectStateFlagsNV : u32, 0x1, FLAG_FRONTFACE
}
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct IndirectCommandsTokenTypeNV(pub i32);
impl IndirectCommandsTokenTypeNV {
    pub const SHADER_GROUP: Self = Self(0);
    pub const STATE_FLAGS: Self = Self(1);
    pub const INDEX_BUFFER: Self = Self(2);
    pub const VERTEX_BUFFER: Self = Self(3);
    pub const PUSH_CONSTANT: Self = Self(4);
    pub const DRAW_INDEXED: Self = Self(5);
    pub const DRAW: Self = Self(6);
    pub const DRAW_TASKS: Self = Self(7);
    /// ext_mesh_shader
    pub const DRAW_MESH_TASKS: Self = Self(1000328000);
}
crate::enum_impl! {
    IndirectCommandsTokenTypeNV : i32, SHADER_GROUP, STATE_FLAGS, INDEX_BUFFER,
    VERTEX_BUFFER, PUSH_CONSTANT, DRAW_INDEXED, DRAW, DRAW_TASKS, DRAW_MESH_TASKS
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
pub unsafe fn cmd_execute_generated_commands_nv(
    command_buffer: crate::vk10::CommandBuffer,
    is_preprocessed: crate::vk10::Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_execute_generated_commands_nv
        .unwrap())(command_buffer, is_preprocessed, p_generated_commands_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        let cmd_execute_generated_commands_nv = (*self.table)
            .cmd_execute_generated_commands_nv
            .unwrap();
        cmd_execute_generated_commands_nv(
            command_buffer,
            is_preprocessed as _,
            generated_commands_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
pub unsafe fn cmd_preprocess_generated_commands_nv(
    command_buffer: crate::vk10::CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_preprocess_generated_commands_nv
        .unwrap())(command_buffer, p_generated_commands_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        let cmd_preprocess_generated_commands_nv = (*self.table)
            .cmd_preprocess_generated_commands_nv
            .unwrap();
        cmd_preprocess_generated_commands_nv(
            command_buffer,
            generated_commands_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
pub unsafe fn cmd_bind_pipeline_shader_group_nv(
    command_buffer: crate::vk10::CommandBuffer,
    pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pipeline: crate::vk10::Pipeline,
    group_index: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_pipeline_shader_group_nv
        .unwrap())(command_buffer, pipeline_bind_point, pipeline, group_index)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        pipeline: crate::vk10::Pipeline,
        group_index: u32,
    ) {
        let cmd_bind_pipeline_shader_group_nv = (*self.table)
            .cmd_bind_pipeline_shader_group_nv
            .unwrap();
        cmd_bind_pipeline_shader_group_nv(
            command_buffer,
            pipeline_bind_point as _,
            pipeline,
            group_index as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
pub unsafe fn get_generated_commands_memory_requirements_nv(
    device: crate::vk10::Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_generated_commands_memory_requirements_nv
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_generated_commands_memory_requirements_nv = (*self.table)
            .get_generated_commands_memory_requirements_nv
            .unwrap();
        let mut memory_requirements = Default::default();
        get_generated_commands_memory_requirements_nv(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
pub unsafe fn create_indirect_commands_layout_nv(
    device: crate::vk10::Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_indirect_commands_layout_nv
        .unwrap())(device, p_create_info, p_allocator, p_indirect_commands_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<IndirectCommandsLayoutNV> {
        let create_indirect_commands_layout_nv = (*self.table)
            .create_indirect_commands_layout_nv
            .unwrap();
        let mut indirect_commands_layout = Default::default();
        let result = create_indirect_commands_layout_nv(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut indirect_commands_layout,
        );
        crate::new_result(indirect_commands_layout, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
pub unsafe fn destroy_indirect_commands_layout_nv(
    device: crate::vk10::Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_indirect_commands_layout_nv
        .unwrap())(device, indirect_commands_layout, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_indirect_commands_layout_nv = (*self.table)
            .destroy_indirect_commands_layout_nv
            .unwrap();
        destroy_indirect_commands_layout_nv(
            self.handle,
            indirect_commands_layout,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_device_generated_commands"
);
