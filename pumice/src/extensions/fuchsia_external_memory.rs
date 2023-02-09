#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html)
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    pub handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
}
impl Default for ImportMemoryZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html)
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryZirconHandlePropertiesFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html)
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
}
impl Default for MemoryGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html)
pub unsafe fn get_memory_zircon_handle_fuchsia(
    device: crate::vk10::Device,
    p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_zircon_handle_fuchsia
        .unwrap())(device, p_get_zircon_handle_info, p_zircon_handle)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html)
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> crate::VulkanResult<crate::extensions::fuchsia_imagepipe_surface::zx_handle_t> {
        let get_memory_zircon_handle_fuchsia = (*self.table)
            .get_memory_zircon_handle_fuchsia
            .unwrap();
        let mut zircon_handle = std::ptr::null_mut();
        let result = get_memory_zircon_handle_fuchsia(
            self.handle,
            get_zircon_handle_info as _,
            &mut zircon_handle,
        );
        crate::new_result(zircon_handle, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
    device: crate::vk10::Device,
    handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    zircon_handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
    p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_zircon_handle_properties_fuchsia
        .unwrap())(device, handle_type, zircon_handle, p_memory_zircon_handle_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        zircon_handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
    ) -> crate::VulkanResult<MemoryZirconHandlePropertiesFUCHSIA> {
        let get_memory_zircon_handle_properties_fuchsia = (*self.table)
            .get_memory_zircon_handle_properties_fuchsia
            .unwrap();
        let mut memory_zircon_handle_properties = Default::default();
        let result = get_memory_zircon_handle_properties_fuchsia(
            self.handle,
            handle_type as _,
            zircon_handle,
            &mut memory_zircon_handle_properties,
        );
        crate::new_result(memory_zircon_handle_properties, result)
    }
}
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_FUCHSIA_external_memory"
);
