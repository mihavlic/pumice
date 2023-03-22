#[doc(alias = "VkVideoSessionParametersCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoSessionParametersCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoSessionParametersCreateFlagsKHR : u32, 0x0,
}
#[doc(alias = "VkVideoBeginCodingFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoBeginCodingFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoBeginCodingFlagsKHR : u32, 0x0,
}
#[doc(alias = "VkVideoEndCodingFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEndCodingFlagsKHR(pub u32);
crate::bitflags_impl! {
    VideoEndCodingFlagsKHR : u32, 0x0,
}
crate::dispatchable_handle!(
    VideoSessionKHR, VIDEO_SESSION_KHR, "VkVideoSessionKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionKHR.html)"
);
crate::dispatchable_handle!(
    VideoSessionParametersKHR, VIDEO_SESSION_PARAMETERS_KHR,
    "VkVideoSessionParametersKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersKHR.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyVideoPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyVideoPropertiesKHR.html)
pub struct QueueFamilyVideoPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}
impl Default for QueueFamilyVideoPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            video_codec_operations: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyQueryResultStatusPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusPropertiesKHR.html)
pub struct QueueFamilyQueryResultStatusPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub query_result_status_support: crate::vk10::Bool32,
}
impl Default for QueueFamilyQueryResultStatusPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            query_result_status_support: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoProfileListInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfileListInfoKHR.html)
pub struct VideoProfileListInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub profile_count: u32,
    pub p_profiles: *const VideoProfileInfoKHR,
}
impl Default for VideoProfileListInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_PROFILE_LIST_INFO_KHR,
            p_next: std::ptr::null(),
            profile_count: Default::default(),
            p_profiles: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVideoFormatInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html)
pub struct PhysicalDeviceVideoFormatInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_usage: crate::vk10::ImageUsageFlags,
}
impl Default for PhysicalDeviceVideoFormatInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR,
            p_next: std::ptr::null(),
            image_usage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoFormatPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html)
pub struct VideoFormatPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub component_mapping: crate::vk10::ComponentMapping,
    pub image_create_flags: crate::vk10::ImageCreateFlags,
    pub image_type: crate::vk10::ImageType,
    pub image_tiling: crate::vk10::ImageTiling,
    pub image_usage_flags: crate::vk10::ImageUsageFlags,
}
impl Default for VideoFormatPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_FORMAT_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            component_mapping: Default::default(),
            image_create_flags: Default::default(),
            image_type: Default::default(),
            image_tiling: Default::default(),
            image_usage_flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoProfileInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfileInfoKHR.html)
pub struct VideoProfileInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub video_codec_operation: VideoCodecOperationFlagsKHR,
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
impl Default for VideoProfileInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_PROFILE_INFO_KHR,
            p_next: std::ptr::null(),
            video_codec_operation: Default::default(),
            chroma_subsampling: Default::default(),
            luma_bit_depth: Default::default(),
            chroma_bit_depth: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html)
pub struct VideoCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: VideoCapabilityFlagsKHR,
    pub min_bitstream_buffer_offset_alignment: crate::vk10::DeviceSize,
    pub min_bitstream_buffer_size_alignment: crate::vk10::DeviceSize,
    pub picture_access_granularity: crate::vk10::Extent2D,
    pub min_coded_extent: crate::vk10::Extent2D,
    pub max_coded_extent: crate::vk10::Extent2D,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub std_header_version: crate::vk10::ExtensionProperties,
}
impl Default for VideoCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            min_bitstream_buffer_offset_alignment: Default::default(),
            min_bitstream_buffer_size_alignment: Default::default(),
            picture_access_granularity: Default::default(),
            min_coded_extent: Default::default(),
            max_coded_extent: Default::default(),
            max_dpb_slots: Default::default(),
            max_active_reference_pictures: Default::default(),
            std_header_version: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoSessionMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionMemoryRequirementsKHR.html)
pub struct VideoSessionMemoryRequirementsKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_bind_index: u32,
    pub memory_requirements: crate::vk10::MemoryRequirements,
}
impl Default for VideoSessionMemoryRequirementsKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
            p_next: std::ptr::null_mut(),
            memory_bind_index: Default::default(),
            memory_requirements: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBindVideoSessionMemoryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindVideoSessionMemoryInfoKHR.html)
pub struct BindVideoSessionMemoryInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory_bind_index: u32,
    pub memory: crate::vk10::DeviceMemory,
    pub memory_offset: crate::vk10::DeviceSize,
    pub memory_size: crate::vk10::DeviceSize,
}
impl Default for BindVideoSessionMemoryInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BIND_VIDEO_SESSION_MEMORY_INFO_KHR,
            p_next: std::ptr::null(),
            memory_bind_index: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            memory_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoPictureResourceInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceInfoKHR.html)
