#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlagsKHR.html)
pub type DescriptorUpdateTemplateCreateFlagsKHR = crate::vk11::DescriptorUpdateTemplateCreateFlags;
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateKHR.html)
pub type DescriptorUpdateTemplateKHR = crate::vk11::DescriptorUpdateTemplate;
#[doc(alias = "VkDescriptorUpdateTemplateTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateTypeKHR.html)
pub type DescriptorUpdateTemplateTypeKHR = crate::vk11::DescriptorUpdateTemplateType;
#[doc(alias = "VkDescriptorUpdateTemplateEntryKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntryKHR.html)
pub type DescriptorUpdateTemplateEntryKHR = crate::vk11::DescriptorUpdateTemplateEntry;
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfoKHR.html)
pub type DescriptorUpdateTemplateCreateInfoKHR = crate::vk11::DescriptorUpdateTemplateCreateInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html)
pub unsafe fn create_descriptor_update_template_khr(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_descriptor_update_template
        .unwrap())(device, p_create_info, p_allocator, p_descriptor_update_template)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html)
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        create_info: &crate::vk11::DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk11::DescriptorUpdateTemplate> {
        let create_descriptor_update_template_khr = (*self.table)
            .create_descriptor_update_template_khr
            .unwrap();
        let mut descriptor_update_template = Default::default();
        let result = create_descriptor_update_template_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_update_template,
        );
        crate::new_result(descriptor_update_template, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html)
pub unsafe fn destroy_descriptor_update_template_khr(
    device: crate::vk10::Device,
    descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_descriptor_update_template
        .unwrap())(device, descriptor_update_template, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html)
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_descriptor_update_template_khr = (*self.table)
            .destroy_descriptor_update_template_khr
            .unwrap();
        destroy_descriptor_update_template_khr(
            self.handle,
            descriptor_update_template,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html)
pub unsafe fn update_descriptor_set_with_template_khr(
    device: crate::vk10::Device,
    descriptor_set: crate::vk10::DescriptorSet,
    descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
    p_data: *const std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .update_descriptor_set_with_template
        .unwrap())(device, descriptor_set, descriptor_update_template, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html)
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: crate::vk10::DescriptorSet,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        data: *const std::os::raw::c_void,
    ) {
        let update_descriptor_set_with_template_khr = (*self.table)
            .update_descriptor_set_with_template_khr
            .unwrap();
        update_descriptor_set_with_template_khr(
            self.handle,
            descriptor_set,
            descriptor_update_template,
            data,
        );
    }
}
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_descriptor_update_template"
);
