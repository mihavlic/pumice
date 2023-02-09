#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html)
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub flags: crate::vk11::SemaphoreImportFlags,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
    pub handle: crate::extensions::khr_win32_surface::HANDLE,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ImportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html)
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_attributes: *const crate::extensions::khr_win32_surface::SECURITY_ATTRIBUTES,
    pub dw_access: crate::extensions::khr_win32_surface::DWORD,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ExportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html)
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl Default for D3D12FenceSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_values_count: Default::default(),
            p_wait_semaphore_values: std::ptr::null(),
            signal_semaphore_values_count: Default::default(),
            p_signal_semaphore_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html)
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
}
impl Default for SemaphoreGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
pub unsafe fn get_semaphore_win_32_handle_khr(
    device: crate::vk10::Device,
    p_get_win_32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_win_32_handle_khr
        .unwrap())(device, p_get_win_32_handle_info, p_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
    pub unsafe fn get_semaphore_win_32_handle_khr(
        &self,
        get_win_32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    ) -> crate::VulkanResult<crate::extensions::khr_win32_surface::HANDLE> {
        let get_semaphore_win_32_handle_khr = (*self.table)
            .get_semaphore_win_32_handle_khr
            .unwrap();
        let mut handle = std::ptr::null_mut();
        let result = get_semaphore_win_32_handle_khr(
            self.handle,
            get_win_32_handle_info as _,
            &mut handle,
        );
        crate::new_result(handle, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
pub unsafe fn import_semaphore_win_32_handle_khr(
    device: crate::vk10::Device,
    p_import_semaphore_win_32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .import_semaphore_win_32_handle_khr
        .unwrap())(device, p_import_semaphore_win_32_handle_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
    pub unsafe fn import_semaphore_win_32_handle_khr(
        &self,
        import_semaphore_win_32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> crate::VulkanResult<()> {
        let import_semaphore_win_32_handle_khr = (*self.table)
            .import_semaphore_win_32_handle_khr
            .unwrap();
        let result = import_semaphore_win_32_handle_khr(
            self.handle,
            import_semaphore_win_32_handle_info as _,
        );
        crate::new_result((), result)
    }
}
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_semaphore_win32"
);
