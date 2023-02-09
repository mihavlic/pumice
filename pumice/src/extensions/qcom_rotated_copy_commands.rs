#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyCommandTransformInfoQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyCommandTransformInfoQCOM.html)
pub struct CopyCommandTransformInfoQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
}
impl Default for CopyCommandTransformInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM,
            p_next: std::ptr::null(),
            transform: Default::default(),
        }
    }
}
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 1;
pub const QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QCOM_rotated_copy_commands"
);
