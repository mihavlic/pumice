#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH264PictureInfo.html)
pub struct StdVideoDecodeH264PictureInfo {
    pub flags: StdVideoDecodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub frame_num: u16,
    pub idr_pic_id: u16,
    pub pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
impl Default for StdVideoDecodeH264PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: Default::default(),
            pic_parameter_set_id: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            frame_num: Default::default(),
            idr_pic_id: Default::default(),
            pic_order_cnt: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH264ReferenceInfo.html)
pub struct StdVideoDecodeH264ReferenceInfo {
    pub flags: StdVideoDecodeH264ReferenceInfoFlags,
    pub frame_num: u16,
    pub reserved: u16,
    pub pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
impl Default for StdVideoDecodeH264ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_num: Default::default(),
            reserved: Default::default(),
            pic_order_cnt: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH264PictureInfoFlags.html)
pub struct StdVideoDecodeH264PictureInfoFlags {}
impl Default for StdVideoDecodeH264PictureInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH264ReferenceInfoFlags.html)
pub struct StdVideoDecodeH264ReferenceInfoFlags {}
impl Default for StdVideoDecodeH264ReferenceInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_0_9_8: u32 = crate::extensions::video_common::make_video_std_version(
    0,
    9,
    8,
);
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH264FieldOrderCount.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoDecodeH264FieldOrderCount(pub i32);
impl StdVideoDecodeH264FieldOrderCount {
    pub const TOP: Self = Self(0);
    pub const BOTTOM: Self = Self(1);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoDecodeH264FieldOrderCount : i32, TOP, BOTTOM, INVALID
}
pub const STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION: u32 = STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_0_9_8;
pub const STD_VULKAN_VIDEO_CODEC_H264_DECODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_STD_vulkan_video_codec_h264_decode"
);
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;
