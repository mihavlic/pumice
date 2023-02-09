#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountAMD.html)
pub unsafe fn cmd_draw_indirect_count_amd(
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
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountAMD.html)
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indirect_count_amd = (*self.table)
            .cmd_draw_indirect_count_amd
            .unwrap();
        cmd_draw_indirect_count_amd(
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
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html)
pub unsafe fn cmd_draw_indexed_indirect_count_amd(
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
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html)
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let cmd_draw_indexed_indirect_count_amd = (*self.table)
            .cmd_draw_indexed_indirect_count_amd
            .unwrap();
        cmd_draw_indexed_indirect_count_amd(
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
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
pub const AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_draw_indirect_count"
);
