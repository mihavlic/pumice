#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html)
pub struct SurfaceProtectedCapabilitiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub supports_protected: crate::vk10::Bool32,
}
impl Default for SurfaceProtectedCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR,
            p_next: std::ptr::null(),
            supports_protected: Default::default(),
        }
    }
}
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_surface_protected_capabilities"
);
