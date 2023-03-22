#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264CapabilitiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html)
pub struct VideoEncodeH264CapabilitiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: VideoEncodeH264CapabilityFlagsEXT,
    pub input_mode_flags: VideoEncodeH264InputModeFlagsEXT,
    pub output_mode_flags: VideoEncodeH264OutputModeFlagsEXT,
    pub max_ppicture_l_0_reference_count: u8,
    pub max_bpicture_l_0_reference_count: u8,
    pub max_l_1_reference_count: u8,
    pub motion_vectors_over_pic_boundaries_flag: crate::vk10::Bool32,
    pub max_bytes_per_pic_denom: u32,
    pub max_bits_per_mb_denom: u32,
    pub log_2_max_mv_length_horizontal: u32,
    pub log_2_max_mv_length_vertical: u32,
}
impl Default for VideoEncodeH264CapabilitiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_CAPABILITIES_EXT,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            input_mode_flags: Default::default(),
            output_mode_flags: Default::default(),
            max_ppicture_l_0_reference_count: Default::default(),
            max_bpicture_l_0_reference_count: Default::default(),
            max_l_1_reference_count: Default::default(),
            motion_vectors_over_pic_boundaries_flag: Default::default(),
            max_bytes_per_pic_denom: Default::default(),
            max_bits_per_mb_denom: Default::default(),
            log_2_max_mv_length_horizontal: Default::default(),
            log_2_max_mv_length_vertical: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264SessionParametersAddInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264SessionParametersAddInfoEXT.html)
pub struct VideoEncodeH264SessionParametersAddInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_spscount: u32,
    pub p_std_spss: *const crate::extensions::h264std::StdVideoH264SequenceParameterSet,
    pub std_ppscount: u32,
    pub p_std_ppss: *const crate::extensions::h264std::StdVideoH264PictureParameterSet,
}
impl Default for VideoEncodeH264SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT,
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
#[doc(alias = "VkVideoEncodeH264SessionParametersCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264SessionParametersCreateInfoEXT.html)
pub struct VideoEncodeH264SessionParametersCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_std_spscount: u32,
    pub max_std_ppscount: u32,
    pub p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoEXT,
}
impl Default for VideoEncodeH264SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_std_spscount: Default::default(),
            max_std_ppscount: Default::default(),
            p_parameters_add_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264DpbSlotInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264DpbSlotInfoEXT.html)
pub struct VideoEncodeH264DpbSlotInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub slot_index: i8,
    pub p_std_reference_info: *const crate::extensions::h264std_encode::StdVideoEncodeH264ReferenceInfo,
}
impl Default for VideoEncodeH264DpbSlotInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT,
            p_next: std::ptr::null(),
            slot_index: Default::default(),
            p_std_reference_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264VclFrameInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264VclFrameInfoEXT.html)
pub struct VideoEncodeH264VclFrameInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_reference_final_lists: *const VideoEncodeH264ReferenceListsInfoEXT,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoEXT,
    pub p_current_picture_info: *const crate::extensions::h264std_encode::StdVideoEncodeH264PictureInfo,
}
impl Default for VideoEncodeH264VclFrameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT,
            p_next: std::ptr::null(),
            p_reference_final_lists: std::ptr::null(),
            nalu_slice_entry_count: Default::default(),
            p_nalu_slice_entries: std::ptr::null(),
            p_current_picture_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264ReferenceListsInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264ReferenceListsInfoEXT.html)
pub struct VideoEncodeH264ReferenceListsInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub reference_list_0_entry_count: u8,
    pub p_reference_list_0_entries: *const VideoEncodeH264DpbSlotInfoEXT,
    pub reference_list_1_entry_count: u8,
    pub p_reference_list_1_entries: *const VideoEncodeH264DpbSlotInfoEXT,
    pub p_mem_mgmt_ctrl_operations: *const crate::extensions::h264std_encode::StdVideoEncodeH264RefMemMgmtCtrlOperations,
}
impl Default for VideoEncodeH264ReferenceListsInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_REFERENCE_LISTS_INFO_EXT,
            p_next: std::ptr::null(),
            reference_list_0_entry_count: Default::default(),
            p_reference_list_0_entries: std::ptr::null(),
            reference_list_1_entry_count: Default::default(),
            p_reference_list_1_entries: std::ptr::null(),
            p_mem_mgmt_ctrl_operations: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264EmitPictureParametersInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264EmitPictureParametersInfoEXT.html)