pub struct VideoPictureResourceInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub coded_offset: crate::vk10::Offset2D,
    pub coded_extent: crate::vk10::Extent2D,
    pub base_array_layer: u32,
    pub image_view_binding: crate::vk10::ImageView,
}
impl Default for VideoPictureResourceInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_PICTURE_RESOURCE_INFO_KHR,
            p_next: std::ptr::null(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            base_array_layer: Default::default(),
            image_view_binding: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoReferenceSlotInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotInfoKHR.html)
pub struct VideoReferenceSlotInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub slot_index: i32,
    pub p_picture_resource: *const VideoPictureResourceInfoKHR,
}
impl Default for VideoReferenceSlotInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_REFERENCE_SLOT_INFO_KHR,
            p_next: std::ptr::null(),
            slot_index: Default::default(),
            p_picture_resource: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoSessionCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html)
pub struct VideoSessionCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    pub p_video_profile: *const VideoProfileInfoKHR,
    pub picture_format: crate::vk10::Format,
    pub max_coded_extent: crate::vk10::Extent2D,
    pub reference_picture_format: crate::vk10::Format,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub p_std_header_version: *const crate::vk10::ExtensionProperties,
}
impl Default for VideoSessionCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_SESSION_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            queue_family_index: Default::default(),
            flags: Default::default(),
            p_video_profile: std::ptr::null(),
            picture_format: Default::default(),
            max_coded_extent: Default::default(),
            reference_picture_format: Default::default(),
            max_dpb_slots: Default::default(),
            max_active_reference_pictures: Default::default(),
            p_std_header_version: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoSessionParametersCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html)
pub struct VideoSessionParametersCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoSessionParametersCreateFlagsKHR,
    pub video_session_parameters_template: VideoSessionParametersKHR,
    pub video_session: VideoSessionKHR,
}
impl Default for VideoSessionParametersCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            video_session_parameters_template: Default::default(),
            video_session: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoSessionParametersUpdateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html)
pub struct VideoSessionParametersUpdateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub update_sequence_count: u32,
}
impl Default for VideoSessionParametersUpdateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
            p_next: std::ptr::null(),
            update_sequence_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoBeginCodingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html)
pub struct VideoBeginCodingInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoBeginCodingFlagsKHR,
    pub video_session: VideoSessionKHR,
    pub video_session_parameters: VideoSessionParametersKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
}
impl Default for VideoBeginCodingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_BEGIN_CODING_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            video_session: Default::default(),
            video_session_parameters: Default::default(),
            reference_slot_count: Default::default(),
            p_reference_slots: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEndCodingInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingInfoKHR.html)
