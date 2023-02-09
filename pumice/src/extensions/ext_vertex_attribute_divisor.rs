#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkVertexInputBindingDivisorDescriptionEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html)
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}
impl Default for VertexInputBindingDivisorDescriptionEXT {
    fn default() -> Self {
        Self {
            binding: Default::default(),
            divisor: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html)
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
impl Default for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            vertex_binding_divisor_count: Default::default(),
            p_vertex_binding_divisors: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html)
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_vertex_attrib_divisor: u32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_vertex_attrib_divisor: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html)
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub vertex_attribute_instance_rate_divisor: crate::vk10::Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_attribute_instance_rate_divisor: Default::default(),
            vertex_attribute_instance_rate_zero_divisor: Default::default(),
        }
    }
}
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_vertex_attribute_divisor"
);
