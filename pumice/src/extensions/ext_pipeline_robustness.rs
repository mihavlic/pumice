#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelineRobustnessFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessFeaturesEXT.html)
pub struct PhysicalDevicePipelineRobustnessFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_robustness: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePipelineRobustnessFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            pipeline_robustness: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRobustnessCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessCreateInfoEXT.html)
pub struct PipelineRobustnessCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub storage_buffers: PipelineRobustnessBufferBehaviorEXT,
    pub uniform_buffers: PipelineRobustnessBufferBehaviorEXT,
    pub vertex_inputs: PipelineRobustnessBufferBehaviorEXT,
    pub images: PipelineRobustnessImageBehaviorEXT,
}
impl Default for PipelineRobustnessCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_ROBUSTNESS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            storage_buffers: Default::default(),
            uniform_buffers: Default::default(),
            vertex_inputs: Default::default(),
            images: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelineRobustnessPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessPropertiesEXT.html)
pub struct PhysicalDevicePipelineRobustnessPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehaviorEXT,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehaviorEXT,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehaviorEXT,
    pub default_robustness_images: PipelineRobustnessImageBehaviorEXT,
}
impl Default for PhysicalDevicePipelineRobustnessPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            default_robustness_storage_buffers: Default::default(),
            default_robustness_uniform_buffers: Default::default(),
            default_robustness_vertex_inputs: Default::default(),
            default_robustness_images: Default::default(),
        }
    }
}
#[doc(alias = "VkPipelineRobustnessBufferBehaviorEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessBufferBehaviorEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRobustnessBufferBehaviorEXT(pub i32);
impl PipelineRobustnessBufferBehaviorEXT {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
}
crate::enum_impl! {
    PipelineRobustnessBufferBehaviorEXT : i32, DEVICE_DEFAULT, DISABLED,
    ROBUST_BUFFER_ACCESS, ROBUST_BUFFER_ACCESS_2
}
#[doc(alias = "VkPipelineRobustnessImageBehaviorEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessImageBehaviorEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRobustnessImageBehaviorEXT(pub i32);
impl PipelineRobustnessImageBehaviorEXT {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
}
crate::enum_impl! {
    PipelineRobustnessImageBehaviorEXT : i32, DEVICE_DEFAULT, DISABLED,
    ROBUST_IMAGE_ACCESS, ROBUST_IMAGE_ACCESS_2
}
pub const EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
pub const EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_pipeline_robustness"
);
