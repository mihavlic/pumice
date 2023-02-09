#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html)
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl Default for DebugMarkerObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html)
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const std::os::raw::c_void,
}
impl Default for DebugMarkerObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: Default::default(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html)
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_marker_name: *const std::os::raw::c_char,
    pub color: [std::os::raw::c_float; 4],
}
impl Default for DebugMarkerMarkerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
            p_next: std::ptr::null(),
            p_marker_name: std::ptr::null(),
            color: unsafe { std::mem::zeroed() },
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
pub unsafe fn debug_marker_set_object_name_ext(
    device: crate::vk10::Device,
    p_name_info: *const DebugMarkerObjectNameInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .debug_marker_set_object_name_ext
        .unwrap())(device, p_name_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> crate::VulkanResult<()> {
        let debug_marker_set_object_name_ext = (*self.table)
            .debug_marker_set_object_name_ext
            .unwrap();
        let result = debug_marker_set_object_name_ext(self.handle, name_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
pub unsafe fn debug_marker_set_object_tag_ext(
    device: crate::vk10::Device,
    p_tag_info: *const DebugMarkerObjectTagInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .debug_marker_set_object_tag_ext
        .unwrap())(device, p_tag_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> crate::VulkanResult<()> {
        let debug_marker_set_object_tag_ext = (*self.table)
            .debug_marker_set_object_tag_ext
            .unwrap();
        let result = debug_marker_set_object_tag_ext(self.handle, tag_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
pub unsafe fn cmd_debug_marker_begin_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_debug_marker_begin_ext
        .unwrap())(command_buffer, p_marker_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) {
        let cmd_debug_marker_begin_ext = (*self.table)
            .cmd_debug_marker_begin_ext
            .unwrap();
        cmd_debug_marker_begin_ext(command_buffer, marker_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
pub unsafe fn cmd_debug_marker_end_ext(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_debug_marker_end_ext
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
    pub unsafe fn cmd_debug_marker_end_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_debug_marker_end_ext = (*self.table).cmd_debug_marker_end_ext.unwrap();
        cmd_debug_marker_end_ext(command_buffer);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
pub unsafe fn cmd_debug_marker_insert_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_debug_marker_insert_ext
        .unwrap())(command_buffer, p_marker_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) {
        let cmd_debug_marker_insert_ext = (*self.table)
            .cmd_debug_marker_insert_ext
            .unwrap();
        cmd_debug_marker_insert_ext(command_buffer, marker_info as _);
    }
}
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_debug_marker"
);
