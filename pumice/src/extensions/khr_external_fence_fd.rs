#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportFenceFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html)
pub struct ImportFenceFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fence: crate::vk10::Fence,
    pub flags: crate::vk11::FenceImportFlags,
    pub handle_type: crate::vk11::ExternalFenceHandleTypeFlags,
    pub fd: std::os::raw::c_int,
}
impl Default for ImportFenceFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFenceGetFdInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetFdInfoKHR.html)
pub struct FenceGetFdInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fence: crate::vk10::Fence,
    pub handle_type: crate::vk11::ExternalFenceHandleTypeFlags,
}
impl Default for FenceGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetFenceFdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html)
pub unsafe fn get_fence_fd_khr(
    device: crate::vk10::Device,
    p_get_fd_info: *const FenceGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_fence_fd_khr
        .unwrap())(device, p_get_fd_info, p_fd)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetFenceFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html)
    pub unsafe fn get_fence_fd_khr(
        &self,
        get_fd_info: &FenceGetFdInfoKHR,
    ) -> crate::VulkanResult<std::os::raw::c_int> {
        let get_fence_fd_khr = (*self.table).get_fence_fd_khr.unwrap();
        let mut fd = Default::default();
        let result = get_fence_fd_khr(self.handle, get_fd_info as _, &mut fd);
        crate::new_result(fd, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkImportFenceFdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html)
pub unsafe fn import_fence_fd_khr(
    device: crate::vk10::Device,
    p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .import_fence_fd_khr
        .unwrap())(device, p_import_fence_fd_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkImportFenceFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html)
    pub unsafe fn import_fence_fd_khr(
        &self,
        import_fence_fd_info: &ImportFenceFdInfoKHR,
    ) -> crate::VulkanResult<()> {
        let import_fence_fd_khr = (*self.table).import_fence_fd_khr.unwrap();
        let result = import_fence_fd_khr(self.handle, import_fence_fd_info as _);
        crate::new_result((), result)
    }
}
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_fence_fd"
);
