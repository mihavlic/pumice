#[doc(alias = "VkPeerMemoryFeatureFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagsKHR.html)
pub type PeerMemoryFeatureFlagsKHR = crate::vk11::PeerMemoryFeatureFlags;
#[doc(alias = "VkMemoryAllocateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsKHR.html)
pub type MemoryAllocateFlagsKHR = crate::vk11::MemoryAllocateFlags;
#[doc(alias = "VkMemoryAllocateFlagsInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfoKHR.html)
pub type MemoryAllocateFlagsInfoKHR = crate::vk11::MemoryAllocateFlagsInfo;
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfoKHR.html)
pub type BindBufferMemoryDeviceGroupInfoKHR = crate::vk11::BindBufferMemoryDeviceGroupInfo;
#[doc(alias = "VkBindImageMemoryDeviceGroupInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfoKHR.html)
pub type BindImageMemoryDeviceGroupInfoKHR = crate::vk11::BindImageMemoryDeviceGroupInfo;
#[doc(alias = "VkDeviceGroupRenderPassBeginInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfoKHR.html)
pub type DeviceGroupRenderPassBeginInfoKHR = crate::vk11::DeviceGroupRenderPassBeginInfo;
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfoKHR.html)
pub type DeviceGroupCommandBufferBeginInfoKHR = crate::vk11::DeviceGroupCommandBufferBeginInfo;
#[doc(alias = "VkDeviceGroupSubmitInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfoKHR.html)
pub type DeviceGroupSubmitInfoKHR = crate::vk11::DeviceGroupSubmitInfo;
#[doc(alias = "VkDeviceGroupBindSparseInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfoKHR.html)
pub type DeviceGroupBindSparseInfoKHR = crate::vk11::DeviceGroupBindSparseInfo;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)
pub unsafe fn get_device_group_peer_memory_features_khr(
    device: crate::vk10::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_group_peer_memory_features
        .unwrap())(
        device,
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> crate::vk11::PeerMemoryFeatureFlags {
        let get_device_group_peer_memory_features_khr = (*self.table)
            .get_device_group_peer_memory_features_khr
            .unwrap();
        let mut peer_memory_features = Default::default();
        get_device_group_peer_memory_features_khr(
            self.handle,
            heap_index as _,
            local_device_index as _,
            remote_device_index as _,
            &mut peer_memory_features,
        );
        peer_memory_features
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html)
pub unsafe fn cmd_set_device_mask_khr(
    command_buffer: crate::vk10::CommandBuffer,
    device_mask: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_device_mask
        .unwrap())(command_buffer, device_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html)
    pub unsafe fn cmd_set_device_mask_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        device_mask: u32,
    ) {
        let cmd_set_device_mask_khr = (*self.table).cmd_set_device_mask_khr.unwrap();
        cmd_set_device_mask_khr(command_buffer, device_mask as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDispatchBaseKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html)
pub unsafe fn cmd_dispatch_base_khr(
    command_buffer: crate::vk10::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_dispatch_base
        .unwrap())(
        command_buffer,
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html)
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        let cmd_dispatch_base_khr = (*self.table).cmd_dispatch_base_khr.unwrap();
        cmd_dispatch_base_khr(
            command_buffer,
            base_group_x as _,
            base_group_y as _,
            base_group_z as _,
            group_count_x as _,
            group_count_y as _,
            group_count_z as _,
        );
    }
}
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_device_group"
);
