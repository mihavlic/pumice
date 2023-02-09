#[doc(alias = "VkBindBufferMemoryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfoKHR.html)
pub type BindBufferMemoryInfoKHR = crate::vk11::BindBufferMemoryInfo;
#[doc(alias = "VkBindImageMemoryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfoKHR.html)
pub type BindImageMemoryInfoKHR = crate::vk11::BindImageMemoryInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindBufferMemory2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html)
pub unsafe fn bind_buffer_memory_2_khr(
    device: crate::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_buffer_memory_2
        .unwrap())(device, bind_info_count, p_bind_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindBufferMemory2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html)
    pub unsafe fn bind_buffer_memory_2_khr(
        &self,
        bind_infos: &[crate::vk11::BindBufferMemoryInfo],
    ) -> crate::VulkanResult<()> {
        let bind_buffer_memory_2_khr = (*self.table).bind_buffer_memory_2_khr.unwrap();
        let bind_info_count = bind_infos.len();
        let result = bind_buffer_memory_2_khr(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindImageMemory2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html)
pub unsafe fn bind_image_memory_2_khr(
    device: crate::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_image_memory_2
        .unwrap())(device, bind_info_count, p_bind_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindImageMemory2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html)
    pub unsafe fn bind_image_memory_2_khr(
        &self,
        bind_infos: &[crate::vk11::BindImageMemoryInfo],
    ) -> crate::VulkanResult<()> {
        let bind_image_memory_2_khr = (*self.table).bind_image_memory_2_khr.unwrap();
        let bind_info_count = bind_infos.len();
        let result = bind_image_memory_2_khr(
            self.handle,
            bind_info_count as _,
            bind_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_bind_memory2"
);
