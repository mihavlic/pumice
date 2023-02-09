crate::dispatchable_handle!(
    DeferredOperationKHR, DEFERRED_OPERATION_KHR, "VkDeferredOperationKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html)"
);
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDeferredOperationKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html)
pub unsafe fn create_deferred_operation_khr(
    device: crate::vk10::Device,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_deferred_operation_khr
        .unwrap())(device, p_allocator, p_deferred_operation)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html)
    pub unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<DeferredOperationKHR> {
        let create_deferred_operation_khr = (*self.table)
            .create_deferred_operation_khr
            .unwrap();
        let mut deferred_operation = Default::default();
        let result = create_deferred_operation_khr(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut deferred_operation,
        );
        crate::new_result(deferred_operation, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyDeferredOperationKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html)
pub unsafe fn destroy_deferred_operation_khr(
    device: crate::vk10::Device,
    operation: DeferredOperationKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_deferred_operation_khr
        .unwrap())(device, operation, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html)
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: DeferredOperationKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_deferred_operation_khr = (*self.table)
            .destroy_deferred_operation_khr
            .unwrap();
        destroy_deferred_operation_khr(
            self.handle,
            operation,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
pub unsafe fn get_deferred_operation_max_concurrency_khr(
    device: crate::vk10::Device,
    operation: DeferredOperationKHR,
) -> u32 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_deferred_operation_max_concurrency_khr
        .unwrap())(device, operation)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: DeferredOperationKHR,
    ) {
        let get_deferred_operation_max_concurrency_khr = (*self.table)
            .get_deferred_operation_max_concurrency_khr
            .unwrap();
        get_deferred_operation_max_concurrency_khr(self.handle, operation);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeferredOperationResultKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html)
pub unsafe fn get_deferred_operation_result_khr(
    device: crate::vk10::Device,
    operation: DeferredOperationKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_deferred_operation_result_khr
        .unwrap())(device, operation)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html)
    pub unsafe fn get_deferred_operation_result_khr(
        &self,
        operation: DeferredOperationKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let get_deferred_operation_result_khr = (*self.table)
            .get_deferred_operation_result_khr
            .unwrap();
        let result = get_deferred_operation_result_khr(self.handle, operation);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDeferredOperationJoinKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html)
pub unsafe fn deferred_operation_join_khr(
    device: crate::vk10::Device,
    operation: DeferredOperationKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .deferred_operation_join_khr
        .unwrap())(device, operation)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html)
    pub unsafe fn deferred_operation_join_khr(
        &self,
        operation: DeferredOperationKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let deferred_operation_join_khr = (*self.table)
            .deferred_operation_join_khr
            .unwrap();
        let result = deferred_operation_join_khr(self.handle, operation);
        crate::new_result(result, result)
    }
}
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_deferred_host_operations"
);
