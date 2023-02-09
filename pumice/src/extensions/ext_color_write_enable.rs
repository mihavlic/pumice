#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html)
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub color_write_enable: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            color_write_enable: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html)
pub struct PipelineColorWriteCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_count: u32,
    pub p_color_write_enables: *const crate::vk10::Bool32,
}
impl Default for PipelineColorWriteCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            attachment_count: Default::default(),
            p_color_write_enables: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html)
pub unsafe fn cmd_set_color_write_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_color_write_enable_ext
        .unwrap())(command_buffer, attachment_count, p_color_write_enables)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html)
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        color_write_enables: &[crate::vk10::Bool32],
    ) {
        let cmd_set_color_write_enable_ext = (*self.table)
            .cmd_set_color_write_enable_ext
            .unwrap();
        let attachment_count = color_write_enables.len();
        cmd_set_color_write_enable_ext(
            command_buffer,
            attachment_count as _,
            color_write_enables.as_ptr(),
        );
    }
}
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_color_write_enable"
);
