#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html)
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub performance_counter_query_pools: crate::vk10::Bool32,
    pub performance_counter_multiple_query_pools: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            performance_counter_query_pools: Default::default(),
            performance_counter_multiple_query_pools: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html)
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub allow_command_buffer_query_copies: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            allow_command_buffer_query_copies: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceCounterKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html)
pub struct PerformanceCounterKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: [u8; crate::vk10::UUID_SIZE as usize],
}
impl Default for PerformanceCounterKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_COUNTER_KHR,
            p_next: std::ptr::null_mut(),
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html)
pub struct PerformanceCounterDescriptionKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub category: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
}
impl Default for PerformanceCounterDescriptionKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            name: unsafe { std::mem::zeroed() },
            category: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html)
pub struct QueryPoolPerformanceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
}
impl Default for QueryPoolPerformanceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            queue_family_index: Default::default(),
            counter_index_count: Default::default(),
            p_counter_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkPerformanceCounterResultKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html)
pub union PerformanceCounterResultKHR {
    pub int_32: i32,
    pub int_64: i64,
    pub uint_32: u32,
    pub uint_64: u64,
    pub float_32: std::os::raw::c_float,
    pub float_64: std::os::raw::c_double,
}
impl Default for PerformanceCounterResultKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html)
pub struct AcquireProfilingLockInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}
impl Default for AcquireProfilingLockInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            timeout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html)
pub struct PerformanceQuerySubmitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub counter_pass_index: u32,
}
impl Default for PerformanceQuerySubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            counter_pass_index: Default::default(),
        }
    }
}
#[doc(alias = "VkPerformanceCounterScopeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterScopeKHR(pub i32);
impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER: Self = Self(0);
    pub const RENDER_PASS: Self = Self(1);
    pub const COMMAND: Self = Self(2);
}
crate::enum_impl! {
    PerformanceCounterScopeKHR : i32, COMMAND_BUFFER, RENDER_PASS, COMMAND
}
#[doc(alias = "VkPerformanceCounterUnitKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterUnitKHR(pub i32);
impl PerformanceCounterUnitKHR {
    pub const GENERIC: Self = Self(0);
    pub const PERCENTAGE: Self = Self(1);
    pub const NANOSECONDS: Self = Self(2);
    pub const BYTES: Self = Self(3);
    pub const BYTES_PER_SECOND: Self = Self(4);
    pub const KELVIN: Self = Self(5);
    pub const WATTS: Self = Self(6);
    pub const VOLTS: Self = Self(7);
    pub const AMPS: Self = Self(8);
    pub const HERTZ: Self = Self(9);
    pub const CYCLES: Self = Self(10);
}
crate::enum_impl! {
    PerformanceCounterUnitKHR : i32, GENERIC, PERCENTAGE, NANOSECONDS, BYTES,
    BYTES_PER_SECOND, KELVIN, WATTS, VOLTS, AMPS, HERTZ, CYCLES
}
#[doc(alias = "VkPerformanceCounterStorageKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterStorageKHR(pub i32);
impl PerformanceCounterStorageKHR {
    pub const INT32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT32: Self = Self(2);
    pub const UINT64: Self = Self(3);
    pub const FLOAT32: Self = Self(4);
    pub const FLOAT64: Self = Self(5);
}
crate::enum_impl! {
    PerformanceCounterStorageKHR : i32, INT32, INT64, UINT32, UINT64, FLOAT32, FLOAT64
}
#[doc(alias = "VkPerformanceCounterDescriptionFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionFlagsKHR(pub u32);
impl PerformanceCounterDescriptionFlagsKHR {
    pub const PERFORMANCE_IMPACTING: Self = Self(1 << 0);
    pub const CONCURRENTLY_IMPACTED: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    PerformanceCounterDescriptionFlagsKHR : u32, 0x3, PERFORMANCE_IMPACTING,
    CONCURRENTLY_IMPACTED
}
#[doc(alias = "VkAcquireProfilingLockFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AcquireProfilingLockFlagsKHR(pub u32);
crate::bitflags_impl! {
    AcquireProfilingLockFlagsKHR : u32, 0x0,
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
    physical_device: crate::vk10::PhysicalDevice,
    queue_family_index: u32,
    p_counter_count: *mut u32,
    p_counters: *mut PerformanceCounterKHR,
    p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_physical_device_queue_family_performance_query_counters_khr
        .unwrap())(
        physical_device,
        queue_family_index,
        p_counter_count,
        p_counters,
        p_counter_descriptions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        counter_count: Option<u32>,
        mut counters_callback: impl FnMut(&mut Vec<PerformanceCounterKHR>),
        mut counter_descriptions_callback: impl FnMut(
            &mut Vec<PerformanceCounterDescriptionKHR>,
        ),
    ) -> crate::VulkanResult<
        (
            (Vec<PerformanceCounterKHR>, Vec<PerformanceCounterDescriptionKHR>),
            crate::vk10::Result,
        ),
    > {
        let enumerate_physical_device_queue_family_performance_query_counters_khr = (*self
            .table)
            .enumerate_physical_device_queue_family_performance_query_counters_khr
            .unwrap();
        let mut counter_count = match counter_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_physical_device_queue_family_performance_query_counters_khr(
                    physical_device,
                    queue_family_index as _,
                    &mut v,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut counters = vec![Default::default(); counter_count as usize];
        counters_callback(&mut counters);
        let mut counter_descriptions = vec![Default::default(); counter_count as usize];
        counter_descriptions_callback(&mut counter_descriptions);
        let result = enumerate_physical_device_queue_family_performance_query_counters_khr(
            physical_device,
            queue_family_index as _,
            &mut counter_count,
            counters.as_mut_ptr(),
            counter_descriptions.as_mut_ptr(),
        );
        crate::new_result(((counters, counter_descriptions), result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
    p_num_passes: *mut u32,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_queue_family_performance_query_passes_khr
        .unwrap())(physical_device, p_performance_query_create_info, p_num_passes)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        let get_physical_device_queue_family_performance_query_passes_khr = (*self.table)
            .get_physical_device_queue_family_performance_query_passes_khr
            .unwrap();
        let mut num_passes = Default::default();
        get_physical_device_queue_family_performance_query_passes_khr(
            physical_device,
            performance_query_create_info as _,
            &mut num_passes,
        );
        num_passes
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireProfilingLockKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html)
pub unsafe fn acquire_profiling_lock_khr(
    device: crate::vk10::Device,
    p_info: *const AcquireProfilingLockInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .acquire_profiling_lock_khr
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html)
    pub unsafe fn acquire_profiling_lock_khr(
        &self,
        info: &AcquireProfilingLockInfoKHR,
    ) -> crate::VulkanResult<()> {
        let acquire_profiling_lock_khr = (*self.table)
            .acquire_profiling_lock_khr
            .unwrap();
        let result = acquire_profiling_lock_khr(self.handle, info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkReleaseProfilingLockKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html)
pub unsafe fn release_profiling_lock_khr(device: crate::vk10::Device) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .release_profiling_lock_khr
        .unwrap())(device)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html)
    pub unsafe fn release_profiling_lock_khr(&self) {
        let release_profiling_lock_khr = (*self.table)
            .release_profiling_lock_khr
            .unwrap();
        release_profiling_lock_khr(self.handle);
    }
}
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_performance_query"
);
