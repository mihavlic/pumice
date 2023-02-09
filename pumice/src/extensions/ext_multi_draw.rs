#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMultiDrawInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html)
pub struct MultiDrawInfoEXT {
    pub first_vertex: u32,
    pub vertex_count: u32,
}
impl Default for MultiDrawInfoEXT {
    fn default() -> Self {
        Self {
            first_vertex: Default::default(),
            vertex_count: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html)
pub struct MultiDrawIndexedInfoEXT {
    pub first_index: u32,
    pub index_count: u32,
    pub vertex_offset: i32,
}
impl Default for MultiDrawIndexedInfoEXT {
    fn default() -> Self {
        Self {
            first_index: Default::default(),
            index_count: Default::default(),
            vertex_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html)
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_multi_draw_count: u32,
}
impl Default for PhysicalDeviceMultiDrawPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_multi_draw_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html)
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub multi_draw: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMultiDrawFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            multi_draw: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMultiEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html)
pub unsafe fn cmd_draw_multi_ext(
    command_buffer: crate::vk10::CommandBuffer,
    draw_count: u32,
    p_vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_multi_ext
        .unwrap())(
        command_buffer,
        draw_count,
        p_vertex_info,
        instance_count,
        first_instance,
        stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMultiEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html)
    pub unsafe fn cmd_draw_multi_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        let cmd_draw_multi_ext = (*self.table).cmd_draw_multi_ext.unwrap();
        let draw_count = vertex_info.len();
        cmd_draw_multi_ext(
            command_buffer,
            draw_count as _,
            vertex_info.as_ptr(),
            instance_count as _,
            first_instance as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html)
pub unsafe fn cmd_draw_multi_indexed_ext(
    command_buffer: crate::vk10::CommandBuffer,
    draw_count: u32,
    p_index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: *const i32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_draw_multi_indexed_ext
        .unwrap())(
        command_buffer,
        draw_count,
        p_index_info,
        instance_count,
        first_instance,
        stride,
        p_vertex_offset,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html)
    pub unsafe fn cmd_draw_multi_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        let cmd_draw_multi_indexed_ext = (*self.table)
            .cmd_draw_multi_indexed_ext
            .unwrap();
        let draw_count = index_info.len();
        cmd_draw_multi_indexed_ext(
            command_buffer,
            draw_count as _,
            index_info.as_ptr(),
            instance_count as _,
            first_instance as _,
            stride as _,
            match vertex_offset {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_multi_draw"
);
