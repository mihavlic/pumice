#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    PipelineRasterizationDepthClipStateCreateFlagsEXT : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html)
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub depth_clip_enable: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_enable: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html)
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: crate::vk10::Bool32,
}
impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_clip_enable: Default::default(),
        }
    }
}
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_depth_clip_enable"
);
