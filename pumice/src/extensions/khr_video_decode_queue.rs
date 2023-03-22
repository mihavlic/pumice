#[doc(alias = "VkVideoDecodeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoDecodeFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoDecodeFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html)
pub struct VideoDecodeCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
impl Default for VideoDecodeCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeUsageInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageInfoKHR.html)
pub struct VideoDecodeUsageInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub video_usage_hints: VideoDecodeUsageFlagsKHR,
}
impl Default for VideoDecodeUsageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_USAGE_INFO_KHR,
            p_next: std::ptr::null(),
            video_usage_hints: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html)
pub struct VideoDecodeInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoDecodeFlagsKHR,
    pub src_buffer: crate::vk10::Buffer,
    pub src_buffer_offset: crate::vk10::DeviceSize,
    pub src_buffer_range: crate::vk10::DeviceSize,
    pub dst_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR,
}
impl Default for VideoDecodeInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            src_buffer: Default::default(),
            src_buffer_offset: Default::default(),
            src_buffer_range: Default::default(),
            dst_picture_resource: Default::default(),
            p_setup_reference_slot: std::ptr::null(),
            reference_slot_count: Default::default(),
            p_reference_slots: std::ptr::null(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoDecodeUsageFlagsKHR(pub u32);
impl VideoDecodeUsageFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const TRANSCODING: Self = Self(1 << 0);
    pub const OFFLINE: Self = Self(1 << 1);
    pub const STREAMING: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    VideoDecodeUsageFlagsKHR : u32, 0x7, DEFAULT, TRANSCODING, OFFLINE, STREAMING
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoDecodeCapabilityFlagsKHR(pub u32);
impl VideoDecodeCapabilityFlagsKHR {
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(1 << 0);
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    VideoDecodeCapabilityFlagsKHR : u32, 0x3, DPB_AND_OUTPUT_COINCIDE,
    DPB_AND_OUTPUT_DISTINCT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDecodeVideoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html)
pub unsafe fn cmd_decode_video_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_decode_info: *const VideoDecodeInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_decode_video_khr
        .unwrap())(command_buffer, p_decode_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html)
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        decode_info: &VideoDecodeInfoKHR,
    ) {
        let cmd_decode_video_khr = (*self.table).cmd_decode_video_khr.unwrap();
        cmd_decode_video_khr(command_buffer, decode_info as _);
    }
}
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 6;
pub const KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_video_decode_queue"
);
