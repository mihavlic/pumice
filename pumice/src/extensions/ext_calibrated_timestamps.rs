#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html)
pub struct CalibratedTimestampInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub time_domain: TimeDomainEXT,
}
impl Default for CalibratedTimestampInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CALIBRATED_TIMESTAMP_INFO_EXT,
            p_next: std::ptr::null(),
            time_domain: Default::default(),
        }
    }
}
#[doc(alias = "VkTimeDomainEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TimeDomainEXT(pub i32);
impl TimeDomainEXT {
    pub const DEVICE: Self = Self(0);
    pub const CLOCK_MONOTONIC: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER: Self = Self(3);
}
crate::enum_impl! {
    TimeDomainEXT : i32, DEVICE, CLOCK_MONOTONIC, CLOCK_MONOTONIC_RAW,
    QUERY_PERFORMANCE_COUNTER
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
    physical_device: crate::vk10::PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_calibrateable_time_domains_ext
        .unwrap())(physical_device, p_time_domain_count, p_time_domains)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        time_domain_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<TimeDomainEXT>, crate::vk10::Result)> {
        let get_physical_device_calibrateable_time_domains_ext = (*self.table)
            .get_physical_device_calibrateable_time_domains_ext
            .unwrap();
        let mut time_domain_count = match time_domain_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_calibrateable_time_domains_ext(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut time_domains = vec![Default::default(); time_domain_count as usize];
        let result = get_physical_device_calibrateable_time_domains_ext(
            physical_device,
            &mut time_domain_count,
            time_domains.as_mut_ptr(),
        );
        crate::new_result((time_domains, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetCalibratedTimestampsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
pub unsafe fn get_calibrated_timestamps_ext(
    device: crate::vk10::Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoEXT,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_calibrated_timestamps_ext
        .unwrap())(
        device,
        timestamp_count,
        p_timestamp_infos,
        p_timestamps,
        p_max_deviation,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoEXT],
    ) -> crate::VulkanResult<(Vec<u64>, u64)> {
        let get_calibrated_timestamps_ext = (*self.table)
            .get_calibrated_timestamps_ext
            .unwrap();
        let timestamp_count = timestamp_infos.len();
        let mut timestamps = vec![Default::default(); timestamp_count as usize];
        let mut max_deviation = Default::default();
        let result = get_calibrated_timestamps_ext(
            self.handle,
            timestamp_count as _,
            timestamp_infos.as_ptr(),
            timestamps.as_mut_ptr(),
            &mut max_deviation,
        );
        crate::new_result((timestamps, max_deviation), result)
    }
}
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_calibrated_timestamps"
);
