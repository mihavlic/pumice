#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
pub unsafe fn cmd_write_buffer_marker_amd(
    command_buffer: crate::vk10::CommandBuffer,
    pipeline_stage: crate::vk10::PipelineStageFlags,
    dst_buffer: crate::vk10::Buffer,
    dst_offset: crate::vk10::DeviceSize,
    marker: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_buffer_marker_amd
        .unwrap())(command_buffer, pipeline_stage, dst_buffer, dst_offset, marker)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stage: Option<crate::vk10::PipelineStageFlags>,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        marker: u32,
    ) {
        let cmd_write_buffer_marker_amd = (*self.table)
            .cmd_write_buffer_marker_amd
            .unwrap();
        cmd_write_buffer_marker_amd(
            command_buffer,
            match pipeline_stage {
                Some(v) => v,
                None => Default::default(),
            },
            dst_buffer,
            dst_offset as _,
            marker as _,
        );
    }
}
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_buffer_marker"
);
