#[doc(alias = "VkCommandPoolTrimFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlagsKHR.html)
pub type CommandPoolTrimFlagsKHR = crate::vk11::CommandPoolTrimFlags;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkTrimCommandPoolKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html)
pub unsafe fn trim_command_pool_khr(
    device: crate::vk10::Device,
    command_pool: crate::vk10::CommandPool,
    flags: crate::vk11::CommandPoolTrimFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .trim_command_pool
        .unwrap())(device, command_pool, flags)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkTrimCommandPoolKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html)
    pub unsafe fn trim_command_pool_khr(
        &self,
        command_pool: crate::vk10::CommandPool,
        flags: Option<crate::vk11::CommandPoolTrimFlags>,
    ) {
        let trim_command_pool_khr = (*self.table).trim_command_pool_khr.unwrap();
        trim_command_pool_khr(
            self.handle,
            command_pool,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
pub const KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;
pub const KHR_MAINTENANCE_1_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_maintenance1"
);
