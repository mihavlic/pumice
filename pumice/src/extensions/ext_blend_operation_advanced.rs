#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html)
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub advanced_blend_coherent_operations: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            advanced_blend_coherent_operations: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html)
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: crate::vk10::Bool32,
    pub advanced_blend_non_premultiplied_src_color: crate::vk10::Bool32,
    pub advanced_blend_non_premultiplied_dst_color: crate::vk10::Bool32,
    pub advanced_blend_correlated_overlap: crate::vk10::Bool32,
    pub advanced_blend_all_operations: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            advanced_blend_max_color_attachments: Default::default(),
            advanced_blend_independent_blend: Default::default(),
            advanced_blend_non_premultiplied_src_color: Default::default(),
            advanced_blend_non_premultiplied_dst_color: Default::default(),
            advanced_blend_correlated_overlap: Default::default(),
            advanced_blend_all_operations: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html)
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_premultiplied: crate::vk10::Bool32,
    pub dst_premultiplied: crate::vk10::Bool32,
    pub blend_overlap: BlendOverlapEXT,
}
impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            src_premultiplied: Default::default(),
            dst_premultiplied: Default::default(),
            blend_overlap: Default::default(),
        }
    }
}
#[doc(alias = "VkBlendOverlapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BlendOverlapEXT(pub i32);
impl BlendOverlapEXT {
    pub const UNCORRELATED: Self = Self(0);
    pub const DISJOINT: Self = Self(1);
    pub const CONJOINT: Self = Self(2);
}
crate::enum_impl! {
    BlendOverlapEXT : i32, UNCORRELATED, DISJOINT, CONJOINT
}
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_blend_operation_advanced"
);
