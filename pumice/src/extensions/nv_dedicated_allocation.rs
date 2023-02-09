#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html)
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub dedicated_allocation: crate::vk10::Bool32,
}
impl Default for DedicatedAllocationImageCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            dedicated_allocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html)
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub dedicated_allocation: crate::vk10::Bool32,
}
impl Default for DedicatedAllocationBufferCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            dedicated_allocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html)
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
    pub buffer: crate::vk10::Buffer,
}
impl Default for DedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_dedicated_allocation"
);
