#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH265PictureInfo.html)
pub struct StdVideoDecodeH265PictureInfo {
    pub flags: StdVideoDecodeH265PictureInfoFlags,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub num_delta_pocs_of_ref_rps_idx: u8,
    pub pic_order_cnt_val: i32,
    pub num_bits_for_stref_pic_set_in_slice: u16,
    pub reserved: u16,
    pub ref_pic_set_st_curr_before: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE
        as usize],
    pub ref_pic_set_st_curr_after: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE
        as usize],
    pub ref_pic_set_lt_curr: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
}
impl Default for StdVideoDecodeH265PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            sps_video_parameter_set_id: Default::default(),
            pps_seq_parameter_set_id: Default::default(),
            pps_pic_parameter_set_id: Default::default(),
            num_delta_pocs_of_ref_rps_idx: Default::default(),
            pic_order_cnt_val: Default::default(),
            num_bits_for_stref_pic_set_in_slice: Default::default(),
            reserved: Default::default(),
            ref_pic_set_st_curr_before: unsafe { std::mem::zeroed() },
            ref_pic_set_st_curr_after: unsafe { std::mem::zeroed() },
            ref_pic_set_lt_curr: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH265ReferenceInfo.html)
pub struct StdVideoDecodeH265ReferenceInfo {
    pub flags: StdVideoDecodeH265ReferenceInfoFlags,
    pub pic_order_cnt_val: i32,
}
impl Default for StdVideoDecodeH265ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pic_order_cnt_val: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH265PictureInfoFlags.html)
pub struct StdVideoDecodeH265PictureInfoFlags {}
impl Default for StdVideoDecodeH265PictureInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoDecodeH265ReferenceInfoFlags.html)
pub struct StdVideoDecodeH265ReferenceInfoFlags {}
impl Default for StdVideoDecodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_0_9_9: u32 = crate::extensions::video_common::make_video_std_version(
    0,
    9,
    9,
);
pub const STD_VULKAN_VIDEO_CODEC_H265_DECODE_SPEC_VERSION: u32 = STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_0_9_9;
pub const STD_VULKAN_VIDEO_CODEC_H265_DECODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_STD_vulkan_video_codec_h265_decode"
);
pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;
