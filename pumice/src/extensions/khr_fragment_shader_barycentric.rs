#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR.html)
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_shader_barycentric: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            fragment_shader_barycentric: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR.html)
pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub tri_strip_vertex_order_independent_of_provoking_vertex: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            tri_strip_vertex_order_independent_of_provoking_vertex: Default::default(),
        }
    }
}
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_fragment_shader_barycentric"
);
