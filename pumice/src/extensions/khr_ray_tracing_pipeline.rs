#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html)
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const std::os::raw::c_void,
}
impl Default for RayTracingShaderGroupCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            kind: Default::default(),
            general_shader: Default::default(),
            closest_hit_shader: Default::default(),
            any_hit_shader: Default::default(),
            intersection_shader: Default::default(),
            p_shader_group_capture_replay_handle: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html)
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk10::PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoKHR,
    pub max_pipeline_ray_recursion_depth: u32,
    pub p_library_info: *const crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    pub p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    pub p_dynamic_state: *const crate::vk10::PipelineDynamicStateCreateInfo,
    pub layout: crate::vk10::PipelineLayout,
    pub base_pipeline_handle: crate::vk10::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: std::ptr::null(),
            group_count: Default::default(),
            p_groups: std::ptr::null(),
            max_pipeline_ray_recursion_depth: Default::default(),
            p_library_info: std::ptr::null(),
            p_library_interface: std::ptr::null(),
            p_dynamic_state: std::ptr::null(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html)
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ray_tracing_pipeline: crate::vk10::Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: crate::vk10::Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: crate::vk10::Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect: crate::vk10::Bool32,
    pub ray_traversal_primitive_culling: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_tracing_pipeline: Default::default(),
            ray_tracing_pipeline_shader_group_handle_capture_replay: Default::default(),
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Default::default(),
            ray_tracing_pipeline_trace_rays_indirect: Default::default(),
            ray_traversal_primitive_culling: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html)
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_group_handle_size: u32,
    pub max_ray_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub shader_group_handle_capture_replay_size: u32,
    pub max_ray_dispatch_invocation_count: u32,
    pub shader_group_handle_alignment: u32,
    pub max_ray_hit_attribute_size: u32,
}
impl Default for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_ray_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            shader_group_handle_capture_replay_size: Default::default(),
            max_ray_dispatch_invocation_count: Default::default(),
            shader_group_handle_alignment: Default::default(),
            max_ray_hit_attribute_size: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html)
pub struct StridedDeviceAddressRegionKHR {
    pub device_address: crate::vk10::DeviceAddress,
    pub stride: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
}
impl Default for StridedDeviceAddressRegionKHR {
    fn default() -> Self {
        Self {
            device_address: Default::default(),
            stride: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html)
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for TraceRaysIndirectCommandKHR {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html)
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_pipeline_ray_payload_size: u32,
    pub max_pipeline_ray_hit_attribute_size: u32,
}
impl Default for RayTracingPipelineInterfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            max_pipeline_ray_payload_size: Default::default(),
            max_pipeline_ray_hit_attribute_size: Default::default(),
        }
    }
}
pub const SHADER_UNUSED_KHR: u32 = !0;
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RayTracingShaderGroupTypeKHR(pub i32);
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
    /// nv_ray_tracing
    pub const GENERAL_NV: Self = Self::GENERAL;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP;
}
crate::enum_impl! {
    RayTracingShaderGroupTypeKHR : i32, GENERAL, TRIANGLES_HIT_GROUP,
    PROCEDURAL_HIT_GROUP
}
#[doc(alias = "VkShaderGroupShaderKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderGroupShaderKHR(pub i32);
impl ShaderGroupShaderKHR {
    pub const GENERAL: Self = Self(0);
    pub const CLOSEST_HIT: Self = Self(1);
    pub const ANY_HIT: Self = Self(2);
    pub const INTERSECTION: Self = Self(3);
}
crate::enum_impl! {
    ShaderGroupShaderKHR : i32, GENERAL, CLOSEST_HIT, ANY_HIT, INTERSECTION
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdTraceRaysKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)
pub unsafe fn cmd_trace_rays_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_trace_rays_khr
        .unwrap())(
        command_buffer,
        p_raygen_shader_binding_table,
        p_miss_shader_binding_table,
        p_hit_shader_binding_table,
        p_callable_shader_binding_table,
        width,
        height,
        depth,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let cmd_trace_rays_khr = (*self.table).cmd_trace_rays_khr.unwrap();
        cmd_trace_rays_khr(
            command_buffer,
            raygen_shader_binding_table as _,
            miss_shader_binding_table as _,
            hit_shader_binding_table as _,
            callable_shader_binding_table as _,
            width as _,
            height as _,
            depth as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
pub unsafe fn get_ray_tracing_shader_group_handles_khr(
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
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<()> {
        let get_ray_tracing_shader_group_handles_khr = (*self.table)
            .get_ray_tracing_shader_group_handles_khr
            .unwrap();
        let result = get_ray_tracing_shader_group_handles_khr(
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
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
    device: crate::vk10::Device,
    pipeline: crate::vk10::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_ray_tracing_capture_replay_shader_group_handles_khr
        .unwrap())(device, pipeline, first_group, group_count, data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<()> {
        let get_ray_tracing_capture_replay_shader_group_handles_khr = (*self.table)
            .get_ray_tracing_capture_replay_shader_group_handles_khr
            .unwrap();
        let result = get_ray_tracing_capture_replay_shader_group_handles_khr(
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
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
pub unsafe fn create_ray_tracing_pipelines_khr(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    pipeline_cache: crate::vk10::PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_pipelines: *mut crate::vk10::Pipeline,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_ray_tracing_pipelines_khr
        .unwrap())(
        device,
        deferred_operation,
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
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        pipeline_cache: crate::vk10::PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<(Vec<crate::vk10::Pipeline>, crate::vk10::Result)> {
        let create_ray_tracing_pipelines_khr = (*self.table)
            .create_ray_tracing_pipelines_khr
            .unwrap();
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as usize];
        let result = create_ray_tracing_pipelines_khr(
            self.handle,
            deferred_operation,
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
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
pub unsafe fn cmd_trace_rays_indirect_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: crate::vk10::DeviceAddress,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_trace_rays_indirect_khr
        .unwrap())(
        command_buffer,
        p_raygen_shader_binding_table,
        p_miss_shader_binding_table,
        p_hit_shader_binding_table,
        p_callable_shader_binding_table,
        indirect_device_address,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: crate::vk10::DeviceAddress,
    ) {
        let cmd_trace_rays_indirect_khr = (*self.table)
            .cmd_trace_rays_indirect_khr
            .unwrap();
        cmd_trace_rays_indirect_khr(
            command_buffer,
            raygen_shader_binding_table as _,
            miss_shader_binding_table as _,
            hit_shader_binding_table as _,
            callable_shader_binding_table as _,
            indirect_device_address as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
    device: crate::vk10::Device,
    pipeline: crate::vk10::Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> crate::vk10::DeviceSize {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_ray_tracing_shader_group_stack_size_khr
        .unwrap())(device, pipeline, group, group_shader)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        pipeline: crate::vk10::Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) {
        let get_ray_tracing_shader_group_stack_size_khr = (*self.table)
            .get_ray_tracing_shader_group_stack_size_khr
            .unwrap();
        get_ray_tracing_shader_group_stack_size_khr(
            self.handle,
            pipeline,
            group as _,
            group_shader as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
    command_buffer: crate::vk10::CommandBuffer,
    pipeline_stack_size: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_ray_tracing_pipeline_stack_size_khr
        .unwrap())(command_buffer, pipeline_stack_size)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        let cmd_set_ray_tracing_pipeline_stack_size_khr = (*self.table)
            .cmd_set_ray_tracing_pipeline_stack_size_khr
            .unwrap();
        cmd_set_ray_tracing_pipeline_stack_size_khr(
            command_buffer,
            pipeline_stack_size as _,
        );
    }
}
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_ray_tracing_pipeline"
);