pub struct VideoEncodeH264EmitPictureParametersInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub sps_id: u8,
    pub emit_sps_enable: crate::vk10::Bool32,
    pub pps_id_entry_count: u32,
    pub pps_id_entries: *const u8,
}
impl Default for VideoEncodeH264EmitPictureParametersInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_INFO_EXT,
            p_next: std::ptr::null(),
            sps_id: Default::default(),
            emit_sps_enable: Default::default(),
            pps_id_entry_count: Default::default(),
            pps_id_entries: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264ProfileInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264ProfileInfoEXT.html)
pub struct VideoEncodeH264ProfileInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_profile_idc: crate::extensions::h264std::StdVideoH264ProfileIdc,
}
impl Default for VideoEncodeH264ProfileInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_PROFILE_INFO_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264NaluSliceInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264NaluSliceInfoEXT.html)
pub struct VideoEncodeH264NaluSliceInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mb_count: u32,
    pub p_reference_final_lists: *const VideoEncodeH264ReferenceListsInfoEXT,
    pub p_slice_header_std: *const crate::extensions::h264std_encode::StdVideoEncodeH264SliceHeader,
}
impl Default for VideoEncodeH264NaluSliceInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT,
            p_next: std::ptr::null(),
            mb_count: Default::default(),
            p_reference_final_lists: std::ptr::null(),
            p_slice_header_std: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264RateControlInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlInfoEXT.html)
pub struct VideoEncodeH264RateControlInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_bframe_count: u32,
    pub rate_control_structure: VideoEncodeH264RateControlStructureEXT,
    pub temporal_layer_count: u8,
}
impl Default for VideoEncodeH264RateControlInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT,
            p_next: std::ptr::null(),
            gop_frame_count: Default::default(),
            idr_period: Default::default(),
            consecutive_bframe_count: Default::default(),
            rate_control_structure: Default::default(),
            temporal_layer_count: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264QpEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264QpEXT.html)
