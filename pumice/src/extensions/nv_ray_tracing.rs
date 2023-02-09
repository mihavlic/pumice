#[doc(alias = "VkGeometryFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagsNV.html)
pub type GeometryFlagsNV = crate::extensions::khr_acceleration_structure::GeometryFlagsKHR;
#[doc(alias = "VkGeometryInstanceFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagsNV.html)
pub type GeometryInstanceFlagsNV = crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR;
#[doc(alias = "VkBuildAccelerationStructureFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagsNV.html)
pub type BuildAccelerationStructureFlagsNV = crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR;
crate::dispatchable_handle!(
    AccelerationStructureNV, ACCELERATION_STRUCTURE_NV, "VkAccelerationStructureNV",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureNV.html)"
);
#[doc(alias = "VkCopyAccelerationStructureModeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeNV.html)
pub type CopyAccelerationStructureModeNV = crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[doc(alias = "VkAccelerationStructureTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeNV.html)
pub type AccelerationStructureTypeNV = crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR;
#[doc(alias = "VkGeometryTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeNV.html)
pub type GeometryTypeNV = crate::extensions::khr_acceleration_structure::GeometryTypeKHR;
#[doc(alias = "VkRayTracingShaderGroupTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeNV.html)
pub type RayTracingShaderGroupTypeNV = crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html)
pub struct RayTracingShaderGroupCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
}
impl Default for RayTracingShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            kind: Default::default(),
            general_shader: Default::default(),
            closest_hit_shader: Default::default(),
            any_hit_shader: Default::default(),
            intersection_shader: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html)
pub struct RayTracingPipelineCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk10::PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoNV,
    pub max_recursion_depth: u32,
    pub layout: crate::vk10::PipelineLayout,
    pub base_pipeline_handle: crate::vk10::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            max_recursion_depth: Default::default(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeometryTrianglesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTrianglesNV.html)
pub struct GeometryTrianglesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub vertex_data: crate::vk10::Buffer,
    pub vertex_offset: crate::vk10::DeviceSize,
    pub vertex_count: u32,
    pub vertex_stride: crate::vk10::DeviceSize,
    pub vertex_format: crate::vk10::Format,
    pub index_data: crate::vk10::Buffer,
    pub index_offset: crate::vk10::DeviceSize,
    pub index_count: u32,
    pub index_type: crate::vk10::IndexType,
    pub transform_data: crate::vk10::Buffer,
    pub transform_offset: crate::vk10::DeviceSize,
}
impl Default for GeometryTrianglesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GEOMETRY_TRIANGLES_NV,
            p_next: std::ptr::null(),
            vertex_data: Default::default(),
            vertex_offset: Default::default(),
            vertex_count: Default::default(),
            vertex_stride: Default::default(),
            vertex_format: Default::default(),
            index_data: Default::default(),
            index_offset: Default::default(),
            index_count: Default::default(),
            index_type: Default::default(),
            transform_data: Default::default(),
            transform_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeometryAABBNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryAABBNV.html)
pub struct GeometryAABBNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub aabb_data: crate::vk10::Buffer,
    pub num_aabbs: u32,
    pub stride: u32,
    pub offset: crate::vk10::DeviceSize,
}
impl Default for GeometryAABBNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GEOMETRY_AABB_NV,
            p_next: std::ptr::null(),
            aabb_data: Default::default(),
            num_aabbs: Default::default(),
            stride: Default::default(),
            offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeometryDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html)
pub struct GeometryDataNV {
    pub triangles: GeometryTrianglesNV,
    pub aabbs: GeometryAABBNV,
}
impl Default for GeometryDataNV {
    fn default() -> Self {
        Self {
            triangles: Default::default(),
            aabbs: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGeometryNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryNV.html)
pub struct GeometryNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub geometry_type: crate::extensions::khr_acceleration_structure::GeometryTypeKHR,
    pub geometry: GeometryDataNV,
    pub flags: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR,
}
impl Default for GeometryNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GEOMETRY_NV,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInfoNV.html)
pub struct AccelerationStructureInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsNV,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub p_geometries: *const GeometryNV,
}
impl Default for AccelerationStructureInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_INFO_NV,
            p_next: std::ptr::null(),
            kind: Default::default(),
            flags: Default::default(),
            instance_count: Default::default(),
            geometry_count: Default::default(),
            p_geometries: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoNV.html)
