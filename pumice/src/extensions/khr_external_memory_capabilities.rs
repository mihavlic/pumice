#[doc(alias = "VkExternalMemoryHandleTypeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagsKHR.html)
pub type ExternalMemoryHandleTypeFlagsKHR = crate::vk11::ExternalMemoryHandleTypeFlags;
#[doc(alias = "VkExternalMemoryFeatureFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagsKHR.html)
pub type ExternalMemoryFeatureFlagsKHR = crate::vk11::ExternalMemoryFeatureFlags;
#[doc(alias = "VkExternalMemoryPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryPropertiesKHR.html)
pub type ExternalMemoryPropertiesKHR = crate::vk11::ExternalMemoryProperties;
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfoKHR.html)
pub type PhysicalDeviceExternalImageFormatInfoKHR = crate::vk11::PhysicalDeviceExternalImageFormatInfo;
#[doc(alias = "VkExternalImageFormatPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesKHR.html)
pub type ExternalImageFormatPropertiesKHR = crate::vk11::ExternalImageFormatProperties;
#[doc(alias = "VkPhysicalDeviceExternalBufferInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfoKHR.html)
pub type PhysicalDeviceExternalBufferInfoKHR = crate::vk11::PhysicalDeviceExternalBufferInfo;
#[doc(alias = "VkExternalBufferPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalBufferPropertiesKHR.html)
pub type ExternalBufferPropertiesKHR = crate::vk11::ExternalBufferProperties;
#[doc(alias = "VkPhysicalDeviceIDPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDPropertiesKHR.html)
pub type PhysicalDeviceIDPropertiesKHR = crate::vk11::PhysicalDeviceIDProperties;
pub const LUID_SIZE_KHR: u32 = crate::vk11::LUID_SIZE;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
pub unsafe fn get_physical_device_external_buffer_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_buffer_properties
        .unwrap())(physical_device, p_external_buffer_info, p_external_buffer_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        external_buffer_info: &crate::vk11::PhysicalDeviceExternalBufferInfo,
    ) -> crate::vk11::ExternalBufferProperties {
        let get_physical_device_external_buffer_properties_khr = (*self.table)
            .get_physical_device_external_buffer_properties_khr
            .unwrap();
        let mut external_buffer_properties = Default::default();
        get_physical_device_external_buffer_properties_khr(
            physical_device,
            external_buffer_info as _,
            &mut external_buffer_properties,
        );
        external_buffer_properties
    }
}
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_memory_capabilities"
);
