#[doc(alias = "VkVideoEncodeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoEncodeFlagsKHR : u32, 0x0,
}
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeRateControlFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoEncodeRateControlFlagsKHR : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeUsageInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageInfoKHR.html)
pub struct VideoEncodeUsageInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub video_usage_hints: VideoEncodeUsageFlagsKHR,
    pub video_content_hints: VideoEncodeContentFlagsKHR,
    pub tuning_mode: VideoEncodeTuningModeKHR,
}
impl Default for VideoEncodeUsageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_USAGE_INFO_KHR,
            p_next: std::ptr::null(),
            video_usage_hints: Default::default(),
            video_content_hints: Default::default(),
            tuning_mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeInfoKHR.html)
pub struct VideoEncodeInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoEncodeFlagsKHR,
    pub quality_level: u32,
    pub dst_bitstream_buffer: crate::vk10::Buffer,
    pub dst_bitstream_buffer_offset: crate::vk10::DeviceSize,
    pub dst_bitstream_buffer_max_range: crate::vk10::DeviceSize,
    pub src_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR,
    pub preceding_externally_encoded_bytes: u32,
}
impl Default for VideoEncodeInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            quality_level: Default::default(),
            dst_bitstream_buffer: Default::default(),
            dst_bitstream_buffer_offset: Default::default(),
            dst_bitstream_buffer_max_range: Default::default(),
            src_picture_resource: Default::default(),
            p_setup_reference_slot: std::ptr::null(),
            reference_slot_count: Default::default(),
            p_reference_slots: std::ptr::null(),
            preceding_externally_encoded_bytes: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html)
pub struct VideoEncodeRateControlInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoEncodeRateControlFlagsKHR,
    pub rate_control_mode: VideoEncodeRateControlModeFlagsKHR,
    pub layer_count: u8,
    pub p_layer_configs: *const VideoEncodeRateControlLayerInfoKHR,
}
impl Default for VideoEncodeRateControlInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rate_control_mode: Default::default(),
            layer_count: Default::default(),
            p_layer_configs: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html)
pub struct VideoEncodeRateControlLayerInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub average_bitrate: u32,
    pub max_bitrate: u32,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
    pub virtual_buffer_size_in_ms: u32,
    pub initial_virtual_buffer_size_in_ms: u32,
}
impl Default for VideoEncodeRateControlLayerInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR,
            p_next: std::ptr::null(),
            average_bitrate: Default::default(),
            max_bitrate: Default::default(),
            frame_rate_numerator: Default::default(),
            frame_rate_denominator: Default::default(),
            virtual_buffer_size_in_ms: Default::default(),
            initial_virtual_buffer_size_in_ms: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilitiesKHR.html)
pub struct VideoEncodeCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: VideoEncodeCapabilityFlagsKHR,
    pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    pub rate_control_layer_count: u8,
    pub quality_level_count: u8,
    pub input_image_data_fill_alignment: crate::vk10::Extent2D,
}
impl Default for VideoEncodeCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            rate_control_modes: Default::default(),
            rate_control_layer_count: Default::default(),
            quality_level_count: Default::default(),
            input_image_data_fill_alignment: Default::default(),
        }
    }
}
#[doc(alias = "VkVideoEncodeUsageFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeUsageFlagsKHR(pub u32);
impl VideoEncodeUsageFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const TRANSCODING: Self = Self(1 << 0);
    pub const STREAMING: Self = Self(1 << 1);
    pub const RECORDING: Self = Self(1 << 2);
    pub const CONFERENCING: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    VideoEncodeUsageFlagsKHR : u32, 0xf, DEFAULT, TRANSCODING, STREAMING, RECORDING,
    CONFERENCING
}
#[doc(alias = "VkVideoEncodeContentFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeContentFlagsKHR(pub u32);
impl VideoEncodeContentFlagsKHR {
    pub const DEFAULT: Self = Self(0);
    pub const CAMERA: Self = Self(1 << 0);
    pub const DESKTOP: Self = Self(1 << 1);
    pub const RENDERED: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    VideoEncodeContentFlagsKHR : u32, 0x7, DEFAULT, CAMERA, DESKTOP, RENDERED
}
#[doc(alias = "VkVideoEncodeTuningModeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeTuningModeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeTuningModeKHR(pub i32);
impl VideoEncodeTuningModeKHR {
    pub const DEFAULT: Self = Self(0);
    pub const HIGH_QUALITY: Self = Self(1);
    pub const LOW_LATENCY: Self = Self(2);
    pub const ULTRA_LOW_LATENCY: Self = Self(3);
    pub const LOSSLESS: Self = Self(4);
}
crate::enum_impl! {
    VideoEncodeTuningModeKHR : i32, DEFAULT, HIGH_QUALITY, LOW_LATENCY,
    ULTRA_LOW_LATENCY, LOSSLESS
}
#[doc(alias = "VkVideoEncodeCapabilityFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeCapabilityFlagsKHR(pub u32);
impl VideoEncodeCapabilityFlagsKHR {
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    VideoEncodeCapabilityFlagsKHR : u32, 0x1, PRECEDING_EXTERNALLY_ENCODED_BYTES
}
#[doc(alias = "VkVideoEncodeRateControlModeFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeRateControlModeFlagsKHR(pub u32);
impl VideoEncodeRateControlModeFlagsKHR {
    pub const NONE: Self = Self(0);
    pub const CBR: Self = Self(1);
    pub const VBR: Self = Self(2);
}
crate::bitflags_impl! {
    VideoEncodeRateControlModeFlagsKHR : u32, 0x3, NONE, CBR, VBR
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEncodeVideoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html)
pub unsafe fn cmd_encode_video_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_encode_info: *const VideoEncodeInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_encode_video_khr
        .unwrap())(command_buffer, p_encode_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html)
    pub unsafe fn cmd_encode_video_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        encode_info: &VideoEncodeInfoKHR,
    ) {
        let cmd_encode_video_khr = (*self.table).cmd_encode_video_khr.unwrap();
        cmd_encode_video_khr(command_buffer, encode_info as _);
    }
}
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 7;
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_video_encode_queue"
);
