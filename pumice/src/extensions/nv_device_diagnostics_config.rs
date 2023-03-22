#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html)
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub diagnostics_config: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            diagnostics_config: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html)
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}
impl Default for DeviceDiagnosticsConfigCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagsNV(pub u32);
impl DeviceDiagnosticsConfigFlagsNV {
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1 << 0);
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(1 << 1);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(1 << 2);
    pub const ENABLE_SHADER_ERROR_REPORTING: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    DeviceDiagnosticsConfigFlagsNV : u32, 0xf, ENABLE_SHADER_DEBUG_INFO,
    ENABLE_RESOURCE_TRACKING, ENABLE_AUTOMATIC_CHECKPOINTS, ENABLE_SHADER_ERROR_REPORTING
}
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 2;
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_device_diagnostics_config"
);
