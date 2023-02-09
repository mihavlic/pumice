#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub dedicated_allocation_image_aliasing: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            dedicated_allocation_image_aliasing: Default::default(),
        }
    }
}
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_dedicated_allocation_image_aliasing"
);
