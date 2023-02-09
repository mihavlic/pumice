#[doc(alias = "VkPhysicalDeviceGroupPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupPropertiesKHR.html)
pub type PhysicalDeviceGroupPropertiesKHR = crate::vk11::PhysicalDeviceGroupProperties;
#[doc(alias = "VkDeviceGroupDeviceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfoKHR.html)
pub type DeviceGroupDeviceCreateInfoKHR = crate::vk11::DeviceGroupDeviceCreateInfo;
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = crate::vk11::MAX_DEVICE_GROUP_SIZE;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html)
pub unsafe fn enumerate_physical_device_groups_khr(
    instance: crate::vk10::Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .enumerate_physical_device_groups
        .unwrap())(
        instance,
        p_physical_device_group_count,
        p_physical_device_group_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html)
    pub unsafe fn enumerate_physical_device_groups_khr(
        &self,
        physical_device_group_count: Option<u32>,
        mut physical_device_group_properties_callback: impl FnMut(
            &mut Vec<crate::vk11::PhysicalDeviceGroupProperties>,
        ),
    ) -> crate::VulkanResult<
        (Vec<crate::vk11::PhysicalDeviceGroupProperties>, crate::vk10::Result),
    > {
        let enumerate_physical_device_groups_khr = (*self.table)
            .enumerate_physical_device_groups_khr
            .unwrap();
        let mut physical_device_group_count = match physical_device_group_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                enumerate_physical_device_groups_khr(
                    self.handle,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut physical_device_group_properties = vec![
            Default::default(); physical_device_group_count as usize
        ];
        physical_device_group_properties_callback(&mut physical_device_group_properties);
        let result = enumerate_physical_device_groups_khr(
            self.handle,
            &mut physical_device_group_count,
            physical_device_group_properties.as_mut_ptr(),
        );
        crate::new_result((physical_device_group_properties, result), result)
    }
}
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_device_group_creation"
);
