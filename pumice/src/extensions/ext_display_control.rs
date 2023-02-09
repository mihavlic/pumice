#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayPowerInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html)
pub struct DisplayPowerInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub power_state: DisplayPowerStateEXT,
}
impl Default for DisplayPowerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_POWER_INFO_EXT,
            p_next: std::ptr::null(),
            power_state: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceEventInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html)
pub struct DeviceEventInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub device_event: DeviceEventTypeEXT,
}
impl Default for DeviceEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            device_event: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDisplayEventInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html)
pub struct DisplayEventInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub display_event: DisplayEventTypeEXT,
}
impl Default for DisplayEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DISPLAY_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            display_event: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html)
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub surface_counters: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
}
impl Default for SwapchainCounterCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            surface_counters: Default::default(),
        }
    }
}
#[doc(alias = "VkDisplayPowerStateEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPowerStateEXT(pub i32);
impl DisplayPowerStateEXT {
    pub const OFF: Self = Self(0);
    pub const SUSPEND: Self = Self(1);
    pub const ON: Self = Self(2);
}
crate::enum_impl! {
    DisplayPowerStateEXT : i32, OFF, SUSPEND, ON
}
#[doc(alias = "VkDeviceEventTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DeviceEventTypeEXT(pub i32);
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG: Self = Self(0);
}
crate::enum_impl! {
    DeviceEventTypeEXT : i32, DISPLAY_HOTPLUG
}
#[doc(alias = "VkDisplayEventTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DisplayEventTypeEXT(pub i32);
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT: Self = Self(0);
}
crate::enum_impl! {
    DisplayEventTypeEXT : i32, FIRST_PIXEL_OUT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDisplayPowerControlEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html)
pub unsafe fn display_power_control_ext(
    device: crate::vk10::Device,
    display: crate::extensions::khr_display::DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .display_power_control_ext
        .unwrap())(device, display, p_display_power_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkDisplayPowerControlEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html)
    pub unsafe fn display_power_control_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> crate::VulkanResult<()> {
        let display_power_control_ext = (*self.table).display_power_control_ext.unwrap();
        let result = display_power_control_ext(
            self.handle,
            display,
            display_power_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkRegisterDeviceEventEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html)
pub unsafe fn register_device_event_ext(
    device: crate::vk10::Device,
    p_device_event_info: *const DeviceEventInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_fence: *mut crate::vk10::Fence,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .register_device_event_ext
        .unwrap())(device, p_device_event_info, p_allocator, p_fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html)
    pub unsafe fn register_device_event_ext(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk10::Fence> {
        let register_device_event_ext = (*self.table).register_device_event_ext.unwrap();
        let mut fence = Default::default();
        let result = register_device_event_ext(
            self.handle,
            device_event_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::new_result(fence, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkRegisterDisplayEventEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html)
pub unsafe fn register_display_event_ext(
    device: crate::vk10::Device,
    display: crate::extensions::khr_display::DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_fence: *mut crate::vk10::Fence,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .register_display_event_ext
        .unwrap())(device, display, p_display_event_info, p_allocator, p_fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html)
    pub unsafe fn register_display_event_ext(
        &self,
        display: crate::extensions::khr_display::DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk10::Fence> {
        let register_display_event_ext = (*self.table)
            .register_display_event_ext
            .unwrap();
        let mut fence = Default::default();
        let result = register_display_event_ext(
            self.handle,
            display,
            display_event_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::new_result(fence, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSwapchainCounterEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html)
pub unsafe fn get_swapchain_counter_ext(
    device: crate::vk10::Device,
    swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    p_counter_value: *mut u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_swapchain_counter_ext
        .unwrap())(device, swapchain, counter, p_counter_value)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html)
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    ) -> crate::VulkanResult<u64> {
        let get_swapchain_counter_ext = (*self.table).get_swapchain_counter_ext.unwrap();
        let mut counter_value = Default::default();
        let result = get_swapchain_counter_ext(
            self.handle,
            swapchain,
            counter as _,
            &mut counter_value,
        );
        crate::new_result(counter_value, result)
    }
}
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
pub const EXT_DISPLAY_CONTROL_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_display_control"
);
