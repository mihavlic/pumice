#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html)
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
impl Default for DeviceMemoryOverallocationCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            overallocation_behavior: Default::default(),
        }
    }
}
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MemoryOverallocationBehaviorAMD(pub i32);
impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
}
crate::enum_impl! {
    MemoryOverallocationBehaviorAMD : i32, DEFAULT, ALLOWED, DISALLOWED
}
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_AMD_memory_overallocation_behavior"
);
