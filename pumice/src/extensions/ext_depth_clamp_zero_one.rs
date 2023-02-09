#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDepthClampZeroOneFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClampZeroOneFeaturesEXT.html)
pub struct PhysicalDeviceDepthClampZeroOneFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub depth_clamp_zero_one: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDepthClampZeroOneFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clamp_zero_one: Default::default(),
        }
    }
}
pub const EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;
pub const EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_depth_clamp_zero_one"
);
