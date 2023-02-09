#[doc(alias = "VkBufferMemoryRequirementsInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2KHR.html)
pub type BufferMemoryRequirementsInfo2KHR = crate::vk11::BufferMemoryRequirementsInfo2;
#[doc(alias = "VkImageMemoryRequirementsInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2KHR.html)
pub type ImageMemoryRequirementsInfo2KHR = crate::vk11::ImageMemoryRequirementsInfo2;
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2KHR.html)
pub type ImageSparseMemoryRequirementsInfo2KHR = crate::vk11::ImageSparseMemoryRequirementsInfo2;
#[doc(alias = "VkMemoryRequirements2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2KHR.html)
pub type MemoryRequirements2KHR = crate::vk11::MemoryRequirements2;
#[doc(alias = "VkSparseImageMemoryRequirements2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2KHR.html)
pub type SparseImageMemoryRequirements2KHR = crate::vk11::SparseImageMemoryRequirements2;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html)
pub unsafe fn get_buffer_memory_requirements_2_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_memory_requirements_2
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html)
    pub unsafe fn get_buffer_memory_requirements_2_khr(
        &self,
        info: &crate::vk11::BufferMemoryRequirementsInfo2,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_buffer_memory_requirements_2_khr = (*self.table)
            .get_buffer_memory_requirements_2_khr
            .unwrap();
        let mut memory_requirements = Default::default();
        get_buffer_memory_requirements_2_khr(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html)
pub unsafe fn get_image_memory_requirements_2_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_memory_requirements_2
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html)
    pub unsafe fn get_image_memory_requirements_2_khr(
        &self,
        info: &crate::vk11::ImageMemoryRequirementsInfo2,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_image_memory_requirements_2_khr = (*self.table)
            .get_image_memory_requirements_2_khr
            .unwrap();
        let mut memory_requirements = Default::default();
        get_image_memory_requirements_2_khr(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html)
pub unsafe fn get_image_sparse_memory_requirements_2_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_sparse_memory_requirements_2
        .unwrap())(
        device,
        p_info,
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html)
    pub unsafe fn get_image_sparse_memory_requirements_2_khr(
        &self,
        info: &crate::vk11::ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirement_count: Option<u32>,
        mut sparse_memory_requirements_callback: impl FnMut(
            &mut Vec<crate::vk11::SparseImageMemoryRequirements2>,
        ),
    ) -> Vec<crate::vk11::SparseImageMemoryRequirements2> {
        let get_image_sparse_memory_requirements_2_khr = (*self.table)
            .get_image_sparse_memory_requirements_2_khr
            .unwrap();
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_image_sparse_memory_requirements_2_khr(
                    self.handle,
                    info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut sparse_memory_requirements = vec![
            Default::default(); sparse_memory_requirement_count as usize
        ];
        sparse_memory_requirements_callback(&mut sparse_memory_requirements);
        get_image_sparse_memory_requirements_2_khr(
            self.handle,
            info as _,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
pub const KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_get_memory_requirements2"
);
