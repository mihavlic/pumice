#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265PictureInfoFlags.html)
pub struct StdVideoEncodeH265PictureInfoFlags {}
impl Default for StdVideoEncodeH265PictureInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265PictureInfo.html)
pub struct StdVideoEncodeH265PictureInfo {
    pub flags: StdVideoEncodeH265PictureInfoFlags,
    pub picture_type: crate::extensions::h265std::StdVideoH265PictureType,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
}
impl Default for StdVideoEncodeH265PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            picture_type: Default::default(),
            sps_video_parameter_set_id: Default::default(),
            pps_seq_parameter_set_id: Default::default(),
            pps_pic_parameter_set_id: Default::default(),
            pic_order_cnt_val: Default::default(),
            temporal_id: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265SliceSegmentHeader.html)
pub struct StdVideoEncodeH265SliceSegmentHeader {
    pub flags: StdVideoEncodeH265SliceSegmentHeaderFlags,
    pub slice_type: crate::extensions::h265std::StdVideoH265SliceType,
    pub slice_segment_address: u32,
    pub short_term_ref_pic_set_idx: u8,
    pub collocated_ref_idx: u8,
    pub num_ref_idx_l_0_active_minus_1: u8,
    pub num_ref_idx_l_1_active_minus_1: u8,
    pub max_num_merge_cand: u8,
    pub slice_cb_qp_offset: i8,
    pub slice_cr_qp_offset: i8,
    pub slice_beta_offset_div_2: i8,
    pub slice_tc_offset_div_2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub p_short_term_ref_pic_set: *const crate::extensions::h265std::StdVideoH265ShortTermRefPicSet,
    pub p_long_term_ref_pics: *const StdVideoEncodeH265SliceSegmentLongTermRefPics,
    pub p_weight_table: *const StdVideoEncodeH265WeightTable,
}
impl Default for StdVideoEncodeH265SliceSegmentHeader {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            slice_type: Default::default(),
            slice_segment_address: Default::default(),
            short_term_ref_pic_set_idx: Default::default(),
            collocated_ref_idx: Default::default(),
            num_ref_idx_l_0_active_minus_1: Default::default(),
            num_ref_idx_l_1_active_minus_1: Default::default(),
            max_num_merge_cand: Default::default(),
            slice_cb_qp_offset: Default::default(),
            slice_cr_qp_offset: Default::default(),
            slice_beta_offset_div_2: Default::default(),
            slice_tc_offset_div_2: Default::default(),
            slice_act_y_qp_offset: Default::default(),
            slice_act_cb_qp_offset: Default::default(),
            slice_act_cr_qp_offset: Default::default(),
            p_short_term_ref_pic_set: std::ptr::null(),
            p_long_term_ref_pics: std::ptr::null(),
            p_weight_table: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265ReferenceInfo.html)
pub struct StdVideoEncodeH265ReferenceInfo {
    pub flags: StdVideoEncodeH265ReferenceInfoFlags,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
}
impl Default for StdVideoEncodeH265ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pic_order_cnt_val: Default::default(),
            temporal_id: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265ReferenceModifications.html)
pub struct StdVideoEncodeH265ReferenceModifications {
    pub flags: StdVideoEncodeH265ReferenceModificationFlags,
    pub reference_list_0_modifications_count: u8,
    pub p_reference_list_0_modifications: *const u8,
    pub reference_list_1_modifications_count: u8,
    pub p_reference_list_1_modifications: *const u8,
}
impl Default for StdVideoEncodeH265ReferenceModifications {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            reference_list_0_modifications_count: Default::default(),
            p_reference_list_0_modifications: std::ptr::null(),
            reference_list_1_modifications_count: Default::default(),
            p_reference_list_1_modifications: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265SliceSegmentHeaderFlags.html)
