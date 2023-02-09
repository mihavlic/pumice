#[doc(alias = "VkDeviceMemoryReportFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceMemoryReportFlagsEXT(pub u32);
crate::bitflags_impl! {
    DeviceMemoryReportFlagsEXT : u32, 0x0,
}
#[doc(alias = "PFN_vkDeviceMemoryReportCallbackEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html)
pub type PfnDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut std::os::raw::c_void,
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html)
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_memory_report: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            device_memory_report: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceDeviceMemoryReportCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html)
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub pfn_user_callback: Option<PfnDeviceMemoryReportCallbackEXT>,
    pub p_user_data: *mut std::os::raw::c_void,
}
impl Default for DeviceDeviceMemoryReportCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pfn_user_callback: None,
            p_user_data: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html)
pub struct DeviceMemoryReportCallbackDataEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub kind: DeviceMemoryReportEventTypeEXT,
    pub memory_object_id: u64,
    pub size: crate::vk10::DeviceSize,
    pub object_type: crate::vk10::ObjectType,
    pub object_handle: u64,
    pub heap_index: u32,
}
impl Default for DeviceMemoryReportCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            kind: Default::default(),
            memory_object_id: Default::default(),
            size: Default::default(),
            object_type: Default::default(),
            object_handle: Default::default(),
            heap_index: Default::default(),
        }
    }
}
#[doc(alias = "VkDeviceMemoryReportEventTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceMemoryReportEventTypeEXT(pub i32);
impl DeviceMemoryReportEventTypeEXT {
    pub const ALLOCATE: Self = Self(0);
    pub const FREE: Self = Self(1);
    pub const IMPORT: Self = Self(2);
    pub const UNIMPORT: Self = Self(3);
    pub const ALLOCATION_FAILED: Self = Self(4);
}
crate::enum_impl! {
    DeviceMemoryReportEventTypeEXT : i32, ALLOCATE, FREE, IMPORT, UNIMPORT,
    ALLOCATION_FAILED
}
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
pub const EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_device_memory_report"
);
