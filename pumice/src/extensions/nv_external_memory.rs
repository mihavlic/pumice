#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html)
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalMemoryImageCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html)
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub handle_types: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExportMemoryAllocateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_external_memory"
);
