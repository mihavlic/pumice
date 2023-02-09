#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html)
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub external_memory_rdma: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            external_memory_rdma: Default::default(),
        }
    }
}
#[doc(alias = "VkRemoteAddressNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRemoteAddressNV.html)
pub type RemoteAddressNV = std::os::raw::c_void;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html)
pub struct MemoryGetRemoteAddressInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
}
impl Default for MemoryGetRemoteAddressInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html)
pub unsafe fn get_memory_remote_address_nv(
    device: crate::vk10::Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    p_address: *mut RemoteAddressNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_remote_address_nv
        .unwrap())(device, p_memory_get_remote_address_info, p_address)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html)
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
        address: *mut RemoteAddressNV,
    ) -> crate::VulkanResult<()> {
        let get_memory_remote_address_nv = (*self.table)
            .get_memory_remote_address_nv
            .unwrap();
        let result = get_memory_remote_address_nv(
            self.handle,
            memory_get_remote_address_info as _,
            address,
        );
        crate::new_result((), result)
    }
}
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_external_memory_rdma"
);
