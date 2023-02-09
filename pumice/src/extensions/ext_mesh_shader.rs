#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html)
pub struct PhysicalDeviceMeshShaderFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub task_shader: crate::vk10::Bool32,
    pub mesh_shader: crate::vk10::Bool32,
    pub multiview_mesh_shader: crate::vk10::Bool32,
    pub primitive_fragment_shading_rate_mesh_shader: crate::vk10::Bool32,
    pub mesh_shader_queries: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMeshShaderFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            task_shader: Default::default(),
            mesh_shader: Default::default(),
            multiview_mesh_shader: Default::default(),
            primitive_fragment_shading_rate_mesh_shader: Default::default(),
            mesh_shader_queries: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html)
pub struct PhysicalDeviceMeshShaderPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_task_work_group_total_count: u32,
    pub max_task_work_group_count: [u32; 3],
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_payload_size: u32,
    pub max_task_shared_memory_size: u32,
    pub max_task_payload_and_shared_memory_size: u32,
    pub max_mesh_work_group_total_count: u32,
    pub max_mesh_work_group_count: [u32; 3],
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_shared_memory_size: u32,
    pub max_mesh_payload_and_shared_memory_size: u32,
    pub max_mesh_output_memory_size: u32,
    pub max_mesh_payload_and_output_memory_size: u32,
    pub max_mesh_output_components: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_output_layers: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub max_preferred_task_work_group_invocations: u32,
    pub max_preferred_mesh_work_group_invocations: u32,
    pub prefers_local_invocation_vertex_output: crate::vk10::Bool32,
    pub prefers_local_invocation_primitive_output: crate::vk10::Bool32,
    pub prefers_compact_vertex_output: crate::vk10::Bool32,
    pub prefers_compact_primitive_output: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMeshShaderPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_task_work_group_total_count: Default::default(),
            max_task_work_group_count: unsafe { std::mem::zeroed() },
            max_task_work_group_invocations: Default::default(),
            max_task_work_group_size: unsafe { std::mem::zeroed() },
            max_task_payload_size: Default::default(),
            max_task_shared_memory_size: Default::default(),
            max_task_payload_and_shared_memory_size: Default::default(),
            max_mesh_work_group_total_count: Default::default(),
            max_mesh_work_group_count: unsafe { std::mem::zeroed() },
            max_mesh_work_group_invocations: Default::default(),
            max_mesh_work_group_size: unsafe { std::mem::zeroed() },
            max_mesh_shared_memory_size: Default::default(),
            max_mesh_payload_and_shared_memory_size: Default::default(),
            max_mesh_output_memory_size: Default::default(),
            max_mesh_payload_and_output_memory_size: Default::default(),
            max_mesh_output_components: Default::default(),
            max_mesh_output_vertices: Default::default(),
            max_mesh_output_primitives: Default::default(),
            max_mesh_output_layers: Default::default(),
            max_mesh_multiview_view_count: Default::default(),
            mesh_output_per_vertex_granularity: Default::default(),
            mesh_output_per_primitive_granularity: Default::default(),
            max_preferred_task_work_group_invocations: Default::default(),
            max_preferred_mesh_work_group_invocations: Default::default(),
            prefers_local_invocation_vertex_output: Default::default(),
            prefers_local_invocation_primitive_output: Default::default(),
            prefers_compact_vertex_output: Default::default(),
            prefers_compact_primitive_output: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrawMeshTasksIndirectCommandEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandEXT.html)
pub struct DrawMeshTasksIndirectCommandEXT {
    pub group_count_x: u32,
    pub group_count_y: u32,
    pub group_count_z: u32,
}
impl Default for DrawMeshTasksIndirectCommandEXT {
    fn default() -> Self {
        Self {
            group_count_x: Default::default(),
            group_count_y: Default::default(),
            group_count_z: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMeshTasksEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)
pub unsafe fn cmd_draw_mesh_tasks_ext(
    command_buffer: crate::vk10::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_ext
        .unwrap())(command_buffer, group_count_x, group_count_y, group_count_z)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let cmd_draw_mesh_tasks_ext = (*self.table).cmd_draw_mesh_tasks_ext.unwrap();
        cmd_draw_mesh_tasks_ext(
            command_buffer,
            group_count_x as _,
            group_count_y as _,
            group_count_z as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html)
pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_indirect_ext
        .unwrap())(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_mesh_tasks_indirect_ext = (*self.table)
            .cmd_draw_mesh_tasks_indirect_ext
            .unwrap();
        cmd_draw_mesh_tasks_indirect_ext(
            command_buffer,
            buffer,
            offset as _,
            draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html)
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_indirect_count_ext
        .unwrap())(
        command_buffer,
        buffer,
        offset,
        count_buffer,
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_mesh_tasks_indirect_count_ext = (*self.table)
            .cmd_draw_mesh_tasks_indirect_count_ext
            .unwrap();
        cmd_draw_mesh_tasks_indirect_count_ext(
            command_buffer,
            buffer,
            offset as _,
            count_buffer,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
    }
}
pub const EXT_MESH_SHADER_SPEC_VERSION: u32 = 1;
pub const EXT_MESH_SHADER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_mesh_shader"
);
