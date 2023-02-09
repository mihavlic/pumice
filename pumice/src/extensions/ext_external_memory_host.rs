#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html)
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    pub p_host_pointer: *mut std::os::raw::c_void,
}
impl Default for ImportMemoryHostPointerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            p_host_pointer: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html)
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryHostPointerPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html)
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_imported_host_pointer_alignment: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_imported_host_pointer_alignment: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
pub unsafe fn get_memory_host_pointer_properties_ext(
    device: crate::vk10::Device,
    handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
    p_host_pointer: *const std::os::raw::c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_host_pointer_properties_ext
        .unwrap())(device, handle_type, p_host_pointer, p_memory_host_pointer_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        host_pointer: *const std::os::raw::c_void,
    ) -> crate::VulkanResult<MemoryHostPointerPropertiesEXT> {
        let get_memory_host_pointer_properties_ext = (*self.table)
            .get_memory_host_pointer_properties_ext
            .unwrap();
        let mut memory_host_pointer_properties = Default::default();
        let result = get_memory_host_pointer_properties_ext(
            self.handle,
            handle_type as _,
            host_pointer,
            &mut memory_host_pointer_properties,
        );
        crate::new_result(memory_host_pointer_properties, result)
    }
}
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
pub const EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_external_memory_host"
);