pub struct StdVideoEncodeH265SliceSegmentHeaderFlags {}
impl Default for StdVideoEncodeH265SliceSegmentHeaderFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265ReferenceInfoFlags.html)
pub struct StdVideoEncodeH265ReferenceInfoFlags {}
impl Default for StdVideoEncodeH265ReferenceInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265ReferenceModificationFlags.html)
pub struct StdVideoEncodeH265ReferenceModificationFlags {}
impl Default for StdVideoEncodeH265ReferenceModificationFlags {
    fn default() -> Self {
        Self {}
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_0_9_9: u32 = crate::extensions::video_common::make_video_std_version(
    0,
    9,
    9,
);
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265WeightTableFlags.html)
pub struct StdVideoEncodeH265WeightTableFlags {
    pub luma_weight_l_0_flag: u16,
    pub chroma_weight_l_0_flag: u16,
    pub luma_weight_l_1_flag: u16,
    pub chroma_weight_l_1_flag: u16,
}
impl Default for StdVideoEncodeH265WeightTableFlags {
    fn default() -> Self {
        Self {
            luma_weight_l_0_flag: Default::default(),
            chroma_weight_l_0_flag: Default::default(),
            luma_weight_l_1_flag: Default::default(),
            chroma_weight_l_1_flag: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265WeightTable.html)
pub struct StdVideoEncodeH265WeightTable {
    pub flags: StdVideoEncodeH265WeightTableFlags,
    pub luma_log_2_weight_denom: u8,
    pub delta_chroma_log_2_weight_denom: i8,
    pub delta_luma_weight_l_0: [i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF
        as usize],
    pub luma_offset_l_0: [i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF
        as usize],
    pub delta_chroma_weight_l_0: [[i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l_0: [[i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_luma_weight_l_1: [i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF
        as usize],
    pub luma_offset_l_1: [i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF
        as usize],
    pub delta_chroma_weight_l_1: [[i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l_1: [[i8; crate::extensions::h265std::STD_VIDEO_H265_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h265std::STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
}
impl Default for StdVideoEncodeH265WeightTable {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            luma_log_2_weight_denom: Default::default(),
            delta_chroma_log_2_weight_denom: Default::default(),
            delta_luma_weight_l_0: unsafe { std::mem::zeroed() },
            luma_offset_l_0: unsafe { std::mem::zeroed() },
            delta_chroma_weight_l_0: unsafe { std::mem::zeroed() },
            delta_chroma_offset_l_0: unsafe { std::mem::zeroed() },
            delta_luma_weight_l_1: unsafe { std::mem::zeroed() },
            luma_offset_l_1: unsafe { std::mem::zeroed() },
            delta_chroma_weight_l_1: unsafe { std::mem::zeroed() },
            delta_chroma_offset_l_1: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH265SliceSegmentLongTermRefPics.html)
pub struct StdVideoEncodeH265SliceSegmentLongTermRefPics {
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub lt_idx_sps: [u8; crate::extensions::h265std::STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS
        as usize],
    pub poc_lsb_lt: [u8; crate::extensions::h265std::STD_VIDEO_H265_MAX_LONG_TERM_PICS
        as usize],
    pub used_by_curr_pic_lt_flag: u16,
    pub delta_poc_msb_present_flag: [u8; crate::extensions::h265std::STD_VIDEO_H265_MAX_DELTA_POC
        as usize],
    pub delta_poc_msb_cycle_lt: [u8; crate::extensions::h265std::STD_VIDEO_H265_MAX_DELTA_POC
        as usize],
}
impl Default for StdVideoEncodeH265SliceSegmentLongTermRefPics {
    fn default() -> Self {
        Self {
            num_long_term_sps: Default::default(),
            num_long_term_pics: Default::default(),
            lt_idx_sps: unsafe { std::mem::zeroed() },
            poc_lsb_lt: unsafe { std::mem::zeroed() },
            used_by_curr_pic_lt_flag: Default::default(),
            delta_poc_msb_present_flag: unsafe { std::mem::zeroed() },
            delta_poc_msb_cycle_lt: unsafe { std::mem::zeroed() },
        }
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION: u32 = STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_0_9_9;
pub const STD_VULKAN_VIDEO_CODEC_H265_ENCODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_STD_vulkan_video_codec_h265_encode"
);
