#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html)
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_push_descriptors: u32,
}
impl Default for PhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            max_push_descriptors: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
pub unsafe fn cmd_push_descriptor_set_khr(
    command_buffer: crate::vk10::CommandBuffer,
    pipeline_bind_point: crate::vk10::PipelineBindPoint,
    layout: crate::vk10::PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_push_descriptor_set_khr
        .unwrap())(
        command_buffer,
        pipeline_bind_point,
        layout,
        set,
        descriptor_write_count,
        p_descriptor_writes,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        layout: crate::vk10::PipelineLayout,
        set: u32,
        descriptor_writes: &[crate::vk10::WriteDescriptorSet],
    ) {
        let cmd_push_descriptor_set_khr = (*self.table)
            .cmd_push_descriptor_set_khr
            .unwrap();
        let descriptor_write_count = descriptor_writes.len();
        cmd_push_descriptor_set_khr(
            command_buffer,
            pipeline_bind_point as _,
            layout,
            set as _,
            descriptor_write_count as _,
            descriptor_writes.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
pub unsafe fn cmd_push_descriptor_set_with_template_khr(
    command_buffer: crate::vk10::CommandBuffer,
    descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
    layout: crate::vk10::PipelineLayout,
    set: u32,
    p_data: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_push_descriptor_set_with_template_khr
        .unwrap())(command_buffer, descriptor_update_template, layout, set, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        layout: crate::vk10::PipelineLayout,
        set: u32,
        data: *const std::os::raw::c_void,
    ) {
        let cmd_push_descriptor_set_with_template_khr = (*self.table)
            .cmd_push_descriptor_set_with_template_khr
            .unwrap();
        cmd_push_descriptor_set_with_template_khr(
            command_buffer,
            descriptor_update_template,
            layout,
            set as _,
            data,
        );
    }
}
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_push_descriptor"
);
