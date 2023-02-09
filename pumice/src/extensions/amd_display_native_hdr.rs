#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html)
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub local_dimming_support: crate::vk10::Bool32,
}
impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
            p_next: std::ptr::null_mut(),
            local_dimming_support: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html)
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub local_dimming_enable: crate::vk10::Bool32,
}
impl Default for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            local_dimming_enable: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetLocalDimmingAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html)
pub unsafe fn set_local_dimming_amd(
    device: crate::vk10::Device,
    swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
    local_dimming_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_local_dimming_amd
        .unwrap())(device, swap_chain, local_dimming_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkSetLocalDimmingAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html)
    pub unsafe fn set_local_dimming_amd(
        &self,
        swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
        local_dimming_enable: bool,
    ) {
        let set_local_dimming_amd = (*self.table).set_local_dimming_amd.unwrap();
        set_local_dimming_amd(self.handle, swap_chain, local_dimming_enable as _);
    }
}
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_display_native_hdr"
);
