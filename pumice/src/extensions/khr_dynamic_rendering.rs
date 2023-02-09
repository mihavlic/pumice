#[doc(alias = "VkRenderingFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagsKHR.html)
pub type RenderingFlagsKHR = crate::vk13::RenderingFlags;
#[doc(alias = "VkPipelineRenderingCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfoKHR.html)
pub type PipelineRenderingCreateInfoKHR = crate::vk13::PipelineRenderingCreateInfo;
#[doc(alias = "VkRenderingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingInfoKHR.html)
pub type RenderingInfoKHR = crate::vk13::RenderingInfo;
#[doc(alias = "VkRenderingAttachmentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfoKHR.html)
pub type RenderingAttachmentInfoKHR = crate::vk13::RenderingAttachmentInfo;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html)
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_view: crate::vk10::ImageView,
    pub image_layout: crate::vk10::ImageLayout,
    pub shading_rate_attachment_texel_size: crate::vk10::Extent2D,
}
impl Default for RenderingFragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            shading_rate_attachment_texel_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html)
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_view: crate::vk10::ImageView,
    pub image_layout: crate::vk10::ImageLayout,
}
impl Default for RenderingFragmentDensityMapAttachmentInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html)
pub type PhysicalDeviceDynamicRenderingFeaturesKHR = crate::vk13::PhysicalDeviceDynamicRenderingFeatures;
#[doc(alias = "VkCommandBufferInheritanceRenderingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfoKHR.html)
pub type CommandBufferInheritanceRenderingInfoKHR = crate::vk13::CommandBufferInheritanceRenderingInfo;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html)
pub struct AttachmentSampleCountInfoAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_samples: *const crate::vk10::SampleCountFlags,
    pub depth_stencil_attachment_samples: crate::vk10::SampleCountFlags,
}
impl Default for AttachmentSampleCountInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
            p_next: std::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachment_samples: std::ptr::null(),
            depth_stencil_attachment_samples: Default::default(),
        }
    }
}
#[doc(alias = "VkAttachmentSampleCountInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoNV.html)
pub type AttachmentSampleCountInfoNV = AttachmentSampleCountInfoAMD;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html)
pub struct MultiviewPerViewAttributesInfoNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub per_view_attributes: crate::vk10::Bool32,
    pub per_view_attributes_position_xonly: crate::vk10::Bool32,
}
impl Default for MultiviewPerViewAttributesInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
            p_next: std::ptr::null(),
            per_view_attributes: Default::default(),
            per_view_attributes_position_xonly: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
pub unsafe fn cmd_begin_rendering_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_rendering_info: *const crate::vk13::RenderingInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_rendering
        .unwrap())(command_buffer, p_rendering_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rendering_info: &crate::vk13::RenderingInfo,
    ) {
        let cmd_begin_rendering_khr = (*self.table).cmd_begin_rendering_khr.unwrap();
        cmd_begin_rendering_khr(command_buffer, rendering_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
pub unsafe fn cmd_end_rendering_khr(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_rendering
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
    pub unsafe fn cmd_end_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_end_rendering_khr = (*self.table).cmd_end_rendering_khr.unwrap();
        cmd_end_rendering_khr(command_buffer);
    }
}
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_dynamic_rendering"
);
