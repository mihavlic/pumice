#[doc(alias = "VkDisplayModeCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayModeCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    DisplayModeCreateFlagsKHR : u32, 0x0,
}
#[doc(alias = "VkDisplaySurfaceCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplaySurfaceCreateFlagsKHR(pub u32);
crate::bitflags_impl! {
    DisplaySurfaceCreateFlagsKHR : u32, 0x0,
}
crate::dispatchable_handle!(
    DisplayKHR, DISPLAY_KHR, "VkDisplayKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html)"
);
crate::dispatchable_handle!(
    DisplayModeKHR, DISPLAY_MODE_KHR, "VkDisplayModeKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html)
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const std::os::raw::c_char,
    pub physical_dimensions: crate::vk10::Extent2D,
    pub physical_resolution: crate::vk10::Extent2D,
    pub supported_transforms: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: crate::vk10::Bool32,
    pub persistent_content: crate::vk10::Bool32,
}
impl Default for DisplayPropertiesKHR {
    fn default() -> Self {
        Self {
            display: Default::default(),
            display_name: std::ptr::null(),
            physical_dimensions: Default::default(),
            physical_resolution: Default::default(),
            supported_transforms: Default::default(),
            plane_reorder_possible: Default::default(),
            persistent_content: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html)
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}
impl Default for DisplayPlanePropertiesKHR {
    fn default() -> Self {
        Self {
            current_display: Default::default(),
            current_stack_index: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDisplayModeParametersKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html)
pub struct DisplayModeParametersKHR {
    pub visible_region: crate::vk10::Extent2D,
    pub refresh_rate: u32,
}
impl Default for DisplayModeParametersKHR {
    fn default() -> Self {
        Self {
            visible_region: Default::default(),
            refresh_rate: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDisplayModePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html)
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModePropertiesKHR {
    fn default() -> Self {
        Self {
            display_mode: Default::default(),
            parameters: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html)
pub struct DisplayModeCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModeCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_MODE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkDisplayPlaneCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html)
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: crate::vk10::Offset2D,
    pub max_src_position: crate::vk10::Offset2D,
    pub min_src_extent: crate::vk10::Extent2D,
    pub max_src_extent: crate::vk10::Extent2D,
    pub min_dst_position: crate::vk10::Offset2D,
    pub max_dst_position: crate::vk10::Offset2D,
    pub min_dst_extent: crate::vk10::Extent2D,
    pub max_dst_extent: crate::vk10::Extent2D,
}
impl Default for DisplayPlaneCapabilitiesKHR {
    fn default() -> Self {
        Self {
            supported_alpha: Default::default(),
            min_src_position: Default::default(),
            max_src_position: Default::default(),
            min_src_extent: Default::default(),
            max_src_extent: Default::default(),
            min_dst_position: Default::default(),
            max_dst_position: Default::default(),
            min_dst_extent: Default::default(),
            max_dst_extent: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html)
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagsKHR,
    pub global_alpha: std::os::raw::c_float,
    pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
    pub image_extent: crate::vk10::Extent2D,
}
impl Default for DisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            display_mode: Default::default(),
            plane_index: Default::default(),
            plane_stack_index: Default::default(),
            transform: Default::default(),
            global_alpha: Default::default(),
            alpha_mode: Default::default(),
            image_extent: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPlaneAlphaFlagsKHR(pub u32);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(1 << 0);
    pub const GLOBAL: Self = Self(1 << 1);
    pub const PER_PIXEL: Self = Self(1 << 2);
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    DisplayPlaneAlphaFlagsKHR : u32, 0xf, OPAQUE, GLOBAL, PER_PIXEL,
    PER_PIXEL_PREMULTIPLIED
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
pub unsafe fn get_physical_device_display_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_display_properties_khr
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<DisplayPropertiesKHR>, crate::vk10::Result)> {
        let get_physical_device_display_properties_khr = (*self.table)
            .get_physical_device_display_properties_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_display_properties_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = get_physical_device_display_properties_khr(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
pub unsafe fn get_physical_device_display_plane_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_display_plane_properties_khr
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<DisplayPlanePropertiesKHR>, crate::vk10::Result)> {
        let get_physical_device_display_plane_properties_khr = (*self.table)
            .get_physical_device_display_plane_properties_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_display_plane_properties_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = get_physical_device_display_plane_properties_khr(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
pub unsafe fn get_display_plane_supported_displays_khr(
    physical_device: crate::vk10::PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_display_plane_supported_displays_khr
        .unwrap())(physical_device, plane_index, p_display_count, p_displays)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        plane_index: u32,
        display_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<DisplayKHR>, crate::vk10::Result)> {
        let get_display_plane_supported_displays_khr = (*self.table)
            .get_display_plane_supported_displays_khr
            .unwrap();
        let mut display_count = match display_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_display_plane_supported_displays_khr(
                    physical_device,
                    plane_index as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut displays = vec![Default::default(); display_count as usize];
        let result = get_display_plane_supported_displays_khr(
            physical_device,
            plane_index as _,
            &mut display_count,
            displays.as_mut_ptr(),
        );
        crate::new_result((displays, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
pub unsafe fn get_display_mode_properties_khr(
    physical_device: crate::vk10::PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_display_mode_properties_khr
        .unwrap())(physical_device, display, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: DisplayKHR,
        property_count: Option<u32>,
    ) -> crate::VulkanResult<(Vec<DisplayModePropertiesKHR>, crate::vk10::Result)> {
        let get_display_mode_properties_khr = (*self.table)
            .get_display_mode_properties_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_display_mode_properties_khr(
                    physical_device,
                    display,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        let result = get_display_mode_properties_khr(
            physical_device,
            display,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDisplayModeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html)
pub unsafe fn create_display_mode_khr(
    physical_device: crate::vk10::PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_mode: *mut DisplayModeKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_display_mode_khr
        .unwrap())(physical_device, display, p_create_info, p_allocator, p_mode)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDisplayModeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html)
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<DisplayModeKHR> {
        let create_display_mode_khr = (*self.table).create_display_mode_khr.unwrap();
        let mut mode = Default::default();
        let result = create_display_mode_khr(
            physical_device,
            display,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut mode,
        );
        crate::new_result(mode, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
pub unsafe fn get_display_plane_capabilities_khr(
    physical_device: crate::vk10::PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_display_plane_capabilities_khr
        .unwrap())(physical_device, mode, plane_index, p_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> crate::VulkanResult<DisplayPlaneCapabilitiesKHR> {
        let get_display_plane_capabilities_khr = (*self.table)
            .get_display_plane_capabilities_khr
            .unwrap();
        let mut capabilities = Default::default();
        let result = get_display_plane_capabilities_khr(
            physical_device,
            mode,
            plane_index as _,
            &mut capabilities,
        );
        crate::new_result(capabilities, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
pub unsafe fn create_display_plane_surface_khr(
    instance: crate::vk10::Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .create_display_plane_surface_khr
        .unwrap())(instance, p_create_info, p_allocator, p_surface)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::extensions::khr_surface::SurfaceKHR> {
        let create_display_plane_surface_khr = (*self.table)
            .create_display_plane_surface_khr
            .unwrap();
        let mut surface = Default::default();
        let result = create_display_plane_surface_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut surface,
        );
        crate::new_result(surface, result)
    }
}
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
pub const KHR_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!("VK_KHR_display");