pub struct VideoEndCodingInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoEndCodingFlagsKHR,
}
impl Default for VideoEndCodingInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_END_CODING_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoCodingControlInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html)
pub struct VideoCodingControlInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: VideoCodingControlFlagsKHR,
}
impl Default for VideoCodingControlInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_CODING_CONTROL_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoCodecOperationFlagsKHR(pub u32);
impl VideoCodecOperationFlagsKHR {
    pub const NONE: Self = Self(0);
    /// ext_video_encode_h264
    pub const ENCODE_H264_EXT: Self = Self(1 << 16);
    /// ext_video_encode_h265
    pub const ENCODE_H265_EXT: Self = Self(1 << 17);
    /// ext_video_decode_h264
    pub const DECODE_H264_EXT: Self = Self(1 << 0);
    /// ext_video_decode_h265
    pub const DECODE_H265_EXT: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    VideoCodecOperationFlagsKHR : u32, 0x30003, NONE, ENCODE_H264_EXT, ENCODE_H265_EXT,
    DECODE_H264_EXT, DECODE_H265_EXT
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoChromaSubsamplingFlagsKHR(pub u32);
impl VideoChromaSubsamplingFlagsKHR {
    pub const INVALID: Self = Self(0);
    pub const MONOCHROME: Self = Self(1 << 0);
    pub const S420: Self = Self(1 << 1);
    pub const S422: Self = Self(1 << 2);
    pub const S444: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    VideoChromaSubsamplingFlagsKHR : u32, 0xf, INVALID, MONOCHROME, S420, S422, S444
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoComponentBitDepthFlagsKHR(pub u32);
impl VideoComponentBitDepthFlagsKHR {
    pub const DEPTH_INVALID_KHR: Self = Self(0);
    pub const DEPTH_8_KHR: Self = Self(1 << 0);
    pub const DEPTH_10_KHR: Self = Self(1 << 2);
    pub const DEPTH_12_KHR: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    VideoComponentBitDepthFlagsKHR : u32, 0x15, DEPTH_INVALID_KHR, DEPTH_8_KHR,
    DEPTH_10_KHR, DEPTH_12_KHR
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoCapabilityFlagsKHR(pub u32);
impl VideoCapabilityFlagsKHR {
    pub const PROTECTED_CONTENT: Self = Self(1 << 0);
    pub const SEPARATE_REFERENCE_IMAGES: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    VideoCapabilityFlagsKHR : u32, 0x3, PROTECTED_CONTENT, SEPARATE_REFERENCE_IMAGES
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoSessionCreateFlagsKHR(pub u32);
impl VideoSessionCreateFlagsKHR {
    pub const PROTECTED_CONTENT: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    VideoSessionCreateFlagsKHR : u32, 0x1, PROTECTED_CONTENT
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoCodingControlFlagsKHR(pub u32);
impl VideoCodingControlFlagsKHR {
    pub const RESET: Self = Self(1 << 0);
    /// khr_video_encode_queue
    pub const ENCODE_RATE_CONTROL: Self = Self(1 << 1);
    pub const ENCODE_RATE_CONTROL_LAYER: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    VideoCodingControlFlagsKHR : u32, 0x7, RESET, ENCODE_RATE_CONTROL,
    ENCODE_RATE_CONTROL_LAYER
}
#[doc(alias = "VkQueryResultStatusKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueryResultStatusKHR(pub i32);
impl QueryResultStatusKHR {
    pub const ERROR: Self = Self(-1);
    pub const NOT_READY: Self = Self(0);
    pub const COMPLETE: Self = Self(1);
}
crate::enum_impl! {
    QueryResultStatusKHR : i32, ERROR, NOT_READY, COMPLETE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
pub unsafe fn get_physical_device_video_capabilities_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_video_profile: *const VideoProfileInfoKHR,
    p_capabilities: *mut VideoCapabilitiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_video_capabilities_khr
        .unwrap())(physical_device, p_video_profile, p_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        video_profile: &VideoProfileInfoKHR,
    ) -> crate::VulkanResult<VideoCapabilitiesKHR> {
        let get_physical_device_video_capabilities_khr = (*self.table)
            .get_physical_device_video_capabilities_khr
            .unwrap();
        let mut capabilities = Default::default();
        let result = get_physical_device_video_capabilities_khr(
            physical_device,
            video_profile as _,
            &mut capabilities,
        );
        crate::new_result(capabilities, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
pub unsafe fn get_physical_device_video_format_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_video_format_properties_khr
        .unwrap())(
        physical_device,
        p_video_format_info,
        p_video_format_property_count,
        p_video_format_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
    pub unsafe fn get_physical_device_video_format_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_property_count: Option<u32>,
        mut video_format_properties_callback: impl FnMut(
            &mut Vec<VideoFormatPropertiesKHR>,
        ),
    ) -> crate::VulkanResult<(Vec<VideoFormatPropertiesKHR>, crate::vk10::Result)> {
        let get_physical_device_video_format_properties_khr = (*self.table)
            .get_physical_device_video_format_properties_khr
            .unwrap();
        let mut video_format_property_count = match video_format_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_video_format_properties_khr(
                    physical_device,
                    video_format_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut video_format_properties = vec![
            Default::default(); video_format_property_count as usize
        ];
        video_format_properties_callback(&mut video_format_properties);
        let result = get_physical_device_video_format_properties_khr(
            physical_device,
            video_format_info as _,
            &mut video_format_property_count,
            video_format_properties.as_mut_ptr(),
        );
        crate::new_result((video_format_properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateVideoSessionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html)
pub unsafe fn create_video_session_khr(
    device: crate::vk10::Device,
    p_create_info: *const VideoSessionCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_video_session: *mut VideoSessionKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_video_session_khr
        .unwrap())(device, p_create_info, p_allocator, p_video_session)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateVideoSessionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html)
    pub unsafe fn create_video_session_khr(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<VideoSessionKHR> {
        let create_video_session_khr = (*self.table).create_video_session_khr.unwrap();
        let mut video_session = Default::default();
        let result = create_video_session_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut video_session,
        );
        crate::new_result(video_session, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyVideoSessionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html)
pub unsafe fn destroy_video_session_khr(
    device: crate::vk10::Device,
    video_session: VideoSessionKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_video_session_khr
        .unwrap())(device, video_session, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html)
    pub unsafe fn destroy_video_session_khr(
        &self,
        video_session: VideoSessionKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_video_session_khr = (*self.table).destroy_video_session_khr.unwrap();
        destroy_video_session_khr(
            self.handle,
            video_session,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateVideoSessionParametersKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html)
pub unsafe fn create_video_session_parameters_khr(
    device: crate::vk10::Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_video_session_parameters_khr
        .unwrap())(device, p_create_info, p_allocator, p_video_session_parameters)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html)
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<VideoSessionParametersKHR> {
        let create_video_session_parameters_khr = (*self.table)
            .create_video_session_parameters_khr
            .unwrap();
        let mut video_session_parameters = Default::default();
        let result = create_video_session_parameters_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut video_session_parameters,
        );
        crate::new_result(video_session_parameters, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkUpdateVideoSessionParametersKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html)
pub unsafe fn update_video_session_parameters_khr(
    device: crate::vk10::Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .update_video_session_parameters_khr
        .unwrap())(device, video_session_parameters, p_update_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html)
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> crate::VulkanResult<()> {
        let update_video_session_parameters_khr = (*self.table)
            .update_video_session_parameters_khr
            .unwrap();
        let result = update_video_session_parameters_khr(
            self.handle,
            video_session_parameters,
            update_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyVideoSessionParametersKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html)
pub unsafe fn destroy_video_session_parameters_khr(
    device: crate::vk10::Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_video_session_parameters_khr
        .unwrap())(device, video_session_parameters, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html)
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_video_session_parameters_khr = (*self.table)
            .destroy_video_session_parameters_khr
            .unwrap();
        destroy_video_session_parameters_khr(
            self.handle,
            video_session_parameters,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html)
pub unsafe fn get_video_session_memory_requirements_khr(
    device: crate::vk10::Device,
    video_session: VideoSessionKHR,
    p_memory_requirements_count: *mut u32,
    p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_video_session_memory_requirements_khr
        .unwrap())(
        device,
        video_session,
        p_memory_requirements_count,
        p_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html)
    pub unsafe fn get_video_session_memory_requirements_khr(
        &self,
        video_session: VideoSessionKHR,
        memory_requirements_count: Option<u32>,
        mut memory_requirements_callback: impl FnMut(
            &mut Vec<VideoSessionMemoryRequirementsKHR>,
        ),
    ) -> crate::VulkanResult<
        (Vec<VideoSessionMemoryRequirementsKHR>, crate::vk10::Result),
    > {
        let get_video_session_memory_requirements_khr = (*self.table)
            .get_video_session_memory_requirements_khr
            .unwrap();
        let mut memory_requirements_count = match memory_requirements_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_video_session_memory_requirements_khr(
                    self.handle,
                    video_session,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut memory_requirements = vec![
            Default::default(); memory_requirements_count as usize
        ];
        memory_requirements_callback(&mut memory_requirements);
        let result = get_video_session_memory_requirements_khr(
            self.handle,
            video_session,
            &mut memory_requirements_count,
            memory_requirements.as_mut_ptr(),
        );
        crate::new_result((memory_requirements, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindVideoSessionMemoryKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html)
pub unsafe fn bind_video_session_memory_khr(
    device: crate::vk10::Device,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_video_session_memory_khr
        .unwrap())(
        device,
        video_session,
        bind_session_memory_info_count,
        p_bind_session_memory_infos,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html)
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> crate::VulkanResult<()> {
        let bind_video_session_memory_khr = (*self.table)
            .bind_video_session_memory_khr
            .unwrap();
        let bind_session_memory_info_count = bind_session_memory_infos.len();
        let result = bind_video_session_memory_khr(
            self.handle,
            video_session,
            bind_session_memory_info_count as _,
            bind_session_memory_infos.as_ptr(),
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginVideoCodingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html)
pub unsafe fn cmd_begin_video_coding_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_begin_info: *const VideoBeginCodingInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_video_coding_khr
        .unwrap())(command_buffer, p_begin_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html)
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR,
    ) {
        let cmd_begin_video_coding_khr = (*self.table)
            .cmd_begin_video_coding_khr
            .unwrap();
        cmd_begin_video_coding_khr(command_buffer, begin_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdControlVideoCodingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html)
pub unsafe fn cmd_control_video_coding_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_coding_control_info: *const VideoCodingControlInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_control_video_coding_khr
        .unwrap())(command_buffer, p_coding_control_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html)
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR,
    ) {
        let cmd_control_video_coding_khr = (*self.table)
            .cmd_control_video_coding_khr
            .unwrap();
        cmd_control_video_coding_khr(command_buffer, coding_control_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndVideoCodingKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html)
pub unsafe fn cmd_end_video_coding_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_end_coding_info: *const VideoEndCodingInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_video_coding_khr
        .unwrap())(command_buffer, p_end_coding_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html)
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR,
    ) {
        let cmd_end_video_coding_khr = (*self.table).cmd_end_video_coding_khr.unwrap();
        cmd_end_video_coding_khr(command_buffer, end_coding_info as _);
    }
}
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 7;
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_video_queue"
);
