#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceCapabilities2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2EXT.html)
pub struct SurfaceCapabilities2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: crate::vk10::Extent2D,
    pub min_image_extent: crate::vk10::Extent2D,
    pub max_image_extent: crate::vk10::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub current_transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    pub supported_usage_flags: crate::vk10::ImageUsageFlags,
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
impl Default for SurfaceCapabilities2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_CAPABILITIES_2_EXT,
            p_next: std::ptr::null_mut(),
            min_image_count: Default::default(),
            max_image_count: Default::default(),
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: Default::default(),
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SurfaceCounterFlagsEXT(pub u32);
impl SurfaceCounterFlagsEXT {
    pub const VBLANK: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SurfaceCounterFlagsEXT : u32, 0x1, VBLANK
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
pub unsafe fn get_physical_device_surface_capabilities_2_ext(
    physical_device: crate::vk10::PhysicalDevice,
    surface: crate::extensions::khr_surface::SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_capabilities_2_ext
        .unwrap())(physical_device, surface, p_surface_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
    pub unsafe fn get_physical_device_surface_capabilities_2_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::VulkanResult<SurfaceCapabilities2EXT> {
        let get_physical_device_surface_capabilities_2_ext = (*self.table)
            .get_physical_device_surface_capabilities_2_ext
            .unwrap();
        let mut surface_capabilities = Default::default();
        let result = get_physical_device_surface_capabilities_2_ext(
            physical_device,
            surface,
            &mut surface_capabilities,
        );
        crate::new_result(surface_capabilities, result)
    }
}
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
pub const EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_display_surface_counter"
);
