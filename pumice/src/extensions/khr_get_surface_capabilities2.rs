#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html)
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub surface: crate::extensions::khr_surface::SurfaceKHR,
}
impl Default for PhysicalDeviceSurfaceInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            p_next: std::ptr::null(),
            surface: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceCapabilities2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2KHR.html)
pub struct SurfaceCapabilities2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub surface_capabilities: crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
}
impl Default for SurfaceCapabilities2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_CAPABILITIES_2_KHR,
            p_next: std::ptr::null_mut(),
            surface_capabilities: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSurfaceFormat2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormat2KHR.html)
pub struct SurfaceFormat2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub surface_format: crate::extensions::khr_surface::SurfaceFormatKHR,
}
impl Default for SurfaceFormat2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SURFACE_FORMAT_2_KHR,
            p_next: std::ptr::null_mut(),
            surface_format: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
pub unsafe fn get_physical_device_surface_capabilities_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_capabilities_2_khr
        .unwrap())(physical_device, p_surface_info, p_surface_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
    pub unsafe fn get_physical_device_surface_capabilities_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::VulkanResult<SurfaceCapabilities2KHR> {
        let get_physical_device_surface_capabilities_2_khr = (*self.table)
            .get_physical_device_surface_capabilities_2_khr
            .unwrap();
        let mut surface_capabilities = Default::default();
        let result = get_physical_device_surface_capabilities_2_khr(
            physical_device,
            surface_info as _,
            &mut surface_capabilities,
        );
        crate::new_result(surface_capabilities, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
pub unsafe fn get_physical_device_surface_formats_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormat2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_surface_formats_2_khr
        .unwrap())(
        physical_device,
        p_surface_info,
        p_surface_format_count,
        p_surface_formats,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
    pub unsafe fn get_physical_device_surface_formats_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_format_count: Option<u32>,
        mut surface_formats_callback: impl FnMut(&mut Vec<SurfaceFormat2KHR>),
    ) -> crate::VulkanResult<(Vec<SurfaceFormat2KHR>, crate::vk10::Result)> {
        let get_physical_device_surface_formats_2_khr = (*self.table)
            .get_physical_device_surface_formats_2_khr
            .unwrap();
        let mut surface_format_count = match surface_format_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_surface_formats_2_khr(
                    physical_device,
                    surface_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut surface_formats = vec![
            Default::default(); surface_format_count as usize
        ];
        surface_formats_callback(&mut surface_formats);
        let result = get_physical_device_surface_formats_2_khr(
            physical_device,
            surface_info as _,
            &mut surface_format_count,
            surface_formats.as_mut_ptr(),
        );
        crate::new_result((surface_formats, result), result)
    }
}
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
pub const KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_get_surface_capabilities2"
);
