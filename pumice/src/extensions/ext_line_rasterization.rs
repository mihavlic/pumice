#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceLineRasterizationFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html)
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub rectangular_lines: crate::vk10::Bool32,
    pub bresenham_lines: crate::vk10::Bool32,
    pub smooth_lines: crate::vk10::Bool32,
    pub stippled_rectangular_lines: crate::vk10::Bool32,
    pub stippled_bresenham_lines: crate::vk10::Bool32,
    pub stippled_smooth_lines: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            rectangular_lines: Default::default(),
            bresenham_lines: Default::default(),
            smooth_lines: Default::default(),
            stippled_rectangular_lines: Default::default(),
            stippled_bresenham_lines: Default::default(),
            stippled_smooth_lines: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceLineRasterizationPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html)
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub line_sub_pixel_precision_bits: u32,
}
impl Default for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            line_sub_pixel_precision_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationLineStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html)
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub line_rasterization_mode: LineRasterizationModeEXT,
    pub stippled_line_enable: crate::vk10::Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
impl Default for PipelineRasterizationLineStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            line_rasterization_mode: Default::default(),
            stippled_line_enable: Default::default(),
            line_stipple_factor: Default::default(),
            line_stipple_pattern: Default::default(),
        }
    }
}
#[doc(alias = "VkLineRasterizationModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct LineRasterizationModeEXT(pub i32);
impl LineRasterizationModeEXT {
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
}
crate::enum_impl! {
    LineRasterizationModeEXT : i32, DEFAULT, RECTANGULAR, BRESENHAM, RECTANGULAR_SMOOTH
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLineStippleEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html)
pub unsafe fn cmd_set_line_stipple_ext(
    command_buffer: crate::vk10::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_line_stipple_ext
        .unwrap())(command_buffer, line_stipple_factor, line_stipple_pattern)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html)
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        let cmd_set_line_stipple_ext = (*self.table).cmd_set_line_stipple_ext.unwrap();
        cmd_set_line_stipple_ext(
            command_buffer,
            line_stipple_factor as _,
            line_stipple_pattern as _,
        );
    }
}
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_line_rasterization"
);
