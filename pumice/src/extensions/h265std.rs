/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ProfileIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265ProfileIdc(pub i32);
impl StdVideoH265ProfileIdc {
    pub const MAIN: Self = Self(1);
    pub const MAIN_10: Self = Self(2);
    pub const MAIN_STILL_PICTURE: Self = Self(3);
    pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
    pub const SCC_EXTENSIONS: Self = Self(9);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH265ProfileIdc : i32, MAIN, MAIN_10, MAIN_STILL_PICTURE,
    FORMAT_RANGE_EXTENSIONS, SCC_EXTENSIONS, INVALID
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265VideoParameterSet.html)
pub struct StdVideoH265VideoParameterSet {
    pub flags: StdVideoH265VpsFlags,
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus_1: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus_1: u32,
    pub reserved_3: u32,
    pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
    pub p_hrd_parameters: *const StdVideoH265HrdParameters,
    pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
}
impl Default for StdVideoH265VideoParameterSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            vps_video_parameter_set_id: Default::default(),
            vps_max_sub_layers_minus_1: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            vps_num_units_in_tick: Default::default(),
            vps_time_scale: Default::default(),
            vps_num_ticks_poc_diff_one_minus_1: Default::default(),
            reserved_3: Default::default(),
            p_dec_pic_buf_mgr: std::ptr::null(),
            p_hrd_parameters: std::ptr::null(),
            p_profile_tier_level: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SequenceParameterSet.html)
pub struct StdVideoH265SequenceParameterSet {
    pub flags: StdVideoH265SpsFlags,
    pub chroma_format_idc: StdVideoH265ChromaFormatIdc,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus_1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub bit_depth_luma_minus_8: u8,
    pub bit_depth_chroma_minus_8: u8,
    pub log_2_max_pic_order_cnt_lsb_minus_4: u8,
    pub log_2_min_luma_coding_block_size_minus_3: u8,
    pub log_2_diff_max_min_luma_coding_block_size: u8,
    pub log_2_min_luma_transform_block_size_minus_2: u8,
    pub log_2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus_1: u8,
    pub pcm_sample_bit_depth_chroma_minus_1: u8,
    pub log_2_min_pcm_luma_coding_block_size_minus_3: u8,
    pub log_2_diff_max_min_pcm_luma_coding_block_size: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializers_minus_1: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
    pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
    pub p_scaling_lists: *const StdVideoH265ScalingLists,
    pub p_short_term_ref_pic_set: *const StdVideoH265ShortTermRefPicSet,
    pub p_long_term_ref_pics_sps: *const StdVideoH265LongTermRefPicsSps,
    pub p_sequence_parameter_set_vui: *const StdVideoH265SequenceParameterSetVui,
    pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
}
impl Default for StdVideoH265SequenceParameterSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            chroma_format_idc: Default::default(),
            pic_width_in_luma_samples: Default::default(),
            pic_height_in_luma_samples: Default::default(),
            sps_video_parameter_set_id: Default::default(),
            sps_max_sub_layers_minus_1: Default::default(),
            sps_seq_parameter_set_id: Default::default(),
            bit_depth_luma_minus_8: Default::default(),
            bit_depth_chroma_minus_8: Default::default(),
            log_2_max_pic_order_cnt_lsb_minus_4: Default::default(),
            log_2_min_luma_coding_block_size_minus_3: Default::default(),
            log_2_diff_max_min_luma_coding_block_size: Default::default(),
            log_2_min_luma_transform_block_size_minus_2: Default::default(),
            log_2_diff_max_min_luma_transform_block_size: Default::default(),
            max_transform_hierarchy_depth_inter: Default::default(),
            max_transform_hierarchy_depth_intra: Default::default(),
            num_short_term_ref_pic_sets: Default::default(),
            num_long_term_ref_pics_sps: Default::default(),
            pcm_sample_bit_depth_luma_minus_1: Default::default(),
            pcm_sample_bit_depth_chroma_minus_1: Default::default(),
            log_2_min_pcm_luma_coding_block_size_minus_3: Default::default(),
            log_2_diff_max_min_pcm_luma_coding_block_size: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            palette_max_size: Default::default(),
            delta_palette_max_predictor_size: Default::default(),
            motion_vector_resolution_control_idc: Default::default(),
            sps_num_palette_predictor_initializers_minus_1: Default::default(),
            conf_win_left_offset: Default::default(),
            conf_win_right_offset: Default::default(),
            conf_win_top_offset: Default::default(),
            conf_win_bottom_offset: Default::default(),
            p_profile_tier_level: std::ptr::null(),
            p_dec_pic_buf_mgr: std::ptr::null(),
            p_scaling_lists: std::ptr::null(),
            p_short_term_ref_pic_set: std::ptr::null(),
            p_long_term_ref_pics_sps: std::ptr::null(),
            p_sequence_parameter_set_vui: std::ptr::null(),
            p_predictor_palette_entries: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265PictureParameterSet.html)
