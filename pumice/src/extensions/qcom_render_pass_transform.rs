#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassTransformBeginInfoQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html)
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
}
impl Default for RenderPassTransformBeginInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceRenderPassTransformInfoQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html)
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub render_area: crate::vk10::Rect2D,
}
impl Default for CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
            render_area: Default::default(),
        }
    }
}
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 3;
pub const QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QCOM_render_pass_transform"
);
