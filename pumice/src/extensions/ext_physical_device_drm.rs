#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html)
pub struct PhysicalDeviceDrmPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub has_primary: crate::vk10::Bool32,
    pub has_render: crate::vk10::Bool32,
    pub primary_major: i64,
    pub primary_minor: i64,
    pub render_major: i64,
    pub render_minor: i64,
}
impl Default for PhysicalDeviceDrmPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            has_primary: Default::default(),
            has_render: Default::default(),
            primary_major: Default::default(),
            primary_minor: Default::default(),
            render_major: Default::default(),
            render_minor: Default::default(),
        }
    }
}
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
pub const EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_physical_device_drm"
);
