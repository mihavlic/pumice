#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html)
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ray_query: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRayQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_query: Default::default(),
        }
    }
}
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
pub const KHR_RAY_QUERY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_ray_query"
);
