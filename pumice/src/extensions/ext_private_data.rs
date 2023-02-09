#[doc(alias = "VkPrivateDataSlotCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlagsEXT.html)
pub type PrivateDataSlotCreateFlagsEXT = crate::vk13::PrivateDataSlotCreateFlags;
#[doc(alias = "VkPrivateDataSlotEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotEXT.html)
pub type PrivateDataSlotEXT = crate::vk13::PrivateDataSlot;
#[doc(alias = "VkDevicePrivateDataCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html)
pub type DevicePrivateDataCreateInfoEXT = crate::vk13::DevicePrivateDataCreateInfo;
#[doc(alias = "VkPrivateDataSlotCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html)
pub type PrivateDataSlotCreateInfoEXT = crate::vk13::PrivateDataSlotCreateInfo;
#[doc(alias = "VkPhysicalDevicePrivateDataFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html)
pub type PhysicalDevicePrivateDataFeaturesEXT = crate::vk13::PhysicalDevicePrivateDataFeatures;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlotEXT.html)
pub unsafe fn create_private_data_slot_ext(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk13::PrivateDataSlotCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_private_data_slot: *mut crate::vk13::PrivateDataSlot,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_private_data_slot
        .unwrap())(device, p_create_info, p_allocator, p_private_data_slot)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlotEXT.html)
    pub unsafe fn create_private_data_slot_ext(
        &self,
        create_info: &crate::vk13::PrivateDataSlotCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk13::PrivateDataSlot> {
        let create_private_data_slot_ext = (*self.table)
            .create_private_data_slot_ext
            .unwrap();
        let mut private_data_slot = Default::default();
        let result = create_private_data_slot_ext(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut private_data_slot,
        );
        crate::new_result(private_data_slot, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlotEXT.html)
pub unsafe fn destroy_private_data_slot_ext(
    device: crate::vk10::Device,
    private_data_slot: crate::vk13::PrivateDataSlot,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_private_data_slot
        .unwrap())(device, private_data_slot, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlotEXT.html)
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: crate::vk13::PrivateDataSlot,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_private_data_slot_ext = (*self.table)
            .destroy_private_data_slot_ext
            .unwrap();
        destroy_private_data_slot_ext(
            self.handle,
            private_data_slot,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetPrivateDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateDataEXT.html)
pub unsafe fn set_private_data_ext(
    device: crate::vk10::Device,
    object_type: crate::vk10::ObjectType,
    object_handle: u64,
    private_data_slot: crate::vk13::PrivateDataSlot,
    data: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_private_data
        .unwrap())(device, object_type, object_handle, private_data_slot, data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetPrivateDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateDataEXT.html)
    pub unsafe fn set_private_data_ext(
        &self,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
        data: u64,
    ) -> crate::VulkanResult<()> {
        let set_private_data_ext = (*self.table).set_private_data_ext.unwrap();
        let result = set_private_data_ext(
            self.handle,
            object_type as _,
            object_handle as _,
            private_data_slot,
            data as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPrivateDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateDataEXT.html)
pub unsafe fn get_private_data_ext(
    device: crate::vk10::Device,
    object_type: crate::vk10::ObjectType,
    object_handle: u64,
    private_data_slot: crate::vk13::PrivateDataSlot,
    p_data: *mut u64,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_private_data
        .unwrap())(device, object_type, object_handle, private_data_slot, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPrivateDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateDataEXT.html)
    pub unsafe fn get_private_data_ext(
        &self,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
    ) -> u64 {
        let get_private_data_ext = (*self.table).get_private_data_ext.unwrap();
        let mut data = Default::default();
        get_private_data_ext(
            self.handle,
            object_type as _,
            object_handle as _,
            private_data_slot,
            &mut data,
        );
        data
    }
}
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
pub const EXT_PRIVATE_DATA_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_private_data"
);
