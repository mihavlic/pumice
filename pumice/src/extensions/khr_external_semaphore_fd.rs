#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreFdInfoKHR.html)
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub flags: crate::vk11::SemaphoreImportFlags,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
    pub fd: std::os::raw::c_int,
}
impl Default for ImportSemaphoreFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetFdInfoKHR.html)
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
}
impl Default for SemaphoreGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreFdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html)
pub unsafe fn get_semaphore_fd_khr(
    device: crate::vk10::Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_fd_khr
        .unwrap())(device, p_get_fd_info, p_fd)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html)
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        get_fd_info: &SemaphoreGetFdInfoKHR,
    ) -> crate::VulkanResult<std::os::raw::c_int> {
        let get_semaphore_fd_khr = (*self.table).get_semaphore_fd_khr.unwrap();
        let mut fd = Default::default();
        let result = get_semaphore_fd_khr(self.handle, get_fd_info as _, &mut fd);
        crate::new_result(fd, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkImportSemaphoreFdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html)
pub unsafe fn import_semaphore_fd_khr(
    device: crate::vk10::Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .import_semaphore_fd_khr
        .unwrap())(device, p_import_semaphore_fd_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html)
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> crate::VulkanResult<()> {
        let import_semaphore_fd_khr = (*self.table).import_semaphore_fd_khr.unwrap();
        let result = import_semaphore_fd_khr(self.handle, import_semaphore_fd_info as _);
        crate::new_result((), result)
    }
}
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_semaphore_fd"
);
