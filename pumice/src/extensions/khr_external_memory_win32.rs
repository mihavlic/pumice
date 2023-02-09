#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html)
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    pub handle: crate::extensions::khr_win32_surface::HANDLE,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ImportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMemoryWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html)
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_attributes: *const crate::extensions::khr_win32_surface::SECURITY_ATTRIBUTES,
    pub dw_access: crate::extensions::khr_win32_surface::DWORD,
    pub name: crate::extensions::khr_win32_surface::LPCWSTR,
}
impl Default for ExportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            p_attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryWin32HandlePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html)
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryWin32HandlePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryGetWin32HandleInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html)
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
}
impl Default for MemoryGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
pub unsafe fn get_memory_win_32_handle_khr(
    device: crate::vk10::Device,
    p_get_win_32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_win_32_handle_khr
        .unwrap())(device, p_get_win_32_handle_info, p_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
    pub unsafe fn get_memory_win_32_handle_khr(
        &self,
        get_win_32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> crate::VulkanResult<crate::extensions::khr_win32_surface::HANDLE> {
        let get_memory_win_32_handle_khr = (*self.table)
            .get_memory_win_32_handle_khr
            .unwrap();
        let mut handle = std::ptr::null_mut();
        let result = get_memory_win_32_handle_khr(
            self.handle,
            get_win_32_handle_info as _,
            &mut handle,
        );
        crate::new_result(handle, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
pub unsafe fn get_memory_win_32_handle_properties_khr(
    device: crate::vk10::Device,
    handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    handle: crate::extensions::khr_win32_surface::HANDLE,
    p_memory_win_32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_win_32_handle_properties_khr
        .unwrap())(device, handle_type, handle, p_memory_win_32_handle_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
    pub unsafe fn get_memory_win_32_handle_properties_khr(
        &self,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        handle: crate::extensions::khr_win32_surface::HANDLE,
    ) -> crate::VulkanResult<MemoryWin32HandlePropertiesKHR> {
        let get_memory_win_32_handle_properties_khr = (*self.table)
            .get_memory_win_32_handle_properties_khr
            .unwrap();
        let mut memory_win_32_handle_properties = Default::default();
        let result = get_memory_win_32_handle_properties_khr(
            self.handle,
            handle_type as _,
            handle,
            &mut memory_win_32_handle_properties,
        );
        crate::new_result(memory_win_32_handle_properties, result)
    }
}
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
pub const KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_external_memory_win32"
);
