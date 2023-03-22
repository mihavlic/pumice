#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html)
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub graphics_pipeline_library: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            graphics_pipeline_library: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html)
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub graphics_pipeline_library_fast_linking: crate::vk10::Bool32,
    pub graphics_pipeline_library_independent_interpolation_decoration: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            graphics_pipeline_library_fast_linking: Default::default(),
            graphics_pipeline_library_independent_interpolation_decoration: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkGraphicsPipelineLibraryCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html)
pub struct GraphicsPipelineLibraryCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: GraphicsPipelineLibraryFlagsEXT,
}
impl Default for GraphicsPipelineLibraryCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GraphicsPipelineLibraryFlagsEXT(pub u32);
impl GraphicsPipelineLibraryFlagsEXT {
    pub const VERTEX_INPUT_INTERFACE: Self = Self(1 << 0);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 1);
    pub const FRAGMENT_SHADER: Self = Self(1 << 2);
    pub const FRAGMENT_OUTPUT_INTERFACE: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    GraphicsPipelineLibraryFlagsEXT : u32, 0xf, VERTEX_INPUT_INTERFACE,
    PRE_RASTERIZATION_SHADERS, FRAGMENT_SHADER, FRAGMENT_OUTPUT_INTERFACE
}
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_graphics_pipeline_library"
);
