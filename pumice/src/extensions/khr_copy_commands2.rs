#[doc(alias = "VkBufferCopy2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2KHR.html)
pub type BufferCopy2KHR = crate::vk13::BufferCopy2;
#[doc(alias = "VkImageCopy2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy2KHR.html)
pub type ImageCopy2KHR = crate::vk13::ImageCopy2;
#[doc(alias = "VkImageBlit2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit2KHR.html)
pub type ImageBlit2KHR = crate::vk13::ImageBlit2;
#[doc(alias = "VkBufferImageCopy2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2KHR.html)
pub type BufferImageCopy2KHR = crate::vk13::BufferImageCopy2;
#[doc(alias = "VkImageResolve2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageResolve2KHR.html)
pub type ImageResolve2KHR = crate::vk13::ImageResolve2;
#[doc(alias = "VkCopyBufferInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2KHR.html)
pub type CopyBufferInfo2KHR = crate::vk13::CopyBufferInfo2;
#[doc(alias = "VkCopyImageInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2KHR.html)
pub type CopyImageInfo2KHR = crate::vk13::CopyImageInfo2;
#[doc(alias = "VkBlitImageInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2KHR.html)
pub type BlitImageInfo2KHR = crate::vk13::BlitImageInfo2;
#[doc(alias = "VkCopyBufferToImageInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2KHR.html)
pub type CopyBufferToImageInfo2KHR = crate::vk13::CopyBufferToImageInfo2;
#[doc(alias = "VkCopyImageToBufferInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2KHR.html)
pub type CopyImageToBufferInfo2KHR = crate::vk13::CopyImageToBufferInfo2;
#[doc(alias = "VkResolveImageInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveImageInfo2KHR.html)
pub type ResolveImageInfo2KHR = crate::vk13::ResolveImageInfo2;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBuffer2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html)
pub unsafe fn cmd_copy_buffer_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_buffer_info: *const crate::vk13::CopyBufferInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer_2
        .unwrap())(command_buffer, p_copy_buffer_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html)
    pub unsafe fn cmd_copy_buffer_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_buffer_info: &crate::vk13::CopyBufferInfo2,
    ) {
        let cmd_copy_buffer_2_khr = (*self.table).cmd_copy_buffer_2_khr.unwrap();
        cmd_copy_buffer_2_khr(command_buffer, copy_buffer_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImage2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html)
pub unsafe fn cmd_copy_image_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_image_info: *const crate::vk13::CopyImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image_2
        .unwrap())(command_buffer, p_copy_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html)
    pub unsafe fn cmd_copy_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_image_info: &crate::vk13::CopyImageInfo2,
    ) {
        let cmd_copy_image_2_khr = (*self.table).cmd_copy_image_2_khr.unwrap();
        cmd_copy_image_2_khr(command_buffer, copy_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBlitImage2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html)
pub unsafe fn cmd_blit_image_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_blit_image_info: *const crate::vk13::BlitImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_blit_image_2
        .unwrap())(command_buffer, p_blit_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html)
    pub unsafe fn cmd_blit_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        blit_image_info: &crate::vk13::BlitImageInfo2,
    ) {
        let cmd_blit_image_2_khr = (*self.table).cmd_blit_image_2_khr.unwrap();
        cmd_blit_image_2_khr(command_buffer, blit_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html)
pub unsafe fn cmd_copy_buffer_to_image_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_buffer_to_image_info: *const crate::vk13::CopyBufferToImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer_to_image_2
        .unwrap())(command_buffer, p_copy_buffer_to_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html)
    pub unsafe fn cmd_copy_buffer_to_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_buffer_to_image_info: &crate::vk13::CopyBufferToImageInfo2,
    ) {
        let cmd_copy_buffer_to_image_2_khr = (*self.table)
            .cmd_copy_buffer_to_image_2_khr
            .unwrap();
        cmd_copy_buffer_to_image_2_khr(command_buffer, copy_buffer_to_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html)
pub unsafe fn cmd_copy_image_to_buffer_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_image_to_buffer_info: *const crate::vk13::CopyImageToBufferInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image_to_buffer_2
        .unwrap())(command_buffer, p_copy_image_to_buffer_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html)
    pub unsafe fn cmd_copy_image_to_buffer_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_image_to_buffer_info: &crate::vk13::CopyImageToBufferInfo2,
    ) {
        let cmd_copy_image_to_buffer_2_khr = (*self.table)
            .cmd_copy_image_to_buffer_2_khr
            .unwrap();
        cmd_copy_image_to_buffer_2_khr(command_buffer, copy_image_to_buffer_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResolveImage2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html)
pub unsafe fn cmd_resolve_image_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_resolve_image_info: *const crate::vk13::ResolveImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_resolve_image_2
        .unwrap())(command_buffer, p_resolve_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html)
    pub unsafe fn cmd_resolve_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        resolve_image_info: &crate::vk13::ResolveImageInfo2,
    ) {
        let cmd_resolve_image_2_khr = (*self.table).cmd_resolve_image_2_khr.unwrap();
        cmd_resolve_image_2_khr(command_buffer, resolve_image_info as _);
    }
}
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
pub const KHR_COPY_COMMANDS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_copy_commands2"
);
