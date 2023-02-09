#[doc(alias = "VkExternalFenceHandleTypeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagsKHR.html)
pub type ExternalFenceHandleTypeFlagsKHR = crate::vk11::ExternalFenceHandleTypeFlags;
#[doc(alias = "VkExternalFenceFeatureFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagsKHR.html)
pub type ExternalFenceFeatureFlagsKHR = crate::vk11::ExternalFenceFeatureFlags;
#[doc(alias = "VkPhysicalDeviceExternalFenceInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfoKHR.html)
pub type PhysicalDeviceExternalFenceInfoKHR = crate::vk11::PhysicalDeviceExternalFenceInfo;
#[doc(alias = "VkExternalFencePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFencePropertiesKHR.html)
pub type ExternalFencePropertiesKHR = crate::vk11::ExternalFenceProperties;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
pub unsafe fn get_physical_device_external_fence_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_fence_properties
        .unwrap())(physical_device, p_external_fence_info, p_external_fence_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_fence_info: &crate::vk11::PhysicalDeviceExternalFenceInfo,
    ) -> crate::vk11::ExternalFenceProperties {
        let get_physical_device_external_fence_properties_khr = (*self.table)
            .get_physical_device_external_fence_properties_khr
            .unwrap();
        let mut external_fence_properties = Default::default();
        get_physical_device_external_fence_properties_khr(
            physical_device,
            external_fence_info as _,
            &mut external_fence_properties,
        );
        external_fence_properties
    }
}
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_fence_capabilities"
);
