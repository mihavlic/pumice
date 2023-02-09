#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewSampleWeightCreateInfoQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewSampleWeightCreateInfoQCOM.html)
pub struct ImageViewSampleWeightCreateInfoQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub filter_center: crate::vk10::Offset2D,
    pub filter_size: crate::vk10::Extent2D,
    pub num_phases: u32,
}
impl Default for ImageViewSampleWeightCreateInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
            p_next: std::ptr::null(),
            filter_center: Default::default(),
            filter_size: Default::default(),
            num_phases: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageProcessingFeaturesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingFeaturesQCOM.html)
pub struct PhysicalDeviceImageProcessingFeaturesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub texture_sample_weighted: crate::vk10::Bool32,
    pub texture_box_filter: crate::vk10::Bool32,
    pub texture_block_match: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImageProcessingFeaturesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
            p_next: std::ptr::null_mut(),
            texture_sample_weighted: Default::default(),
            texture_box_filter: Default::default(),
            texture_block_match: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageProcessingPropertiesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingPropertiesQCOM.html)
pub struct PhysicalDeviceImageProcessingPropertiesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_weight_filter_phases: u32,
    pub max_weight_filter_dimension: crate::vk10::Extent2D,
    pub max_block_match_region: crate::vk10::Extent2D,
    pub max_box_filter_block_size: crate::vk10::Extent2D,
}
impl Default for PhysicalDeviceImageProcessingPropertiesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
            p_next: std::ptr::null_mut(),
            max_weight_filter_phases: Default::default(),
            max_weight_filter_dimension: Default::default(),
            max_block_match_region: Default::default(),
            max_box_filter_block_size: Default::default(),
        }
    }
}
pub const QCOM_IMAGE_PROCESSING_SPEC_VERSION: u32 = 1;
pub const QCOM_IMAGE_PROCESSING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QCOM_image_processing"
);
