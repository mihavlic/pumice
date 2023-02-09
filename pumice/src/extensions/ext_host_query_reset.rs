#[doc(alias = "VkPhysicalDeviceHostQueryResetFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceHostQueryResetFeaturesEXT.html)
pub type PhysicalDeviceHostQueryResetFeaturesEXT = crate::vk12::PhysicalDeviceHostQueryResetFeatures;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetQueryPoolEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPoolEXT.html)
pub unsafe fn reset_query_pool_ext(
    device: crate::vk10::Device,
    query_pool: crate::vk10::QueryPool,
    first_query: u32,
    query_count: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_query_pool
        .unwrap())(device, query_pool, first_query, query_count)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkResetQueryPoolEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPoolEXT.html)
    pub unsafe fn reset_query_pool_ext(
        &self,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let reset_query_pool_ext = (*self.table).reset_query_pool_ext.unwrap();
        reset_query_pool_ext(
            self.handle,
            query_pool,
            first_query as _,
            query_count as _,
        );
    }
}
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
pub const EXT_HOST_QUERY_RESET_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_host_query_reset"
);
