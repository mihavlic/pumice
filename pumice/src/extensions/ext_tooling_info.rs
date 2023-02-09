#[doc(alias = "VkToolPurposeFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagsEXT.html)
pub type ToolPurposeFlagsEXT = crate::vk13::ToolPurposeFlags;
#[doc(alias = "VkPhysicalDeviceToolPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolPropertiesEXT.html)
pub type PhysicalDeviceToolPropertiesEXT = crate::vk13::PhysicalDeviceToolProperties;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html)
pub unsafe fn get_physical_device_tool_properties_ext(
    physical_device: crate::vk10::PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut crate::vk13::PhysicalDeviceToolProperties,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_tool_properties
        .unwrap())(physical_device, p_tool_count, p_tool_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html)
    pub unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        tool_count: Option<u32>,
        mut tool_properties_callback: impl FnMut(
            &mut Vec<crate::vk13::PhysicalDeviceToolProperties>,
        ),
    ) -> crate::VulkanResult<
        (Vec<crate::vk13::PhysicalDeviceToolProperties>, crate::vk10::Result),
    > {
        let get_physical_device_tool_properties_ext = (*self.table)
            .get_physical_device_tool_properties_ext
            .unwrap();
        let mut tool_count = match tool_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_tool_properties_ext(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut tool_properties = vec![Default::default(); tool_count as usize];
        tool_properties_callback(&mut tool_properties);
        let result = get_physical_device_tool_properties_ext(
            physical_device,
            &mut tool_count,
            tool_properties.as_mut_ptr(),
        );
        crate::new_result((tool_properties, result), result)
    }
}
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
pub const EXT_TOOLING_INFO_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_tooling_info"
);
