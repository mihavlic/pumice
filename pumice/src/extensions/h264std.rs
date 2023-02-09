/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264ProfileIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264ProfileIdc(pub i32);
impl StdVideoH264ProfileIdc {
    pub const BASELINE: Self = Self(66);
    pub const MAIN: Self = Self(77);
    pub const HIGH: Self = Self(100);
    pub const HIGH_444_PREDICTIVE: Self = Self(244);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264ProfileIdc : i32, BASELINE, MAIN, HIGH, HIGH_444_PREDICTIVE, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264LevelIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264LevelIdc(pub i32);
impl StdVideoH264LevelIdc {
    pub const I1_0: Self = Self(0);
    pub const I1_1: Self = Self(1);
    pub const I1_2: Self = Self(2);
    pub const I1_3: Self = Self(3);
    pub const I2_0: Self = Self(4);
    pub const I2_1: Self = Self(5);
    pub const I2_2: Self = Self(6);
    pub const I3_0: Self = Self(7);
    pub const I3_1: Self = Self(8);
    pub const I3_2: Self = Self(9);
    pub const I4_0: Self = Self(10);
    pub const I4_1: Self = Self(11);
    pub const I4_2: Self = Self(12);
    pub const I5_0: Self = Self(13);
    pub const I5_1: Self = Self(14);
    pub const I5_2: Self = Self(15);
    pub const I6_0: Self = Self(16);
    pub const I6_1: Self = Self(17);
    pub const I6_2: Self = Self(18);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264LevelIdc : i32, I1_0, I1_1, I1_2, I1_3, I2_0, I2_1, I2_2, I3_0, I3_1,
    I3_2, I4_0, I4_1, I4_2, I5_0, I5_1, I5_2, I6_0, I6_1, I6_2, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264ChromaFormatIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264ChromaFormatIdc(pub i32);
impl StdVideoH264ChromaFormatIdc {
    pub const MONOCHROME: Self = Self(0);
    pub const I420: Self = Self(1);
    pub const I422: Self = Self(2);
    pub const I444: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264ChromaFormatIdc : i32, MONOCHROME, I420, I422, I444, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264PocType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264PocType(pub i32);
impl StdVideoH264PocType {
    pub const T0: Self = Self(0);
    pub const T1: Self = Self(1);
    pub const T2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264PocType : i32, T0, T1, T2, INVALID
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264SpsFlags.html)
pub struct StdVideoH264SpsFlags {}
impl Default for StdVideoH264SpsFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264ScalingLists.html)
pub struct StdVideoH264ScalingLists {
    pub scaling_list_present_mask: u16,
    pub use_default_scaling_matrix_mask: u16,
    pub scaling_list_4x_4: [[u8; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS
        as usize]; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub scaling_list_8x_8: [[u8; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS
        as usize]; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS as usize],
}
impl Default for StdVideoH264ScalingLists {
    fn default() -> Self {
        Self {
            scaling_list_present_mask: Default::default(),
            use_default_scaling_matrix_mask: Default::default(),
            scaling_list_4x_4: unsafe { std::mem::zeroed() },
            scaling_list_8x_8: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264SequenceParameterSetVui.html)
pub struct StdVideoH264SequenceParameterSetVui {
    pub flags: StdVideoH264SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved_1: u32,
    pub p_hrd_parameters: *const StdVideoH264HrdParameters,
}
impl Default for StdVideoH264SequenceParameterSetVui {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            aspect_ratio_idc: Default::default(),
            sar_width: Default::default(),
            sar_height: Default::default(),
            video_format: Default::default(),
            colour_primaries: Default::default(),
            transfer_characteristics: Default::default(),
            matrix_coefficients: Default::default(),
            num_units_in_tick: Default::default(),
            time_scale: Default::default(),
            max_num_reorder_frames: Default::default(),
            max_dec_frame_buffering: Default::default(),
            chroma_sample_loc_type_top_field: Default::default(),
            chroma_sample_loc_type_bottom_field: Default::default(),
            reserved_1: Default::default(),
            p_hrd_parameters: std::ptr::null(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264AspectRatioIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264AspectRatioIdc(pub i32);
impl StdVideoH264AspectRatioIdc {
    pub const UNSPECIFIED: Self = Self(0);
    pub const SQUARE: Self = Self(1);
    pub const I12_11: Self = Self(2);
    pub const I10_11: Self = Self(3);
    pub const I16_11: Self = Self(4);
    pub const I40_33: Self = Self(5);
    pub const I24_11: Self = Self(6);
    pub const I20_11: Self = Self(7);
    pub const I32_11: Self = Self(8);
    pub const I80_33: Self = Self(9);
    pub const I18_11: Self = Self(10);
    pub const I15_11: Self = Self(11);
    pub const I64_33: Self = Self(12);
    pub const I160_99: Self = Self(13);
    pub const I4_3: Self = Self(14);
    pub const I3_2: Self = Self(15);
    pub const I2_1: Self = Self(16);
    pub const EXTENDED_SAR: Self = Self(255);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264AspectRatioIdc : i32, UNSPECIFIED, SQUARE, I12_11, I10_11, I16_11,
    I40_33, I24_11, I20_11, I32_11, I80_33, I18_11, I15_11, I64_33, I160_99, I4_3, I3_2,
    I2_1, EXTENDED_SAR, INVALID
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264HrdParameters.html)
pub struct StdVideoH264HrdParameters {
    pub cpb_cnt_minus_1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub reserved_1: u8,
    pub bit_rate_value_minus_1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus_1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cbr_flag: [u8; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub initial_cpb_removal_delay_length_minus_1: u32,
    pub cpb_removal_delay_length_minus_1: u32,
    pub dpb_output_delay_length_minus_1: u32,
    pub time_offset_length: u32,
}
impl Default for StdVideoH264HrdParameters {
    fn default() -> Self {
        Self {
            cpb_cnt_minus_1: Default::default(),
            bit_rate_scale: Default::default(),
            cpb_size_scale: Default::default(),
            reserved_1: Default::default(),
            bit_rate_value_minus_1: unsafe { std::mem::zeroed() },
            cpb_size_value_minus_1: unsafe { std::mem::zeroed() },
            cbr_flag: unsafe { std::mem::zeroed() },
            initial_cpb_removal_delay_length_minus_1: Default::default(),
            cpb_removal_delay_length_minus_1: Default::default(),
            dpb_output_delay_length_minus_1: Default::default(),
            time_offset_length: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264SpsVuiFlags.html)
pub struct StdVideoH264SpsVuiFlags {}
impl Default for StdVideoH264SpsVuiFlags {
    fn default() -> Self {
        Self {}
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264WeightedBipredIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264WeightedBipredIdc(pub i32);
impl StdVideoH264WeightedBipredIdc {
    pub const DEFAULT: Self = Self(0);
    pub const EXPLICIT: Self = Self(1);
    pub const IMPLICIT: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264WeightedBipredIdc : i32, DEFAULT, EXPLICIT, IMPLICIT, INVALID
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264PpsFlags.html)
pub struct StdVideoH264PpsFlags {}
impl Default for StdVideoH264PpsFlags {
    fn default() -> Self {
        Self {}
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264SliceType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264SliceType(pub i32);
impl StdVideoH264SliceType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264SliceType : i32, P, B, I, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264CabacInitIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264CabacInitIdc(pub i32);
impl StdVideoH264CabacInitIdc {
    pub const I0: Self = Self(0);
    pub const I1: Self = Self(1);
    pub const I2: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264CabacInitIdc : i32, I0, I1, I2, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264DisableDeblockingFilterIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264DisableDeblockingFilterIdc(pub i32);
impl StdVideoH264DisableDeblockingFilterIdc {
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
    pub const PARTIAL: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264DisableDeblockingFilterIdc : i32, DISABLED, ENABLED, PARTIAL, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264PictureType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264PictureType(pub i32);
impl StdVideoH264PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(5);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264PictureType : i32, P, B, I, IDR, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264ModificationOfPicNumsIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264ModificationOfPicNumsIdc(pub i32);
impl StdVideoH264ModificationOfPicNumsIdc {
    pub const SHORT_TERM_SUBTRACT: Self = Self(0);
    pub const SHORT_TERM_ADD: Self = Self(1);
    pub const LONG_TERM: Self = Self(2);
    pub const END: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264ModificationOfPicNumsIdc : i32, SHORT_TERM_SUBTRACT, SHORT_TERM_ADD,
    LONG_TERM, END, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264MemMgmtControlOp.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264MemMgmtControlOp(pub i32);
impl StdVideoH264MemMgmtControlOp {
    pub const END: Self = Self(0);
    pub const UNMARK_SHORT_TERM: Self = Self(1);
    pub const UNMARK_LONG_TERM: Self = Self(2);
    pub const MARK_LONG_TERM: Self = Self(3);
    pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
    pub const UNMARK_ALL: Self = Self(5);
    pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264MemMgmtControlOp : i32, END, UNMARK_SHORT_TERM, UNMARK_LONG_TERM,
    MARK_LONG_TERM, SET_MAX_LONG_TERM_INDEX, UNMARK_ALL, MARK_CURRENT_AS_LONG_TERM,
    INVALID
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264SequenceParameterSet.html)
pub struct StdVideoH264SequenceParameterSet {
    pub flags: StdVideoH264SpsFlags,
    pub profile_idc: StdVideoH264ProfileIdc,
    pub level_idc: StdVideoH264LevelIdc,
    pub chroma_format_idc: StdVideoH264ChromaFormatIdc,
    pub seq_parameter_set_id: u8,
    pub bit_depth_luma_minus_8: u8,
    pub bit_depth_chroma_minus_8: u8,
    pub log_2_max_frame_num_minus_4: u8,
    pub pic_order_cnt_type: StdVideoH264PocType,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub log_2_max_pic_order_cnt_lsb_minus_4: u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    pub reserved_1: u8,
    pub pic_width_in_mbs_minus_1: u32,
    pub pic_height_in_map_units_minus_1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub reserved_2: u32,
    pub p_offset_for_ref_frame: *const i32,
    pub p_scaling_lists: *const StdVideoH264ScalingLists,
    pub p_sequence_parameter_set_vui: *const StdVideoH264SequenceParameterSetVui,
}
impl Default for StdVideoH264SequenceParameterSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            profile_idc: Default::default(),
            level_idc: Default::default(),
            chroma_format_idc: Default::default(),
            seq_parameter_set_id: Default::default(),
            bit_depth_luma_minus_8: Default::default(),
            bit_depth_chroma_minus_8: Default::default(),
            log_2_max_frame_num_minus_4: Default::default(),
            pic_order_cnt_type: Default::default(),
            offset_for_non_ref_pic: Default::default(),
            offset_for_top_to_bottom_field: Default::default(),
            log_2_max_pic_order_cnt_lsb_minus_4: Default::default(),
            num_ref_frames_in_pic_order_cnt_cycle: Default::default(),
            max_num_ref_frames: Default::default(),
            reserved_1: Default::default(),
            pic_width_in_mbs_minus_1: Default::default(),
            pic_height_in_map_units_minus_1: Default::default(),
            frame_crop_left_offset: Default::default(),
            frame_crop_right_offset: Default::default(),
            frame_crop_top_offset: Default::default(),
            frame_crop_bottom_offset: Default::default(),
            reserved_2: Default::default(),
            p_offset_for_ref_frame: std::ptr::null(),
            p_scaling_lists: std::ptr::null(),
            p_sequence_parameter_set_vui: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264PictureParameterSet.html)
pub struct StdVideoH264PictureParameterSet {
    pub flags: StdVideoH264PpsFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l_0_default_active_minus_1: u8,
    pub num_ref_idx_l_1_default_active_minus_1: u8,
    pub weighted_bipred_idc: StdVideoH264WeightedBipredIdc,
    pub pic_init_qp_minus_26: i8,
    pub pic_init_qs_minus_26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub p_scaling_lists: *const StdVideoH264ScalingLists,
}
impl Default for StdVideoH264PictureParameterSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: Default::default(),
            pic_parameter_set_id: Default::default(),
            num_ref_idx_l_0_default_active_minus_1: Default::default(),
            num_ref_idx_l_1_default_active_minus_1: Default::default(),
            weighted_bipred_idc: Default::default(),
            pic_init_qp_minus_26: Default::default(),
            pic_init_qs_minus_26: Default::default(),
            chroma_qp_index_offset: Default::default(),
            second_chroma_qp_index_offset: Default::default(),
            p_scaling_lists: std::ptr::null(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH264NonVclNaluType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH264NonVclNaluType(pub i32);
impl StdVideoH264NonVclNaluType {
    pub const SPS: Self = Self(0);
    pub const PPS: Self = Self(1);
    pub const AUD: Self = Self(2);
    pub const PREFIX: Self = Self(3);
    pub const END_OF_SEQUENCE: Self = Self(4);
    pub const END_OF_STREAM: Self = Self(5);
    pub const PRECODED: Self = Self(6);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH264NonVclNaluType : i32, SPS, PPS, AUD, PREFIX, END_OF_SEQUENCE,
    END_OF_STREAM, PRECODED, INVALID
}
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;
