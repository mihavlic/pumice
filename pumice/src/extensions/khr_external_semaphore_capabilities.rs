#[doc(alias = "VkExternalSemaphoreHandleTypeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagsKHR.html)
pub type ExternalSemaphoreHandleTypeFlagsKHR = crate::vk11::ExternalSemaphoreHandleTypeFlags;
#[doc(alias = "VkExternalSemaphoreFeatureFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagsKHR.html)
pub type ExternalSemaphoreFeatureFlagsKHR = crate::vk11::ExternalSemaphoreFeatureFlags;
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfoKHR.html)
pub type PhysicalDeviceExternalSemaphoreInfoKHR = crate::vk11::PhysicalDeviceExternalSemaphoreInfo;
#[doc(alias = "VkExternalSemaphorePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphorePropertiesKHR.html)
pub type ExternalSemaphorePropertiesKHR = crate::vk11::ExternalSemaphoreProperties;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
pub unsafe fn get_physical_device_external_semaphore_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_semaphore_properties
        .unwrap())(
        physical_device,
        p_external_semaphore_info,
        p_external_semaphore_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_semaphore_info: &crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
    ) -> crate::vk11::ExternalSemaphoreProperties {
        let get_physical_device_external_semaphore_properties_khr = (*self.table)
            .get_physical_device_external_semaphore_properties_khr
            .unwrap();
        let mut external_semaphore_properties = Default::default();
        get_physical_device_external_semaphore_properties_khr(
            physical_device,
            external_semaphore_info as _,
            &mut external_semaphore_properties,
        );
        external_semaphore_properties
    }
}
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_semaphore_capabilities"
);
