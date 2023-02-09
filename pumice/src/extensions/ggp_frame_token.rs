#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentFrameTokenGGP")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentFrameTokenGGP.html)
pub struct PresentFrameTokenGGP {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub frame_token: crate::extensions::ggp_stream_descriptor_surface::GgpFrameToken,
}
impl Default for PresentFrameTokenGGP {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRESENT_FRAME_TOKEN_GGP,
            p_next: std::ptr::null(),
            frame_token: Default::default(),
        }
    }
}
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
pub const GGP_FRAME_TOKEN_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_GGP_frame_token"
);
