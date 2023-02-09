#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    PipelineDiscardRectangleStateCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html)
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_discard_rectangles: u32,
}
impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_discard_rectangles: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html)
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const crate::vk10::Rect2D,
}
impl Default for PipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangle_count: Default::default(),
            p_discard_rectangles: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkDiscardRectangleModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(pub i32);
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE: Self = Self(0);
    pub const EXCLUSIVE: Self = Self(1);
}
crate::enum_impl! {
    DiscardRectangleModeEXT : i32, INCLUSIVE, EXCLUSIVE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
pub unsafe fn cmd_set_discard_rectangle_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const crate::vk10::Rect2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_discard_rectangle_ext
        .unwrap())(
        command_buffer,
        first_discard_rectangle,
        discard_rectangle_count,
        p_discard_rectangles,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[crate::vk10::Rect2D],
    ) {
        let cmd_set_discard_rectangle_ext = (*self.table)
            .cmd_set_discard_rectangle_ext
            .unwrap();
        let discard_rectangle_count = discard_rectangles.len();
        cmd_set_discard_rectangle_ext(
            command_buffer,
            first_discard_rectangle as _,
            discard_rectangle_count as _,
            discard_rectangles.as_ptr(),
        );
    }
}
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_discard_rectangles"
);
