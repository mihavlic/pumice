#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html)
pub struct ImportMemoryFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    pub fd: std::os::raw::c_int,
}
impl Default for ImportMemoryFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryFdPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html)
pub struct MemoryFdPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryFdPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryGetFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html)
pub struct MemoryGetFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
}
impl Default for MemoryGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryFdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html)
pub unsafe fn get_memory_fd_khr(
    device: crate::vk10::Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_fd_khr
        .unwrap())(device, p_get_fd_info, p_fd)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html)
    pub unsafe fn get_memory_fd_khr(
        &self,
        get_fd_info: &MemoryGetFdInfoKHR,
    ) -> crate::VulkanResult<std::os::raw::c_int> {
        let get_memory_fd_khr = (*self.table).get_memory_fd_khr.unwrap();
        let mut fd = Default::default();
        let result = get_memory_fd_khr(self.handle, get_fd_info as _, &mut fd);
        crate::new_result(fd, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
pub unsafe fn get_memory_fd_properties_khr(
    device: crate::vk10::Device,
    handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    fd: std::os::raw::c_int,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_fd_properties_khr
        .unwrap())(device, handle_type, fd, p_memory_fd_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        fd: std::os::raw::c_int,
    ) -> crate::VulkanResult<MemoryFdPropertiesKHR> {
        let get_memory_fd_properties_khr = (*self.table)
            .get_memory_fd_properties_khr
            .unwrap();
        let mut memory_fd_properties = Default::default();
        let result = get_memory_fd_properties_khr(
            self.handle,
            handle_type as _,
            fd as _,
            &mut memory_fd_properties,
        );
        crate::new_result(memory_fd_properties, result)
    }
}
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_memory_fd"
);
