crate::dispatchable_handle!(
    PerformanceConfigurationINTEL, PERFORMANCE_CONFIGURATION_INTEL,
    "VkPerformanceConfigurationINTEL",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html)"
);
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkPerformanceValueDataINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html)
pub union PerformanceValueDataINTEL {
    pub value_32: u32,
    pub value_64: u64,
    pub value_float: std::os::raw::c_float,
    pub value_bool: crate::vk10::Bool32,
    pub value_string: *const std::os::raw::c_char,
}
impl Default for PerformanceValueDataINTEL {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceValueINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html)
pub struct PerformanceValueINTEL {
    pub kind: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}
impl Default for PerformanceValueINTEL {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            data: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html)
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_user_data: *mut std::os::raw::c_void,
}
impl Default for InitializePerformanceApiInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL,
            p_next: std::ptr::null(),
            p_user_data: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html)
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
impl Default for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
            p_next: std::ptr::null(),
            performance_counters_sampling: Default::default(),
        }
    }
}
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfoINTEL.html)
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html)
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub marker: u64,
}
impl Default for PerformanceMarkerInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html)
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub marker: u32,
}
impl Default for PerformanceStreamMarkerInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL,
            p_next: std::ptr::null(),
            marker: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html)
pub struct PerformanceOverrideInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: PerformanceOverrideTypeINTEL,
    pub enable: crate::vk10::Bool32,
    pub parameter: u64,
}
impl Default for PerformanceOverrideInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL,
            p_next: std::ptr::null(),
            kind: Default::default(),
            enable: Default::default(),
            parameter: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html)
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: PerformanceConfigurationTypeINTEL,
}
impl Default for PerformanceConfigurationAcquireInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
            p_next: std::ptr::null(),
            kind: Default::default(),
        }
    }
}
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationTypeINTEL(pub i32);
impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
}
crate::enum_impl! {
    PerformanceConfigurationTypeINTEL : i32, COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED
}
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryPoolSamplingModeINTEL(pub i32);
impl QueryPoolSamplingModeINTEL {
    pub const MANUAL: Self = Self(0);
}
crate::enum_impl! {
    QueryPoolSamplingModeINTEL : i32, MANUAL
}
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceOverrideTypeINTEL(pub i32);
impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE: Self = Self(0);
    pub const FLUSH_GPU_CACHES: Self = Self(1);
}
crate::enum_impl! {
    PerformanceOverrideTypeINTEL : i32, NULL_HARDWARE, FLUSH_GPU_CACHES
}
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceParameterTypeINTEL(pub i32);
impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS: Self = Self(1);
}
crate::enum_impl! {
    PerformanceParameterTypeINTEL : i32, HW_COUNTERS_SUPPORTED, STREAM_MARKER_VALID_BITS
}
#[doc(alias = "VkPerformanceValueTypeINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceValueTypeINTEL(pub i32);
impl PerformanceValueTypeINTEL {
    pub const UINT32: Self = Self(0);
    pub const UINT64: Self = Self(1);
    pub const FLOAT: Self = Self(2);
    pub const BOOL: Self = Self(3);
    pub const STRING: Self = Self(4);
}
crate::enum_impl! {
    PerformanceValueTypeINTEL : i32, UINT32, UINT64, FLOAT, BOOL, STRING
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkInitializePerformanceApiINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html)
pub unsafe fn initialize_performance_api_intel(
    device: crate::vk10::Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .initialize_performance_api_intel
        .unwrap())(device, p_initialize_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html)
    pub unsafe fn initialize_performance_api_intel(
        &self,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> crate::VulkanResult<()> {
        let initialize_performance_api_intel = (*self.table)
            .initialize_performance_api_intel
            .unwrap();
        let result = initialize_performance_api_intel(self.handle, initialize_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
pub unsafe fn uninitialize_performance_api_intel(device: crate::vk10::Device) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .uninitialize_performance_api_intel
        .unwrap())(device)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
    pub unsafe fn uninitialize_performance_api_intel(&self) {
        let uninitialize_performance_api_intel = (*self.table)
            .uninitialize_performance_api_intel
            .unwrap();
        uninitialize_performance_api_intel(self.handle);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
pub unsafe fn cmd_set_performance_marker_intel(
    command_buffer: crate::vk10::CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_performance_marker_intel
        .unwrap())(command_buffer, p_marker_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> crate::VulkanResult<()> {
        let cmd_set_performance_marker_intel = (*self.table)
            .cmd_set_performance_marker_intel
            .unwrap();
        let result = cmd_set_performance_marker_intel(command_buffer, marker_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
pub unsafe fn cmd_set_performance_stream_marker_intel(
    command_buffer: crate::vk10::CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_performance_stream_marker_intel
        .unwrap())(command_buffer, p_marker_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> crate::VulkanResult<()> {
        let cmd_set_performance_stream_marker_intel = (*self.table)
            .cmd_set_performance_stream_marker_intel
            .unwrap();
        let result = cmd_set_performance_stream_marker_intel(
            command_buffer,
            marker_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
pub unsafe fn cmd_set_performance_override_intel(
    command_buffer: crate::vk10::CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_performance_override_intel
        .unwrap())(command_buffer, p_override_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> crate::VulkanResult<()> {
        let cmd_set_performance_override_intel = (*self.table)
            .cmd_set_performance_override_intel
            .unwrap();
        let result = cmd_set_performance_override_intel(
            command_buffer,
            override_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
pub unsafe fn acquire_performance_configuration_intel(
    device: crate::vk10::Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .acquire_performance_configuration_intel
        .unwrap())(device, p_acquire_info, p_configuration)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> crate::VulkanResult<PerformanceConfigurationINTEL> {
        let acquire_performance_configuration_intel = (*self.table)
            .acquire_performance_configuration_intel
            .unwrap();
        let mut configuration = Default::default();
        let result = acquire_performance_configuration_intel(
            self.handle,
            acquire_info as _,
            &mut configuration,
        );
        crate::new_result(configuration, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
pub unsafe fn release_performance_configuration_intel(
    device: crate::vk10::Device,
    configuration: PerformanceConfigurationINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .release_performance_configuration_intel
        .unwrap())(device, configuration)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
    pub unsafe fn release_performance_configuration_intel(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::VulkanResult<()> {
        let release_performance_configuration_intel = (*self.table)
            .release_performance_configuration_intel
            .unwrap();
        let result = release_performance_configuration_intel(self.handle, configuration);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
pub unsafe fn queue_set_performance_configuration_intel(
    queue: crate::vk10::Queue,
    configuration: PerformanceConfigurationINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_set_performance_configuration_intel
        .unwrap())(queue, configuration)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: crate::vk10::Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::VulkanResult<()> {
        let queue_set_performance_configuration_intel = (*self.table)
            .queue_set_performance_configuration_intel
            .unwrap();
        let result = queue_set_performance_configuration_intel(queue, configuration);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPerformanceParameterINTEL")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html)
pub unsafe fn get_performance_parameter_intel(
    device: crate::vk10::Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_performance_parameter_intel
        .unwrap())(device, parameter, p_value)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html)
    pub unsafe fn get_performance_parameter_intel(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::VulkanResult<PerformanceValueINTEL> {
        let get_performance_parameter_intel = (*self.table)
            .get_performance_parameter_intel
            .unwrap();
        let mut value = Default::default();
        let result = get_performance_parameter_intel(
            self.handle,
            parameter as _,
            &mut value,
        );
        crate::new_result(value, result)
    }
}
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_INTEL_performance_query"
);
