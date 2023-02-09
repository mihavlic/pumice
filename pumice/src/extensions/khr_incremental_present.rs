#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentRegionsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html)
pub struct PresentRegionsKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR,
}
impl Default for PresentRegionsKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRESENT_REGIONS_KHR,
            p_next: std::ptr::null(),
            swapchain_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPresentRegionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html)
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
}
impl Default for PresentRegionKHR {
    fn default() -> Self {
        Self {
            rectangle_count: Default::default(),
            p_rectangles: std::ptr::null(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkRectLayerKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html)
pub struct RectLayerKHR {
    pub offset: crate::vk10::Offset2D,
    pub extent: crate::vk10::Extent2D,
    pub layer: u32,
}
impl Default for RectLayerKHR {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            extent: Default::default(),
            layer: Default::default(),
        }
    }
}
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_incremental_present"
);
