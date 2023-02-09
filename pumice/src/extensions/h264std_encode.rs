#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264SliceHeader.html)
pub struct StdVideoEncodeH264SliceHeader {
    pub flags: StdVideoEncodeH264SliceHeaderFlags,
    pub first_mb_in_slice: u32,
    pub slice_type: crate::extensions::h264std::StdVideoH264SliceType,
    pub idr_pic_id: u16,
    pub num_ref_idx_l_0_active_minus_1: u8,
    pub num_ref_idx_l_1_active_minus_1: u8,
    pub cabac_init_idc: crate::extensions::h264std::StdVideoH264CabacInitIdc,
    pub disable_deblocking_filter_idc: crate::extensions::h264std::StdVideoH264DisableDeblockingFilterIdc,
    pub slice_alpha_c_0_offset_div_2: i8,
    pub slice_beta_offset_div_2: i8,
    pub p_weight_table: *const StdVideoEncodeH264WeightTable,
}
impl Default for StdVideoEncodeH264SliceHeader {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            first_mb_in_slice: Default::default(),
            slice_type: Default::default(),
            idr_pic_id: Default::default(),
            num_ref_idx_l_0_active_minus_1: Default::default(),
            num_ref_idx_l_1_active_minus_1: Default::default(),
            cabac_init_idc: Default::default(),
            disable_deblocking_filter_idc: Default::default(),
            slice_alpha_c_0_offset_div_2: Default::default(),
            slice_beta_offset_div_2: Default::default(),
            p_weight_table: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264PictureInfo.html)
pub struct StdVideoEncodeH264PictureInfo {
    pub flags: StdVideoEncodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub picture_type: crate::extensions::h264std::StdVideoH264PictureType,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
}
impl Default for StdVideoEncodeH264PictureInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: Default::default(),
            pic_parameter_set_id: Default::default(),
            picture_type: Default::default(),
            frame_num: Default::default(),
            pic_order_cnt: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264ReferenceInfo.html)
pub struct StdVideoEncodeH264ReferenceInfo {
    pub flags: StdVideoEncodeH264ReferenceInfoFlags,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
}
impl Default for StdVideoEncodeH264ReferenceInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            frame_num: Default::default(),
            pic_order_cnt: Default::default(),
            long_term_pic_num: Default::default(),
            long_term_frame_idx: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264SliceHeaderFlags.html)
pub struct StdVideoEncodeH264SliceHeaderFlags {}
impl Default for StdVideoEncodeH264SliceHeaderFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264RefMemMgmtCtrlOperations.html)
pub struct StdVideoEncodeH264RefMemMgmtCtrlOperations {
    pub flags: StdVideoEncodeH264RefMgmtFlags,
    pub ref_list_0_mod_op_count: u8,
    pub p_ref_list_0_mod_operations: *const StdVideoEncodeH264RefListModEntry,
    pub ref_list_1_mod_op_count: u8,
    pub p_ref_list_1_mod_operations: *const StdVideoEncodeH264RefListModEntry,
    pub ref_pic_marking_op_count: u8,
    pub p_ref_pic_marking_operations: *const StdVideoEncodeH264RefPicMarkingEntry,
}
impl Default for StdVideoEncodeH264RefMemMgmtCtrlOperations {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            ref_list_0_mod_op_count: Default::default(),
            p_ref_list_0_mod_operations: std::ptr::null(),
            ref_list_1_mod_op_count: Default::default(),
            p_ref_list_1_mod_operations: std::ptr::null(),
            ref_pic_marking_op_count: Default::default(),
            p_ref_pic_marking_operations: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264PictureInfoFlags.html)
pub struct StdVideoEncodeH264PictureInfoFlags {}
impl Default for StdVideoEncodeH264PictureInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264ReferenceInfoFlags.html)
pub struct StdVideoEncodeH264ReferenceInfoFlags {}
impl Default for StdVideoEncodeH264ReferenceInfoFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264RefMgmtFlags.html)
pub struct StdVideoEncodeH264RefMgmtFlags {}
impl Default for StdVideoEncodeH264RefMgmtFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264RefListModEntry.html)
pub struct StdVideoEncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: crate::extensions::h264std::StdVideoH264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus_1: u16,
    pub long_term_pic_num: u16,
}
impl Default for StdVideoEncodeH264RefListModEntry {
    fn default() -> Self {
        Self {
            modification_of_pic_nums_idc: Default::default(),
            abs_diff_pic_num_minus_1: Default::default(),
            long_term_pic_num: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264RefPicMarkingEntry.html)
pub struct StdVideoEncodeH264RefPicMarkingEntry {
    pub operation: crate::extensions::h264std::StdVideoH264MemMgmtControlOp,
    pub difference_of_pic_nums_minus_1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus_1: u16,
}
impl Default for StdVideoEncodeH264RefPicMarkingEntry {
    fn default() -> Self {
        Self {
            operation: Default::default(),
            difference_of_pic_nums_minus_1: Default::default(),
            long_term_pic_num: Default::default(),
            long_term_frame_idx: Default::default(),
            max_long_term_frame_idx_plus_1: Default::default(),
        }
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_0_9_8: u32 = crate::extensions::video_common::make_video_std_version(
    0,
    9,
    8,
);
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264WeightTableFlags.html)
pub struct StdVideoEncodeH264WeightTableFlags {
    pub luma_weight_l_0_flag: u32,
    pub chroma_weight_l_0_flag: u32,
    pub luma_weight_l_1_flag: u32,
    pub chroma_weight_l_1_flag: u32,
}
impl Default for StdVideoEncodeH264WeightTableFlags {
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
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoEncodeH264WeightTable.html)
pub struct StdVideoEncodeH264WeightTable {
    pub flags: StdVideoEncodeH264WeightTableFlags,
    pub luma_log_2_weight_denom: u8,
    pub chroma_log_2_weight_denom: u8,
    pub luma_weight_l_0: [i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF
        as usize],
    pub luma_offset_l_0: [i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF
        as usize],
    pub chroma_weight_l_0: [[i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l_0: [[i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_weight_l_1: [i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF
        as usize],
    pub luma_offset_l_1: [i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF
        as usize],
    pub chroma_weight_l_1: [[i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l_1: [[i8; crate::extensions::h264std::STD_VIDEO_H264_MAX_CHROMA_PLANES
        as usize]; crate::extensions::h264std::STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
}
impl Default for StdVideoEncodeH264WeightTable {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            luma_log_2_weight_denom: Default::default(),
            chroma_log_2_weight_denom: Default::default(),
            luma_weight_l_0: unsafe { std::mem::zeroed() },
            luma_offset_l_0: unsafe { std::mem::zeroed() },
            chroma_weight_l_0: unsafe { std::mem::zeroed() },
            chroma_offset_l_0: unsafe { std::mem::zeroed() },
            luma_weight_l_1: unsafe { std::mem::zeroed() },
            luma_offset_l_1: unsafe { std::mem::zeroed() },
            chroma_weight_l_1: unsafe { std::mem::zeroed() },
            chroma_offset_l_1: unsafe { std::mem::zeroed() },
        }
    }
}
pub const STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION: u32 = STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_0_9_8;
pub const STD_VULKAN_VIDEO_CODEC_H264_ENCODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_STD_vulkan_video_codec_h264_encode"
);
