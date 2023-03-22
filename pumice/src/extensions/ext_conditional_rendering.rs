#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html)
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}
impl Default for ConditionalRenderingBeginInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            offset: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html)
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub conditional_rendering_enable: crate::vk10::Bool32,
}
impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
            p_next: std::ptr::null(),
            conditional_rendering_enable: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html)
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub conditional_rendering: crate::vk10::Bool32,
    pub inherited_conditional_rendering: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            conditional_rendering: Default::default(),
            inherited_conditional_rendering: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ConditionalRenderingFlagsEXT(pub u32);
impl ConditionalRenderingFlagsEXT {
    pub const INVERTED: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    ConditionalRenderingFlagsEXT : u32, 0x1, INVERTED
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
pub unsafe fn cmd_begin_conditional_rendering_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_conditional_rendering_ext
        .unwrap())(command_buffer, p_conditional_rendering_begin)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) {
        let cmd_begin_conditional_rendering_ext = (*self.table)
            .cmd_begin_conditional_rendering_ext
            .unwrap();
        cmd_begin_conditional_rendering_ext(
            command_buffer,
            conditional_rendering_begin as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
pub unsafe fn cmd_end_conditional_rendering_ext(
    command_buffer: crate::vk10::CommandBuffer,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_conditional_rendering_ext
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
    pub unsafe fn cmd_end_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_end_conditional_rendering_ext = (*self.table)
            .cmd_end_conditional_rendering_ext
            .unwrap();
        cmd_end_conditional_rendering_ext(command_buffer);
    }
}
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_conditional_rendering"
);
