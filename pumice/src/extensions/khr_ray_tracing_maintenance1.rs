#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkTraceRaysIndirectCommand2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommand2KHR.html)
pub struct TraceRaysIndirectCommand2KHR {
    pub raygen_shader_record_address: crate::vk10::DeviceAddress,
    pub raygen_shader_record_size: crate::vk10::DeviceSize,
    pub miss_shader_binding_table_address: crate::vk10::DeviceAddress,
    pub miss_shader_binding_table_size: crate::vk10::DeviceSize,
    pub miss_shader_binding_table_stride: crate::vk10::DeviceSize,
    pub hit_shader_binding_table_address: crate::vk10::DeviceAddress,
    pub hit_shader_binding_table_size: crate::vk10::DeviceSize,
    pub hit_shader_binding_table_stride: crate::vk10::DeviceSize,
    pub callable_shader_binding_table_address: crate::vk10::DeviceAddress,
    pub callable_shader_binding_table_size: crate::vk10::DeviceSize,
    pub callable_shader_binding_table_stride: crate::vk10::DeviceSize,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for TraceRaysIndirectCommand2KHR {
    fn default() -> Self {
        Self {
            raygen_shader_record_address: Default::default(),
            raygen_shader_record_size: Default::default(),
            miss_shader_binding_table_address: Default::default(),
            miss_shader_binding_table_size: Default::default(),
            miss_shader_binding_table_stride: Default::default(),
            hit_shader_binding_table_address: Default::default(),
            hit_shader_binding_table_size: Default::default(),
            hit_shader_binding_table_stride: Default::default(),
            callable_shader_binding_table_address: Default::default(),
            callable_shader_binding_table_size: Default::default(),
            callable_shader_binding_table_stride: Default::default(),
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR.html)
pub struct PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ray_tracing_maintenance_1: crate::vk10::Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect_2: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_tracing_maintenance_1: Default::default(),
            ray_tracing_pipeline_trace_rays_indirect_2: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html)
pub unsafe fn cmd_trace_rays_indirect_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    indirect_device_address: crate::vk10::DeviceAddress,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_trace_rays_indirect_2_khr
        .unwrap())(command_buffer, indirect_device_address)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html)
    pub unsafe fn cmd_trace_rays_indirect_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        indirect_device_address: crate::vk10::DeviceAddress,
    ) {
        let cmd_trace_rays_indirect_2_khr = (*self.table)
            .cmd_trace_rays_indirect_2_khr
            .unwrap();
        cmd_trace_rays_indirect_2_khr(command_buffer, indirect_device_address as _);
    }
}
pub const KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
pub const KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_ray_tracing_maintenance1"
);
