#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportFenceWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html)
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fence: crate::vk10::Fence,
    pub flags: crate::vk11::FenceImportFlags,
    pub handle_type: crate::vk11::ExternalFenceHandleTypeFlags,
    pub handle: crate::extensions::khr_win32_surface::HANDLE,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ImportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportFenceWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html)
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_attributes: *const crate::extensions::khr_win32_surface::SECURITY_ATTRIBUTES,
    pub dw_access: crate::extensions::khr_win32_surface::DWORD,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ExportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html)
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fence: crate::vk10::Fence,
    pub handle_type: crate::vk11::ExternalFenceHandleTypeFlags,
}
impl Default for FenceGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetFenceWin32HandleKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html)
pub unsafe fn get_fence_win_32_handle_khr(
    device: crate::vk10::Device,
    p_get_win_32_handle_info: *const FenceGetWin32HandleInfoKHR,
    p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_fence_win_32_handle_khr
        .unwrap())(device, p_get_win_32_handle_info, p_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html)
    pub unsafe fn get_fence_win_32_handle_khr(
        &self,
        get_win_32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> crate::VulkanResult<crate::extensions::khr_win32_surface::HANDLE> {
        let get_fence_win_32_handle_khr = (*self.table)
            .get_fence_win_32_handle_khr
            .unwrap();
        let mut handle = std::ptr::null_mut();
        let result = get_fence_win_32_handle_khr(
            self.handle,
            get_win_32_handle_info as _,
            &mut handle,
        );
        crate::new_result(handle, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkImportFenceWin32HandleKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html)
pub unsafe fn import_fence_win_32_handle_khr(
    device: crate::vk10::Device,
    p_import_fence_win_32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .import_fence_win_32_handle_khr
        .unwrap())(device, p_import_fence_win_32_handle_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html)
    pub unsafe fn import_fence_win_32_handle_khr(
        &self,
        import_fence_win_32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> crate::VulkanResult<()> {
        let import_fence_win_32_handle_khr = (*self.table)
            .import_fence_win_32_handle_khr
            .unwrap();
        let result = import_fence_win_32_handle_khr(
            self.handle,
            import_fence_win_32_handle_info as _,
        );
        crate::new_result((), result)
    }
}
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_fence_win32"
);
