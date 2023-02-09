#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMeshShaderFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html)
pub struct PhysicalDeviceMeshShaderFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub task_shader: crate::vk10::Bool32,
    pub mesh_shader: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMeshShaderFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            task_shader: Default::default(),
            mesh_shader: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMeshShaderPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html)
pub struct PhysicalDeviceMeshShaderPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
}
impl Default for PhysicalDeviceMeshShaderPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_draw_mesh_tasks_count: Default::default(),
            max_task_work_group_invocations: Default::default(),
            max_task_work_group_size: unsafe { std::mem::zeroed() },
            max_task_total_memory_size: Default::default(),
            max_task_output_count: Default::default(),
            max_mesh_work_group_invocations: Default::default(),
            max_mesh_work_group_size: unsafe { std::mem::zeroed() },
            max_mesh_total_memory_size: Default::default(),
            max_mesh_output_vertices: Default::default(),
            max_mesh_output_primitives: Default::default(),
            max_mesh_multiview_view_count: Default::default(),
            mesh_output_per_vertex_granularity: Default::default(),
            mesh_output_per_primitive_granularity: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrawMeshTasksIndirectCommandNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html)
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}
impl Default for DrawMeshTasksIndirectCommandNV {
    fn default() -> Self {
        Self {
            task_count: Default::default(),
            first_task: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMeshTasksNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html)
pub unsafe fn cmd_draw_mesh_tasks_nv(
    command_buffer: crate::vk10::CommandBuffer,
    task_count: u32,
    first_task: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_nv
        .unwrap())(command_buffer, task_count, first_task)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        let cmd_draw_mesh_tasks_nv = (*self.table).cmd_draw_mesh_tasks_nv.unwrap();
        cmd_draw_mesh_tasks_nv(command_buffer, task_count as _, first_task as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_indirect_nv
        .unwrap())(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_mesh_tasks_indirect_nv = (*self.table)
            .cmd_draw_mesh_tasks_indirect_nv
            .unwrap();
        cmd_draw_mesh_tasks_indirect_nv(
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
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_mesh_tasks_indirect_count_nv
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
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_mesh_tasks_indirect_count_nv = (*self.table)
            .cmd_draw_mesh_tasks_indirect_count_nv
            .unwrap();
        cmd_draw_mesh_tasks_indirect_count_nv(
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
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
pub const NV_MESH_SHADER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_mesh_shader"
);
