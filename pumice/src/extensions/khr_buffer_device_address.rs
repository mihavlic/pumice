#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesKHR.html)
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR = crate::vk12::PhysicalDeviceBufferDeviceAddressFeatures;
#[doc(alias = "VkBufferDeviceAddressInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfoKHR.html)
pub type BufferDeviceAddressInfoKHR = crate::vk12::BufferDeviceAddressInfo;
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfoKHR.html)
pub type BufferOpaqueCaptureAddressCreateInfoKHR = crate::vk12::BufferOpaqueCaptureAddressCreateInfo;
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfoKHR.html)
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR = crate::vk12::MemoryOpaqueCaptureAddressAllocateInfo;
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfoKHR.html)
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR = crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html)
pub unsafe fn get_buffer_opaque_capture_address_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk12::BufferDeviceAddressInfo,
) -> u64 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_opaque_capture_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html)
    pub unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        info: &crate::vk12::BufferDeviceAddressInfo,
    ) {
        let get_buffer_opaque_capture_address_khr = (*self.table)
            .get_buffer_opaque_capture_address_khr
            .unwrap();
        get_buffer_opaque_capture_address_khr(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html)
pub unsafe fn get_buffer_device_address_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk12::BufferDeviceAddressInfo,
) -> crate::vk10::DeviceAddress {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_device_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html)
    pub unsafe fn get_buffer_device_address_khr(
        &self,
        info: &crate::vk12::BufferDeviceAddressInfo,
    ) {
        let get_buffer_device_address_khr = (*self.table)
            .get_buffer_device_address_khr
            .unwrap();
        get_buffer_device_address_khr(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
pub unsafe fn get_device_memory_opaque_capture_address_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_memory_opaque_capture_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        info: &crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
    ) {
        let get_device_memory_opaque_capture_address_khr = (*self.table)
            .get_device_memory_opaque_capture_address_khr
            .unwrap();
        get_device_memory_opaque_capture_address_khr(self.handle, info as _);
    }
}
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
pub const KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_buffer_device_address"
);
