#[doc(alias = "VkPipelineInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoEXT.html)
pub type PipelineInfoEXT = crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelinePropertiesIdentifierEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelinePropertiesIdentifierEXT.html)
pub struct PipelinePropertiesIdentifierEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_identifier: [u8; crate::vk10::UUID_SIZE as usize],
}
impl Default for PipelinePropertiesIdentifierEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT,
            p_next: std::ptr::null_mut(),
            pipeline_identifier: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelinePropertiesFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html)
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_properties_identifier: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePipelinePropertiesFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            pipeline_properties_identifier: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPipelinePropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html)
pub unsafe fn get_pipeline_properties_ext(
    device: crate::vk10::Device,
    p_pipeline_info: *const PipelineInfoEXT,
    p_pipeline_properties: *mut crate::vk10::BaseOutStructure,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_pipeline_properties_ext
        .unwrap())(device, p_pipeline_info, p_pipeline_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html)
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        pipeline_info: &PipelineInfoEXT,
    ) -> crate::VulkanResult<crate::vk10::BaseOutStructure> {
        let get_pipeline_properties_ext = (*self.table)
            .get_pipeline_properties_ext
            .unwrap();
        let mut pipeline_properties = Default::default();
        let result = get_pipeline_properties_ext(
            self.handle,
            pipeline_info as _,
            &mut pipeline_properties,
        );
        crate::new_result(pipeline_properties, result)
    }
}
pub const EXT_PIPELINE_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const EXT_PIPELINE_PROPERTIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_pipeline_properties"
);
