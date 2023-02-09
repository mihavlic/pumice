#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html)
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub index_type_uint_8: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            index_type_uint_8: Default::default(),
        }
    }
}
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_index_type_uint8"
);
