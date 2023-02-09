#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html)
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
    pub pci_function: u32,
}
impl Default for PhysicalDevicePCIBusInfoPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            pci_domain: Default::default(),
            pci_bus: Default::default(),
            pci_device: Default::default(),
            pci_function: Default::default(),
        }
    }
}
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
pub const EXT_PCI_BUS_INFO_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_pci_bus_info"
);