pub struct StdVideoH265PictureParameterSet {
    pub flags: StdVideoH265PpsFlags,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub sps_video_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l_0_default_active_minus_1: u8,
    pub num_ref_idx_l_1_default_active_minus_1: u8,
    pub init_qp_minus_26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_beta_offset_div_2: i8,
    pub pps_tc_offset_div_2: i8,
    pub log_2_parallel_merge_level_minus_2: u8,
    pub log_2_max_transform_skip_block_size_minus_2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus_1: u8,
    pub cb_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub cr_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
    pub log_2_sao_offset_scale_luma: u8,
    pub log_2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus_5: i8,
    pub pps_act_cb_qp_offset_plus_5: i8,
    pub pps_act_cr_qp_offset_plus_3: i8,
    pub pps_num_palette_predictor_initializers: u8,
    pub luma_bit_depth_entry_minus_8: u8,
    pub chroma_bit_depth_entry_minus_8: u8,
    pub num_tile_columns_minus_1: u8,
    pub num_tile_rows_minus_1: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub column_width_minus_1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE
        as usize],
    pub row_height_minus_1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE
        as usize],
    pub reserved_3: u32,
    pub p_scaling_lists: *const StdVideoH265ScalingLists,
    pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
}
impl Default for StdVideoH265PictureParameterSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pps_pic_parameter_set_id: Default::default(),
            pps_seq_parameter_set_id: Default::default(),
            sps_video_parameter_set_id: Default::default(),
            num_extra_slice_header_bits: Default::default(),
            num_ref_idx_l_0_default_active_minus_1: Default::default(),
            num_ref_idx_l_1_default_active_minus_1: Default::default(),
            init_qp_minus_26: Default::default(),
            diff_cu_qp_delta_depth: Default::default(),
            pps_cb_qp_offset: Default::default(),
            pps_cr_qp_offset: Default::default(),
            pps_beta_offset_div_2: Default::default(),
            pps_tc_offset_div_2: Default::default(),
            log_2_parallel_merge_level_minus_2: Default::default(),
            log_2_max_transform_skip_block_size_minus_2: Default::default(),
            diff_cu_chroma_qp_offset_depth: Default::default(),
            chroma_qp_offset_list_len_minus_1: Default::default(),
            cb_qp_offset_list: unsafe { std::mem::zeroed() },
            cr_qp_offset_list: unsafe { std::mem::zeroed() },
            log_2_sao_offset_scale_luma: Default::default(),
            log_2_sao_offset_scale_chroma: Default::default(),
            pps_act_y_qp_offset_plus_5: Default::default(),
            pps_act_cb_qp_offset_plus_5: Default::default(),
            pps_act_cr_qp_offset_plus_3: Default::default(),
            pps_num_palette_predictor_initializers: Default::default(),
            luma_bit_depth_entry_minus_8: Default::default(),
            chroma_bit_depth_entry_minus_8: Default::default(),
            num_tile_columns_minus_1: Default::default(),
            num_tile_rows_minus_1: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            column_width_minus_1: unsafe { std::mem::zeroed() },
            row_height_minus_1: unsafe { std::mem::zeroed() },
            reserved_3: Default::default(),
            p_scaling_lists: std::ptr::null(),
            p_predictor_palette_entries: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265DecPicBufMgr.html)
