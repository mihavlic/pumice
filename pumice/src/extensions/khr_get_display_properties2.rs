#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html)
pub struct DisplayProperties2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub display_properties: crate::extensions::khr_display::DisplayPropertiesKHR,
}
impl Default for DisplayProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneProperties2KHR.html)
pub struct DisplayPlaneProperties2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub display_plane_properties: crate::extensions::khr_display::DisplayPlanePropertiesKHR,
}
impl Default for DisplayPlaneProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_plane_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayModeProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeProperties2KHR.html)
pub struct DisplayModeProperties2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub display_mode_properties: crate::extensions::khr_display::DisplayModePropertiesKHR,
}
impl Default for DisplayModeProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_MODE_PROPERTIES_2_KHR,
            p_next: std::ptr::null_mut(),
            display_mode_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html)
pub struct DisplayPlaneInfo2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mode: crate::extensions::khr_display::DisplayModeKHR,
    pub plane_index: u32,
}
impl Default for DisplayPlaneInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_PLANE_INFO_2_KHR,
            p_next: std::ptr::null(),
            mode: Default::default(),
            plane_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html)
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub capabilities: crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
}
impl Default for DisplayPlaneCapabilities2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR,
            p_next: std::ptr::null_mut(),
            capabilities: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
pub unsafe fn get_physical_device_display_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_display_properties_2_khr
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
    pub unsafe fn get_physical_device_display_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<DisplayProperties2KHR>),
    ) -> crate::VulkanResult<(Vec<DisplayProperties2KHR>, crate::vk10::Result)> {
        let get_physical_device_display_properties_2_khr = (*self.table)
            .get_physical_device_display_properties_2_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_display_properties_2_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        properties_callback(&mut properties);
        let result = get_physical_device_display_properties_2_khr(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
pub unsafe fn get_physical_device_display_plane_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_display_plane_properties_2_khr
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
    pub unsafe fn get_physical_device_display_plane_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<DisplayPlaneProperties2KHR>),
    ) -> crate::VulkanResult<(Vec<DisplayPlaneProperties2KHR>, crate::vk10::Result)> {
        let get_physical_device_display_plane_properties_2_khr = (*self.table)
            .get_physical_device_display_plane_properties_2_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_display_plane_properties_2_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        properties_callback(&mut properties);
        let result = get_physical_device_display_plane_properties_2_khr(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
pub unsafe fn get_display_mode_properties_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_display_mode_properties_2_khr
        .unwrap())(physical_device, display, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
    pub unsafe fn get_display_mode_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<DisplayModeProperties2KHR>),
    ) -> crate::VulkanResult<(Vec<DisplayModeProperties2KHR>, crate::vk10::Result)> {
        let get_display_mode_properties_2_khr = (*self.table)
            .get_display_mode_properties_2_khr
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_display_mode_properties_2_khr(
                    physical_device,
                    display,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        properties_callback(&mut properties);
        let result = get_display_mode_properties_2_khr(
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
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
pub unsafe fn get_display_plane_capabilities_2_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_display_plane_capabilities_2_khr
        .unwrap())(physical_device, p_display_plane_info, p_capabilities)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
    pub unsafe fn get_display_plane_capabilities_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> crate::VulkanResult<DisplayPlaneCapabilities2KHR> {
        let get_display_plane_capabilities_2_khr = (*self.table)
            .get_display_plane_capabilities_2_khr
            .unwrap();
        let mut capabilities = Default::default();
        let result = get_display_plane_capabilities_2_khr(
            physical_device,
            display_plane_info as _,
            &mut capabilities,
        );
        crate::new_result(capabilities, result)
    }
}
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_get_display_properties2"
);
