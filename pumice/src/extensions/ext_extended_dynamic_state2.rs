#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html)
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub extended_dynamic_state_2: crate::vk10::Bool32,
    pub extended_dynamic_state_2_logic_op: crate::vk10::Bool32,
    pub extended_dynamic_state_2_patch_control_points: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state_2: Default::default(),
            extended_dynamic_state_2_logic_op: Default::default(),
            extended_dynamic_state_2_patch_control_points: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html)
pub unsafe fn cmd_set_patch_control_points_ext(
    command_buffer: crate::vk10::CommandBuffer,
    patch_control_points: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_patch_control_points_ext
        .unwrap())(command_buffer, patch_control_points)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html)
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        patch_control_points: u32,
    ) {
        let cmd_set_patch_control_points_ext = (*self.table)
            .cmd_set_patch_control_points_ext
            .unwrap();
        cmd_set_patch_control_points_ext(command_buffer, patch_control_points as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html)
pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    rasterizer_discard_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_rasterizer_discard_enable
        .unwrap())(command_buffer, rasterizer_discard_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html)
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        let cmd_set_rasterizer_discard_enable_ext = (*self.table)
            .cmd_set_rasterizer_discard_enable_ext
            .unwrap();
        cmd_set_rasterizer_discard_enable_ext(
            command_buffer,
            rasterizer_discard_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html)
pub unsafe fn cmd_set_depth_bias_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_bias_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bias_enable
        .unwrap())(command_buffer, depth_bias_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html)
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_enable: bool,
    ) {
        let cmd_set_depth_bias_enable_ext = (*self.table)
            .cmd_set_depth_bias_enable_ext
            .unwrap();
        cmd_set_depth_bias_enable_ext(command_buffer, depth_bias_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLogicOpEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html)
pub unsafe fn cmd_set_logic_op_ext(
    command_buffer: crate::vk10::CommandBuffer,
    logic_op: crate::vk10::LogicOp,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_logic_op_ext
        .unwrap())(command_buffer, logic_op)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html)
    pub unsafe fn cmd_set_logic_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        logic_op: crate::vk10::LogicOp,
    ) {
        let cmd_set_logic_op_ext = (*self.table).cmd_set_logic_op_ext.unwrap();
        cmd_set_logic_op_ext(command_buffer, logic_op as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html)
pub unsafe fn cmd_set_primitive_restart_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    primitive_restart_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_primitive_restart_enable
        .unwrap())(command_buffer, primitive_restart_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html)
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        let cmd_set_primitive_restart_enable_ext = (*self.table)
            .cmd_set_primitive_restart_enable_ext
            .unwrap();
        cmd_set_primitive_restart_enable_ext(
            command_buffer,
            primitive_restart_enable as _,
        );
    }
}
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_extended_dynamic_state2"
);
