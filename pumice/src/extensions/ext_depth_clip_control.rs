#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html)
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub depth_clip_control: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDepthClipControlFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_control: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html)
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub negative_one_to_one: crate::vk10::Bool32,
}
impl Default for PipelineViewportDepthClipControlCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            negative_one_to_one: Default::default(),
        }
    }
}
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
pub const EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_depth_clip_control"
);
