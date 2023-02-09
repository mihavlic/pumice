#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265ProfileInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265ProfileInfoEXT.html)
pub struct VideoDecodeH265ProfileInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_profile_idc: crate::extensions::h265std::StdVideoH265ProfileIdc,
}
impl Default for VideoDecodeH265ProfileInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_PROFILE_INFO_EXT,
            p_next: std::ptr::null(),
            std_profile_idc: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265CapabilitiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265CapabilitiesEXT.html)
pub struct VideoDecodeH265CapabilitiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_level_idc: crate::extensions::h265std::StdVideoH265LevelIdc,
}
impl Default for VideoDecodeH265CapabilitiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_CAPABILITIES_EXT,
            p_next: std::ptr::null_mut(),
            max_level_idc: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265SessionParametersAddInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265SessionParametersAddInfoEXT.html)
pub struct VideoDecodeH265SessionParametersAddInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub std_vpscount: u32,
    pub p_std_vpss: *const crate::extensions::h265std::StdVideoH265VideoParameterSet,
    pub std_spscount: u32,
    pub p_std_spss: *const crate::extensions::h265std::StdVideoH265SequenceParameterSet,
    pub std_ppscount: u32,
    pub p_std_ppss: *const crate::extensions::h265std::StdVideoH265PictureParameterSet,
}
impl Default for VideoDecodeH265SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT,
            p_next: std::ptr::null(),
            std_vpscount: Default::default(),
            p_std_vpss: std::ptr::null(),
            std_spscount: Default::default(),
            p_std_spss: std::ptr::null(),
            std_ppscount: Default::default(),
            p_std_ppss: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265SessionParametersCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265SessionParametersCreateInfoEXT.html)
pub struct VideoDecodeH265SessionParametersCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_std_vpscount: u32,
    pub max_std_spscount: u32,
    pub max_std_ppscount: u32,
    pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoEXT,
}
impl Default for VideoDecodeH265SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            max_std_vpscount: Default::default(),
            max_std_spscount: Default::default(),
            max_std_ppscount: Default::default(),
            p_parameters_add_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265PictureInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265PictureInfoEXT.html)
pub struct VideoDecodeH265PictureInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_std_picture_info: *mut crate::extensions::h265std_decode::StdVideoDecodeH265PictureInfo,
    pub slice_count: u32,
    pub p_slice_offsets: *const u32,
}
impl Default for VideoDecodeH265PictureInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_PICTURE_INFO_EXT,
            p_next: std::ptr::null(),
            p_std_picture_info: std::ptr::null_mut(),
            slice_count: Default::default(),
            p_slice_offsets: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkVideoDecodeH265DpbSlotInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH265DpbSlotInfoEXT.html)
pub struct VideoDecodeH265DpbSlotInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_std_reference_info: *const crate::extensions::h265std_decode::StdVideoDecodeH265ReferenceInfo,
}
impl Default for VideoDecodeH265DpbSlotInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT,
            p_next: std::ptr::null(),
            p_std_reference_info: std::ptr::null(),
        }
    }
}
pub const EXT_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 5;
pub const EXT_VIDEO_DECODE_H265_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_video_decode_h265"
);
