#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html)
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub vertex_input_dynamic_state: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_input_dynamic_state: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription2EXT.html)
pub struct VertexInputBindingDescription2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub binding: u32,
    pub stride: u32,
    pub input_rate: crate::vk10::VertexInputRate,
    pub divisor: u32,
}
impl Default for VertexInputBindingDescription2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
            p_next: std::ptr::null_mut(),
            binding: Default::default(),
            stride: Default::default(),
            input_rate: Default::default(),
            divisor: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription2EXT.html)
pub struct VertexInputAttributeDescription2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub location: u32,
    pub binding: u32,
    pub format: crate::vk10::Format,
    pub offset: u32,
}
impl Default for VertexInputAttributeDescription2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
            p_next: std::ptr::null_mut(),
            location: Default::default(),
            binding: Default::default(),
            format: Default::default(),
            offset: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetVertexInputEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html)
pub unsafe fn cmd_set_vertex_input_ext(
    command_buffer: crate::vk10::CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_vertex_input_ext
        .unwrap())(
        command_buffer,
        vertex_binding_description_count,
        p_vertex_binding_descriptions,
        vertex_attribute_description_count,
        p_vertex_attribute_descriptions,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html)
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) {
        let cmd_set_vertex_input_ext = (*self.table).cmd_set_vertex_input_ext.unwrap();
        let vertex_binding_description_count = vertex_binding_descriptions.len();
        let vertex_attribute_description_count = vertex_attribute_descriptions.len();
        cmd_set_vertex_input_ext(
            command_buffer,
            vertex_binding_description_count as _,
            vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count as _,
            vertex_attribute_descriptions.as_ptr(),
        );
    }
}
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_vertex_input_dynamic_state"
);
