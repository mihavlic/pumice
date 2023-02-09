#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html)
pub unsafe fn cmd_draw_indirect_count_khr(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indirect_count
        .unwrap())(
        command_buffer,
        buffer,
        offset,
        count_buffer,
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html)
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indirect_count_khr = (*self.table)
            .cmd_draw_indirect_count_khr
            .unwrap();
        cmd_draw_indirect_count_khr(
            command_buffer,
            buffer,
            offset as _,
            count_buffer,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html)
pub unsafe fn cmd_draw_indexed_indirect_count_khr(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_indexed_indirect_count
        .unwrap())(
        command_buffer,
        buffer,
        offset,
        count_buffer,
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html)
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indexed_indirect_count_khr = (*self.table)
            .cmd_draw_indexed_indirect_count_khr
            .unwrap();
        cmd_draw_indexed_indirect_count_khr(
            command_buffer,
            buffer,
            offset as _,
            count_buffer,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
    }
}
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
pub const KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_draw_indirect_count"
);
