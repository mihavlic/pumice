#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3PropertiesKHR.html)
pub type PhysicalDeviceMaintenance3PropertiesKHR = crate::vk11::PhysicalDeviceMaintenance3Properties;
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupportKHR.html)
pub type DescriptorSetLayoutSupportKHR = crate::vk11::DescriptorSetLayoutSupport;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html)
pub unsafe fn get_descriptor_set_layout_support_khr(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
    p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_descriptor_set_layout_support
        .unwrap())(device, p_create_info, p_support)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html)
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        create_info: &crate::vk10::DescriptorSetLayoutCreateInfo,
    ) -> crate::vk11::DescriptorSetLayoutSupport {
        let get_descriptor_set_layout_support_khr = (*self.table)
            .get_descriptor_set_layout_support_khr
            .unwrap();
        let mut support = Default::default();
        get_descriptor_set_layout_support_khr(
            self.handle,
            create_info as _,
            &mut support,
        );
        support
    }
}
pub const KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;
pub const KHR_MAINTENANCE_3_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_maintenance3"
);
