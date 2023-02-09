#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html)
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub invocation_mask: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
            p_next: std::ptr::null_mut(),
            invocation_mask: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html)
pub unsafe fn cmd_bind_invocation_mask_huawei(
    command_buffer: crate::vk10::CommandBuffer,
    image_view: crate::vk10::ImageView,
    image_layout: crate::vk10::ImageLayout,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_invocation_mask_huawei
        .unwrap())(command_buffer, image_view, image_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html)
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image_view: crate::vk10::ImageView,
        image_layout: crate::vk10::ImageLayout,
    ) {
        let cmd_bind_invocation_mask_huawei = (*self.table)
            .cmd_bind_invocation_mask_huawei
            .unwrap();
        cmd_bind_invocation_mask_huawei(command_buffer, image_view, image_layout as _);
    }
}
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
pub const HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_HUAWEI_invocation_mask"
);
