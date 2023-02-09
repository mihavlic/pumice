#[doc(alias = "VkAttachmentDescription2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2KHR.html)
pub type AttachmentDescription2KHR = crate::vk12::AttachmentDescription2;
#[doc(alias = "VkAttachmentReference2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2KHR.html)
pub type AttachmentReference2KHR = crate::vk12::AttachmentReference2;
#[doc(alias = "VkSubpassDescription2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2KHR.html)
pub type SubpassDescription2KHR = crate::vk12::SubpassDescription2;
#[doc(alias = "VkSubpassDependency2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2KHR.html)
pub type SubpassDependency2KHR = crate::vk12::SubpassDependency2;
#[doc(alias = "VkRenderPassCreateInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2KHR.html)
pub type RenderPassCreateInfo2KHR = crate::vk12::RenderPassCreateInfo2;
#[doc(alias = "VkSubpassBeginInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfoKHR.html)
pub type SubpassBeginInfoKHR = crate::vk12::SubpassBeginInfo;
#[doc(alias = "VkSubpassEndInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfoKHR.html)
pub type SubpassEndInfoKHR = crate::vk12::SubpassEndInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
pub unsafe fn create_render_pass_2_khr(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk12::RenderPassCreateInfo2,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_render_pass: *mut crate::vk10::RenderPass,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_render_pass_2
        .unwrap())(device, p_create_info, p_allocator, p_render_pass)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
    pub unsafe fn create_render_pass_2_khr(
        &self,
        create_info: &crate::vk12::RenderPassCreateInfo2,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk10::RenderPass> {
        let create_render_pass_2_khr = (*self.table).create_render_pass_2_khr.unwrap();
        let mut render_pass = Default::default();
        let result = create_render_pass_2_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut render_pass,
        );
        crate::new_result(render_pass, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
pub unsafe fn cmd_begin_render_pass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
    p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_render_pass_2
        .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
    pub unsafe fn cmd_begin_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        render_pass_begin: &crate::vk10::RenderPassBeginInfo,
        subpass_begin_info: &crate::vk12::SubpassBeginInfo,
    ) {
        let cmd_begin_render_pass_2_khr = (*self.table)
            .cmd_begin_render_pass_2_khr
            .unwrap();
        cmd_begin_render_pass_2_khr(
            command_buffer,
            render_pass_begin as _,
            subpass_begin_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdNextSubpass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
pub unsafe fn cmd_next_subpass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
    p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_next_subpass_2
        .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
    pub unsafe fn cmd_next_subpass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_begin_info: &crate::vk12::SubpassBeginInfo,
        subpass_end_info: &crate::vk12::SubpassEndInfo,
    ) {
        let cmd_next_subpass_2_khr = (*self.table).cmd_next_subpass_2_khr.unwrap();
        cmd_next_subpass_2_khr(
            command_buffer,
            subpass_begin_info as _,
            subpass_end_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
pub unsafe fn cmd_end_render_pass_2_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_render_pass_2
        .unwrap())(command_buffer, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
    pub unsafe fn cmd_end_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_end_info: &crate::vk12::SubpassEndInfo,
    ) {
        let cmd_end_render_pass_2_khr = (*self.table).cmd_end_render_pass_2_khr.unwrap();
        cmd_end_render_pass_2_khr(command_buffer, subpass_end_info as _);
    }
}
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
pub const KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_create_renderpass2"
);
