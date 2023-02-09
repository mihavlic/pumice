#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html)
pub struct DrmFormatModifierPropertiesListEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}
impl Default for DrmFormatModifierPropertiesListEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifier_properties: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html)
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: crate::vk10::FormatFeatureFlags,
}
impl Default for DrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self {
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html)
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: crate::vk10::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier: Default::default(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html)
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
}
impl Default for ImageDrmFormatModifierListCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifiers: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html)
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const crate::vk10::SubresourceLayout,
}
impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            p_plane_layouts: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html)
pub struct ImageDrmFormatModifierPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub drm_format_modifier: u64,
}
impl Default for ImageDrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            drm_format_modifier: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html)
pub struct DrmFormatModifierPropertiesList2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}
impl Default for DrmFormatModifierPropertiesList2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT,
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifier_properties: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDrmFormatModifierProperties2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html)
pub struct DrmFormatModifierProperties2EXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: crate::vk13::FormatFeatureFlags2,
}
impl Default for DrmFormatModifierProperties2EXT {
    fn default() -> Self {
        Self {
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
pub unsafe fn get_image_drm_format_modifier_properties_ext(
    device: crate::vk10::Device,
    image: crate::vk10::Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_drm_format_modifier_properties_ext
        .unwrap())(device, image, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: crate::vk10::Image,
    ) -> crate::VulkanResult<ImageDrmFormatModifierPropertiesEXT> {
        let get_image_drm_format_modifier_properties_ext = (*self.table)
            .get_image_drm_format_modifier_properties_ext
            .unwrap();
        let mut properties = Default::default();
        let result = get_image_drm_format_modifier_properties_ext(
            self.handle,
            image,
            &mut properties,
        );
        crate::new_result(properties, result)
    }
}
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_image_drm_format_modifier"
);