pub struct VideoEncodeH264QpEXT {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
impl Default for VideoEncodeH264QpEXT {
    fn default() -> Self {
        Self {
            qp_i: Default::default(),
            qp_p: Default::default(),
            qp_b: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264FrameSizeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html)
pub struct VideoEncodeH264FrameSizeEXT {
    pub frame_isize: u32,
    pub frame_psize: u32,
    pub frame_bsize: u32,
}
impl Default for VideoEncodeH264FrameSizeEXT {
    fn default() -> Self {
        Self {
            frame_isize: Default::default(),
            frame_psize: Default::default(),
            frame_bsize: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoEncodeH264RateControlLayerInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlLayerInfoEXT.html)
pub struct VideoEncodeH264RateControlLayerInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub temporal_layer_id: u8,
    pub use_initial_rc_qp: crate::vk10::Bool32,
    pub initial_rc_qp: VideoEncodeH264QpEXT,
    pub use_min_qp: crate::vk10::Bool32,
    pub min_qp: VideoEncodeH264QpEXT,
    pub use_max_qp: crate::vk10::Bool32,
    pub max_qp: VideoEncodeH264QpEXT,
    pub use_max_frame_size: crate::vk10::Bool32,
    pub max_frame_size: VideoEncodeH264FrameSizeEXT,
}
impl Default for VideoEncodeH264RateControlLayerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT,
            p_next: std::ptr::null(),
            temporal_layer_id: Default::default(),
            use_initial_rc_qp: Default::default(),
            initial_rc_qp: Default::default(),
            use_min_qp: Default::default(),
            min_qp: Default::default(),
            use_max_qp: Default::default(),
            max_qp: Default::default(),
            use_max_frame_size: Default::default(),
            max_frame_size: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeH264CapabilityFlagsEXT(pub u32);
impl VideoEncodeH264CapabilityFlagsEXT {
    pub const DIRECT_8X8_INFERENCE_ENABLED: Self = Self(1 << 0);
    pub const DIRECT_8X8_INFERENCE_DISABLED: Self = Self(1 << 1);
    pub const SEPARATE_COLOUR_PLANE: Self = Self(1 << 2);
    pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS: Self = Self(1 << 3);
    pub const SCALING_LISTS: Self = Self(1 << 4);
    pub const HRD_COMPLIANCE: Self = Self(1 << 5);
    pub const CHROMA_QP_OFFSET: Self = Self(1 << 6);
    pub const SECOND_CHROMA_QP_OFFSET: Self = Self(1 << 7);
    pub const PIC_INIT_QP_MINUS26: Self = Self(1 << 8);
    pub const WEIGHTED_PRED: Self = Self(1 << 9);
    pub const WEIGHTED_BIPRED_EXPLICIT: Self = Self(1 << 10);
    pub const WEIGHTED_BIPRED_IMPLICIT: Self = Self(1 << 11);
    pub const WEIGHTED_PRED_NO_TABLE: Self = Self(1 << 12);
    pub const TRANSFORM_8X8: Self = Self(1 << 13);
    pub const CABAC: Self = Self(1 << 14);
    pub const CAVLC: Self = Self(1 << 15);
    pub const DEBLOCKING_FILTER_DISABLED: Self = Self(1 << 16);
    pub const DEBLOCKING_FILTER_ENABLED: Self = Self(1 << 17);
    pub const DEBLOCKING_FILTER_PARTIAL: Self = Self(1 << 18);
    pub const DISABLE_DIRECT_SPATIAL_MV_PRED: Self = Self(1 << 19);
    pub const MULTIPLE_SLICE_PER_FRAME: Self = Self(1 << 20);
    pub const SLICE_MB_COUNT: Self = Self(1 << 21);
    pub const ROW_UNALIGNED_SLICE: Self = Self(1 << 22);
    pub const DIFFERENT_SLICE_TYPE: Self = Self(1 << 23);
    pub const B_FRAME_IN_L1_LIST: Self = Self(1 << 24);
}
crate::bitflags_impl! {
    VideoEncodeH264CapabilityFlagsEXT : u32, 0x1ffffff, DIRECT_8X8_INFERENCE_ENABLED,
    DIRECT_8X8_INFERENCE_DISABLED, SEPARATE_COLOUR_PLANE,
    QPPRIME_Y_ZERO_TRANSFORM_BYPASS, SCALING_LISTS, HRD_COMPLIANCE, CHROMA_QP_OFFSET,
    SECOND_CHROMA_QP_OFFSET, PIC_INIT_QP_MINUS26, WEIGHTED_PRED,
    WEIGHTED_BIPRED_EXPLICIT, WEIGHTED_BIPRED_IMPLICIT, WEIGHTED_PRED_NO_TABLE,
    TRANSFORM_8X8, CABAC, CAVLC, DEBLOCKING_FILTER_DISABLED, DEBLOCKING_FILTER_ENABLED,
    DEBLOCKING_FILTER_PARTIAL, DISABLE_DIRECT_SPATIAL_MV_PRED, MULTIPLE_SLICE_PER_FRAME,
    SLICE_MB_COUNT, ROW_UNALIGNED_SLICE, DIFFERENT_SLICE_TYPE, B_FRAME_IN_L1_LIST
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeH264InputModeFlagsEXT(pub u32);
impl VideoEncodeH264InputModeFlagsEXT {
    pub const FRAME: Self = Self(1 << 0);
    pub const SLICE: Self = Self(1 << 1);
    pub const NON_VCL: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    VideoEncodeH264InputModeFlagsEXT : u32, 0x7, FRAME, SLICE, NON_VCL
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeH264OutputModeFlagsEXT(pub u32);
impl VideoEncodeH264OutputModeFlagsEXT {
    pub const FRAME: Self = Self(1 << 0);
    pub const SLICE: Self = Self(1 << 1);
    pub const NON_VCL: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    VideoEncodeH264OutputModeFlagsEXT : u32, 0x7, FRAME, SLICE, NON_VCL
}
#[doc(alias = "VkVideoEncodeH264RateControlStructureEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlStructureEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VideoEncodeH264RateControlStructureEXT(pub i32);
impl VideoEncodeH264RateControlStructureEXT {
    pub const UNKNOWN: Self = Self(0);
    pub const FLAT: Self = Self(1);
    pub const DYADIC: Self = Self(2);
}
crate::enum_impl! {
    VideoEncodeH264RateControlStructureEXT : i32, UNKNOWN, FLAT, DYADIC
}
pub const EXT_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 9;
pub const EXT_VIDEO_ENCODE_H264_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_video_encode_h264"
);