pub struct StdVideoH265DecPicBufMgr {
    pub max_latency_increase_plus_1: [u32; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub max_dec_pic_buffering_minus_1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub max_num_reorder_pics: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
}
impl Default for StdVideoH265DecPicBufMgr {
    fn default() -> Self {
        Self {
            max_latency_increase_plus_1: unsafe { std::mem::zeroed() },
            max_dec_pic_buffering_minus_1: unsafe { std::mem::zeroed() },
            max_num_reorder_pics: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265HrdParameters.html)
pub struct StdVideoH265HrdParameters {
    pub flags: StdVideoH265HrdFlags,
    pub tick_divisor_minus_2: u8,
    pub du_cpb_removal_delay_increment_length_minus_1: u8,
    pub dpb_output_delay_du_length_minus_1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus_1: u8,
    pub au_cpb_removal_delay_length_minus_1: u8,
    pub dpb_output_delay_length_minus_1: u8,
    pub cpb_cnt_minus_1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    pub elemental_duration_in_tc_minus_1: [u16; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE
        as usize],
    pub reserved: [u16; 3],
    pub p_sub_layer_hrd_parameters_nal: *const StdVideoH265SubLayerHrdParameters,
    pub p_sub_layer_hrd_parameters_vcl: *const StdVideoH265SubLayerHrdParameters,
}
impl Default for StdVideoH265HrdParameters {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            tick_divisor_minus_2: Default::default(),
            du_cpb_removal_delay_increment_length_minus_1: Default::default(),
            dpb_output_delay_du_length_minus_1: Default::default(),
            bit_rate_scale: Default::default(),
            cpb_size_scale: Default::default(),
            cpb_size_du_scale: Default::default(),
            initial_cpb_removal_delay_length_minus_1: Default::default(),
            au_cpb_removal_delay_length_minus_1: Default::default(),
            dpb_output_delay_length_minus_1: Default::default(),
            cpb_cnt_minus_1: unsafe { std::mem::zeroed() },
            elemental_duration_in_tc_minus_1: unsafe { std::mem::zeroed() },
            reserved: unsafe { std::mem::zeroed() },
            p_sub_layer_hrd_parameters_nal: std::ptr::null(),
            p_sub_layer_hrd_parameters_vcl: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265VpsFlags.html)
pub struct StdVideoH265VpsFlags {}
impl Default for StdVideoH265VpsFlags {
    fn default() -> Self {
        Self {}
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265LevelIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265LevelIdc(pub i32);
impl StdVideoH265LevelIdc {
    pub const I1_0: Self = Self(0);
    pub const I2_0: Self = Self(1);
    pub const I2_1: Self = Self(2);
    pub const I3_0: Self = Self(3);
    pub const I3_1: Self = Self(4);
    pub const I4_0: Self = Self(5);
    pub const I4_1: Self = Self(6);
    pub const I5_0: Self = Self(7);
    pub const I5_1: Self = Self(8);
    pub const I5_2: Self = Self(9);
    pub const I6_0: Self = Self(10);
    pub const I6_1: Self = Self(11);
    pub const I6_2: Self = Self(12);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH265LevelIdc : i32, I1_0, I2_0, I2_1, I3_0, I3_1, I4_0, I4_1, I5_0, I5_1,
    I5_2, I6_0, I6_1, I6_2, INVALID
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SpsFlags.html)
pub struct StdVideoH265SpsFlags {}
impl Default for StdVideoH265SpsFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ScalingLists.html)
pub struct StdVideoH265ScalingLists {
    pub scaling_list_4x_4: [[u8; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS
        as usize]; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub scaling_list_8x_8: [[u8; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS
        as usize]; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS as usize],
    pub scaling_list_16x_16: [[u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS
        as usize]; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
    pub scaling_list_32x_32: [[u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS
        as usize]; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
    pub scaling_list_dccoef_16x_16: [u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS
        as usize],
    pub scaling_list_dccoef_32x_32: [u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS
        as usize],
}
impl Default for StdVideoH265ScalingLists {
    fn default() -> Self {
        Self {
            scaling_list_4x_4: unsafe { std::mem::zeroed() },
            scaling_list_8x_8: unsafe { std::mem::zeroed() },
            scaling_list_16x_16: unsafe { std::mem::zeroed() },
            scaling_list_32x_32: unsafe { std::mem::zeroed() },
            scaling_list_dccoef_16x_16: unsafe { std::mem::zeroed() },
            scaling_list_dccoef_32x_32: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SequenceParameterSetVui.html)
pub struct StdVideoH265SequenceParameterSetVui {
    pub flags: StdVideoH265SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH265AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus_1: u32,
    pub min_spatial_segmentation_idc: u16,
    pub reserved_3: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log_2_max_mv_length_horizontal: u8,
    pub log_2_max_mv_length_vertical: u8,
    pub p_hrd_parameters: *const StdVideoH265HrdParameters,
}
impl Default for StdVideoH265SequenceParameterSetVui {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            aspect_ratio_idc: Default::default(),
            sar_width: Default::default(),
            sar_height: Default::default(),
            video_format: Default::default(),
            colour_primaries: Default::default(),
            transfer_characteristics: Default::default(),
            matrix_coeffs: Default::default(),
            chroma_sample_loc_type_top_field: Default::default(),
            chroma_sample_loc_type_bottom_field: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            def_disp_win_left_offset: Default::default(),
            def_disp_win_right_offset: Default::default(),
            def_disp_win_top_offset: Default::default(),
            def_disp_win_bottom_offset: Default::default(),
            vui_num_units_in_tick: Default::default(),
            vui_time_scale: Default::default(),
            vui_num_ticks_poc_diff_one_minus_1: Default::default(),
            min_spatial_segmentation_idc: Default::default(),
            reserved_3: Default::default(),
            max_bytes_per_pic_denom: Default::default(),
            max_bits_per_min_cu_denom: Default::default(),
            log_2_max_mv_length_horizontal: Default::default(),
            log_2_max_mv_length_vertical: Default::default(),
            p_hrd_parameters: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265PredictorPaletteEntries.html)
pub struct StdVideoH265PredictorPaletteEntries {
    pub predictor_palette_entries: [[u16; STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE
        as usize]; STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE as usize],
}
impl Default for StdVideoH265PredictorPaletteEntries {
    fn default() -> Self {
        Self {
            predictor_palette_entries: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265PpsFlags.html)
pub struct StdVideoH265PpsFlags {}
impl Default for StdVideoH265PpsFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SubLayerHrdParameters.html)
pub struct StdVideoH265SubLayerHrdParameters {
    pub bit_rate_value_minus_1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus_1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_du_value_minus_1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub bit_rate_du_value_minus_1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
    pub cbr_flag: u32,
}
impl Default for StdVideoH265SubLayerHrdParameters {
    fn default() -> Self {
        Self {
            bit_rate_value_minus_1: unsafe { std::mem::zeroed() },
            cpb_size_value_minus_1: unsafe { std::mem::zeroed() },
            cpb_size_du_value_minus_1: unsafe { std::mem::zeroed() },
            bit_rate_du_value_minus_1: unsafe { std::mem::zeroed() },
            cbr_flag: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265HrdFlags.html)
pub struct StdVideoH265HrdFlags {}
impl Default for StdVideoH265HrdFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SpsVuiFlags.html)
pub struct StdVideoH265SpsVuiFlags {}
impl Default for StdVideoH265SpsVuiFlags {
    fn default() -> Self {
        Self {}
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265SliceType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265SliceType(pub i32);
impl StdVideoH265SliceType {
    pub const B: Self = Self(0);
    pub const P: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH265SliceType : i32, B, P, I, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265PictureType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265PictureType(pub i32);
impl StdVideoH265PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH265PictureType : i32, P, B, I, IDR, INVALID
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ProfileTierLevelFlags.html)
pub struct StdVideoH265ProfileTierLevelFlags {}
impl Default for StdVideoH265ProfileTierLevelFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ProfileTierLevel.html)
pub struct StdVideoH265ProfileTierLevel {
    pub flags: StdVideoH265ProfileTierLevelFlags,
    pub general_profile_idc: StdVideoH265ProfileIdc,
    pub general_level_idc: StdVideoH265LevelIdc,
}
impl Default for StdVideoH265ProfileTierLevel {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            general_profile_idc: Default::default(),
            general_level_idc: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ShortTermRefPicSetFlags.html)
pub struct StdVideoH265ShortTermRefPicSetFlags {}
impl Default for StdVideoH265ShortTermRefPicSetFlags {
    fn default() -> Self {
        Self {}
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ShortTermRefPicSet.html)
pub struct StdVideoH265ShortTermRefPicSet {
    pub flags: StdVideoH265ShortTermRefPicSetFlags,
    pub delta_idx_minus_1: u32,
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus_1: u16,
    pub used_by_curr_pic_flag: u16,
    pub used_by_curr_pic_s_0_flag: u16,
    pub used_by_curr_pic_s_1_flag: u16,
    pub reserved_1: u16,
    pub reserved_2: u8,
    pub reserved_3: u8,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s_0_minus_1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
    pub delta_poc_s_1_minus_1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
}
impl Default for StdVideoH265ShortTermRefPicSet {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            delta_idx_minus_1: Default::default(),
            use_delta_flag: Default::default(),
            abs_delta_rps_minus_1: Default::default(),
            used_by_curr_pic_flag: Default::default(),
            used_by_curr_pic_s_0_flag: Default::default(),
            used_by_curr_pic_s_1_flag: Default::default(),
            reserved_1: Default::default(),
            reserved_2: Default::default(),
            reserved_3: Default::default(),
            num_negative_pics: Default::default(),
            num_positive_pics: Default::default(),
            delta_poc_s_0_minus_1: unsafe { std::mem::zeroed() },
            delta_poc_s_1_minus_1: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265LongTermRefPicsSps.html)
pub struct StdVideoH265LongTermRefPicsSps {
    pub used_by_curr_pic_lt_sps_flag: u32,
    pub lt_ref_pic_poc_lsb_sps: [u32; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS
        as usize],
}
impl Default for StdVideoH265LongTermRefPicsSps {
    fn default() -> Self {
        Self {
            used_by_curr_pic_lt_sps_flag: Default::default(),
            lt_ref_pic_poc_lsb_sps: unsafe { std::mem::zeroed() },
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265ChromaFormatIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265ChromaFormatIdc(pub i32);
impl StdVideoH265ChromaFormatIdc {
    pub const MONOCHROME: Self = Self(0);
    pub const I420: Self = Self(1);
    pub const I422: Self = Self(2);
    pub const I444: Self = Self(3);
    pub const INVALID: Self = Self(2147483647);
}
crate::enum_impl! {
    StdVideoH265ChromaFormatIdc : i32, MONOCHROME, I420, I422, I444, INVALID
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/StdVideoH265AspectRatioIdc.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct StdVideoH265AspectRatioIdc(pub i32);
impl StdVideoH265AspectRatioIdc {
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
    StdVideoH265AspectRatioIdc : i32, UNSPECIFIED, SQUARE, I12_11, I10_11, I16_11,
    I40_33, I24_11, I20_11, I32_11, I80_33, I18_11, I15_11, I64_33, I160_99, I4_3, I3_2,
    I2_1, EXTENDED_SAR, INVALID
}
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;
pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;
