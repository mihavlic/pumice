#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html)
pub struct DeviceQueueGlobalPriorityCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub global_priority: QueueGlobalPriorityKHR,
}
impl Default for DeviceQueueGlobalPriorityCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            global_priority: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html)
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub global_priority_query: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            global_priority_query: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html)
pub struct QueueFamilyGlobalPriorityPropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub priority_count: u32,
    pub priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
}
impl Default for QueueFamilyGlobalPriorityPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            priority_count: Default::default(),
            priorities: unsafe { std::mem::zeroed() },
        }
    }
}
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
#[doc(alias = "VkQueueGlobalPriorityKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct QueueGlobalPriorityKHR(pub i32);
impl QueueGlobalPriorityKHR {
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
    pub const LOW_EXT: Self = Self::LOW;
    pub const MEDIUM_EXT: Self = Self::MEDIUM;
    pub const HIGH_EXT: Self = Self::HIGH;
    pub const REALTIME_EXT: Self = Self::REALTIME;
}
crate::enum_impl! {
    QueueGlobalPriorityKHR : i32, LOW, MEDIUM, HIGH, REALTIME
}
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
pub const KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_global_priority"
);
