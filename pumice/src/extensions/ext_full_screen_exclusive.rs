#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html)
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}
impl Default for SurfaceFullScreenExclusiveInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html)
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub hmonitor: crate::extensions::khr_win32_surface::HMONITOR,
}
impl Default for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            p_next: std::ptr::null(),
            hmonitor: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html)
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub full_screen_exclusive_supported: crate::vk10::Bool32,
}
impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
            p_next: std::ptr::null_mut(),
            full_screen_exclusive_supported: Default::default(),
        }
    }
}
#[doc(alias = "VkFullScreenExclusiveEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FullScreenExclusiveEXT(pub i32);
impl FullScreenExclusiveEXT {
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
    pub const APPLICATION_CONTROLLED: Self = Self(3);
}
crate::enum_impl! {
    FullScreenExclusiveEXT : i32, DEFAULT, ALLOWED, DISALLOWED, APPLICATION_CONTROLLED
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
pub unsafe fn get_physical_device_surface_present_modes_2_ext(
    physical_device: crate::vk10::PhysicalDevice,
    p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_present_modes_2_ext
        .unwrap())(
        physical_device,
        p_surface_info,
        p_present_mode_count,
        p_present_modes,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
    pub unsafe fn get_physical_device_surface_present_modes_2_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        present_mode_count: Option<u32>,
    ) -> crate::VulkanResult<
        (Vec<crate::extensions::khr_surface::PresentModeKHR>, crate::vk10::Result),
    > {
        let get_physical_device_surface_present_modes_2_ext = (*self.table)
            .get_physical_device_surface_present_modes_2_ext
            .unwrap();
        let mut present_mode_count = match present_mode_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_surface_present_modes_2_ext(
                    physical_device,
                    surface_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut present_modes = vec![Default::default(); present_mode_count as usize];
        let result = get_physical_device_surface_present_modes_2_ext(
            physical_device,
            surface_info as _,
            &mut present_mode_count,
            present_modes.as_mut_ptr(),
        );
        crate::new_result((present_modes, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
pub unsafe fn get_device_group_surface_present_modes_2_ext(
    device: crate::vk10::Device,
    p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_group_surface_present_modes_2_ext
        .unwrap())(device, p_surface_info, p_modes)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
    pub unsafe fn get_device_group_surface_present_modes_2_ext(
        &self,
        surface_info: &crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        modes: &mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::VulkanResult<()> {
        let get_device_group_surface_present_modes_2_ext = (*self.table)
            .get_device_group_surface_present_modes_2_ext
            .unwrap();
        let result = get_device_group_surface_present_modes_2_ext(
            self.handle,
            surface_info as _,
            modes as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
pub unsafe fn acquire_full_screen_exclusive_mode_ext(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .acquire_full_screen_exclusive_mode_ext
        .unwrap())(device, swapchain)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::VulkanResult<()> {
        let acquire_full_screen_exclusive_mode_ext = (*self.table)
            .acquire_full_screen_exclusive_mode_ext
            .unwrap();
        let result = acquire_full_screen_exclusive_mode_ext(self.handle, swapchain);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
pub unsafe fn release_full_screen_exclusive_mode_ext(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .release_full_screen_exclusive_mode_ext
        .unwrap())(device, swapchain)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::VulkanResult<()> {
        let release_full_screen_exclusive_mode_ext = (*self.table)
            .release_full_screen_exclusive_mode_ext
            .unwrap();
        let result = release_full_screen_exclusive_mode_ext(self.handle, swapchain);
        crate::new_result((), result)
    }
}
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_full_screen_exclusive"
);
