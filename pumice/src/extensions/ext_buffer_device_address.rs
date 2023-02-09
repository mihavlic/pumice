#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html)
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub buffer_device_address: crate::vk10::Bool32,
    pub buffer_device_address_capture_replay: crate::vk10::Bool32,
    pub buffer_device_address_multi_device: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBufferAddressFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferAddressFeaturesEXT.html)
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[doc(alias = "VkBufferDeviceAddressInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfoEXT.html)
pub type BufferDeviceAddressInfoEXT = crate::vk12::BufferDeviceAddressInfo;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html)
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_address: crate::vk10::DeviceAddress,
}
impl Default for BufferDeviceAddressCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            device_address: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html)
pub unsafe fn get_buffer_device_address_ext(
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
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html)
    pub unsafe fn get_buffer_device_address_ext(
        &self,
        info: &crate::vk12::BufferDeviceAddressInfo,
    ) {
        let get_buffer_device_address_ext = (*self.table)
            .get_buffer_device_address_ext
            .unwrap();
        get_buffer_device_address_ext(self.handle, info as _);
    }
}
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_buffer_device_address"
);
