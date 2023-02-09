#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSampleLocationEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html)
pub struct SampleLocationEXT {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
}
impl Default for SampleLocationEXT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSampleLocationsInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html)
pub struct SampleLocationsInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub sample_locations_per_pixel: crate::vk10::SampleCountFlags,
    pub sample_location_grid_size: crate::vk10::Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
}
impl Default for SampleLocationsInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations_count: Default::default(),
            p_sample_locations: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html)
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for AttachmentSampleLocationsEXT {
    fn default() -> Self {
        Self {
            attachment_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassSampleLocationsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html)
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for SubpassSampleLocationsEXT {
    fn default() -> Self {
        Self {
            subpass_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html)
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
impl Default for RenderPassSampleLocationsBeginInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            attachment_initial_sample_locations_count: Default::default(),
            p_attachment_initial_sample_locations: std::ptr::null(),
            post_subpass_sample_locations_count: Default::default(),
            p_post_subpass_sample_locations: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html)
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub sample_locations_enable: crate::vk10::Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl Default for PipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_enable: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html)
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub sample_location_sample_counts: crate::vk10::SampleCountFlags,
    pub max_sample_location_grid_size: crate::vk10::Extent2D,
    pub sample_location_coordinate_range: [std::os::raw::c_float; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            sample_location_sample_counts: Default::default(),
            max_sample_location_grid_size: Default::default(),
            sample_location_coordinate_range: unsafe { std::mem::zeroed() },
            sample_location_sub_pixel_bits: Default::default(),
            variable_sample_locations: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMultisamplePropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html)
pub struct MultisamplePropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_sample_location_grid_size: crate::vk10::Extent2D,
}
impl Default for MultisamplePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_sample_location_grid_size: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
pub unsafe fn cmd_set_sample_locations_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_sample_locations_info: *const SampleLocationsInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_sample_locations_ext
        .unwrap())(command_buffer, p_sample_locations_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        sample_locations_info: &SampleLocationsInfoEXT,
    ) {
        let cmd_set_sample_locations_ext = (*self.table)
            .cmd_set_sample_locations_ext
            .unwrap();
        cmd_set_sample_locations_ext(command_buffer, sample_locations_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
pub unsafe fn get_physical_device_multisample_properties_ext(
    physical_device: crate::vk10::PhysicalDevice,
    samples: crate::vk10::SampleCountFlags,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
) {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_multisample_properties_ext
        .unwrap())(physical_device, samples, p_multisample_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        samples: crate::vk10::SampleCountFlags,
    ) -> MultisamplePropertiesEXT {
        let get_physical_device_multisample_properties_ext = (*self.table)
            .get_physical_device_multisample_properties_ext
            .unwrap();
        let mut multisample_properties = Default::default();
        get_physical_device_multisample_properties_ext(
            physical_device,
            samples as _,
            &mut multisample_properties,
        );
        multisample_properties
    }
}
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_sample_locations"
);
