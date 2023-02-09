#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html)
pub struct PhysicalDevicePresentIdFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_id: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePresentIdFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            present_id: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentIdKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentIdKHR.html)
pub struct PresentIdKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}
impl Default for PresentIdKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRESENT_ID_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_present_ids: std::ptr::null(),
        }
    }
}
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
pub const KHR_PRESENT_ID_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_present_id"
);
