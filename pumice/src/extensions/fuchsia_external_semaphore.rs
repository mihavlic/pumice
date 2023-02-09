#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html)
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub flags: crate::vk11::SemaphoreImportFlags,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
    pub zircon_handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
}
impl Default for ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            zircon_handle: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html)
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub handle_type: crate::vk11::ExternalSemaphoreHandleTypeFlags,
}
impl Default for SemaphoreGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html)
pub unsafe fn get_semaphore_zircon_handle_fuchsia(
    device: crate::vk10::Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_zircon_handle_fuchsia
        .unwrap())(device, p_get_zircon_handle_info, p_zircon_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html)
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> crate::VulkanResult<crate::extensions::fuchsia_imagepipe_surface::zx_handle_t> {
        let get_semaphore_zircon_handle_fuchsia = (*self.table)
            .get_semaphore_zircon_handle_fuchsia
            .unwrap();
        let mut zircon_handle = std::ptr::null_mut();
        let result = get_semaphore_zircon_handle_fuchsia(
            self.handle,
            get_zircon_handle_info as _,
            &mut zircon_handle,
        );
        crate::new_result(zircon_handle, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html)
pub unsafe fn import_semaphore_zircon_handle_fuchsia(
    device: crate::vk10::Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .import_semaphore_zircon_handle_fuchsia
        .unwrap())(device, p_import_semaphore_zircon_handle_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html)
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> crate::VulkanResult<()> {
        let import_semaphore_zircon_handle_fuchsia = (*self.table)
            .import_semaphore_zircon_handle_fuchsia
            .unwrap();
        let result = import_semaphore_zircon_handle_fuchsia(
            self.handle,
            import_semaphore_zircon_handle_info as _,
        );
        crate::new_result((), result)
    }
}
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_FUCHSIA_external_semaphore"
);
