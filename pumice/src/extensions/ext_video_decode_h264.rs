#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264ProfileInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264ProfileInfoEXT.html)
pub struct VideoDecodeH264ProfileInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_profile_idc: crate::extensions::h264std::StdVideoH264ProfileIdc,
    pub picture_layout: VideoDecodeH264PictureLayoutFlagsEXT,
}
impl Default for VideoDecodeH264ProfileInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_PROFILE_INFO_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: Default::default(),
            picture_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264CapabilitiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264CapabilitiesEXT.html)
pub struct VideoDecodeH264CapabilitiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_level_idc: crate::extensions::h264std::StdVideoH264LevelIdc,
    pub field_offset_granularity: crate::vk10::Offset2D,
}
impl Default for VideoDecodeH264CapabilitiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_CAPABILITIES_EXT,
            p_next: std::ptr::null_mut(),
            max_level_idc: Default::default(),
            field_offset_granularity: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264SessionParametersAddInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersAddInfoEXT.html)
pub struct VideoDecodeH264SessionParametersAddInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_spscount: u32,
    pub p_std_spss: *const crate::extensions::h264std::StdVideoH264SequenceParameterSet,
    pub std_ppscount: u32,
    pub p_std_ppss: *const crate::extensions::h264std::StdVideoH264PictureParameterSet,
}
impl Default for VideoDecodeH264SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT,
            p_next: std::ptr::null(),
            std_spscount: Default::default(),
            p_std_spss: std::ptr::null(),
            std_ppscount: Default::default(),
            p_std_ppss: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264SessionParametersCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264SessionParametersCreateInfoEXT.html)
pub struct VideoDecodeH264SessionParametersCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_std_spscount: u32,
    pub max_std_ppscount: u32,
    pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoEXT,
}
impl Default for VideoDecodeH264SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_std_spscount: Default::default(),
            max_std_ppscount: Default::default(),
            p_parameters_add_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264PictureInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureInfoEXT.html)
pub struct VideoDecodeH264PictureInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_std_picture_info: *const crate::extensions::h264std_decode::StdVideoDecodeH264PictureInfo,
    pub slice_count: u32,
    pub p_slice_offsets: *const u32,
}
impl Default for VideoDecodeH264PictureInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_PICTURE_INFO_EXT,
            p_next: std::ptr::null(),
            p_std_picture_info: std::ptr::null(),
            slice_count: Default::default(),
            p_slice_offsets: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH264DpbSlotInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264DpbSlotInfoEXT.html)
pub struct VideoDecodeH264DpbSlotInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_std_reference_info: *const crate::extensions::h264std_decode::StdVideoDecodeH264ReferenceInfo,
}
impl Default for VideoDecodeH264DpbSlotInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT,
            p_next: std::ptr::null(),
            p_std_reference_info: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkVideoDecodeH264PictureLayoutFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoDecodeH264PictureLayoutFlagsEXT(pub u32);
impl VideoDecodeH264PictureLayoutFlagsEXT {
    pub const PROGRESSIVE: Self = Self(0);
    pub const INTERLACED_INTERLEAVED_LINES: Self = Self(1 << 0);
    pub const INTERLACED_SEPARATE_PLANES: Self = Self(1 << 1);
}
crate::bitflags_impl! {
    VideoDecodeH264PictureLayoutFlagsEXT : u32, 0x3, PROGRESSIVE,
    INTERLACED_INTERLEAVED_LINES, INTERLACED_SEPARATE_PLANES
}
pub const EXT_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 7;
pub const EXT_VIDEO_DECODE_H264_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_video_decode_h264"
);
