#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePresentBarrierFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentBarrierFeaturesNV.html)
pub struct PhysicalDevicePresentBarrierFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_barrier: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePresentBarrierFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            present_barrier: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceCapabilitiesPresentBarrierNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesPresentBarrierNV.html)
pub struct SurfaceCapabilitiesPresentBarrierNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_barrier_supported: crate::vk10::Bool32,
}
impl Default for SurfaceCapabilitiesPresentBarrierNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
            p_next: std::ptr::null_mut(),
            present_barrier_supported: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSwapchainPresentBarrierCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentBarrierCreateInfoNV.html)
pub struct SwapchainPresentBarrierCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub present_barrier_enable: crate::vk10::Bool32,
}
impl Default for SwapchainPresentBarrierCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV,
            p_next: std::ptr::null_mut(),
            present_barrier_enable: Default::default(),
        }
    }
}
pub const NV_PRESENT_BARRIER_SPEC_VERSION: u32 = 1;
pub const NV_PRESENT_BARRIER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_present_barrier"
);
