#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html)
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    pub handle: crate::extensions::khr_win32_surface::HANDLE,
}
impl Default for ImportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html)
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_attributes: *const crate::extensions::khr_win32_surface::SECURITY_ATTRIBUTES,
    pub dw_access: crate::extensions::khr_win32_surface::DWORD,
}
impl Default for ExportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryWin32HandleNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html)
pub unsafe fn get_memory_win_32_handle_nv(
    device: crate::vk10::Device,
    memory: crate::vk10::DeviceMemory,
    handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_win_32_handle_nv
        .unwrap())(device, memory, handle_type, p_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html)
    pub unsafe fn get_memory_win_32_handle_nv(
        &self,
        memory: crate::vk10::DeviceMemory,
        handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::VulkanResult<crate::extensions::khr_win32_surface::HANDLE> {
        let get_memory_win_32_handle_nv = (*self.table)
            .get_memory_win_32_handle_nv
            .unwrap();
        let mut handle = std::ptr::null_mut();
        let result = get_memory_win_32_handle_nv(
            self.handle,
            memory,
            handle_type as _,
            &mut handle,
        );
        crate::new_result(handle, result)
    }
}
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_external_memory_win32"
);
