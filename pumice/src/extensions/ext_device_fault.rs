#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFaultFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFaultFeaturesEXT.html)
pub struct PhysicalDeviceFaultFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_fault: crate::vk10::Bool32,
    pub device_fault_vendor_binary: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFaultFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            device_fault: Default::default(),
            device_fault_vendor_binary: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDeviceFaultAddressInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressInfoEXT.html)
pub struct DeviceFaultAddressInfoEXT {
    pub address_type: DeviceFaultAddressTypeEXT,
    pub reported_address: crate::vk10::DeviceAddress,
    pub address_precision: crate::vk10::DeviceSize,
}
impl Default for DeviceFaultAddressInfoEXT {
    fn default() -> Self {
        Self {
            address_type: Default::default(),
            reported_address: Default::default(),
            address_precision: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDeviceFaultVendorInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorInfoEXT.html)
pub struct DeviceFaultVendorInfoEXT {
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub vendor_fault_code: u64,
    pub vendor_fault_data: u64,
}
impl Default for DeviceFaultVendorInfoEXT {
    fn default() -> Self {
        Self {
            description: unsafe { std::mem::zeroed() },
            vendor_fault_code: Default::default(),
            vendor_fault_data: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceFaultCountsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultCountsEXT.html)
pub struct DeviceFaultCountsEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub address_info_count: u32,
    pub vendor_info_count: u32,
    pub vendor_binary_size: crate::vk10::DeviceSize,
}
impl Default for DeviceFaultCountsEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_FAULT_COUNTS_EXT,
            p_next: std::ptr::null_mut(),
            address_info_count: Default::default(),
            vendor_info_count: Default::default(),
            vendor_binary_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceFaultInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultInfoEXT.html)
pub struct DeviceFaultInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub p_address_infos: *mut DeviceFaultAddressInfoEXT,
    pub p_vendor_infos: *mut DeviceFaultVendorInfoEXT,
    pub p_vendor_binary_data: *mut std::os::raw::c_void,
}
impl Default for DeviceFaultInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_FAULT_INFO_EXT,
            p_next: std::ptr::null_mut(),
            description: unsafe { std::mem::zeroed() },
            p_address_infos: std::ptr::null_mut(),
            p_vendor_infos: std::ptr::null_mut(),
            p_vendor_binary_data: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDeviceFaultVendorBinaryHeaderVersionOneEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionOneEXT.html)
pub struct DeviceFaultVendorBinaryHeaderVersionOneEXT {
    pub header_size: u32,
    pub header_version: DeviceFaultVendorBinaryHeaderVersionEXT,
    pub vendor_id: u32,
    pub device_id: u32,
    pub driver_version: u32,
    pub pipeline_cache_uuid: [u8; crate::vk10::UUID_SIZE as usize],
    pub application_name_offset: u32,
    pub application_version: u32,
    pub engine_name_offset: u32,
}
impl Default for DeviceFaultVendorBinaryHeaderVersionOneEXT {
    fn default() -> Self {
        Self {
            header_size: Default::default(),
            header_version: Default::default(),
            vendor_id: Default::default(),
            device_id: Default::default(),
            driver_version: Default::default(),
            pipeline_cache_uuid: unsafe { std::mem::zeroed() },
            application_name_offset: Default::default(),
            application_version: Default::default(),
            engine_name_offset: Default::default(),
        }
    }
}
#[doc(alias = "VkDeviceFaultAddressTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceFaultAddressTypeEXT(pub i32);
impl DeviceFaultAddressTypeEXT {
    pub const NONE: Self = Self(0);
    pub const READ_INVALID: Self = Self(1);
    pub const WRITE_INVALID: Self = Self(2);
    pub const EXECUTE_INVALID: Self = Self(3);
    pub const INSTRUCTION_POINTER_UNKNOWN: Self = Self(4);
    pub const INSTRUCTION_POINTER_INVALID: Self = Self(5);
    pub const INSTRUCTION_POINTER_FAULT: Self = Self(6);
}
crate::enum_impl! {
    DeviceFaultAddressTypeEXT : i32, NONE, READ_INVALID, WRITE_INVALID, EXECUTE_INVALID,
    INSTRUCTION_POINTER_UNKNOWN, INSTRUCTION_POINTER_INVALID, INSTRUCTION_POINTER_FAULT
}
#[doc(alias = "VkDeviceFaultVendorBinaryHeaderVersionEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceFaultVendorBinaryHeaderVersionEXT(pub i32);
impl DeviceFaultVendorBinaryHeaderVersionEXT {
    pub const ONE: Self = Self(1);
}
crate::enum_impl! {
    DeviceFaultVendorBinaryHeaderVersionEXT : i32, ONE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceFaultInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html)
pub unsafe fn get_device_fault_info_ext(
    device: crate::vk10::Device,
    p_fault_counts: *mut DeviceFaultCountsEXT,
    p_fault_info: *mut DeviceFaultInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_fault_info_ext
        .unwrap())(device, p_fault_counts, p_fault_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeviceFaultInfoEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html)
    pub unsafe fn get_device_fault_info_ext(
        &self,
        fault_info: Option<&mut DeviceFaultInfoEXT>,
    ) -> crate::VulkanResult<(DeviceFaultCountsEXT, crate::vk10::Result)> {
        let get_device_fault_info_ext = (*self.table).get_device_fault_info_ext.unwrap();
        let mut fault_counts = Default::default();
        let result = get_device_fault_info_ext(
            self.handle,
            &mut fault_counts,
            match fault_info {
                Some(v) => v,
                None => std::ptr::null_mut(),
            },
        );
        crate::new_result((fault_counts, result), result)
    }
}
pub const EXT_DEVICE_FAULT_SPEC_VERSION: u32 = 1;
pub const EXT_DEVICE_FAULT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_device_fault"
);