pub struct AccelerationStructureCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub compacted_size: crate::vk10::DeviceSize,
    pub info: AccelerationStructureInfoNV,
}
impl Default for AccelerationStructureCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            compacted_size: Default::default(),
            info: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html)
pub struct BindAccelerationStructureMemoryInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acceleration_structure: AccelerationStructureNV,
    pub memory: crate::vk10::DeviceMemory,
    pub memory_offset: crate::vk10::DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl Default for BindAccelerationStructureMemoryInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            device_index_count: Default::default(),
            p_device_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html)
pub struct WriteDescriptorSetAccelerationStructureNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureNV,
}
impl Default for WriteDescriptorSetAccelerationStructureNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
            p_next: std::ptr::null(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html)
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: AccelerationStructureNV,
}
impl Default for AccelerationStructureMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            kind: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html)
pub struct PhysicalDeviceRayTracingPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
}
impl Default for PhysicalDeviceRayTracingPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            max_geometry_count: Default::default(),
            max_instance_count: Default::default(),
            max_triangle_count: Default::default(),
            max_descriptor_set_acceleration_structures: Default::default(),
        }
    }
}
#[doc(alias = "VkAabbPositionsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsNV.html)
pub type AabbPositionsNV = crate::extensions::khr_acceleration_structure::AabbPositionsKHR;
#[doc(alias = "VkTransformMatrixNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixNV.html)
pub type TransformMatrixNV = crate::extensions::khr_acceleration_structure::TransformMatrixKHR;
#[doc(alias = "VkAccelerationStructureInstanceNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceNV.html)
pub type AccelerationStructureInstanceNV = crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
pub const SHADER_UNUSED_NV: u32 = crate::extensions::khr_ray_tracing_pipeline::SHADER_UNUSED_KHR;
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(pub i32);
impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const OBJECT: Self = Self(0);
    pub const BUILD_SCRATCH: Self = Self(1);
    pub const UPDATE_SCRATCH: Self = Self(2);
}
crate::enum_impl! {
    AccelerationStructureMemoryRequirementsTypeNV : i32, OBJECT, BUILD_SCRATCH,
    UPDATE_SCRATCH
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCompileDeferredNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html)
pub unsafe fn compile_deferred_nv(
    device: crate::vk10::Device,
    pipeline: crate::vk10::Pipeline,
    shader: u32,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .compile_deferred_nv
        .unwrap())(device, pipeline, shader)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCompileDeferredNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html)
    pub unsafe fn compile_deferred_nv(
        &self,
        pipeline: crate::vk10::Pipeline,
        shader: u32,
    ) -> crate::VulkanResult<()> {
        let compile_deferred_nv = (*self.table).compile_deferred_nv.unwrap();
        let result = compile_deferred_nv(self.handle, pipeline, shader as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateAccelerationStructureNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html)
pub unsafe fn create_acceleration_structure_nv(
    device: crate::vk10::Device,
    p_create_info: *const AccelerationStructureCreateInfoNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_acceleration_structure_nv
        .unwrap())(device, p_create_info, p_allocator, p_acceleration_structure)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html)
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<AccelerationStructureNV> {
        let create_acceleration_structure_nv = (*self.table)
            .create_acceleration_structure_nv
            .unwrap();
        let mut acceleration_structure = Default::default();
        let result = create_acceleration_structure_nv(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut acceleration_structure,
        );
        crate::new_result(acceleration_structure, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyAccelerationStructureNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html)
pub unsafe fn destroy_acceleration_structure_nv(
    device: crate::vk10::Device,
    acceleration_structure: AccelerationStructureNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_acceleration_structure_nv
        .unwrap())(device, acceleration_structure, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html)
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_acceleration_structure_nv = (*self.table)
            .destroy_acceleration_structure_nv
            .unwrap();
        destroy_acceleration_structure_nv(
            self.handle,
            acceleration_structure,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
pub unsafe fn get_acceleration_structure_memory_requirements_nv(
    device: crate::vk10::Device,
    p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
    p_memory_requirements: *mut crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_acceleration_structure_memory_requirements_nv
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR {
        let get_acceleration_structure_memory_requirements_nv = (*self.table)
            .get_acceleration_structure_memory_requirements_nv
            .unwrap();
        let mut memory_requirements = Default::default();
        get_acceleration_structure_memory_requirements_nv(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
pub unsafe fn bind_acceleration_structure_memory_nv(
    device: crate::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_acceleration_structure_memory_nv
        .unwrap())(device, bind_info_count, p_bind_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> crate::VulkanResult<()> {
        let bind_acceleration_structure_memory_nv = (*self.table)
            .bind_acceleration_structure_memory_nv
            .unwrap();
        let bind_info_count = bind_infos.len();
        let result = bind_acceleration_structure_memory_nv(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
pub unsafe fn cmd_copy_acceleration_structure_nv(
    command_buffer: crate::vk10::CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_acceleration_structure_nv
        .unwrap())(command_buffer, dst, src, mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) {
        let cmd_copy_acceleration_structure_nv = (*self.table)
            .cmd_copy_acceleration_structure_nv
            .unwrap();
        cmd_copy_acceleration_structure_nv(command_buffer, dst, src, mode as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
pub unsafe fn cmd_write_acceleration_structures_properties_nv(
    command_buffer: crate::vk10::CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureNV,
    query_type: crate::vk10::QueryType,
    query_pool: crate::vk10::QueryPool,
    first_query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_acceleration_structures_properties_nv
        .unwrap())(
        command_buffer,
        acceleration_structure_count,
        p_acceleration_structures,
        query_type,
        query_pool,
        first_query,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        let cmd_write_acceleration_structures_properties_nv = (*self.table)
            .cmd_write_acceleration_structures_properties_nv
            .unwrap();
        let acceleration_structure_count = acceleration_structures.len();
        cmd_write_acceleration_structures_properties_nv(
            command_buffer,
            acceleration_structure_count as _,
            acceleration_structures.as_ptr(),
            query_type as _,
            query_pool,
            first_query as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
pub unsafe fn cmd_build_acceleration_structure_nv(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const AccelerationStructureInfoNV,
    instance_data: crate::vk10::Buffer,
    instance_offset: crate::vk10::DeviceSize,
    update: crate::vk10::Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: crate::vk10::Buffer,
    scratch_offset: crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_build_acceleration_structure_nv
        .unwrap())(
        command_buffer,
        p_info,
        instance_data,
        instance_offset,
        update,
        dst,
        src,
        scratch,
        scratch_offset,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &AccelerationStructureInfoNV,
        instance_data: crate::vk10::Buffer,
        instance_offset: crate::vk10::DeviceSize,
        update: bool,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: crate::vk10::Buffer,
        scratch_offset: crate::vk10::DeviceSize,
    ) {
        let cmd_build_acceleration_structure_nv = (*self.table)
            .cmd_build_acceleration_structure_nv
            .unwrap();
        cmd_build_acceleration_structure_nv(
            command_buffer,
            info as _,
            instance_data,
            instance_offset as _,
            update as _,
            dst,
            src,
            scratch,
            scratch_offset as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdTraceRaysNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html)
pub unsafe fn cmd_trace_rays_nv(
    command_buffer: crate::vk10::CommandBuffer,
    raygen_shader_binding_table_buffer: crate::vk10::Buffer,
    raygen_shader_binding_offset: crate::vk10::DeviceSize,
    miss_shader_binding_table_buffer: crate::vk10::Buffer,
    miss_shader_binding_offset: crate::vk10::DeviceSize,
    miss_shader_binding_stride: crate::vk10::DeviceSize,
    hit_shader_binding_table_buffer: crate::vk10::Buffer,
    hit_shader_binding_offset: crate::vk10::DeviceSize,
    hit_shader_binding_stride: crate::vk10::DeviceSize,
    callable_shader_binding_table_buffer: crate::vk10::Buffer,
    callable_shader_binding_offset: crate::vk10::DeviceSize,
    callable_shader_binding_stride: crate::vk10::DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_trace_rays_nv
        .unwrap())(
        command_buffer,
        raygen_shader_binding_table_buffer,
        raygen_shader_binding_offset,
        miss_shader_binding_table_buffer,
        miss_shader_binding_offset,
        miss_shader_binding_stride,
        hit_shader_binding_table_buffer,
        hit_shader_binding_offset,
        hit_shader_binding_stride,
        callable_shader_binding_table_buffer,
        callable_shader_binding_offset,
        callable_shader_binding_stride,
        width,
        height,
        depth,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html)
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        raygen_shader_binding_table_buffer: crate::vk10::Buffer,
        raygen_shader_binding_offset: crate::vk10::DeviceSize,
        miss_shader_binding_table_buffer: crate::vk10::Buffer,
        miss_shader_binding_offset: crate::vk10::DeviceSize,
        miss_shader_binding_stride: crate::vk10::DeviceSize,
        hit_shader_binding_table_buffer: crate::vk10::Buffer,
        hit_shader_binding_offset: crate::vk10::DeviceSize,
        hit_shader_binding_stride: crate::vk10::DeviceSize,
        callable_shader_binding_table_buffer: crate::vk10::Buffer,
        callable_shader_binding_offset: crate::vk10::DeviceSize,
        callable_shader_binding_stride: crate::vk10::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let cmd_trace_rays_nv = (*self.table).cmd_trace_rays_nv.unwrap();
        cmd_trace_rays_nv(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset as _,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset as _,
            miss_shader_binding_stride as _,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset as _,
            hit_shader_binding_stride as _,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset as _,
            callable_shader_binding_stride as _,
            width as _,
            height as _,
            depth as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html)
pub unsafe fn get_ray_tracing_shader_group_handles_nv(
    device: crate::vk10::Device,
    pipeline: crate::vk10::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_ray_tracing_shader_group_handles_khr
        .unwrap())(device, pipeline, first_group, group_count, data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html)
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<()> {
        let get_ray_tracing_shader_group_handles_nv = (*self.table)
            .get_ray_tracing_shader_group_handles_nv
            .unwrap();
        let result = get_ray_tracing_shader_group_handles_nv(
            self.handle,
            pipeline,
            first_group as _,
            group_count as _,
            data_size,
            data,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
pub unsafe fn get_acceleration_structure_handle_nv(
    device: crate::vk10::Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_acceleration_structure_handle_nv
        .unwrap())(device, acceleration_structure, data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
    pub unsafe fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<()> {
        let get_acceleration_structure_handle_nv = (*self.table)
            .get_acceleration_structure_handle_nv
            .unwrap();
        let result = get_acceleration_structure_handle_nv(
            self.handle,
            acceleration_structure,
            data_size,
            data,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
pub unsafe fn create_ray_tracing_pipelines_nv(
    device: crate::vk10::Device,
    pipeline_cache: crate::vk10::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_pipelines: *mut crate::vk10::Pipeline,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_ray_tracing_pipelines_nv
        .unwrap())(
        device,
        pipeline_cache,
        create_info_count,
        p_create_infos,
        p_allocator,
        p_pipelines,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: crate::vk10::PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<(Vec<crate::vk10::Pipeline>, crate::vk10::Result)> {
        let create_ray_tracing_pipelines_nv = (*self.table)
            .create_ray_tracing_pipelines_nv
            .unwrap();
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as usize];
        let result = create_ray_tracing_pipelines_nv(
            self.handle,
            pipeline_cache,
            create_info_count as _,
            create_infos.as_ptr(),
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::new_result((pipelines, result), result)
    }
}
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
pub const NV_RAY_TRACING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_ray_tracing"
);
