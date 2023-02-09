#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html)
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_density_map: crate::vk10::Bool32,
    pub fragment_density_map_dynamic: crate::vk10::Bool32,
    pub fragment_density_map_non_subsampled_images: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_density_map: Default::default(),
            fragment_density_map_dynamic: Default::default(),
            fragment_density_map_non_subsampled_images: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html)
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_fragment_density_texel_size: crate::vk10::Extent2D,
    pub max_fragment_density_texel_size: crate::vk10::Extent2D,
    pub fragment_density_invocations: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_fragment_density_texel_size: Default::default(),
            max_fragment_density_texel_size: Default::default(),
            fragment_density_invocations: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html)
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fragment_density_map_attachment: crate::vk10::AttachmentReference,
}
impl Default for RenderPassFragmentDensityMapCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            fragment_density_map_attachment: Default::default(),
        }
    }
}
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_fragment_density_map"
);
