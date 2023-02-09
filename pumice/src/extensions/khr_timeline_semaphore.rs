#[doc(alias = "VkSemaphoreWaitFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagsKHR.html)
pub type SemaphoreWaitFlagsKHR = crate::vk12::SemaphoreWaitFlags;
#[doc(alias = "VkSemaphoreTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeKHR.html)
pub type SemaphoreTypeKHR = crate::vk12::SemaphoreType;
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html)
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = crate::vk12::PhysicalDeviceTimelineSemaphoreFeatures;
#[doc(alias = "VkPhysicalDeviceTimelineSemaphorePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html)
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = crate::vk12::PhysicalDeviceTimelineSemaphoreProperties;
#[doc(alias = "VkSemaphoreTypeCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfoKHR.html)
pub type SemaphoreTypeCreateInfoKHR = crate::vk12::SemaphoreTypeCreateInfo;
#[doc(alias = "VkTimelineSemaphoreSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfoKHR.html)
pub type TimelineSemaphoreSubmitInfoKHR = crate::vk12::TimelineSemaphoreSubmitInfo;
#[doc(alias = "VkSemaphoreWaitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfoKHR.html)
pub type SemaphoreWaitInfoKHR = crate::vk12::SemaphoreWaitInfo;
#[doc(alias = "VkSemaphoreSignalInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfoKHR.html)
pub type SemaphoreSignalInfoKHR = crate::vk12::SemaphoreSignalInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
pub unsafe fn get_semaphore_counter_value_khr(
    device: crate::vk10::Device,
    semaphore: crate::vk10::Semaphore,
    p_value: *mut u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_counter_value
        .unwrap())(device, semaphore, p_value)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: crate::vk10::Semaphore,
    ) -> crate::VulkanResult<u64> {
        let get_semaphore_counter_value_khr = (*self.table)
            .get_semaphore_counter_value_khr
            .unwrap();
        let mut value = Default::default();
        let result = get_semaphore_counter_value_khr(self.handle, semaphore, &mut value);
        crate::new_result(value, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWaitSemaphoresKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
pub unsafe fn wait_semaphores_khr(
    device: crate::vk10::Device,
    p_wait_info: *const crate::vk12::SemaphoreWaitInfo,
    timeout: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .wait_semaphores
        .unwrap())(device, p_wait_info, timeout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
    pub unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &crate::vk12::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let wait_semaphores_khr = (*self.table).wait_semaphores_khr.unwrap();
        let result = wait_semaphores_khr(self.handle, wait_info as _, timeout as _);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSignalSemaphoreKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
pub unsafe fn signal_semaphore_khr(
    device: crate::vk10::Device,
    p_signal_info: *const crate::vk12::SemaphoreSignalInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .signal_semaphore
        .unwrap())(device, p_signal_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
    pub unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &crate::vk12::SemaphoreSignalInfo,
    ) -> crate::VulkanResult<()> {
        let signal_semaphore_khr = (*self.table).signal_semaphore_khr.unwrap();
        let result = signal_semaphore_khr(self.handle, signal_info as _);
        crate::new_result((), result)
    }
}
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_timeline_semaphore"
);
