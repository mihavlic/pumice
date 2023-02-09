#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceAddressBindingReportFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAddressBindingReportFeaturesEXT.html)
pub struct PhysicalDeviceAddressBindingReportFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub report_address_binding: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceAddressBindingReportFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            report_address_binding: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceAddressBindingCallbackDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingCallbackDataEXT.html)
pub struct DeviceAddressBindingCallbackDataEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: DeviceAddressBindingFlagsEXT,
    pub base_address: crate::vk10::DeviceAddress,
    pub size: crate::vk10::DeviceSize,
    pub binding_type: DeviceAddressBindingTypeEXT,
}
impl Default for DeviceAddressBindingCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            base_address: Default::default(),
            size: Default::default(),
            binding_type: Default::default(),
        }
    }
}
#[doc(alias = "VkDeviceAddressBindingFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceAddressBindingFlagsEXT(pub u32);
impl DeviceAddressBindingFlagsEXT {
    pub const INTERNAL_OBJECT: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    DeviceAddressBindingFlagsEXT : u32, 0x1, INTERNAL_OBJECT
}
#[doc(alias = "VkDeviceAddressBindingTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceAddressBindingTypeEXT(pub i32);
impl DeviceAddressBindingTypeEXT {
    pub const BIND: Self = Self(0);
    pub const UNBIND: Self = Self(1);
}
crate::enum_impl! {
    DeviceAddressBindingTypeEXT : i32, BIND, UNBIND
}
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION: u32 = 1;
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_device_address_binding_report"
);
