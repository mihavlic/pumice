use crate::{
    cstr,
    loader::{DeviceLoad, EntryLoad, InstanceLoad},
    util::ApiLoadConfig,
};

/// Oh, yes. Little Bobby Tables we call him.

macro_rules! load_fns {
    ($table:ident, $loader:ident, $(($name:ident, $str:literal))+) => {
        $(
            $table.$name = ::std::mem::transmute($loader.load($crate::cstr!($str).as_ptr()));
        )+
    };
}

/// https://github.com/maxbla/const-zero#how-does-it-work
union ConstZeroedHack<T, const S: usize> {
    bytes: [u8; S],
    inner: std::mem::ManuallyDrop<T>,
}

impl<T, const S: usize> ConstZeroedHack<T, S> {
    const unsafe fn zero() -> T {
        std::mem::ManuallyDrop::into_inner(Self { bytes: [0; S] }.inner)
    }
}
#[cfg(feature = "global")]
pub static mut GLOBAL_ENTRY_TABLE: EntryTable = EntryTable::new_empty();
#[cfg(feature = "global")]
pub static mut GLOBAL_INSTANCE_TABLE: InstanceTable = InstanceTable::new_empty();
#[cfg(feature = "global")]
pub static mut GLOBAL_DEVICE_TABLE: DeviceTable = DeviceTable::new_empty();
pub struct EntryTable {
    pub create_instance: Option<
        unsafe extern "system" fn(
            p_create_info: *const crate::vk10::InstanceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_instance: *mut crate::vk10::Instance,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_layer_properties: Option<
        unsafe extern "system" fn(
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::LayerProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_extension_properties: Option<
        unsafe extern "system" fn(
            p_layer_name: *const std::os::raw::c_char,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::ExtensionProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_instance_version: Option<
        unsafe extern "system" fn(p_api_version: *mut u32) -> crate::vk10::Result,
    >,
}
impl EntryTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<EntryTable>();
            ConstZeroedHack::<EntryTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl EntryLoad) {
        unsafe {
            load_fns! {
                self, loader, (create_instance, "vkCreateInstance")
                (enumerate_instance_layer_properties,
                "vkEnumerateInstanceLayerProperties")
                (enumerate_instance_extension_properties,
                "vkEnumerateInstanceExtensionProperties") (enumerate_instance_version,
                "vkEnumerateInstanceVersion")
            }
        }
    }
}
#[cfg(feature = "raw")]
impl EntryTable {
    #[track_caller]
    #[doc(alias = "vkCreateInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
    pub unsafe fn create_instance(
        &self,
        p_create_info: *const crate::vk10::InstanceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_instance: *mut crate::vk10::Instance,
    ) -> crate::vk10::Result {
        (self.create_instance.unwrap())(p_create_info, p_allocator, p_instance)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::LayerProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_instance_layer_properties
            .unwrap())(p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        p_layer_name: *const std::os::raw::c_char,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::ExtensionProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_instance_extension_properties
            .unwrap())(p_layer_name, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
    pub unsafe fn enumerate_instance_version(
        &self,
        p_api_version: *mut u32,
    ) -> crate::vk10::Result {
        (self.enumerate_instance_version.unwrap())(p_api_version)
    }
}
impl Default for EntryTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
pub struct InstanceTable {
    pub destroy_instance: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub enumerate_physical_devices: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_physical_device_count: *mut u32,
            p_physical_devices: *mut crate::vk10::PhysicalDevice,
        ) -> crate::vk10::Result,
    >,
    pub get_instance_proc_addr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_name: *const std::os::raw::c_char,
        ) -> crate::vk10::PfnVoidFunction,
    >,
    pub get_physical_device_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk10::PhysicalDeviceProperties,
        ),
    >,
    pub get_physical_device_queue_family_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk10::QueueFamilyProperties,
        ),
    >,
    pub get_physical_device_memory_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk10::PhysicalDeviceMemoryProperties,
        ),
    >,
    pub get_physical_device_features: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk10::PhysicalDeviceFeatures,
        ),
    >,
    pub get_physical_device_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk10::FormatProperties,
        ),
    >,
    pub get_physical_device_image_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            kind: crate::vk10::ImageType,
            tiling: crate::vk10::ImageTiling,
            usage: crate::vk10::ImageUsageFlags,
            flags: crate::vk10::ImageCreateFlags,
            p_image_format_properties: *mut crate::vk10::ImageFormatProperties,
        ) -> crate::vk10::Result,
    >,
    pub create_device: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_create_info: *const crate::vk10::DeviceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_device: *mut crate::vk10::Device,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_device_layer_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::LayerProperties,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_device_extension_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_layer_name: *const std::os::raw::c_char,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::ExtensionProperties,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_sparse_image_format_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            kind: crate::vk10::ImageType,
            samples: crate::vk10::SampleCountFlags,
            usage: crate::vk10::ImageUsageFlags,
            tiling: crate::vk10::ImageTiling,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk10::SparseImageFormatProperties,
        ),
    >,
    pub get_physical_device_features_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
        ),
    >,
    pub get_physical_device_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
        ),
    >,
    pub get_physical_device_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk11::FormatProperties2,
        ),
    >,
    pub get_physical_device_image_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
            p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_queue_family_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
        ),
    >,
    pub get_physical_device_memory_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
        ),
    >,
    pub get_physical_device_sparse_image_format_properties_2: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk11::SparseImageFormatProperties2,
        ),
    >,
    pub get_physical_device_external_buffer_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
            p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
        ),
    >,
    pub get_physical_device_external_semaphore_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
            p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
        ),
    >,
    pub get_physical_device_external_fence_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
            p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
        ),
    >,
    pub enumerate_physical_device_groups: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_physical_device_group_count: *mut u32,
            p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_tool_properties: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_tool_count: *mut u32,
            p_tool_properties: *mut crate::vk13::PhysicalDeviceToolProperties,
        ) -> crate::vk10::Result,
    >,
    pub destroy_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_physical_device_surface_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_supported: *mut crate::vk10::Bool32,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_capabilities_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_formats_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_surface_format_count: *mut u32,
            p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_present_modes_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_present_rectangles_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_rect_count: *mut u32,
            p_rects: *mut crate::vk10::Rect2D,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_display_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_display::DisplayPropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_display_plane_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_display::DisplayPlanePropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_display_plane_supported_displays_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            plane_index: u32,
            p_display_count: *mut u32,
            p_displays: *mut crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_display_mode_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            display: crate::extensions::khr_display::DisplayKHR,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_display::DisplayModePropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_display_mode_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            display: crate::extensions::khr_display::DisplayKHR,
            p_create_info: *const crate::extensions::khr_display::DisplayModeCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_mode: *mut crate::extensions::khr_display::DisplayModeKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_display_plane_capabilities_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            mode: crate::extensions::khr_display::DisplayModeKHR,
            plane_index: u32,
            p_capabilities: *mut crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_display_plane_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_xlib_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_xlib_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            dpy: *mut crate::extensions::khr_xcb_surface::Display,
            visual_id: crate::extensions::khr_xcb_surface::VisualID,
        ) -> crate::vk10::Bool32,
    >,
    pub create_xcb_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_xcb_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            connection: *mut crate::extensions::khr_xcb_surface::xcb_connection_t,
            visual_id: crate::extensions::khr_xcb_surface::xcb_visualid_t,
        ) -> crate::vk10::Bool32,
    >,
    pub create_wayland_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_wayland_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            display: *mut crate::extensions::khr_wayland_surface::wl_display,
        ) -> crate::vk10::Bool32,
    >,
    pub create_android_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_win_32_surface_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_win_32_presentation_support_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
        ) -> crate::vk10::Bool32,
    >,
    pub create_debug_report_callback_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_callback: *mut crate::extensions::ext_debug_report::DebugReportCallbackEXT,
        ) -> crate::vk10::Result,
    >,
    pub destroy_debug_report_callback_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            callback: crate::extensions::ext_debug_report::DebugReportCallbackEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub debug_report_message_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT,
            object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
            object: u64,
            location: usize,
            message_code: i32,
            p_layer_prefix: *const std::os::raw::c_char,
            p_message: *const std::os::raw::c_char,
        ),
    >,
    pub get_physical_device_video_capabilities_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_video_profile: *const crate::extensions::khr_video_queue::VideoProfileInfoKHR,
            p_capabilities: *mut crate::extensions::khr_video_queue::VideoCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_video_format_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_video_format_info: *const crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR,
            p_video_format_property_count: *mut u32,
            p_video_format_properties: *mut crate::extensions::khr_video_queue::VideoFormatPropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_stream_descriptor_surface_ggp: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_external_image_format_properties_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            kind: crate::vk10::ImageType,
            tiling: crate::vk10::ImageTiling,
            usage: crate::vk10::ImageUsageFlags,
            flags: crate::vk10::ImageCreateFlags,
            external_handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
            p_external_image_format_properties: *mut crate::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_features_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
        ),
    >,
    pub get_physical_device_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
        ),
    >,
    pub get_physical_device_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            format: crate::vk10::Format,
            p_format_properties: *mut crate::vk11::FormatProperties2,
        ),
    >,
    pub get_physical_device_image_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
            p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_queue_family_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
        ),
    >,
    pub get_physical_device_memory_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
        ),
    >,
    pub get_physical_device_sparse_image_format_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
            p_property_count: *mut u32,
            p_properties: *mut crate::vk11::SparseImageFormatProperties2,
        ),
    >,
    pub create_vi_surface_nn: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub enumerate_physical_device_groups_khr: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_physical_device_group_count: *mut u32,
            p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_external_buffer_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
            p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
        ),
    >,
    pub get_physical_device_external_semaphore_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
            p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
        ),
    >,
    pub release_display_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            display: crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_xlib_display_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            dpy: *mut crate::extensions::khr_xcb_surface::Display,
            display: crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_rand_routput_display_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            dpy: *mut crate::extensions::khr_xcb_surface::Display,
            rr_output: crate::extensions::khr_xcb_surface::RROutput,
            p_display: *mut crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_capabilities_2_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_surface_capabilities: *mut crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_external_fence_properties_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
            p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
        ),
    >,
    pub enumerate_physical_device_queue_family_performance_query_counters_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            p_counter_count: *mut u32,
            p_counters: *mut crate::extensions::khr_performance_query::PerformanceCounterKHR,
            p_counter_descriptions: *mut crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_queue_family_performance_query_passes_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_performance_query_create_info: *const crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR,
            p_num_passes: *mut u32,
        ),
    >,
    pub get_physical_device_surface_capabilities_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
            p_surface_capabilities: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_formats_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
            p_surface_format_count: *mut u32,
            p_surface_formats: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_display_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayProperties2KHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_display_plane_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR,
        ) -> crate::vk10::Result,
    >,
    pub get_display_mode_properties_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            display: crate::extensions::khr_display::DisplayKHR,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR,
        ) -> crate::vk10::Result,
    >,
    pub get_display_plane_capabilities_2_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_display_plane_info: *const crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR,
            p_capabilities: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
        ) -> crate::vk10::Result,
    >,
    pub create_iossurface_mvk: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_mac_ossurface_mvk: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_debug_utils_messenger_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        ) -> crate::vk10::Result,
    >,
    pub destroy_debug_utils_messenger_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub submit_debug_utils_message_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
            message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
        ),
    >,
    pub get_physical_device_multisample_properties_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            samples: crate::vk10::SampleCountFlags,
            p_multisample_properties: *mut crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
        ),
    >,
    pub get_physical_device_calibrateable_time_domains_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_time_domain_count: *mut u32,
            p_time_domains: *mut crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
        ) -> crate::vk10::Result,
    >,
    pub create_image_pipe_surface_fuchsia: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_metal_surface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_fragment_shading_rates_khr: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_fragment_shading_rate_count: *mut u32,
            p_fragment_shading_rates: *mut crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_tool_properties_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_tool_count: *mut u32,
            p_tool_properties: *mut crate::vk13::PhysicalDeviceToolProperties,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_cooperative_matrix_properties_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_combination_count: *mut u32,
            p_combinations: *mut crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_surface_present_modes_2_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_headless_surface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_drm_display_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            drm_fd: i32,
            display: crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_drm_display_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            drm_fd: i32,
            connector_id: u32,
            display: *mut crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_winrt_display_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            display: crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_winrt_display_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            device_relative_id: u32,
            p_display: *mut crate::extensions::khr_display::DisplayKHR,
        ) -> crate::vk10::Result,
    >,
    pub create_direct_fbsurface_ext: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_direct_fbpresentation_support_ext: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            dfb: *mut crate::extensions::ext_directfb_surface::IDirectFB,
        ) -> crate::vk10::Bool32,
    >,
    pub create_screen_surface_qnx: Option<
        unsafe extern "system" fn(
            instance: crate::vk10::Instance,
            p_create_info: *const crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_physical_device_screen_presentation_support_qnx: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            queue_family_index: u32,
            window: *mut crate::extensions::qnx_screen_surface::_screen_window,
        ) -> crate::vk10::Bool32,
    >,
    pub get_physical_device_optical_flow_image_formats_nv: Option<
        unsafe extern "system" fn(
            physical_device: crate::vk10::PhysicalDevice,
            p_optical_flow_image_format_info: *const crate::extensions::nv_optical_flow::OpticalFlowImageFormatInfoNV,
            p_format_count: *mut u32,
            p_image_format_properties: *mut crate::extensions::nv_optical_flow::OpticalFlowImageFormatPropertiesNV,
        ) -> crate::vk10::Result,
    >,
}
impl InstanceTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<InstanceTable>();
            ConstZeroedHack::<InstanceTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl InstanceLoad, conf: &ApiLoadConfig) {
        unsafe {
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 0, 0)) {
                load_fns! {
                    self, loader, (destroy_instance, "vkDestroyInstance")
                    (enumerate_physical_devices, "vkEnumeratePhysicalDevices")
                    (get_instance_proc_addr, "vkGetInstanceProcAddr")
                    (get_physical_device_properties, "vkGetPhysicalDeviceProperties")
                    (get_physical_device_queue_family_properties,
                    "vkGetPhysicalDeviceQueueFamilyProperties")
                    (get_physical_device_memory_properties,
                    "vkGetPhysicalDeviceMemoryProperties") (get_physical_device_features,
                    "vkGetPhysicalDeviceFeatures")
                    (get_physical_device_format_properties,
                    "vkGetPhysicalDeviceFormatProperties")
                    (get_physical_device_image_format_properties,
                    "vkGetPhysicalDeviceImageFormatProperties") (create_device,
                    "vkCreateDevice") (enumerate_device_layer_properties,
                    "vkEnumerateDeviceLayerProperties")
                    (enumerate_device_extension_properties,
                    "vkEnumerateDeviceExtensionProperties")
                    (get_physical_device_sparse_image_format_properties,
                    "vkGetPhysicalDeviceSparseImageFormatProperties")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 1, 0)) {
                load_fns! {
                    self, loader, (get_physical_device_features_2,
                    "vkGetPhysicalDeviceFeatures2") (get_physical_device_properties_2,
                    "vkGetPhysicalDeviceProperties2")
                    (get_physical_device_format_properties_2,
                    "vkGetPhysicalDeviceFormatProperties2")
                    (get_physical_device_image_format_properties_2,
                    "vkGetPhysicalDeviceImageFormatProperties2")
                    (get_physical_device_queue_family_properties_2,
                    "vkGetPhysicalDeviceQueueFamilyProperties2")
                    (get_physical_device_memory_properties_2,
                    "vkGetPhysicalDeviceMemoryProperties2")
                    (get_physical_device_sparse_image_format_properties_2,
                    "vkGetPhysicalDeviceSparseImageFormatProperties2")
                    (get_physical_device_external_buffer_properties,
                    "vkGetPhysicalDeviceExternalBufferProperties")
                    (get_physical_device_external_semaphore_properties,
                    "vkGetPhysicalDeviceExternalSemaphoreProperties")
                    (get_physical_device_external_fence_properties,
                    "vkGetPhysicalDeviceExternalFenceProperties")
                    (enumerate_physical_device_groups, "vkEnumeratePhysicalDeviceGroups")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 3, 0)) {
                load_fns! {
                    self, loader, (get_physical_device_tool_properties,
                    "vkGetPhysicalDeviceToolProperties")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_surface")) {
                load_fns! {
                    self, loader, (destroy_surface_khr, "vkDestroySurfaceKHR")
                    (get_physical_device_surface_support_khr,
                    "vkGetPhysicalDeviceSurfaceSupportKHR")
                    (get_physical_device_surface_capabilities_khr,
                    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
                    (get_physical_device_surface_formats_khr,
                    "vkGetPhysicalDeviceSurfaceFormatsKHR")
                    (get_physical_device_surface_present_modes_khr,
                    "vkGetPhysicalDeviceSurfacePresentModesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_swapchain")) {
                load_fns! {
                    self, loader, (get_physical_device_present_rectangles_khr,
                    "vkGetPhysicalDevicePresentRectanglesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_display")) {
                load_fns! {
                    self, loader, (get_physical_device_display_properties_khr,
                    "vkGetPhysicalDeviceDisplayPropertiesKHR")
                    (get_physical_device_display_plane_properties_khr,
                    "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")
                    (get_display_plane_supported_displays_khr,
                    "vkGetDisplayPlaneSupportedDisplaysKHR")
                    (get_display_mode_properties_khr, "vkGetDisplayModePropertiesKHR")
                    (create_display_mode_khr, "vkCreateDisplayModeKHR")
                    (get_display_plane_capabilities_khr,
                    "vkGetDisplayPlaneCapabilitiesKHR")
                    (create_display_plane_surface_khr, "vkCreateDisplayPlaneSurfaceKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_xlib_surface")) {
                load_fns! {
                    self, loader, (create_xlib_surface_khr, "vkCreateXlibSurfaceKHR")
                    (get_physical_device_xlib_presentation_support_khr,
                    "vkGetPhysicalDeviceXlibPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_xcb_surface")) {
                load_fns! {
                    self, loader, (create_xcb_surface_khr, "vkCreateXcbSurfaceKHR")
                    (get_physical_device_xcb_presentation_support_khr,
                    "vkGetPhysicalDeviceXcbPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_wayland_surface")) {
                load_fns! {
                    self, loader, (create_wayland_surface_khr,
                    "vkCreateWaylandSurfaceKHR")
                    (get_physical_device_wayland_presentation_support_khr,
                    "vkGetPhysicalDeviceWaylandPresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_android_surface")) {
                load_fns! {
                    self, loader, (create_android_surface_khr,
                    "vkCreateAndroidSurfaceKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_win32_surface")) {
                load_fns! {
                    self, loader, (create_win_32_surface_khr, "vkCreateWin32SurfaceKHR")
                    (get_physical_device_win_32_presentation_support_khr,
                    "vkGetPhysicalDeviceWin32PresentationSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_report")) {
                load_fns! {
                    self, loader, (create_debug_report_callback_ext,
                    "vkCreateDebugReportCallbackEXT") (destroy_debug_report_callback_ext,
                    "vkDestroyDebugReportCallbackEXT") (debug_report_message_ext,
                    "vkDebugReportMessageEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_video_queue")) {
                load_fns! {
                    self, loader, (get_physical_device_video_capabilities_khr,
                    "vkGetPhysicalDeviceVideoCapabilitiesKHR")
                    (get_physical_device_video_format_properties_khr,
                    "vkGetPhysicalDeviceVideoFormatPropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_GGP_stream_descriptor_surface")) {
                load_fns! {
                    self, loader, (create_stream_descriptor_surface_ggp,
                    "vkCreateStreamDescriptorSurfaceGGP")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_external_memory_capabilities")) {
                load_fns! {
                    self, loader,
                    (get_physical_device_external_image_format_properties_nv,
                    "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_get_physical_device_properties2")) {
                load_fns! {
                    self, loader, (get_physical_device_features_2_khr,
                    "vkGetPhysicalDeviceFeatures2KHR")
                    (get_physical_device_properties_2_khr,
                    "vkGetPhysicalDeviceProperties2KHR")
                    (get_physical_device_format_properties_2_khr,
                    "vkGetPhysicalDeviceFormatProperties2KHR")
                    (get_physical_device_image_format_properties_2_khr,
                    "vkGetPhysicalDeviceImageFormatProperties2KHR")
                    (get_physical_device_queue_family_properties_2_khr,
                    "vkGetPhysicalDeviceQueueFamilyProperties2KHR")
                    (get_physical_device_memory_properties_2_khr,
                    "vkGetPhysicalDeviceMemoryProperties2KHR")
                    (get_physical_device_sparse_image_format_properties_2_khr,
                    "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_NN_vi_surface")) {
                load_fns! {
                    self, loader, (create_vi_surface_nn, "vkCreateViSurfaceNN")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_device_group_creation")) {
                load_fns! {
                    self, loader, (enumerate_physical_device_groups_khr,
                    "vkEnumeratePhysicalDeviceGroupsKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_memory_capabilities")) {
                load_fns! {
                    self, loader, (get_physical_device_external_buffer_properties_khr,
                    "vkGetPhysicalDeviceExternalBufferPropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_semaphore_capabilities")) {
                load_fns! {
                    self, loader, (get_physical_device_external_semaphore_properties_khr,
                    "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_direct_mode_display")) {
                load_fns! {
                    self, loader, (release_display_ext, "vkReleaseDisplayEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_acquire_xlib_display")) {
                load_fns! {
                    self, loader, (acquire_xlib_display_ext, "vkAcquireXlibDisplayEXT")
                    (get_rand_routput_display_ext, "vkGetRandROutputDisplayEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_display_surface_counter")) {
                load_fns! {
                    self, loader, (get_physical_device_surface_capabilities_2_ext,
                    "vkGetPhysicalDeviceSurfaceCapabilities2EXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_fence_capabilities")) {
                load_fns! {
                    self, loader, (get_physical_device_external_fence_properties_khr,
                    "vkGetPhysicalDeviceExternalFencePropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_performance_query")) {
                load_fns! {
                    self, loader,
                    (enumerate_physical_device_queue_family_performance_query_counters_khr,
                    "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")
                    (get_physical_device_queue_family_performance_query_passes_khr,
                    "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_get_surface_capabilities2")) {
                load_fns! {
                    self, loader, (get_physical_device_surface_capabilities_2_khr,
                    "vkGetPhysicalDeviceSurfaceCapabilities2KHR")
                    (get_physical_device_surface_formats_2_khr,
                    "vkGetPhysicalDeviceSurfaceFormats2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_get_display_properties2")) {
                load_fns! {
                    self, loader, (get_physical_device_display_properties_2_khr,
                    "vkGetPhysicalDeviceDisplayProperties2KHR")
                    (get_physical_device_display_plane_properties_2_khr,
                    "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")
                    (get_display_mode_properties_2_khr, "vkGetDisplayModeProperties2KHR")
                    (get_display_plane_capabilities_2_khr,
                    "vkGetDisplayPlaneCapabilities2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_MVK_ios_surface")) {
                load_fns! {
                    self, loader, (create_iossurface_mvk, "vkCreateIOSSurfaceMVK")
                }
            }
            if conf.extension_enabled(cstr!("VK_MVK_macos_surface")) {
                load_fns! {
                    self, loader, (create_mac_ossurface_mvk, "vkCreateMacOSSurfaceMVK")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_utils")) {
                load_fns! {
                    self, loader, (create_debug_utils_messenger_ext,
                    "vkCreateDebugUtilsMessengerEXT") (destroy_debug_utils_messenger_ext,
                    "vkDestroyDebugUtilsMessengerEXT") (submit_debug_utils_message_ext,
                    "vkSubmitDebugUtilsMessageEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_sample_locations")) {
                load_fns! {
                    self, loader, (get_physical_device_multisample_properties_ext,
                    "vkGetPhysicalDeviceMultisamplePropertiesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_calibrated_timestamps")) {
                load_fns! {
                    self, loader, (get_physical_device_calibrateable_time_domains_ext,
                    "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_FUCHSIA_imagepipe_surface")) {
                load_fns! {
                    self, loader, (create_image_pipe_surface_fuchsia,
                    "vkCreateImagePipeSurfaceFUCHSIA")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_metal_surface")) {
                load_fns! {
                    self, loader, (create_metal_surface_ext, "vkCreateMetalSurfaceEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_fragment_shading_rate")) {
                load_fns! {
                    self, loader, (get_physical_device_fragment_shading_rates_khr,
                    "vkGetPhysicalDeviceFragmentShadingRatesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_tooling_info")) {
                load_fns! {
                    self, loader, (get_physical_device_tool_properties_ext,
                    "vkGetPhysicalDeviceToolPropertiesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_cooperative_matrix")) {
                load_fns! {
                    self, loader, (get_physical_device_cooperative_matrix_properties_nv,
                    "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_coverage_reduction_mode")) {
                load_fns! {
                    self, loader,
                    (get_physical_device_supported_framebuffer_mixed_samples_combinations_nv,
                    "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_full_screen_exclusive")) {
                load_fns! {
                    self, loader, (get_physical_device_surface_present_modes_2_ext,
                    "vkGetPhysicalDeviceSurfacePresentModes2EXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_headless_surface")) {
                load_fns! {
                    self, loader, (create_headless_surface_ext,
                    "vkCreateHeadlessSurfaceEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_acquire_drm_display")) {
                load_fns! {
                    self, loader, (acquire_drm_display_ext, "vkAcquireDrmDisplayEXT")
                    (get_drm_display_ext, "vkGetDrmDisplayEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_acquire_winrt_display")) {
                load_fns! {
                    self, loader, (acquire_winrt_display_nv, "vkAcquireWinrtDisplayNV")
                    (get_winrt_display_nv, "vkGetWinrtDisplayNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_directfb_surface")) {
                load_fns! {
                    self, loader, (create_direct_fbsurface_ext,
                    "vkCreateDirectFBSurfaceEXT")
                    (get_physical_device_direct_fbpresentation_support_ext,
                    "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_QNX_screen_surface")) {
                load_fns! {
                    self, loader, (create_screen_surface_qnx, "vkCreateScreenSurfaceQNX")
                    (get_physical_device_screen_presentation_support_qnx,
                    "vkGetPhysicalDeviceScreenPresentationSupportQNX")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_optical_flow")) {
                load_fns! {
                    self, loader, (get_physical_device_optical_flow_image_formats_nv,
                    "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")
                }
            }
        }
    }
}
#[cfg(feature = "raw")]
impl InstanceTable {
    #[track_caller]
    #[doc(alias = "vkDestroyInstance")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
    pub unsafe fn destroy_instance(
        &self,
        instance: crate::vk10::Instance,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_instance.unwrap())(instance, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: crate::vk10::Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut crate::vk10::PhysicalDevice,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_devices
            .unwrap())(instance, p_physical_device_count, p_physical_devices)
    }
    #[track_caller]
    #[doc(alias = "vkGetInstanceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: crate::vk10::Instance,
        p_name: *const std::os::raw::c_char,
    ) -> crate::vk10::PfnVoidFunction {
        (self.get_instance_proc_addr.unwrap())(instance, p_name)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk10::PhysicalDeviceProperties,
    ) {
        (self.get_physical_device_properties.unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk10::QueueFamilyProperties,
    ) {
        (self
            .get_physical_device_queue_family_properties
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk10::PhysicalDeviceMemoryProperties,
    ) {
        (self
            .get_physical_device_memory_properties
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk10::PhysicalDeviceFeatures,
    ) {
        (self.get_physical_device_features.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk10::FormatProperties,
    ) {
        (self
            .get_physical_device_format_properties
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        tiling: crate::vk10::ImageTiling,
        usage: crate::vk10::ImageUsageFlags,
        flags: crate::vk10::ImageCreateFlags,
        p_image_format_properties: *mut crate::vk10::ImageFormatProperties,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_image_format_properties
            .unwrap())(
            physical_device,
            format,
            kind,
            tiling,
            usage,
            flags,
            p_image_format_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
    pub unsafe fn create_device(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_create_info: *const crate::vk10::DeviceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_device: *mut crate::vk10::Device,
    ) -> crate::vk10::Result {
        (self
            .create_device
            .unwrap())(physical_device, p_create_info, p_allocator, p_device)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::LayerProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_device_layer_properties
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_layer_name: *const std::os::raw::c_char,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::ExtensionProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_device_extension_properties
            .unwrap())(physical_device, p_layer_name, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        samples: crate::vk10::SampleCountFlags,
        usage: crate::vk10::ImageUsageFlags,
        tiling: crate::vk10::ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk10::SparseImageFormatProperties,
    ) {
        (self
            .get_physical_device_sparse_image_format_properties
            .unwrap())(
            physical_device,
            format,
            kind,
            samples,
            usage,
            tiling,
            p_property_count,
            p_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
    pub unsafe fn get_physical_device_features_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
    ) {
        (self.get_physical_device_features_2.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
    pub unsafe fn get_physical_device_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
    ) {
        (self.get_physical_device_properties_2.unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
    pub unsafe fn get_physical_device_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk11::FormatProperties2,
    ) {
        (self
            .get_physical_device_format_properties_2
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
    pub unsafe fn get_physical_device_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_image_format_properties_2
            .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
    pub unsafe fn get_physical_device_queue_family_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
    ) {
        (self
            .get_physical_device_queue_family_properties_2
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
    pub unsafe fn get_physical_device_memory_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
    ) {
        (self
            .get_physical_device_memory_properties_2
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk11::SparseImageFormatProperties2,
    ) {
        (self
            .get_physical_device_sparse_image_format_properties_2
            .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
    ) {
        (self
            .get_physical_device_external_buffer_properties
            .unwrap())(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
    ) {
        (self
            .get_physical_device_external_semaphore_properties
            .unwrap())(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
    ) {
        (self
            .get_physical_device_external_fence_properties
            .unwrap())(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: crate::vk10::Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_device_groups
            .unwrap())(
            instance,
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html)
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut crate::vk13::PhysicalDeviceToolProperties,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_tool_properties
            .unwrap())(physical_device, p_tool_count, p_tool_properties)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_surface_khr.unwrap())(instance, surface, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_supported: *mut crate::vk10::Bool32,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_support_khr
            .unwrap())(physical_device, queue_family_index, surface, p_supported)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_capabilities: *mut crate::extensions::khr_surface::SurfaceCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_capabilities_khr
            .unwrap())(physical_device, surface, p_surface_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut crate::extensions::khr_surface::SurfaceFormatKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_formats_khr
            .unwrap())(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_present_modes_khr
            .unwrap())(physical_device, surface, p_present_mode_count, p_present_modes)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut crate::vk10::Rect2D,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_present_rectangles_khr
            .unwrap())(physical_device, surface, p_rect_count, p_rects)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_display::DisplayPropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_display_properties_khr
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_display::DisplayPlanePropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_display_plane_properties_khr
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self
            .get_display_plane_supported_displays_khr
            .unwrap())(physical_device, plane_index, p_display_count, p_displays)
    }
    #[track_caller]
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_display::DisplayModePropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_display_mode_properties_khr
            .unwrap())(physical_device, display, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDisplayModeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html)
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        p_create_info: *const crate::extensions::khr_display::DisplayModeCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_mode: *mut crate::extensions::khr_display::DisplayModeKHR,
    ) -> crate::vk10::Result {
        (self
            .create_display_mode_khr
            .unwrap())(physical_device, display, p_create_info, p_allocator, p_mode)
    }
    #[track_caller]
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        mode: crate::extensions::khr_display::DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut crate::extensions::khr_display::DisplayPlaneCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_display_plane_capabilities_khr
            .unwrap())(physical_device, mode, plane_index, p_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_display_plane_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_xlib_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        visual_id: crate::extensions::khr_xcb_surface::VisualID,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_xlib_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, dpy, visual_id)
    }
    #[track_caller]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_xcb_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut crate::extensions::khr_xcb_surface::xcb_connection_t,
        visual_id: crate::extensions::khr_xcb_surface::xcb_visualid_t,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_xcb_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, connection, visual_id)
    }
    #[track_caller]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_wayland_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        display: *mut crate::extensions::khr_wayland_surface::wl_display,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_wayland_presentation_support_khr
            .unwrap())(physical_device, queue_family_index, display)
    }
    #[track_caller]
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_android_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
    pub unsafe fn create_win_32_surface_khr(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_win_32_surface_khr
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
    pub unsafe fn get_physical_device_win_32_presentation_support_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_win_32_presentation_support_khr
            .unwrap())(physical_device, queue_family_index)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_callback: *mut crate::extensions::ext_debug_report::DebugReportCallbackEXT,
    ) -> crate::vk10::Result {
        (self
            .create_debug_report_callback_ext
            .unwrap())(instance, p_create_info, p_allocator, p_callback)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        instance: crate::vk10::Instance,
        callback: crate::extensions::ext_debug_report::DebugReportCallbackEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_debug_report_callback_ext
            .unwrap())(instance, callback, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkDebugReportMessageEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html)
    pub unsafe fn debug_report_message_ext(
        &self,
        instance: crate::vk10::Instance,
        flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT,
        object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const std::os::raw::c_char,
        p_message: *const std::os::raw::c_char,
    ) {
        (self
            .debug_report_message_ext
            .unwrap())(
            instance,
            flags,
            object_type,
            object,
            location,
            message_code,
            p_layer_prefix,
            p_message,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceVideoCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_video_profile: *const crate::extensions::khr_video_queue::VideoProfileInfoKHR,
        p_capabilities: *mut crate::extensions::khr_video_queue::VideoCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_video_capabilities_khr
            .unwrap())(physical_device, p_video_profile, p_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceVideoFormatPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
    pub unsafe fn get_physical_device_video_format_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_video_format_info: *const crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut crate::extensions::khr_video_queue::VideoFormatPropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_video_format_properties_khr
            .unwrap())(
            physical_device,
            p_video_format_info,
            p_video_format_property_count,
            p_video_format_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_stream_descriptor_surface_ggp
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        tiling: crate::vk10::ImageTiling,
        usage: crate::vk10::ImageUsageFlags,
        flags: crate::vk10::ImageCreateFlags,
        external_handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut crate::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_external_image_format_properties_nv
            .unwrap())(
            physical_device,
            format,
            kind,
            tiling,
            usage,
            flags,
            external_handle_type,
            p_external_image_format_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html)
    pub unsafe fn get_physical_device_features_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_features: *mut crate::vk11::PhysicalDeviceFeatures2,
    ) {
        (self.get_physical_device_features_2_khr.unwrap())(physical_device, p_features)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html)
    pub unsafe fn get_physical_device_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_properties: *mut crate::vk11::PhysicalDeviceProperties2,
    ) {
        (self
            .get_physical_device_properties_2_khr
            .unwrap())(physical_device, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        p_format_properties: *mut crate::vk11::FormatProperties2,
    ) {
        (self
            .get_physical_device_format_properties_2_khr
            .unwrap())(physical_device, format, p_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_image_format_info: *const crate::vk11::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut crate::vk11::ImageFormatProperties2,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_image_format_properties_2_khr
            .unwrap())(physical_device, p_image_format_info, p_image_format_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
    pub unsafe fn get_physical_device_queue_family_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut crate::vk11::QueueFamilyProperties2,
    ) {
        (self
            .get_physical_device_queue_family_properties_2_khr
            .unwrap())(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html)
    pub unsafe fn get_physical_device_memory_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_memory_properties: *mut crate::vk11::PhysicalDeviceMemoryProperties2,
    ) {
        (self
            .get_physical_device_memory_properties_2_khr
            .unwrap())(physical_device, p_memory_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_format_info: *const crate::vk11::PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut crate::vk11::SparseImageFormatProperties2,
    ) {
        (self
            .get_physical_device_sparse_image_format_properties_2_khr
            .unwrap())(physical_device, p_format_info, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCreateViSurfaceNN")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html)
    pub unsafe fn create_vi_surface_nn(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_vi_surface_nn
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html)
    pub unsafe fn enumerate_physical_device_groups_khr(
        &self,
        instance: crate::vk10::Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut crate::vk11::PhysicalDeviceGroupProperties,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_device_groups_khr
            .unwrap())(
            instance,
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_buffer_info: *const crate::vk11::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut crate::vk11::ExternalBufferProperties,
    ) {
        (self
            .get_physical_device_external_buffer_properties_khr
            .unwrap())(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_semaphore_info: *const crate::vk11::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut crate::vk11::ExternalSemaphoreProperties,
    ) {
        (self
            .get_physical_device_external_semaphore_properties_khr
            .unwrap())(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkReleaseDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html)
    pub unsafe fn release_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self.release_display_ext.unwrap())(physical_device, display)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html)
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self.acquire_xlib_display_ext.unwrap())(physical_device, dpy, display)
    }
    #[track_caller]
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html)
    pub unsafe fn get_rand_routput_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        dpy: *mut crate::extensions::khr_xcb_surface::Display,
        rr_output: crate::extensions::khr_xcb_surface::RROutput,
        p_display: *mut crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self
            .get_rand_routput_display_ext
            .unwrap())(physical_device, dpy, rr_output, p_display)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
    pub unsafe fn get_physical_device_surface_capabilities_2_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_surface_capabilities: *mut crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_capabilities_2_ext
            .unwrap())(physical_device, surface, p_surface_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_external_fence_info: *const crate::vk11::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut crate::vk11::ExternalFenceProperties,
    ) {
        (self
            .get_physical_device_external_fence_properties_khr
            .unwrap())(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut crate::extensions::khr_performance_query::PerformanceCounterKHR,
        p_counter_descriptions: *mut crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR,
    ) -> crate::vk10::Result {
        (self
            .enumerate_physical_device_queue_family_performance_query_counters_khr
            .unwrap())(
            physical_device,
            queue_family_index,
            p_counter_count,
            p_counters,
            p_counter_descriptions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_performance_query_create_info: *const crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR,
        p_num_passes: *mut u32,
    ) {
        (self
            .get_physical_device_queue_family_performance_query_passes_khr
            .unwrap())(physical_device, p_performance_query_create_info, p_num_passes)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
    pub unsafe fn get_physical_device_surface_capabilities_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_capabilities_2_khr
            .unwrap())(physical_device, p_surface_info, p_surface_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
    pub unsafe fn get_physical_device_surface_formats_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_formats_2_khr
            .unwrap())(
            physical_device,
            p_surface_info,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
    pub unsafe fn get_physical_device_display_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayProperties2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_display_properties_2_khr
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
    pub unsafe fn get_physical_device_display_plane_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_display_plane_properties_2_khr
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
    pub unsafe fn get_display_mode_properties_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_display_mode_properties_2_khr
            .unwrap())(physical_device, display, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
    pub unsafe fn get_display_plane_capabilities_2_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_display_plane_info: *const crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR,
        p_capabilities: *mut crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR,
    ) -> crate::vk10::Result {
        (self
            .get_display_plane_capabilities_2_khr
            .unwrap())(physical_device, p_display_plane_info, p_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html)
    pub unsafe fn create_iossurface_mvk(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_iossurface_mvk
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
    pub unsafe fn create_mac_ossurface_mvk(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_mac_ossurface_mvk
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
    ) -> crate::vk10::Result {
        (self
            .create_debug_utils_messenger_ext
            .unwrap())(instance, p_create_info, p_allocator, p_messenger)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: crate::vk10::Instance,
        messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_debug_utils_messenger_ext
            .unwrap())(instance, messenger, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: crate::vk10::Instance,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
        message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    ) {
        (self
            .submit_debug_utils_message_ext
            .unwrap())(instance, message_severity, message_types, p_callback_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        samples: crate::vk10::SampleCountFlags,
        p_multisample_properties: *mut crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
    ) {
        (self
            .get_physical_device_multisample_properties_ext
            .unwrap())(physical_device, samples, p_multisample_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut crate::extensions::ext_calibrated_timestamps::TimeDomainEXT,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_calibrateable_time_domains_ext
            .unwrap())(physical_device, p_time_domain_count, p_time_domains)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_image_pipe_surface_fuchsia
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
    pub unsafe fn create_metal_surface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_metal_surface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_fragment_shading_rate_count: *mut u32,
        p_fragment_shading_rates: *mut crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_fragment_shading_rates_khr
            .unwrap())(
            physical_device,
            p_fragment_shading_rate_count,
            p_fragment_shading_rates,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html)
    pub unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut crate::vk13::PhysicalDeviceToolProperties,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_tool_properties_ext
            .unwrap())(physical_device, p_tool_count, p_tool_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_cooperative_matrix_properties_nv
            .unwrap())(physical_device, p_property_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
            .unwrap())(physical_device, p_combination_count, p_combinations)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
    pub unsafe fn get_physical_device_surface_present_modes_2_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut crate::extensions::khr_surface::PresentModeKHR,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_surface_present_modes_2_ext
            .unwrap())(
            physical_device,
            p_surface_info,
            p_present_mode_count,
            p_present_modes,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_headless_surface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html)
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        drm_fd: i32,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self.acquire_drm_display_ext.unwrap())(physical_device, drm_fd, display)
    }
    #[track_caller]
    #[doc(alias = "vkGetDrmDisplayEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html)
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self
            .get_drm_display_ext
            .unwrap())(physical_device, drm_fd, connector_id, display)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html)
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self.acquire_winrt_display_nv.unwrap())(physical_device, display)
    }
    #[track_caller]
    #[doc(alias = "vkGetWinrtDisplayNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html)
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut crate::extensions::khr_display::DisplayKHR,
    ) -> crate::vk10::Result {
        (self
            .get_winrt_display_nv
            .unwrap())(physical_device, device_relative_id, p_display)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
    pub unsafe fn create_direct_fbsurface_ext(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_direct_fbsurface_ext
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
    pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut crate::extensions::ext_directfb_surface::IDirectFB,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_direct_fbpresentation_support_ext
            .unwrap())(physical_device, queue_family_index, dfb)
    }
    #[track_caller]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html)
    pub unsafe fn create_screen_surface_qnx(
        &self,
        instance: crate::vk10::Instance,
        p_create_info: *const crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_surface: *mut crate::extensions::khr_surface::SurfaceKHR,
    ) -> crate::vk10::Result {
        (self
            .create_screen_surface_qnx
            .unwrap())(instance, p_create_info, p_allocator, p_surface)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        queue_family_index: u32,
        window: *mut crate::extensions::qnx_screen_surface::_screen_window,
    ) -> crate::vk10::Bool32 {
        (self
            .get_physical_device_screen_presentation_support_qnx
            .unwrap())(physical_device, queue_family_index, window)
    }
    #[track_caller]
    #[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        p_optical_flow_image_format_info: *const crate::extensions::nv_optical_flow::OpticalFlowImageFormatInfoNV,
        p_format_count: *mut u32,
        p_image_format_properties: *mut crate::extensions::nv_optical_flow::OpticalFlowImageFormatPropertiesNV,
    ) -> crate::vk10::Result {
        (self
            .get_physical_device_optical_flow_image_formats_nv
            .unwrap())(
            physical_device,
            p_optical_flow_image_format_info,
            p_format_count,
            p_image_format_properties,
        )
    }
}
impl Default for InstanceTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
pub struct DeviceTable {
    pub get_device_proc_addr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_name: *const std::os::raw::c_char,
        ) -> crate::vk10::PfnVoidFunction,
    >,
    pub destroy_device: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_device_queue: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            queue_family_index: u32,
            queue_index: u32,
            p_queue: *mut crate::vk10::Queue,
        ),
    >,
    pub queue_submit: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            submit_count: u32,
            p_submits: *const crate::vk10::SubmitInfo,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub queue_wait_idle: Option<
        unsafe extern "system" fn(queue: crate::vk10::Queue) -> crate::vk10::Result,
    >,
    pub device_wait_idle: Option<
        unsafe extern "system" fn(device: crate::vk10::Device) -> crate::vk10::Result,
    >,
    pub allocate_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::MemoryAllocateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_memory: *mut crate::vk10::DeviceMemory,
        ) -> crate::vk10::Result,
    >,
    pub free_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub map_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            offset: crate::vk10::DeviceSize,
            size: crate::vk10::DeviceSize,
            flags: crate::vk10::MemoryMapFlags,
            pp_data: *mut *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub unmap_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
        ),
    >,
    pub flush_mapped_memory_ranges: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory_range_count: u32,
            p_memory_ranges: *const crate::vk10::MappedMemoryRange,
        ) -> crate::vk10::Result,
    >,
    pub invalidate_mapped_memory_ranges: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory_range_count: u32,
            p_memory_ranges: *const crate::vk10::MappedMemoryRange,
        ) -> crate::vk10::Result,
    >,
    pub get_device_memory_commitment: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            p_committed_memory_in_bytes: *mut crate::vk10::DeviceSize,
        ),
    >,
    pub get_buffer_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            p_memory_requirements: *mut crate::vk10::MemoryRequirements,
        ),
    >,
    pub bind_buffer_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            memory: crate::vk10::DeviceMemory,
            memory_offset: crate::vk10::DeviceSize,
        ) -> crate::vk10::Result,
    >,
    pub get_image_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_memory_requirements: *mut crate::vk10::MemoryRequirements,
        ),
    >,
    pub bind_image_memory: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            memory: crate::vk10::DeviceMemory,
            memory_offset: crate::vk10::DeviceSize,
        ) -> crate::vk10::Result,
    >,
    pub get_image_sparse_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk10::SparseImageMemoryRequirements,
        ),
    >,
    pub queue_bind_sparse: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            bind_info_count: u32,
            p_bind_info: *const crate::vk10::BindSparseInfo,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub create_fence: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::FenceCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_fence: *mut crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub destroy_fence: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence: crate::vk10::Fence,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_fences: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence_count: u32,
            p_fences: *const crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub get_fence_status: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub wait_for_fences: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            fence_count: u32,
            p_fences: *const crate::vk10::Fence,
            wait_all: crate::vk10::Bool32,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub create_semaphore: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::SemaphoreCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_semaphore: *mut crate::vk10::Semaphore,
        ) -> crate::vk10::Result,
    >,
    pub destroy_semaphore: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            semaphore: crate::vk10::Semaphore,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::EventCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_event: *mut crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub destroy_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_event_status: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub set_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub reset_event: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            event: crate::vk10::Event,
        ) -> crate::vk10::Result,
    >,
    pub create_query_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::QueryPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_query_pool: *mut crate::vk10::QueryPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_query_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_query_pool_results: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
            stride: crate::vk10::DeviceSize,
            flags: crate::vk10::QueryResultFlags,
        ) -> crate::vk10::Result,
    >,
    pub create_buffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::BufferCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_buffer: *mut crate::vk10::Buffer,
        ) -> crate::vk10::Result,
    >,
    pub destroy_buffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: crate::vk10::Buffer,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_buffer_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::BufferViewCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_view: *mut crate::vk10::BufferView,
        ) -> crate::vk10::Result,
    >,
    pub destroy_buffer_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer_view: crate::vk10::BufferView,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_image: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ImageCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_image: *mut crate::vk10::Image,
        ) -> crate::vk10::Result,
    >,
    pub destroy_image: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_image_subresource_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_subresource: *const crate::vk10::ImageSubresource,
            p_layout: *mut crate::vk10::SubresourceLayout,
        ),
    >,
    pub create_image_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ImageViewCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_view: *mut crate::vk10::ImageView,
        ) -> crate::vk10::Result,
    >,
    pub destroy_image_view: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image_view: crate::vk10::ImageView,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_shader_module: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_shader_module: *mut crate::vk10::ShaderModule,
        ) -> crate::vk10::Result,
    >,
    pub destroy_shader_module: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            shader_module: crate::vk10::ShaderModule,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_pipeline_cache: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::PipelineCacheCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipeline_cache: *mut crate::vk10::PipelineCache,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline_cache: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_pipeline_cache_data: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            p_data_size: *mut usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub merge_pipeline_caches: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            dst_cache: crate::vk10::PipelineCache,
            src_cache_count: u32,
            p_src_caches: *const crate::vk10::PipelineCache,
        ) -> crate::vk10::Result,
    >,
    pub create_graphics_pipelines: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::vk10::GraphicsPipelineCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub create_compute_pipelines: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::vk10::ComputePipelineCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_pipeline_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::PipelineLayoutCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipeline_layout: *mut crate::vk10::PipelineLayout,
        ) -> crate::vk10::Result,
    >,
    pub destroy_pipeline_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_layout: crate::vk10::PipelineLayout,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_sampler: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::SamplerCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_sampler: *mut crate::vk10::Sampler,
        ) -> crate::vk10::Result,
    >,
    pub destroy_sampler: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            sampler: crate::vk10::Sampler,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_descriptor_set_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_set_layout: *mut crate::vk10::DescriptorSetLayout,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_set_layout: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set_layout: crate::vk10::DescriptorSetLayout,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_descriptor_pool: *mut crate::vk10::DescriptorPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_descriptor_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            flags: crate::vk10::DescriptorPoolResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub allocate_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::DescriptorSetAllocateInfo,
            p_descriptor_sets: *mut crate::vk10::DescriptorSet,
        ) -> crate::vk10::Result,
    >,
    pub free_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_pool: crate::vk10::DescriptorPool,
            descriptor_set_count: u32,
            p_descriptor_sets: *const crate::vk10::DescriptorSet,
        ) -> crate::vk10::Result,
    >,
    pub update_descriptor_sets: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_write_count: u32,
            p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
            descriptor_copy_count: u32,
            p_descriptor_copies: *const crate::vk10::CopyDescriptorSet,
        ),
    >,
    pub create_framebuffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::FramebufferCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_framebuffer: *mut crate::vk10::Framebuffer,
        ) -> crate::vk10::Result,
    >,
    pub destroy_framebuffer: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            framebuffer: crate::vk10::Framebuffer,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_render_pass: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::RenderPassCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_render_pass: *mut crate::vk10::RenderPass,
        ) -> crate::vk10::Result,
    >,
    pub destroy_render_pass: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            render_pass: crate::vk10::RenderPass,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_render_area_granularity: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            render_pass: crate::vk10::RenderPass,
            p_granularity: *mut crate::vk10::Extent2D,
        ),
    >,
    pub create_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::CommandPoolCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_command_pool: *mut crate::vk10::CommandPool,
        ) -> crate::vk10::Result,
    >,
    pub destroy_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub reset_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            flags: crate::vk10::CommandPoolResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub allocate_command_buffers: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocate_info: *const crate::vk10::CommandBufferAllocateInfo,
            p_command_buffers: *mut crate::vk10::CommandBuffer,
        ) -> crate::vk10::Result,
    >,
    pub free_command_buffers: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            command_buffer_count: u32,
            p_command_buffers: *const crate::vk10::CommandBuffer,
        ),
    >,
    pub begin_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_begin_info: *const crate::vk10::CommandBufferBeginInfo,
        ) -> crate::vk10::Result,
    >,
    pub end_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
        ) -> crate::vk10::Result,
    >,
    pub reset_command_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            flags: crate::vk10::CommandBufferResetFlags,
        ) -> crate::vk10::Result,
    >,
    pub cmd_bind_pipeline: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            pipeline: crate::vk10::Pipeline,
        ),
    >,
    pub cmd_set_viewport: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
            p_viewports: *const crate::vk10::Viewport,
        ),
    >,
    pub cmd_set_scissor: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_scissor: u32,
            scissor_count: u32,
            p_scissors: *const crate::vk10::Rect2D,
        ),
    >,
    pub cmd_set_line_width: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            line_width: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_depth_bias: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bias_constant_factor: std::os::raw::c_float,
            depth_bias_clamp: std::os::raw::c_float,
            depth_bias_slope_factor: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_blend_constants: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            blend_constants: *const [std::os::raw::c_float; 4],
        ),
    >,
    pub cmd_set_depth_bounds: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            min_depth_bounds: std::os::raw::c_float,
            max_depth_bounds: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_stencil_compare_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            compare_mask: u32,
        ),
    >,
    pub cmd_set_stencil_write_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            write_mask: u32,
        ),
    >,
    pub cmd_set_stencil_reference: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            reference: u32,
        ),
    >,
    pub cmd_bind_descriptor_sets: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            layout: crate::vk10::PipelineLayout,
            first_set: u32,
            descriptor_set_count: u32,
            p_descriptor_sets: *const crate::vk10::DescriptorSet,
            dynamic_offset_count: u32,
            p_dynamic_offsets: *const u32,
        ),
    >,
    pub cmd_bind_index_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            index_type: crate::vk10::IndexType,
        ),
    >,
    pub cmd_bind_vertex_buffers: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_binding: u32,
            binding_count: u32,
            p_buffers: *const crate::vk10::Buffer,
            p_offsets: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_draw: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            vertex_count: u32,
            instance_count: u32,
            first_vertex: u32,
            first_instance: u32,
        ),
    >,
    pub cmd_draw_indexed: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            index_count: u32,
            instance_count: u32,
            first_index: u32,
            vertex_offset: i32,
            first_instance: u32,
        ),
    >,
    pub cmd_draw_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_indexed_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_dispatch: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub cmd_dispatch_indirect: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_copy_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_buffer: crate::vk10::Buffer,
            dst_buffer: crate::vk10::Buffer,
            region_count: u32,
            p_regions: *const crate::vk10::BufferCopy,
        ),
    >,
    pub cmd_copy_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageCopy,
        ),
    >,
    pub cmd_blit_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageBlit,
            filter: crate::vk10::Filter,
        ),
    >,
    pub cmd_copy_buffer_to_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_buffer: crate::vk10::Buffer,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::BufferImageCopy,
        ),
    >,
    pub cmd_copy_image_to_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_buffer: crate::vk10::Buffer,
            region_count: u32,
            p_regions: *const crate::vk10::BufferImageCopy,
        ),
    >,
    pub cmd_update_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            data_size: crate::vk10::DeviceSize,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_fill_buffer: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            size: crate::vk10::DeviceSize,
            data: u32,
        ),
    >,
    pub cmd_clear_color_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image: crate::vk10::Image,
            image_layout: crate::vk10::ImageLayout,
            p_color: *const crate::vk10::ClearColorValue,
            range_count: u32,
            p_ranges: *const crate::vk10::ImageSubresourceRange,
        ),
    >,
    pub cmd_clear_depth_stencil_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image: crate::vk10::Image,
            image_layout: crate::vk10::ImageLayout,
            p_depth_stencil: *const crate::vk10::ClearDepthStencilValue,
            range_count: u32,
            p_ranges: *const crate::vk10::ImageSubresourceRange,
        ),
    >,
    pub cmd_clear_attachments: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            attachment_count: u32,
            p_attachments: *const crate::vk10::ClearAttachment,
            rect_count: u32,
            p_rects: *const crate::vk10::ClearRect,
        ),
    >,
    pub cmd_resolve_image: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_image: crate::vk10::Image,
            src_image_layout: crate::vk10::ImageLayout,
            dst_image: crate::vk10::Image,
            dst_image_layout: crate::vk10::ImageLayout,
            region_count: u32,
            p_regions: *const crate::vk10::ImageResolve,
        ),
    >,
    pub cmd_set_event: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk10::PipelineStageFlags,
        ),
    >,
    pub cmd_reset_event: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk10::PipelineStageFlags,
        ),
    >,
    pub cmd_wait_events: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event_count: u32,
            p_events: *const crate::vk10::Event,
            src_stage_mask: crate::vk10::PipelineStageFlags,
            dst_stage_mask: crate::vk10::PipelineStageFlags,
            memory_barrier_count: u32,
            p_memory_barriers: *const crate::vk10::MemoryBarrier,
            buffer_memory_barrier_count: u32,
            p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
            image_memory_barrier_count: u32,
            p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
        ),
    >,
    pub cmd_pipeline_barrier: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            src_stage_mask: crate::vk10::PipelineStageFlags,
            dst_stage_mask: crate::vk10::PipelineStageFlags,
            dependency_flags: crate::vk10::DependencyFlags,
            memory_barrier_count: u32,
            p_memory_barriers: *const crate::vk10::MemoryBarrier,
            buffer_memory_barrier_count: u32,
            p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
            image_memory_barrier_count: u32,
            p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
        ),
    >,
    pub cmd_begin_query: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
            flags: crate::vk10::QueryControlFlags,
        ),
    >,
    pub cmd_end_query: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_reset_query_pool: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
        ),
    >,
    pub cmd_write_timestamp: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_stage: crate::vk10::PipelineStageFlags,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_copy_query_pool_results: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            stride: crate::vk10::DeviceSize,
            flags: crate::vk10::QueryResultFlags,
        ),
    >,
    pub cmd_push_constants: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            layout: crate::vk10::PipelineLayout,
            stage_flags: crate::vk10::ShaderStageFlags,
            offset: u32,
            size: u32,
            p_values: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_begin_render_pass: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
            contents: crate::vk10::SubpassContents,
        ),
    >,
    pub cmd_next_subpass: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            contents: crate::vk10::SubpassContents,
        ),
    >,
    pub cmd_end_render_pass: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_execute_commands: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            command_buffer_count: u32,
            p_command_buffers: *const crate::vk10::CommandBuffer,
        ),
    >,
    pub trim_command_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            flags: crate::vk11::CommandPoolTrimFlags,
        ),
    >,
    pub get_device_group_peer_memory_features: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            heap_index: u32,
            local_device_index: u32,
            remote_device_index: u32,
            p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
        ),
    >,
    pub bind_buffer_memory_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub bind_image_memory_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_device_mask: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            device_mask: u32,
        ),
    >,
    pub cmd_dispatch_base: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            base_group_x: u32,
            base_group_y: u32,
            base_group_z: u32,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub create_descriptor_update_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_update_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub update_descriptor_set_with_template: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set: crate::vk10::DescriptorSet,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub get_buffer_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_sparse_memory_requirements_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
        ),
    >,
    pub create_sampler_ycbcr_conversion: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
        ) -> crate::vk10::Result,
    >,
    pub destroy_sampler_ycbcr_conversion: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_device_queue_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_queue_info: *const crate::vk11::DeviceQueueInfo2,
            p_queue: *mut crate::vk10::Queue,
        ),
    >,
    pub get_descriptor_set_layout_support: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
            p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
        ),
    >,
    pub reset_query_pool: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
        ),
    >,
    pub create_render_pass_2: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk12::RenderPassCreateInfo2,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_render_pass: *mut crate::vk10::RenderPass,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_render_pass_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
            p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
        ),
    >,
    pub cmd_next_subpass_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
            p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
        ),
    >,
    pub cmd_end_render_pass_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
        ),
    >,
    pub get_semaphore_counter_value: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            semaphore: crate::vk10::Semaphore,
            p_value: *mut u64,
        ) -> crate::vk10::Result,
    >,
    pub wait_semaphores: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_wait_info: *const crate::vk12::SemaphoreWaitInfo,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub signal_semaphore: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_signal_info: *const crate::vk12::SemaphoreSignalInfo,
        ) -> crate::vk10::Result,
    >,
    pub cmd_draw_indirect_count: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_indexed_indirect_count: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub get_buffer_opaque_capture_address: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::BufferDeviceAddressInfo,
        ) -> u64,
    >,
    pub get_buffer_device_address: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::BufferDeviceAddressInfo,
        ) -> crate::vk10::DeviceAddress,
    >,
    pub get_device_memory_opaque_capture_address: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
        ) -> u64,
    >,
    pub get_device_buffer_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceBufferMemoryRequirements,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_device_image_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceImageMemoryRequirements,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_device_image_sparse_memory_requirements: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceImageMemoryRequirements,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
        ),
    >,
    pub cmd_set_cull_mode: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            cull_mode: crate::vk10::CullModeFlags,
        ),
    >,
    pub cmd_set_front_face: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            front_face: crate::vk10::FrontFace,
        ),
    >,
    pub cmd_set_primitive_topology: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            primitive_topology: crate::vk10::PrimitiveTopology,
        ),
    >,
    pub cmd_set_viewport_with_count: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            viewport_count: u32,
            p_viewports: *const crate::vk10::Viewport,
        ),
    >,
    pub cmd_set_scissor_with_count: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            scissor_count: u32,
            p_scissors: *const crate::vk10::Rect2D,
        ),
    >,
    pub cmd_bind_vertex_buffers_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_binding: u32,
            binding_count: u32,
            p_buffers: *const crate::vk10::Buffer,
            p_offsets: *const crate::vk10::DeviceSize,
            p_sizes: *const crate::vk10::DeviceSize,
            p_strides: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_set_depth_test_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_write_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_write_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_compare_op: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_compare_op: crate::vk10::CompareOp,
        ),
    >,
    pub cmd_set_depth_bounds_test_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bounds_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_stencil_test_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stencil_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_stencil_op: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            fail_op: crate::vk10::StencilOp,
            pass_op: crate::vk10::StencilOp,
            depth_fail_op: crate::vk10::StencilOp,
            compare_op: crate::vk10::CompareOp,
        ),
    >,
    pub cmd_set_rasterizer_discard_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            rasterizer_discard_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_bias_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bias_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_primitive_restart_enable: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            primitive_restart_enable: crate::vk10::Bool32,
        ),
    >,
    pub create_private_data_slot: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk13::PrivateDataSlotCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_private_data_slot: *mut crate::vk13::PrivateDataSlot,
        ) -> crate::vk10::Result,
    >,
    pub destroy_private_data_slot: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            private_data_slot: crate::vk13::PrivateDataSlot,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub set_private_data: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            object_type: crate::vk10::ObjectType,
            object_handle: u64,
            private_data_slot: crate::vk13::PrivateDataSlot,
            data: u64,
        ) -> crate::vk10::Result,
    >,
    pub get_private_data: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            object_type: crate::vk10::ObjectType,
            object_handle: u64,
            private_data_slot: crate::vk13::PrivateDataSlot,
            p_data: *mut u64,
        ),
    >,
    pub cmd_copy_buffer_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_buffer_info: *const crate::vk13::CopyBufferInfo2,
        ),
    >,
    pub cmd_copy_image_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_image_info: *const crate::vk13::CopyImageInfo2,
        ),
    >,
    pub cmd_blit_image_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_blit_image_info: *const crate::vk13::BlitImageInfo2,
        ),
    >,
    pub cmd_copy_buffer_to_image_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_buffer_to_image_info: *const crate::vk13::CopyBufferToImageInfo2,
        ),
    >,
    pub cmd_copy_image_to_buffer_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_image_to_buffer_info: *const crate::vk13::CopyImageToBufferInfo2,
        ),
    >,
    pub cmd_resolve_image_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_resolve_image_info: *const crate::vk13::ResolveImageInfo2,
        ),
    >,
    pub cmd_set_event_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            p_dependency_info: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub cmd_reset_event_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk13::PipelineStageFlags2,
        ),
    >,
    pub cmd_wait_events_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event_count: u32,
            p_events: *const crate::vk10::Event,
            p_dependency_infos: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub cmd_pipeline_barrier_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_dependency_info: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub queue_submit_2: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            submit_count: u32,
            p_submits: *const crate::vk13::SubmitInfo2,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_timestamp_2: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stage: crate::vk13::PipelineStageFlags2,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_begin_rendering: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_rendering_info: *const crate::vk13::RenderingInfo,
        ),
    >,
    pub cmd_end_rendering: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub create_swapchain_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub destroy_swapchain_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_swapchain_images_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_swapchain_image_count: *mut u32,
            p_swapchain_images: *mut crate::vk10::Image,
        ) -> crate::vk10::Result,
    >,
    pub acquire_next_image_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            timeout: u64,
            semaphore: crate::vk10::Semaphore,
            fence: crate::vk10::Fence,
            p_image_index: *mut u32,
        ) -> crate::vk10::Result,
    >,
    pub queue_present_khr: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_present_capabilities_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_device_group_present_capabilities: *mut crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_surface_present_modes_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            surface: crate::extensions::khr_surface::SurfaceKHR,
            p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_next_image_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
            p_image_index: *mut u32,
        ) -> crate::vk10::Result,
    >,
    pub create_shared_swapchains_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain_count: u32,
            p_create_infos: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_swapchains: *mut crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub debug_marker_set_object_name_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_name_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub debug_marker_set_object_tag_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_tag_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_debug_marker_begin_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
        ),
    >,
    pub cmd_debug_marker_end_ext: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_debug_marker_insert_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
        ),
    >,
    pub create_video_session_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_video_session: *mut crate::extensions::khr_video_queue::VideoSessionKHR,
        ) -> crate::vk10::Result,
    >,
    pub destroy_video_session_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_video_session_parameters_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_video_session_parameters: *mut crate::extensions::khr_video_queue::VideoSessionParametersKHR,
        ) -> crate::vk10::Result,
    >,
    pub update_video_session_parameters_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
            p_update_info: *const crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub destroy_video_session_parameters_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_video_session_memory_requirements_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
            p_memory_requirements_count: *mut u32,
            p_memory_requirements: *mut crate::extensions::khr_video_queue::VideoSessionMemoryRequirementsKHR,
        ) -> crate::vk10::Result,
    >,
    pub bind_video_session_memory_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
            bind_session_memory_info_count: u32,
            p_bind_session_memory_infos: *const crate::extensions::khr_video_queue::BindVideoSessionMemoryInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_video_coding_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_begin_info: *const crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR,
        ),
    >,
    pub cmd_control_video_coding_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_coding_control_info: *const crate::extensions::khr_video_queue::VideoCodingControlInfoKHR,
        ),
    >,
    pub cmd_end_video_coding_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_end_coding_info: *const crate::extensions::khr_video_queue::VideoEndCodingInfoKHR,
        ),
    >,
    pub cmd_decode_video_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_decode_info: *const crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR,
        ),
    >,
    pub cmd_bind_transform_feedback_buffers_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_binding: u32,
            binding_count: u32,
            p_buffers: *const crate::vk10::Buffer,
            p_offsets: *const crate::vk10::DeviceSize,
            p_sizes: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_begin_transform_feedback_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_counter_buffer: u32,
            counter_buffer_count: u32,
            p_counter_buffers: *const crate::vk10::Buffer,
            p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_end_transform_feedback_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_counter_buffer: u32,
            counter_buffer_count: u32,
            p_counter_buffers: *const crate::vk10::Buffer,
            p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_begin_query_indexed_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
            flags: crate::vk10::QueryControlFlags,
            index: u32,
        ),
    >,
    pub cmd_end_query_indexed_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            query_pool: crate::vk10::QueryPool,
            query: u32,
            index: u32,
        ),
    >,
    pub cmd_draw_indirect_byte_count_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            instance_count: u32,
            first_instance: u32,
            counter_buffer: crate::vk10::Buffer,
            counter_buffer_offset: crate::vk10::DeviceSize,
            counter_offset: u32,
            vertex_stride: u32,
        ),
    >,
    pub create_cu_module_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_module: *mut crate::extensions::nvx_binary_import::CuModuleNVX,
        ) -> crate::vk10::Result,
    >,
    pub create_cu_function_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_function: *mut crate::extensions::nvx_binary_import::CuFunctionNVX,
        ) -> crate::vk10::Result,
    >,
    pub destroy_cu_module_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            module: crate::extensions::nvx_binary_import::CuModuleNVX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub destroy_cu_function_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            function: crate::extensions::nvx_binary_import::CuFunctionNVX,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub cmd_cu_launch_kernel_nvx: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_launch_info: *const crate::extensions::nvx_binary_import::CuLaunchInfoNVX,
        ),
    >,
    pub get_image_view_handle_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
        ) -> u32,
    >,
    pub get_image_view_address_nvx: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image_view: crate::vk10::ImageView,
            p_properties: *mut crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
        ) -> crate::vk10::Result,
    >,
    pub cmd_draw_indirect_count_amd: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_indexed_indirect_count_amd: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub get_shader_info_amd: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            shader_stage: crate::vk10::ShaderStageFlags,
            info_type: crate::extensions::amd_shader_info::ShaderInfoTypeAMD,
            p_info_size: *mut usize,
            p_info: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_rendering_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_rendering_info: *const crate::vk13::RenderingInfo,
        ),
    >,
    pub cmd_end_rendering_khr: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub get_memory_win_32_handle_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
            p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_peer_memory_features_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            heap_index: u32,
            local_device_index: u32,
            remote_device_index: u32,
            p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
        ),
    >,
    pub cmd_set_device_mask_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            device_mask: u32,
        ),
    >,
    pub cmd_dispatch_base_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            base_group_x: u32,
            base_group_y: u32,
            base_group_z: u32,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub trim_command_pool_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            command_pool: crate::vk10::CommandPool,
            flags: crate::vk11::CommandPoolTrimFlags,
        ),
    >,
    pub get_memory_win_32_handle_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_win_32_handle_info: *const crate::extensions::khr_external_memory_win32::MemoryGetWin32HandleInfoKHR,
            p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
        ) -> crate::vk10::Result,
    >,
    pub get_memory_win_32_handle_properties_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
            handle: crate::extensions::khr_win32_surface::HANDLE,
            p_memory_win_32_handle_properties: *mut crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_memory_fd_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_fd_info: *const crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
            p_fd: *mut std::os::raw::c_int,
        ) -> crate::vk10::Result,
    >,
    pub get_memory_fd_properties_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
            fd: std::os::raw::c_int,
            p_memory_fd_properties: *mut crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_semaphore_win_32_handle_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_win_32_handle_info: *const crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR,
            p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
        ) -> crate::vk10::Result,
    >,
    pub import_semaphore_win_32_handle_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_import_semaphore_win_32_handle_info: *const crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_semaphore_fd_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_fd_info: *const crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
            p_fd: *mut std::os::raw::c_int,
        ) -> crate::vk10::Result,
    >,
    pub import_semaphore_fd_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_import_semaphore_fd_info: *const crate::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_push_descriptor_set_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            layout: crate::vk10::PipelineLayout,
            set: u32,
            descriptor_write_count: u32,
            p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
        ),
    >,
    pub cmd_push_descriptor_set_with_template_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            layout: crate::vk10::PipelineLayout,
            set: u32,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_begin_conditional_rendering_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_conditional_rendering_begin: *const crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT,
        ),
    >,
    pub cmd_end_conditional_rendering_ext: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub create_descriptor_update_template_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
        ) -> crate::vk10::Result,
    >,
    pub destroy_descriptor_update_template_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub update_descriptor_set_with_template_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set: crate::vk10::DescriptorSet,
            descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
            p_data: *const std::os::raw::c_void,
        ),
    >,
    pub cmd_set_viewport_wscaling_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
            p_viewport_wscalings: *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV,
        ),
    >,
    pub display_power_control_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            display: crate::extensions::khr_display::DisplayKHR,
            p_display_power_info: *const crate::extensions::ext_display_control::DisplayPowerInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub register_device_event_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_device_event_info: *const crate::extensions::ext_display_control::DeviceEventInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_fence: *mut crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub register_display_event_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            display: crate::extensions::khr_display::DisplayKHR,
            p_display_event_info: *const crate::extensions::ext_display_control::DisplayEventInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_fence: *mut crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub get_swapchain_counter_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
            p_counter_value: *mut u64,
        ) -> crate::vk10::Result,
    >,
    pub get_refresh_cycle_duration_google: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_display_timing_properties: *mut crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
        ) -> crate::vk10::Result,
    >,
    pub get_past_presentation_timing_google: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            p_presentation_timing_count: *mut u32,
            p_presentation_timings: *mut crate::extensions::google_display_timing::PastPresentationTimingGOOGLE,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_discard_rectangle_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_discard_rectangle: u32,
            discard_rectangle_count: u32,
            p_discard_rectangles: *const crate::vk10::Rect2D,
        ),
    >,
    pub set_hdr_metadata_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain_count: u32,
            p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
            p_metadata: *const crate::extensions::ext_hdr_metadata::HdrMetadataEXT,
        ),
    >,
    pub create_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk12::RenderPassCreateInfo2,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_render_pass: *mut crate::vk10::RenderPass,
        ) -> crate::vk10::Result,
    >,
    pub cmd_begin_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
            p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
        ),
    >,
    pub cmd_next_subpass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
            p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
        ),
    >,
    pub cmd_end_render_pass_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
        ),
    >,
    pub get_swapchain_status_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_fence_win_32_handle_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_win_32_handle_info: *const crate::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR,
            p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
        ) -> crate::vk10::Result,
    >,
    pub import_fence_win_32_handle_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_import_fence_win_32_handle_info: *const crate::extensions::khr_external_fence_win32::ImportFenceWin32HandleInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_fence_fd_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_fd_info: *const crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR,
            p_fd: *mut std::os::raw::c_int,
        ) -> crate::vk10::Result,
    >,
    pub import_fence_fd_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_import_fence_fd_info: *const crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_profiling_lock_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub release_profiling_lock_khr: Option<
        unsafe extern "system" fn(device: crate::vk10::Device),
    >,
    pub set_debug_utils_object_name_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub set_debug_utils_object_tag_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub queue_begin_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub queue_end_debug_utils_label_ext: Option<
        unsafe extern "system" fn(queue: crate::vk10::Queue),
    >,
    pub queue_insert_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub cmd_begin_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub cmd_end_debug_utils_label_ext: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_insert_debug_utils_label_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
        ),
    >,
    pub get_android_hardware_buffer_properties_android: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            buffer: *const crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
            p_properties: *mut crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID,
        ) -> crate::vk10::Result,
    >,
    pub get_memory_android_hardware_buffer_android: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID,
            p_buffer: *mut *mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_sample_locations_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_sample_locations_info: *const crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
        ),
    >,
    pub get_buffer_memory_requirements_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_memory_requirements_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_image_sparse_memory_requirements_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
        ),
    >,
    pub destroy_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub cmd_copy_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR,
        ),
    >,
    pub copy_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_copy_acceleration_structure_to_memory_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR,
        ),
    >,
    pub copy_acceleration_structure_to_memory_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_copy_memory_to_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR,
        ),
    >,
    pub copy_memory_to_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_acceleration_structures_properties_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            acceleration_structure_count: u32,
            p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
            query_type: crate::vk10::QueryType,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
        ),
    >,
    pub write_acceleration_structures_properties_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            acceleration_structure_count: u32,
            p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
            query_type: crate::vk10::QueryType,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
            stride: usize,
        ) -> crate::vk10::Result,
    >,
    pub get_device_acceleration_structure_compatibility_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_version_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR,
            p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
        ),
    >,
    pub create_acceleration_structure_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_acceleration_structure: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_build_acceleration_structures_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            info_count: u32,
            p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
            pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
        ),
    >,
    pub cmd_build_acceleration_structures_indirect_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            info_count: u32,
            p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
            p_indirect_device_addresses: *const crate::vk10::DeviceAddress,
            p_indirect_strides: *const u32,
            pp_max_primitive_counts: *const *const u32,
        ),
    >,
    pub build_acceleration_structures_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            info_count: u32,
            p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
            pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_acceleration_structure_device_address_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR,
        ) -> crate::vk10::DeviceAddress,
    >,
    pub get_acceleration_structure_build_sizes_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
            p_build_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
            p_max_primitive_counts: *const u32,
            p_size_info: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR,
        ),
    >,
    pub cmd_trace_rays_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            width: u32,
            height: u32,
            depth: u32,
        ),
    >,
    pub get_ray_tracing_shader_group_handles_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            first_group: u32,
            group_count: u32,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            first_group: u32,
            group_count: u32,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub create_ray_tracing_pipelines_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub cmd_trace_rays_indirect_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
            indirect_device_address: crate::vk10::DeviceAddress,
        ),
    >,
    pub get_ray_tracing_shader_group_stack_size_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            group: u32,
            group_shader: crate::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR,
        ) -> crate::vk10::DeviceSize,
    >,
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_stack_size: u32,
        ),
    >,
    pub create_sampler_ycbcr_conversion_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
        ) -> crate::vk10::Result,
    >,
    pub destroy_sampler_ycbcr_conversion_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub bind_buffer_memory_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub bind_image_memory_2_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
        ) -> crate::vk10::Result,
    >,
    pub get_image_drm_format_modifier_properties_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_properties: *mut crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
        ) -> crate::vk10::Result,
    >,
    pub create_validation_cache_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_validation_cache: *mut crate::extensions::ext_validation_cache::ValidationCacheEXT,
        ) -> crate::vk10::Result,
    >,
    pub destroy_validation_cache_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_validation_cache_data_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
            p_data_size: *mut usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub merge_validation_caches_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
            src_cache_count: u32,
            p_src_caches: *const crate::extensions::ext_validation_cache::ValidationCacheEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_bind_shading_rate_image_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image_view: crate::vk10::ImageView,
            image_layout: crate::vk10::ImageLayout,
        ),
    >,
    pub cmd_set_viewport_shading_rate_palette_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
            p_shading_rate_palettes: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV,
        ),
    >,
    pub cmd_set_coarse_sample_order_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
            custom_sample_order_count: u32,
            p_custom_sample_orders: *const crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV,
        ),
    >,
    pub compile_deferred_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            shader: u32,
        ) -> crate::vk10::Result,
    >,
    pub create_acceleration_structure_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_acceleration_structure: *mut crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        ) -> crate::vk10::Result,
    >,
    pub destroy_acceleration_structure_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_acceleration_structure_memory_requirements_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV,
            p_memory_requirements: *mut crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
        ),
    >,
    pub bind_acceleration_structure_memory_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            bind_info_count: u32,
            p_bind_infos: *const crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV,
        ) -> crate::vk10::Result,
    >,
    pub cmd_copy_acceleration_structure_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
        ),
    >,
    pub cmd_write_acceleration_structures_properties_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            acceleration_structure_count: u32,
            p_acceleration_structures: *const crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            query_type: crate::vk10::QueryType,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
        ),
    >,
    pub cmd_build_acceleration_structure_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
            instance_data: crate::vk10::Buffer,
            instance_offset: crate::vk10::DeviceSize,
            update: crate::vk10::Bool32,
            dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            scratch: crate::vk10::Buffer,
            scratch_offset: crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_trace_rays_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            raygen_shader_binding_table_buffer: crate::vk10::Buffer,
            raygen_shader_binding_offset: crate::vk10::DeviceSize,
            miss_shader_binding_table_buffer: crate::vk10::Buffer,
            miss_shader_binding_offset: crate::vk10::DeviceSize,
            miss_shader_binding_stride: crate::vk10::DeviceSize,
            hit_shader_binding_table_buffer: crate::vk10::Buffer,
            hit_shader_binding_offset: crate::vk10::DeviceSize,
            hit_shader_binding_stride: crate::vk10::DeviceSize,
            callable_shader_binding_table_buffer: crate::vk10::Buffer,
            callable_shader_binding_offset: crate::vk10::DeviceSize,
            callable_shader_binding_stride: crate::vk10::DeviceSize,
            width: u32,
            height: u32,
            depth: u32,
        ),
    >,
    pub get_ray_tracing_shader_group_handles_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline: crate::vk10::Pipeline,
            first_group: u32,
            group_count: u32,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub get_acceleration_structure_handle_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
        ) -> crate::vk10::Result,
    >,
    pub create_ray_tracing_pipelines_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            pipeline_cache: crate::vk10::PipelineCache,
            create_info_count: u32,
            p_create_infos: *const crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_pipelines: *mut crate::vk10::Pipeline,
        ) -> crate::vk10::Result,
    >,
    pub get_descriptor_set_layout_support_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
            p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
        ),
    >,
    pub cmd_draw_indirect_count_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_indexed_indirect_count_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub get_memory_host_pointer_properties_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
            p_host_pointer: *const std::os::raw::c_void,
            p_memory_host_pointer_properties: *mut crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_buffer_marker_amd: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_stage: crate::vk10::PipelineStageFlags,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            marker: u32,
        ),
    >,
    pub get_calibrated_timestamps_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            timestamp_count: u32,
            p_timestamp_infos: *const crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT,
            p_timestamps: *mut u64,
            p_max_deviation: *mut u64,
        ) -> crate::vk10::Result,
    >,
    pub cmd_draw_mesh_tasks_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            task_count: u32,
            first_task: u32,
        ),
    >,
    pub cmd_draw_mesh_tasks_indirect_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_mesh_tasks_indirect_count_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_set_exclusive_scissor_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_exclusive_scissor: u32,
            exclusive_scissor_count: u32,
            p_exclusive_scissors: *const crate::vk10::Rect2D,
        ),
    >,
    pub cmd_set_checkpoint_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_checkpoint_marker: *const std::os::raw::c_void,
        ),
    >,
    pub get_queue_checkpoint_data_nv: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_checkpoint_data_count: *mut u32,
            p_checkpoint_data: *mut crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV,
        ),
    >,
    pub get_semaphore_counter_value_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            semaphore: crate::vk10::Semaphore,
            p_value: *mut u64,
        ) -> crate::vk10::Result,
    >,
    pub wait_semaphores_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_wait_info: *const crate::vk12::SemaphoreWaitInfo,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub signal_semaphore_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_signal_info: *const crate::vk12::SemaphoreSignalInfo,
        ) -> crate::vk10::Result,
    >,
    pub initialize_performance_api_intel: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_initialize_info: *const crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL,
        ) -> crate::vk10::Result,
    >,
    pub uninitialize_performance_api_intel: Option<
        unsafe extern "system" fn(device: crate::vk10::Device),
    >,
    pub cmd_set_performance_marker_intel: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_marker_info: *const crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_performance_stream_marker_intel: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_marker_info: *const crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_performance_override_intel: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_override_info: *const crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL,
        ) -> crate::vk10::Result,
    >,
    pub acquire_performance_configuration_intel: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_acquire_info: *const crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL,
            p_configuration: *mut crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
        ) -> crate::vk10::Result,
    >,
    pub release_performance_configuration_intel: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
        ) -> crate::vk10::Result,
    >,
    pub queue_set_performance_configuration_intel: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
        ) -> crate::vk10::Result,
    >,
    pub get_performance_parameter_intel: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL,
            p_value: *mut crate::extensions::intel_performance_query::PerformanceValueINTEL,
        ) -> crate::vk10::Result,
    >,
    pub set_local_dimming_amd: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
            local_dimming_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_fragment_shading_rate_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_fragment_size: *const crate::vk10::Extent2D,
            combiner_ops: *const [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
        ),
    >,
    pub get_buffer_device_address_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::BufferDeviceAddressInfo,
        ) -> crate::vk10::DeviceAddress,
    >,
    pub wait_for_present_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
            present_id: u64,
            timeout: u64,
        ) -> crate::vk10::Result,
    >,
    pub get_device_group_surface_present_modes_2_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
            p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
        ) -> crate::vk10::Result,
    >,
    pub acquire_full_screen_exclusive_mode_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub release_full_screen_exclusive_mode_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_buffer_opaque_capture_address_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::BufferDeviceAddressInfo,
        ) -> u64,
    >,
    pub get_buffer_device_address_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::BufferDeviceAddressInfo,
        ) -> crate::vk10::DeviceAddress,
    >,
    pub get_device_memory_opaque_capture_address_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
        ) -> u64,
    >,
    pub cmd_set_line_stipple_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            line_stipple_factor: u32,
            line_stipple_pattern: u16,
        ),
    >,
    pub reset_query_pool_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
            query_count: u32,
        ),
    >,
    pub cmd_set_cull_mode_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            cull_mode: crate::vk10::CullModeFlags,
        ),
    >,
    pub cmd_set_front_face_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            front_face: crate::vk10::FrontFace,
        ),
    >,
    pub cmd_set_primitive_topology_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            primitive_topology: crate::vk10::PrimitiveTopology,
        ),
    >,
    pub cmd_set_viewport_with_count_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            viewport_count: u32,
            p_viewports: *const crate::vk10::Viewport,
        ),
    >,
    pub cmd_set_scissor_with_count_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            scissor_count: u32,
            p_scissors: *const crate::vk10::Rect2D,
        ),
    >,
    pub cmd_bind_vertex_buffers_2_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_binding: u32,
            binding_count: u32,
            p_buffers: *const crate::vk10::Buffer,
            p_offsets: *const crate::vk10::DeviceSize,
            p_sizes: *const crate::vk10::DeviceSize,
            p_strides: *const crate::vk10::DeviceSize,
        ),
    >,
    pub cmd_set_depth_test_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_write_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_write_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_compare_op_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_compare_op: crate::vk10::CompareOp,
        ),
    >,
    pub cmd_set_depth_bounds_test_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bounds_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_stencil_test_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stencil_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_stencil_op_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            face_mask: crate::vk10::StencilFaceFlags,
            fail_op: crate::vk10::StencilOp,
            pass_op: crate::vk10::StencilOp,
            depth_fail_op: crate::vk10::StencilOp,
            compare_op: crate::vk10::CompareOp,
        ),
    >,
    pub create_deferred_operation_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_deferred_operation: *mut crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        ) -> crate::vk10::Result,
    >,
    pub destroy_deferred_operation_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_deferred_operation_max_concurrency_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        ) -> u32,
    >,
    pub get_deferred_operation_result_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        ) -> crate::vk10::Result,
    >,
    pub deferred_operation_join_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_pipeline_executable_properties_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_pipeline_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR,
            p_executable_count: *mut u32,
            p_properties: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_pipeline_executable_statistics_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_executable_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR,
            p_statistic_count: *mut u32,
            p_statistics: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR,
        ) -> crate::vk10::Result,
    >,
    pub get_pipeline_executable_internal_representations_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_executable_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR,
            p_internal_representation_count: *mut u32,
            p_internal_representations: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR,
        ) -> crate::vk10::Result,
    >,
    pub cmd_execute_generated_commands_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            is_preprocessed: crate::vk10::Bool32,
            p_generated_commands_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV,
        ),
    >,
    pub cmd_preprocess_generated_commands_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_generated_commands_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV,
        ),
    >,
    pub cmd_bind_pipeline_shader_group_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            pipeline_bind_point: crate::vk10::PipelineBindPoint,
            pipeline: crate::vk10::Pipeline,
            group_index: u32,
        ),
    >,
    pub get_generated_commands_memory_requirements_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub create_indirect_commands_layout_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_indirect_commands_layout: *mut crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
        ) -> crate::vk10::Result,
    >,
    pub destroy_indirect_commands_layout_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            indirect_commands_layout: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub create_private_data_slot_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk13::PrivateDataSlotCreateInfo,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_private_data_slot: *mut crate::vk13::PrivateDataSlot,
        ) -> crate::vk10::Result,
    >,
    pub destroy_private_data_slot_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            private_data_slot: crate::vk13::PrivateDataSlot,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub set_private_data_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            object_type: crate::vk10::ObjectType,
            object_handle: u64,
            private_data_slot: crate::vk13::PrivateDataSlot,
            data: u64,
        ) -> crate::vk10::Result,
    >,
    pub get_private_data_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            object_type: crate::vk10::ObjectType,
            object_handle: u64,
            private_data_slot: crate::vk13::PrivateDataSlot,
            p_data: *mut u64,
        ),
    >,
    pub cmd_encode_video_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_encode_info: *const crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR,
        ),
    >,
    pub export_metal_objects_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_metal_objects_info: *mut crate::extensions::ext_metal_objects::ExportMetalObjectsInfoEXT,
        ),
    >,
    pub cmd_set_event_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            p_dependency_info: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub cmd_reset_event_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event: crate::vk10::Event,
            stage_mask: crate::vk13::PipelineStageFlags2,
        ),
    >,
    pub cmd_wait_events_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            event_count: u32,
            p_events: *const crate::vk10::Event,
            p_dependency_infos: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub cmd_pipeline_barrier_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_dependency_info: *const crate::vk13::DependencyInfo,
        ),
    >,
    pub queue_submit_2_khr: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            submit_count: u32,
            p_submits: *const crate::vk13::SubmitInfo2,
            fence: crate::vk10::Fence,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_timestamp_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stage: crate::vk13::PipelineStageFlags2,
            query_pool: crate::vk10::QueryPool,
            query: u32,
        ),
    >,
    pub cmd_write_buffer_marker_2_amd: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stage: crate::vk13::PipelineStageFlags2,
            dst_buffer: crate::vk10::Buffer,
            dst_offset: crate::vk10::DeviceSize,
            marker: u32,
        ),
    >,
    pub get_queue_checkpoint_data_2_nv: Option<
        unsafe extern "system" fn(
            queue: crate::vk10::Queue,
            p_checkpoint_data_count: *mut u32,
            p_checkpoint_data: *mut crate::extensions::khr_synchronization2::CheckpointData2NV,
        ),
    >,
    pub cmd_set_fragment_shading_rate_enum_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
            combiner_ops: *const [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
        ),
    >,
    pub cmd_draw_mesh_tasks_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        ),
    >,
    pub cmd_draw_mesh_tasks_indirect_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_mesh_tasks_indirect_count_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            buffer: crate::vk10::Buffer,
            offset: crate::vk10::DeviceSize,
            count_buffer: crate::vk10::Buffer,
            count_buffer_offset: crate::vk10::DeviceSize,
            max_draw_count: u32,
            stride: u32,
        ),
    >,
    pub cmd_copy_buffer_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_buffer_info: *const crate::vk13::CopyBufferInfo2,
        ),
    >,
    pub cmd_copy_image_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_image_info: *const crate::vk13::CopyImageInfo2,
        ),
    >,
    pub cmd_blit_image_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_blit_image_info: *const crate::vk13::BlitImageInfo2,
        ),
    >,
    pub cmd_copy_buffer_to_image_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_buffer_to_image_info: *const crate::vk13::CopyBufferToImageInfo2,
        ),
    >,
    pub cmd_copy_image_to_buffer_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_copy_image_to_buffer_info: *const crate::vk13::CopyImageToBufferInfo2,
        ),
    >,
    pub cmd_resolve_image_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_resolve_image_info: *const crate::vk13::ResolveImageInfo2,
        ),
    >,
    pub get_image_subresource_layout_2_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            image: crate::vk10::Image,
            p_subresource: *const crate::extensions::ext_image_compression_control::ImageSubresource2EXT,
            p_layout: *mut crate::extensions::ext_image_compression_control::SubresourceLayout2EXT,
        ),
    >,
    pub get_device_fault_info_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_fault_counts: *mut crate::extensions::ext_device_fault::DeviceFaultCountsEXT,
            p_fault_info: *mut crate::extensions::ext_device_fault::DeviceFaultInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_vertex_input_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            vertex_binding_description_count: u32,
            p_vertex_binding_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT,
            vertex_attribute_description_count: u32,
            p_vertex_attribute_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT,
        ),
    >,
    pub get_memory_zircon_handle_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA,
            p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
        ) -> crate::vk10::Result,
    >,
    pub get_memory_zircon_handle_properties_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
            zircon_handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
            p_memory_zircon_handle_properties: *mut crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub get_semaphore_zircon_handle_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA,
            p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
        ) -> crate::vk10::Result,
    >,
    pub import_semaphore_zircon_handle_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_import_semaphore_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub create_buffer_collection_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_collection: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub set_buffer_collection_buffer_constraints_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
            p_buffer_constraints_info: *const crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub set_buffer_collection_image_constraints_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
            p_image_constraints_info: *const crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub destroy_buffer_collection_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub get_buffer_collection_properties_fuchsia: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
            p_properties: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA,
        ) -> crate::vk10::Result,
    >,
    pub get_device_subpass_shading_max_workgroup_size_huawei: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            renderpass: crate::vk10::RenderPass,
            p_max_workgroup_size: *mut crate::vk10::Extent2D,
        ) -> crate::vk10::Result,
    >,
    pub cmd_subpass_shading_huawei: Option<
        unsafe extern "system" fn(command_buffer: crate::vk10::CommandBuffer),
    >,
    pub cmd_bind_invocation_mask_huawei: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            image_view: crate::vk10::ImageView,
            image_layout: crate::vk10::ImageLayout,
        ),
    >,
    pub get_memory_remote_address_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_memory_get_remote_address_info: *const crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV,
            p_address: *mut crate::extensions::nv_external_memory_rdma::RemoteAddressNV,
        ) -> crate::vk10::Result,
    >,
    pub get_pipeline_properties_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_pipeline_info: *const crate::extensions::ext_pipeline_properties::PipelineInfoEXT,
            p_pipeline_properties: *mut crate::vk10::BaseOutStructure,
        ) -> crate::vk10::Result,
    >,
    pub cmd_set_patch_control_points_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            patch_control_points: u32,
        ),
    >,
    pub cmd_set_rasterizer_discard_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            rasterizer_discard_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_bias_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_bias_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_logic_op_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            logic_op: crate::vk10::LogicOp,
        ),
    >,
    pub cmd_set_primitive_restart_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            primitive_restart_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_color_write_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            attachment_count: u32,
            p_color_write_enables: *const crate::vk10::Bool32,
        ),
    >,
    pub cmd_trace_rays_indirect_2_khr: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            indirect_device_address: crate::vk10::DeviceAddress,
        ),
    >,
    pub cmd_draw_multi_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            draw_count: u32,
            p_vertex_info: *const crate::extensions::ext_multi_draw::MultiDrawInfoEXT,
            instance_count: u32,
            first_instance: u32,
            stride: u32,
        ),
    >,
    pub cmd_draw_multi_indexed_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            draw_count: u32,
            p_index_info: *const crate::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT,
            instance_count: u32,
            first_instance: u32,
            stride: u32,
            p_vertex_offset: *const i32,
        ),
    >,
    pub create_micromap_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::ext_opacity_micromap::MicromapCreateInfoEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_micromap: *mut crate::extensions::ext_opacity_micromap::MicromapEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_build_micromaps_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            info_count: u32,
            p_infos: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
        ),
    >,
    pub build_micromaps_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            info_count: u32,
            p_infos: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub destroy_micromap_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            micromap: crate::extensions::ext_opacity_micromap::MicromapEXT,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub cmd_copy_micromap_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT,
        ),
    >,
    pub copy_micromap_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_copy_micromap_to_memory_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT,
        ),
    >,
    pub copy_micromap_to_memory_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_copy_memory_to_micromap_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT,
        ),
    >,
    pub copy_memory_to_micromap_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
            p_info: *const crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT,
        ) -> crate::vk10::Result,
    >,
    pub cmd_write_micromaps_properties_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            micromap_count: u32,
            p_micromaps: *const crate::extensions::ext_opacity_micromap::MicromapEXT,
            query_type: crate::vk10::QueryType,
            query_pool: crate::vk10::QueryPool,
            first_query: u32,
        ),
    >,
    pub write_micromaps_properties_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            micromap_count: u32,
            p_micromaps: *const crate::extensions::ext_opacity_micromap::MicromapEXT,
            query_type: crate::vk10::QueryType,
            data_size: usize,
            p_data: *mut std::os::raw::c_void,
            stride: usize,
        ) -> crate::vk10::Result,
    >,
    pub get_device_micromap_compatibility_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_version_info: *const crate::extensions::ext_opacity_micromap::MicromapVersionInfoEXT,
            p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
        ),
    >,
    pub get_micromap_build_sizes_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
            p_build_info: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
            p_size_info: *mut crate::extensions::ext_opacity_micromap::MicromapBuildSizesInfoEXT,
        ),
    >,
    pub set_device_memory_priority_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            memory: crate::vk10::DeviceMemory,
            priority: std::os::raw::c_float,
        ),
    >,
    pub get_device_buffer_memory_requirements_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceBufferMemoryRequirements,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_device_image_memory_requirements_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceImageMemoryRequirements,
            p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
        ),
    >,
    pub get_device_image_sparse_memory_requirements_khr: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_info: *const crate::vk13::DeviceImageMemoryRequirements,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
        ),
    >,
    pub get_descriptor_set_layout_host_mapping_info_valve: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_binding_reference: *const crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetBindingReferenceVALVE,
            p_host_mapping: *mut crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetLayoutHostMappingInfoVALVE,
        ),
    >,
    pub get_descriptor_set_host_mapping_valve: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            descriptor_set: crate::vk10::DescriptorSet,
            pp_data: *mut *mut std::os::raw::c_void,
        ),
    >,
    pub cmd_set_tessellation_domain_origin_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            domain_origin: crate::vk11::TessellationDomainOrigin,
        ),
    >,
    pub cmd_set_depth_clamp_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_clamp_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_polygon_mode_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            polygon_mode: crate::vk10::PolygonMode,
        ),
    >,
    pub cmd_set_rasterization_samples_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            rasterization_samples: crate::vk10::SampleCountFlags,
        ),
    >,
    pub cmd_set_sample_mask_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            samples: crate::vk10::SampleCountFlags,
            p_sample_mask: *const crate::vk10::SampleMask,
        ),
    >,
    pub cmd_set_alpha_to_coverage_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            alpha_to_coverage_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_alpha_to_one_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            alpha_to_one_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_logic_op_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            logic_op_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_color_blend_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_attachment: u32,
            attachment_count: u32,
            p_color_blend_enables: *const crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_color_blend_equation_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_attachment: u32,
            attachment_count: u32,
            p_color_blend_equations: *const crate::extensions::ext_extended_dynamic_state3::ColorBlendEquationEXT,
        ),
    >,
    pub cmd_set_color_write_mask_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_attachment: u32,
            attachment_count: u32,
            p_color_write_masks: *const crate::vk10::ColorComponentFlags,
        ),
    >,
    pub cmd_set_rasterization_stream_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            rasterization_stream: u32,
        ),
    >,
    pub cmd_set_conservative_rasterization_mode_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            conservative_rasterization_mode: crate::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT,
        ),
    >,
    pub cmd_set_extra_primitive_overestimation_size_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            extra_primitive_overestimation_size: std::os::raw::c_float,
        ),
    >,
    pub cmd_set_depth_clip_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            depth_clip_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_sample_locations_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            sample_locations_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_color_blend_advanced_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_attachment: u32,
            attachment_count: u32,
            p_color_blend_advanced: *const crate::extensions::ext_extended_dynamic_state3::ColorBlendAdvancedEXT,
        ),
    >,
    pub cmd_set_provoking_vertex_mode_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
        ),
    >,
    pub cmd_set_line_rasterization_mode_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
        ),
    >,
    pub cmd_set_line_stipple_enable_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            stippled_line_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_depth_clip_negative_one_to_one_ext: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            negative_one_to_one: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_viewport_wscaling_enable_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            viewport_wscaling_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_viewport_swizzle_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
            p_viewport_swizzles: *const crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV,
        ),
    >,
    pub cmd_set_coverage_to_color_enable_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_to_color_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_coverage_to_color_location_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_to_color_location: u32,
        ),
    >,
    pub cmd_set_coverage_modulation_mode_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
        ),
    >,
    pub cmd_set_coverage_modulation_table_enable_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_modulation_table_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_coverage_modulation_table_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_modulation_table_count: u32,
            p_coverage_modulation_table: *const std::os::raw::c_float,
        ),
    >,
    pub cmd_set_shading_rate_image_enable_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            shading_rate_image_enable: crate::vk10::Bool32,
        ),
    >,
    pub cmd_set_coverage_reduction_mode_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
        ),
    >,
    pub cmd_set_representative_fragment_test_enable_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            representative_fragment_test_enable: crate::vk10::Bool32,
        ),
    >,
    pub get_shader_module_identifier_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            shader_module: crate::vk10::ShaderModule,
            p_identifier: *mut crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT,
        ),
    >,
    pub get_shader_module_create_info_identifier_ext: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
            p_identifier: *mut crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT,
        ),
    >,
    pub create_optical_flow_session_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_create_info: *const crate::extensions::nv_optical_flow::OpticalFlowSessionCreateInfoNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
            p_session: *mut crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
        ) -> crate::vk10::Result,
    >,
    pub destroy_optical_flow_session_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
            p_allocator: *const crate::vk10::AllocationCallbacks,
        ),
    >,
    pub bind_optical_flow_session_image_nv: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
            binding_point: crate::extensions::nv_optical_flow::OpticalFlowSessionBindingPointNV,
            view: crate::vk10::ImageView,
            layout: crate::vk10::ImageLayout,
        ) -> crate::vk10::Result,
    >,
    pub cmd_optical_flow_execute_nv: Option<
        unsafe extern "system" fn(
            command_buffer: crate::vk10::CommandBuffer,
            session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
            p_execute_info: *const crate::extensions::nv_optical_flow::OpticalFlowExecuteInfoNV,
        ),
    >,
    pub get_framebuffer_tile_properties_qcom: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            framebuffer: crate::vk10::Framebuffer,
            p_properties_count: *mut u32,
            p_properties: *mut crate::extensions::qcom_tile_properties::TilePropertiesQCOM,
        ) -> crate::vk10::Result,
    >,
    pub get_dynamic_rendering_tile_properties_qcom: Option<
        unsafe extern "system" fn(
            device: crate::vk10::Device,
            p_rendering_info: *const crate::vk13::RenderingInfo,
            p_properties: *mut crate::extensions::qcom_tile_properties::TilePropertiesQCOM,
        ) -> crate::vk10::Result,
    >,
}
impl DeviceTable {
    pub const fn new_empty() -> Self {
        unsafe {
            const SIZE: usize = std::mem::size_of::<DeviceTable>();
            ConstZeroedHack::<DeviceTable, SIZE>::zero()
        }
    }
    pub fn load(&mut self, loader: &impl DeviceLoad, conf: &ApiLoadConfig) {
        unsafe {
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 0, 0)) {
                load_fns! {
                    self, loader, (get_device_proc_addr, "vkGetDeviceProcAddr")
                    (destroy_device, "vkDestroyDevice") (get_device_queue,
                    "vkGetDeviceQueue") (queue_submit, "vkQueueSubmit") (queue_wait_idle,
                    "vkQueueWaitIdle") (device_wait_idle, "vkDeviceWaitIdle")
                    (allocate_memory, "vkAllocateMemory") (free_memory, "vkFreeMemory")
                    (map_memory, "vkMapMemory") (unmap_memory, "vkUnmapMemory")
                    (flush_mapped_memory_ranges, "vkFlushMappedMemoryRanges")
                    (invalidate_mapped_memory_ranges, "vkInvalidateMappedMemoryRanges")
                    (get_device_memory_commitment, "vkGetDeviceMemoryCommitment")
                    (get_buffer_memory_requirements, "vkGetBufferMemoryRequirements")
                    (bind_buffer_memory, "vkBindBufferMemory")
                    (get_image_memory_requirements, "vkGetImageMemoryRequirements")
                    (bind_image_memory, "vkBindImageMemory")
                    (get_image_sparse_memory_requirements,
                    "vkGetImageSparseMemoryRequirements") (queue_bind_sparse,
                    "vkQueueBindSparse") (create_fence, "vkCreateFence") (destroy_fence,
                    "vkDestroyFence") (reset_fences, "vkResetFences") (get_fence_status,
                    "vkGetFenceStatus") (wait_for_fences, "vkWaitForFences")
                    (create_semaphore, "vkCreateSemaphore") (destroy_semaphore,
                    "vkDestroySemaphore") (create_event, "vkCreateEvent") (destroy_event,
                    "vkDestroyEvent") (get_event_status, "vkGetEventStatus") (set_event,
                    "vkSetEvent") (reset_event, "vkResetEvent") (create_query_pool,
                    "vkCreateQueryPool") (destroy_query_pool, "vkDestroyQueryPool")
                    (get_query_pool_results, "vkGetQueryPoolResults") (create_buffer,
                    "vkCreateBuffer") (destroy_buffer, "vkDestroyBuffer")
                    (create_buffer_view, "vkCreateBufferView") (destroy_buffer_view,
                    "vkDestroyBufferView") (create_image, "vkCreateImage")
                    (destroy_image, "vkDestroyImage") (get_image_subresource_layout,
                    "vkGetImageSubresourceLayout") (create_image_view,
                    "vkCreateImageView") (destroy_image_view, "vkDestroyImageView")
                    (create_shader_module, "vkCreateShaderModule")
                    (destroy_shader_module, "vkDestroyShaderModule")
                    (create_pipeline_cache, "vkCreatePipelineCache")
                    (destroy_pipeline_cache, "vkDestroyPipelineCache")
                    (get_pipeline_cache_data, "vkGetPipelineCacheData")
                    (merge_pipeline_caches, "vkMergePipelineCaches")
                    (create_graphics_pipelines, "vkCreateGraphicsPipelines")
                    (create_compute_pipelines, "vkCreateComputePipelines")
                    (destroy_pipeline, "vkDestroyPipeline") (create_pipeline_layout,
                    "vkCreatePipelineLayout") (destroy_pipeline_layout,
                    "vkDestroyPipelineLayout") (create_sampler, "vkCreateSampler")
                    (destroy_sampler, "vkDestroySampler") (create_descriptor_set_layout,
                    "vkCreateDescriptorSetLayout") (destroy_descriptor_set_layout,
                    "vkDestroyDescriptorSetLayout") (create_descriptor_pool,
                    "vkCreateDescriptorPool") (destroy_descriptor_pool,
                    "vkDestroyDescriptorPool") (reset_descriptor_pool,
                    "vkResetDescriptorPool") (allocate_descriptor_sets,
                    "vkAllocateDescriptorSets") (free_descriptor_sets,
                    "vkFreeDescriptorSets") (update_descriptor_sets,
                    "vkUpdateDescriptorSets") (create_framebuffer, "vkCreateFramebuffer")
                    (destroy_framebuffer, "vkDestroyFramebuffer") (create_render_pass,
                    "vkCreateRenderPass") (destroy_render_pass, "vkDestroyRenderPass")
                    (get_render_area_granularity, "vkGetRenderAreaGranularity")
                    (create_command_pool, "vkCreateCommandPool") (destroy_command_pool,
                    "vkDestroyCommandPool") (reset_command_pool, "vkResetCommandPool")
                    (allocate_command_buffers, "vkAllocateCommandBuffers")
                    (free_command_buffers, "vkFreeCommandBuffers") (begin_command_buffer,
                    "vkBeginCommandBuffer") (end_command_buffer, "vkEndCommandBuffer")
                    (reset_command_buffer, "vkResetCommandBuffer") (cmd_bind_pipeline,
                    "vkCmdBindPipeline") (cmd_set_viewport, "vkCmdSetViewport")
                    (cmd_set_scissor, "vkCmdSetScissor") (cmd_set_line_width,
                    "vkCmdSetLineWidth") (cmd_set_depth_bias, "vkCmdSetDepthBias")
                    (cmd_set_blend_constants, "vkCmdSetBlendConstants")
                    (cmd_set_depth_bounds, "vkCmdSetDepthBounds")
                    (cmd_set_stencil_compare_mask, "vkCmdSetStencilCompareMask")
                    (cmd_set_stencil_write_mask, "vkCmdSetStencilWriteMask")
                    (cmd_set_stencil_reference, "vkCmdSetStencilReference")
                    (cmd_bind_descriptor_sets, "vkCmdBindDescriptorSets")
                    (cmd_bind_index_buffer, "vkCmdBindIndexBuffer")
                    (cmd_bind_vertex_buffers, "vkCmdBindVertexBuffers") (cmd_draw,
                    "vkCmdDraw") (cmd_draw_indexed, "vkCmdDrawIndexed")
                    (cmd_draw_indirect, "vkCmdDrawIndirect") (cmd_draw_indexed_indirect,
                    "vkCmdDrawIndexedIndirect") (cmd_dispatch, "vkCmdDispatch")
                    (cmd_dispatch_indirect, "vkCmdDispatchIndirect") (cmd_copy_buffer,
                    "vkCmdCopyBuffer") (cmd_copy_image, "vkCmdCopyImage")
                    (cmd_blit_image, "vkCmdBlitImage") (cmd_copy_buffer_to_image,
                    "vkCmdCopyBufferToImage") (cmd_copy_image_to_buffer,
                    "vkCmdCopyImageToBuffer") (cmd_update_buffer, "vkCmdUpdateBuffer")
                    (cmd_fill_buffer, "vkCmdFillBuffer") (cmd_clear_color_image,
                    "vkCmdClearColorImage") (cmd_clear_depth_stencil_image,
                    "vkCmdClearDepthStencilImage") (cmd_clear_attachments,
                    "vkCmdClearAttachments") (cmd_resolve_image, "vkCmdResolveImage")
                    (cmd_set_event, "vkCmdSetEvent") (cmd_reset_event, "vkCmdResetEvent")
                    (cmd_wait_events, "vkCmdWaitEvents") (cmd_pipeline_barrier,
                    "vkCmdPipelineBarrier") (cmd_begin_query, "vkCmdBeginQuery")
                    (cmd_end_query, "vkCmdEndQuery") (cmd_reset_query_pool,
                    "vkCmdResetQueryPool") (cmd_write_timestamp, "vkCmdWriteTimestamp")
                    (cmd_copy_query_pool_results, "vkCmdCopyQueryPoolResults")
                    (cmd_push_constants, "vkCmdPushConstants") (cmd_begin_render_pass,
                    "vkCmdBeginRenderPass") (cmd_next_subpass, "vkCmdNextSubpass")
                    (cmd_end_render_pass, "vkCmdEndRenderPass") (cmd_execute_commands,
                    "vkCmdExecuteCommands")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 1, 0)) {
                load_fns! {
                    self, loader, (trim_command_pool, "vkTrimCommandPool")
                    (get_device_group_peer_memory_features,
                    "vkGetDeviceGroupPeerMemoryFeatures") (bind_buffer_memory_2,
                    "vkBindBufferMemory2") (bind_image_memory_2, "vkBindImageMemory2")
                    (cmd_set_device_mask, "vkCmdSetDeviceMask") (cmd_dispatch_base,
                    "vkCmdDispatchBase") (create_descriptor_update_template,
                    "vkCreateDescriptorUpdateTemplate")
                    (destroy_descriptor_update_template,
                    "vkDestroyDescriptorUpdateTemplate")
                    (update_descriptor_set_with_template,
                    "vkUpdateDescriptorSetWithTemplate")
                    (get_buffer_memory_requirements_2, "vkGetBufferMemoryRequirements2")
                    (get_image_memory_requirements_2, "vkGetImageMemoryRequirements2")
                    (get_image_sparse_memory_requirements_2,
                    "vkGetImageSparseMemoryRequirements2")
                    (create_sampler_ycbcr_conversion, "vkCreateSamplerYcbcrConversion")
                    (destroy_sampler_ycbcr_conversion, "vkDestroySamplerYcbcrConversion")
                    (get_device_queue_2, "vkGetDeviceQueue2")
                    (get_descriptor_set_layout_support,
                    "vkGetDescriptorSetLayoutSupport")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 2, 0)) {
                load_fns! {
                    self, loader, (reset_query_pool, "vkResetQueryPool")
                    (create_render_pass_2, "vkCreateRenderPass2")
                    (cmd_begin_render_pass_2, "vkCmdBeginRenderPass2")
                    (cmd_next_subpass_2, "vkCmdNextSubpass2") (cmd_end_render_pass_2,
                    "vkCmdEndRenderPass2") (get_semaphore_counter_value,
                    "vkGetSemaphoreCounterValue") (wait_semaphores, "vkWaitSemaphores")
                    (signal_semaphore, "vkSignalSemaphore") (cmd_draw_indirect_count,
                    "vkCmdDrawIndirectCount") (cmd_draw_indexed_indirect_count,
                    "vkCmdDrawIndexedIndirectCount") (get_buffer_opaque_capture_address,
                    "vkGetBufferOpaqueCaptureAddress") (get_buffer_device_address,
                    "vkGetBufferDeviceAddress")
                    (get_device_memory_opaque_capture_address,
                    "vkGetDeviceMemoryOpaqueCaptureAddress")
                }
            }
            if conf.api_version_enabled(crate::vk10::make_api_version(0, 1, 3, 0)) {
                load_fns! {
                    self, loader, (get_device_buffer_memory_requirements,
                    "vkGetDeviceBufferMemoryRequirements")
                    (get_device_image_memory_requirements,
                    "vkGetDeviceImageMemoryRequirements")
                    (get_device_image_sparse_memory_requirements,
                    "vkGetDeviceImageSparseMemoryRequirements") (cmd_set_cull_mode,
                    "vkCmdSetCullMode") (cmd_set_front_face, "vkCmdSetFrontFace")
                    (cmd_set_primitive_topology, "vkCmdSetPrimitiveTopology")
                    (cmd_set_viewport_with_count, "vkCmdSetViewportWithCount")
                    (cmd_set_scissor_with_count, "vkCmdSetScissorWithCount")
                    (cmd_bind_vertex_buffers_2, "vkCmdBindVertexBuffers2")
                    (cmd_set_depth_test_enable, "vkCmdSetDepthTestEnable")
                    (cmd_set_depth_write_enable, "vkCmdSetDepthWriteEnable")
                    (cmd_set_depth_compare_op, "vkCmdSetDepthCompareOp")
                    (cmd_set_depth_bounds_test_enable, "vkCmdSetDepthBoundsTestEnable")
                    (cmd_set_stencil_test_enable, "vkCmdSetStencilTestEnable")
                    (cmd_set_stencil_op, "vkCmdSetStencilOp")
                    (cmd_set_rasterizer_discard_enable,
                    "vkCmdSetRasterizerDiscardEnable") (cmd_set_depth_bias_enable,
                    "vkCmdSetDepthBiasEnable") (cmd_set_primitive_restart_enable,
                    "vkCmdSetPrimitiveRestartEnable") (create_private_data_slot,
                    "vkCreatePrivateDataSlot") (destroy_private_data_slot,
                    "vkDestroyPrivateDataSlot") (set_private_data, "vkSetPrivateData")
                    (get_private_data, "vkGetPrivateData") (cmd_copy_buffer_2,
                    "vkCmdCopyBuffer2") (cmd_copy_image_2, "vkCmdCopyImage2")
                    (cmd_blit_image_2, "vkCmdBlitImage2") (cmd_copy_buffer_to_image_2,
                    "vkCmdCopyBufferToImage2") (cmd_copy_image_to_buffer_2,
                    "vkCmdCopyImageToBuffer2") (cmd_resolve_image_2,
                    "vkCmdResolveImage2") (cmd_set_event_2, "vkCmdSetEvent2")
                    (cmd_reset_event_2, "vkCmdResetEvent2") (cmd_wait_events_2,
                    "vkCmdWaitEvents2") (cmd_pipeline_barrier_2, "vkCmdPipelineBarrier2")
                    (queue_submit_2, "vkQueueSubmit2") (cmd_write_timestamp_2,
                    "vkCmdWriteTimestamp2") (cmd_begin_rendering, "vkCmdBeginRendering")
                    (cmd_end_rendering, "vkCmdEndRendering")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_swapchain")) {
                load_fns! {
                    self, loader, (create_swapchain_khr, "vkCreateSwapchainKHR")
                    (destroy_swapchain_khr, "vkDestroySwapchainKHR")
                    (get_swapchain_images_khr, "vkGetSwapchainImagesKHR")
                    (acquire_next_image_khr, "vkAcquireNextImageKHR") (queue_present_khr,
                    "vkQueuePresentKHR") (get_device_group_present_capabilities_khr,
                    "vkGetDeviceGroupPresentCapabilitiesKHR")
                    (get_device_group_surface_present_modes_khr,
                    "vkGetDeviceGroupSurfacePresentModesKHR") (acquire_next_image_2_khr,
                    "vkAcquireNextImage2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_display_swapchain")) {
                load_fns! {
                    self, loader, (create_shared_swapchains_khr,
                    "vkCreateSharedSwapchainsKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_marker")) {
                load_fns! {
                    self, loader, (debug_marker_set_object_name_ext,
                    "vkDebugMarkerSetObjectNameEXT") (debug_marker_set_object_tag_ext,
                    "vkDebugMarkerSetObjectTagEXT") (cmd_debug_marker_begin_ext,
                    "vkCmdDebugMarkerBeginEXT") (cmd_debug_marker_end_ext,
                    "vkCmdDebugMarkerEndEXT") (cmd_debug_marker_insert_ext,
                    "vkCmdDebugMarkerInsertEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_video_queue")) {
                load_fns! {
                    self, loader, (create_video_session_khr, "vkCreateVideoSessionKHR")
                    (destroy_video_session_khr, "vkDestroyVideoSessionKHR")
                    (create_video_session_parameters_khr,
                    "vkCreateVideoSessionParametersKHR")
                    (update_video_session_parameters_khr,
                    "vkUpdateVideoSessionParametersKHR")
                    (destroy_video_session_parameters_khr,
                    "vkDestroyVideoSessionParametersKHR")
                    (get_video_session_memory_requirements_khr,
                    "vkGetVideoSessionMemoryRequirementsKHR")
                    (bind_video_session_memory_khr, "vkBindVideoSessionMemoryKHR")
                    (cmd_begin_video_coding_khr, "vkCmdBeginVideoCodingKHR")
                    (cmd_control_video_coding_khr, "vkCmdControlVideoCodingKHR")
                    (cmd_end_video_coding_khr, "vkCmdEndVideoCodingKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_video_decode_queue")) {
                load_fns! {
                    self, loader, (cmd_decode_video_khr, "vkCmdDecodeVideoKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_transform_feedback")) {
                load_fns! {
                    self, loader, (cmd_bind_transform_feedback_buffers_ext,
                    "vkCmdBindTransformFeedbackBuffersEXT")
                    (cmd_begin_transform_feedback_ext, "vkCmdBeginTransformFeedbackEXT")
                    (cmd_end_transform_feedback_ext, "vkCmdEndTransformFeedbackEXT")
                    (cmd_begin_query_indexed_ext, "vkCmdBeginQueryIndexedEXT")
                    (cmd_end_query_indexed_ext, "vkCmdEndQueryIndexedEXT")
                    (cmd_draw_indirect_byte_count_ext, "vkCmdDrawIndirectByteCountEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NVX_binary_import")) {
                load_fns! {
                    self, loader, (create_cu_module_nvx, "vkCreateCuModuleNVX")
                    (create_cu_function_nvx, "vkCreateCuFunctionNVX")
                    (destroy_cu_module_nvx, "vkDestroyCuModuleNVX")
                    (destroy_cu_function_nvx, "vkDestroyCuFunctionNVX")
                    (cmd_cu_launch_kernel_nvx, "vkCmdCuLaunchKernelNVX")
                }
            }
            if conf.extension_enabled(cstr!("VK_NVX_image_view_handle")) {
                load_fns! {
                    self, loader, (get_image_view_handle_nvx, "vkGetImageViewHandleNVX")
                    (get_image_view_address_nvx, "vkGetImageViewAddressNVX")
                }
            }
            if conf.extension_enabled(cstr!("VK_AMD_draw_indirect_count")) {
                load_fns! {
                    self, loader, (cmd_draw_indirect_count_amd,
                    "vkCmdDrawIndirectCountAMD") (cmd_draw_indexed_indirect_count_amd,
                    "vkCmdDrawIndexedIndirectCountAMD")
                }
            }
            if conf.extension_enabled(cstr!("VK_AMD_shader_info")) {
                load_fns! {
                    self, loader, (get_shader_info_amd, "vkGetShaderInfoAMD")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_dynamic_rendering")) {
                load_fns! {
                    self, loader, (cmd_begin_rendering_khr, "vkCmdBeginRenderingKHR")
                    (cmd_end_rendering_khr, "vkCmdEndRenderingKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_external_memory_win32")) {
                load_fns! {
                    self, loader, (get_memory_win_32_handle_nv,
                    "vkGetMemoryWin32HandleNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_device_group")) {
                load_fns! {
                    self, loader, (get_device_group_peer_memory_features_khr,
                    "vkGetDeviceGroupPeerMemoryFeaturesKHR") (cmd_set_device_mask_khr,
                    "vkCmdSetDeviceMaskKHR") (cmd_dispatch_base_khr,
                    "vkCmdDispatchBaseKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_maintenance1")) {
                load_fns! {
                    self, loader, (trim_command_pool_khr, "vkTrimCommandPoolKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_memory_win32")) {
                load_fns! {
                    self, loader, (get_memory_win_32_handle_khr,
                    "vkGetMemoryWin32HandleKHR")
                    (get_memory_win_32_handle_properties_khr,
                    "vkGetMemoryWin32HandlePropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_memory_fd")) {
                load_fns! {
                    self, loader, (get_memory_fd_khr, "vkGetMemoryFdKHR")
                    (get_memory_fd_properties_khr, "vkGetMemoryFdPropertiesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_semaphore_win32")) {
                load_fns! {
                    self, loader, (get_semaphore_win_32_handle_khr,
                    "vkGetSemaphoreWin32HandleKHR") (import_semaphore_win_32_handle_khr,
                    "vkImportSemaphoreWin32HandleKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_semaphore_fd")) {
                load_fns! {
                    self, loader, (get_semaphore_fd_khr, "vkGetSemaphoreFdKHR")
                    (import_semaphore_fd_khr, "vkImportSemaphoreFdKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_push_descriptor")) {
                load_fns! {
                    self, loader, (cmd_push_descriptor_set_khr,
                    "vkCmdPushDescriptorSetKHR")
                    (cmd_push_descriptor_set_with_template_khr,
                    "vkCmdPushDescriptorSetWithTemplateKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_conditional_rendering")) {
                load_fns! {
                    self, loader, (cmd_begin_conditional_rendering_ext,
                    "vkCmdBeginConditionalRenderingEXT")
                    (cmd_end_conditional_rendering_ext,
                    "vkCmdEndConditionalRenderingEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_descriptor_update_template")) {
                load_fns! {
                    self, loader, (create_descriptor_update_template_khr,
                    "vkCreateDescriptorUpdateTemplateKHR")
                    (destroy_descriptor_update_template_khr,
                    "vkDestroyDescriptorUpdateTemplateKHR")
                    (update_descriptor_set_with_template_khr,
                    "vkUpdateDescriptorSetWithTemplateKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_clip_space_w_scaling")) {
                load_fns! {
                    self, loader, (cmd_set_viewport_wscaling_nv,
                    "vkCmdSetViewportWScalingNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_display_control")) {
                load_fns! {
                    self, loader, (display_power_control_ext, "vkDisplayPowerControlEXT")
                    (register_device_event_ext, "vkRegisterDeviceEventEXT")
                    (register_display_event_ext, "vkRegisterDisplayEventEXT")
                    (get_swapchain_counter_ext, "vkGetSwapchainCounterEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_GOOGLE_display_timing")) {
                load_fns! {
                    self, loader, (get_refresh_cycle_duration_google,
                    "vkGetRefreshCycleDurationGOOGLE")
                    (get_past_presentation_timing_google,
                    "vkGetPastPresentationTimingGOOGLE")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_discard_rectangles")) {
                load_fns! {
                    self, loader, (cmd_set_discard_rectangle_ext,
                    "vkCmdSetDiscardRectangleEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_hdr_metadata")) {
                load_fns! {
                    self, loader, (set_hdr_metadata_ext, "vkSetHdrMetadataEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_create_renderpass2")) {
                load_fns! {
                    self, loader, (create_render_pass_2_khr, "vkCreateRenderPass2KHR")
                    (cmd_begin_render_pass_2_khr, "vkCmdBeginRenderPass2KHR")
                    (cmd_next_subpass_2_khr, "vkCmdNextSubpass2KHR")
                    (cmd_end_render_pass_2_khr, "vkCmdEndRenderPass2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_shared_presentable_image")) {
                load_fns! {
                    self, loader, (get_swapchain_status_khr, "vkGetSwapchainStatusKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_fence_win32")) {
                load_fns! {
                    self, loader, (get_fence_win_32_handle_khr,
                    "vkGetFenceWin32HandleKHR") (import_fence_win_32_handle_khr,
                    "vkImportFenceWin32HandleKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_external_fence_fd")) {
                load_fns! {
                    self, loader, (get_fence_fd_khr, "vkGetFenceFdKHR")
                    (import_fence_fd_khr, "vkImportFenceFdKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_performance_query")) {
                load_fns! {
                    self, loader, (acquire_profiling_lock_khr,
                    "vkAcquireProfilingLockKHR") (release_profiling_lock_khr,
                    "vkReleaseProfilingLockKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_debug_utils")) {
                load_fns! {
                    self, loader, (set_debug_utils_object_name_ext,
                    "vkSetDebugUtilsObjectNameEXT") (set_debug_utils_object_tag_ext,
                    "vkSetDebugUtilsObjectTagEXT") (queue_begin_debug_utils_label_ext,
                    "vkQueueBeginDebugUtilsLabelEXT") (queue_end_debug_utils_label_ext,
                    "vkQueueEndDebugUtilsLabelEXT") (queue_insert_debug_utils_label_ext,
                    "vkQueueInsertDebugUtilsLabelEXT") (cmd_begin_debug_utils_label_ext,
                    "vkCmdBeginDebugUtilsLabelEXT") (cmd_end_debug_utils_label_ext,
                    "vkCmdEndDebugUtilsLabelEXT") (cmd_insert_debug_utils_label_ext,
                    "vkCmdInsertDebugUtilsLabelEXT")
                }
            }
            if conf
                .extension_enabled(
                    cstr!("VK_ANDROID_external_memory_android_hardware_buffer"),
                )
            {
                load_fns! {
                    self, loader, (get_android_hardware_buffer_properties_android,
                    "vkGetAndroidHardwareBufferPropertiesANDROID")
                    (get_memory_android_hardware_buffer_android,
                    "vkGetMemoryAndroidHardwareBufferANDROID")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_sample_locations")) {
                load_fns! {
                    self, loader, (cmd_set_sample_locations_ext,
                    "vkCmdSetSampleLocationsEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_get_memory_requirements2")) {
                load_fns! {
                    self, loader, (get_buffer_memory_requirements_2_khr,
                    "vkGetBufferMemoryRequirements2KHR")
                    (get_image_memory_requirements_2_khr,
                    "vkGetImageMemoryRequirements2KHR")
                    (get_image_sparse_memory_requirements_2_khr,
                    "vkGetImageSparseMemoryRequirements2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_acceleration_structure")) {
                load_fns! {
                    self, loader, (destroy_acceleration_structure_khr,
                    "vkDestroyAccelerationStructureKHR")
                    (cmd_copy_acceleration_structure_khr,
                    "vkCmdCopyAccelerationStructureKHR")
                    (copy_acceleration_structure_khr, "vkCopyAccelerationStructureKHR")
                    (cmd_copy_acceleration_structure_to_memory_khr,
                    "vkCmdCopyAccelerationStructureToMemoryKHR")
                    (copy_acceleration_structure_to_memory_khr,
                    "vkCopyAccelerationStructureToMemoryKHR")
                    (cmd_copy_memory_to_acceleration_structure_khr,
                    "vkCmdCopyMemoryToAccelerationStructureKHR")
                    (copy_memory_to_acceleration_structure_khr,
                    "vkCopyMemoryToAccelerationStructureKHR")
                    (cmd_write_acceleration_structures_properties_khr,
                    "vkCmdWriteAccelerationStructuresPropertiesKHR")
                    (write_acceleration_structures_properties_khr,
                    "vkWriteAccelerationStructuresPropertiesKHR")
                    (get_device_acceleration_structure_compatibility_khr,
                    "vkGetDeviceAccelerationStructureCompatibilityKHR")
                    (create_acceleration_structure_khr,
                    "vkCreateAccelerationStructureKHR")
                    (cmd_build_acceleration_structures_khr,
                    "vkCmdBuildAccelerationStructuresKHR")
                    (cmd_build_acceleration_structures_indirect_khr,
                    "vkCmdBuildAccelerationStructuresIndirectKHR")
                    (build_acceleration_structures_khr,
                    "vkBuildAccelerationStructuresKHR")
                    (get_acceleration_structure_device_address_khr,
                    "vkGetAccelerationStructureDeviceAddressKHR")
                    (get_acceleration_structure_build_sizes_khr,
                    "vkGetAccelerationStructureBuildSizesKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_ray_tracing_pipeline")) {
                load_fns! {
                    self, loader, (cmd_trace_rays_khr, "vkCmdTraceRaysKHR")
                    (get_ray_tracing_shader_group_handles_khr,
                    "vkGetRayTracingShaderGroupHandlesKHR")
                    (get_ray_tracing_capture_replay_shader_group_handles_khr,
                    "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")
                    (create_ray_tracing_pipelines_khr, "vkCreateRayTracingPipelinesKHR")
                    (cmd_trace_rays_indirect_khr, "vkCmdTraceRaysIndirectKHR")
                    (get_ray_tracing_shader_group_stack_size_khr,
                    "vkGetRayTracingShaderGroupStackSizeKHR")
                    (cmd_set_ray_tracing_pipeline_stack_size_khr,
                    "vkCmdSetRayTracingPipelineStackSizeKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_sampler_ycbcr_conversion")) {
                load_fns! {
                    self, loader, (create_sampler_ycbcr_conversion_khr,
                    "vkCreateSamplerYcbcrConversionKHR")
                    (destroy_sampler_ycbcr_conversion_khr,
                    "vkDestroySamplerYcbcrConversionKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_bind_memory2")) {
                load_fns! {
                    self, loader, (bind_buffer_memory_2_khr, "vkBindBufferMemory2KHR")
                    (bind_image_memory_2_khr, "vkBindImageMemory2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_image_drm_format_modifier")) {
                load_fns! {
                    self, loader, (get_image_drm_format_modifier_properties_ext,
                    "vkGetImageDrmFormatModifierPropertiesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_validation_cache")) {
                load_fns! {
                    self, loader, (create_validation_cache_ext,
                    "vkCreateValidationCacheEXT") (destroy_validation_cache_ext,
                    "vkDestroyValidationCacheEXT") (get_validation_cache_data_ext,
                    "vkGetValidationCacheDataEXT") (merge_validation_caches_ext,
                    "vkMergeValidationCachesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_shading_rate_image")) {
                load_fns! {
                    self, loader, (cmd_bind_shading_rate_image_nv,
                    "vkCmdBindShadingRateImageNV")
                    (cmd_set_viewport_shading_rate_palette_nv,
                    "vkCmdSetViewportShadingRatePaletteNV")
                    (cmd_set_coarse_sample_order_nv, "vkCmdSetCoarseSampleOrderNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_ray_tracing")) {
                load_fns! {
                    self, loader, (compile_deferred_nv, "vkCompileDeferredNV")
                    (create_acceleration_structure_nv, "vkCreateAccelerationStructureNV")
                    (destroy_acceleration_structure_nv,
                    "vkDestroyAccelerationStructureNV")
                    (get_acceleration_structure_memory_requirements_nv,
                    "vkGetAccelerationStructureMemoryRequirementsNV")
                    (bind_acceleration_structure_memory_nv,
                    "vkBindAccelerationStructureMemoryNV")
                    (cmd_copy_acceleration_structure_nv,
                    "vkCmdCopyAccelerationStructureNV")
                    (cmd_write_acceleration_structures_properties_nv,
                    "vkCmdWriteAccelerationStructuresPropertiesNV")
                    (cmd_build_acceleration_structure_nv,
                    "vkCmdBuildAccelerationStructureNV") (cmd_trace_rays_nv,
                    "vkCmdTraceRaysNV") (get_ray_tracing_shader_group_handles_nv,
                    "vkGetRayTracingShaderGroupHandlesNV")
                    (get_acceleration_structure_handle_nv,
                    "vkGetAccelerationStructureHandleNV")
                    (create_ray_tracing_pipelines_nv, "vkCreateRayTracingPipelinesNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_maintenance3")) {
                load_fns! {
                    self, loader, (get_descriptor_set_layout_support_khr,
                    "vkGetDescriptorSetLayoutSupportKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_draw_indirect_count")) {
                load_fns! {
                    self, loader, (cmd_draw_indirect_count_khr,
                    "vkCmdDrawIndirectCountKHR") (cmd_draw_indexed_indirect_count_khr,
                    "vkCmdDrawIndexedIndirectCountKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_external_memory_host")) {
                load_fns! {
                    self, loader, (get_memory_host_pointer_properties_ext,
                    "vkGetMemoryHostPointerPropertiesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_AMD_buffer_marker")) {
                load_fns! {
                    self, loader, (cmd_write_buffer_marker_amd,
                    "vkCmdWriteBufferMarkerAMD")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_calibrated_timestamps")) {
                load_fns! {
                    self, loader, (get_calibrated_timestamps_ext,
                    "vkGetCalibratedTimestampsEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_mesh_shader")) {
                load_fns! {
                    self, loader, (cmd_draw_mesh_tasks_nv, "vkCmdDrawMeshTasksNV")
                    (cmd_draw_mesh_tasks_indirect_nv, "vkCmdDrawMeshTasksIndirectNV")
                    (cmd_draw_mesh_tasks_indirect_count_nv,
                    "vkCmdDrawMeshTasksIndirectCountNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_scissor_exclusive")) {
                load_fns! {
                    self, loader, (cmd_set_exclusive_scissor_nv,
                    "vkCmdSetExclusiveScissorNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_device_diagnostic_checkpoints")) {
                load_fns! {
                    self, loader, (cmd_set_checkpoint_nv, "vkCmdSetCheckpointNV")
                    (get_queue_checkpoint_data_nv, "vkGetQueueCheckpointDataNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_timeline_semaphore")) {
                load_fns! {
                    self, loader, (get_semaphore_counter_value_khr,
                    "vkGetSemaphoreCounterValueKHR") (wait_semaphores_khr,
                    "vkWaitSemaphoresKHR") (signal_semaphore_khr, "vkSignalSemaphoreKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_INTEL_performance_query")) {
                load_fns! {
                    self, loader, (initialize_performance_api_intel,
                    "vkInitializePerformanceApiINTEL")
                    (uninitialize_performance_api_intel,
                    "vkUninitializePerformanceApiINTEL")
                    (cmd_set_performance_marker_intel, "vkCmdSetPerformanceMarkerINTEL")
                    (cmd_set_performance_stream_marker_intel,
                    "vkCmdSetPerformanceStreamMarkerINTEL")
                    (cmd_set_performance_override_intel,
                    "vkCmdSetPerformanceOverrideINTEL")
                    (acquire_performance_configuration_intel,
                    "vkAcquirePerformanceConfigurationINTEL")
                    (release_performance_configuration_intel,
                    "vkReleasePerformanceConfigurationINTEL")
                    (queue_set_performance_configuration_intel,
                    "vkQueueSetPerformanceConfigurationINTEL")
                    (get_performance_parameter_intel, "vkGetPerformanceParameterINTEL")
                }
            }
            if conf.extension_enabled(cstr!("VK_AMD_display_native_hdr")) {
                load_fns! {
                    self, loader, (set_local_dimming_amd, "vkSetLocalDimmingAMD")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_fragment_shading_rate")) {
                load_fns! {
                    self, loader, (cmd_set_fragment_shading_rate_khr,
                    "vkCmdSetFragmentShadingRateKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_buffer_device_address")) {
                load_fns! {
                    self, loader, (get_buffer_device_address_ext,
                    "vkGetBufferDeviceAddressEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_present_wait")) {
                load_fns! {
                    self, loader, (wait_for_present_khr, "vkWaitForPresentKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_full_screen_exclusive")) {
                load_fns! {
                    self, loader, (get_device_group_surface_present_modes_2_ext,
                    "vkGetDeviceGroupSurfacePresentModes2EXT")
                    (acquire_full_screen_exclusive_mode_ext,
                    "vkAcquireFullScreenExclusiveModeEXT")
                    (release_full_screen_exclusive_mode_ext,
                    "vkReleaseFullScreenExclusiveModeEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_buffer_device_address")) {
                load_fns! {
                    self, loader, (get_buffer_opaque_capture_address_khr,
                    "vkGetBufferOpaqueCaptureAddressKHR") (get_buffer_device_address_khr,
                    "vkGetBufferDeviceAddressKHR")
                    (get_device_memory_opaque_capture_address_khr,
                    "vkGetDeviceMemoryOpaqueCaptureAddressKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_line_rasterization")) {
                load_fns! {
                    self, loader, (cmd_set_line_stipple_ext, "vkCmdSetLineStippleEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_host_query_reset")) {
                load_fns! {
                    self, loader, (reset_query_pool_ext, "vkResetQueryPoolEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_extended_dynamic_state")) {
                load_fns! {
                    self, loader, (cmd_set_cull_mode_ext, "vkCmdSetCullModeEXT")
                    (cmd_set_front_face_ext, "vkCmdSetFrontFaceEXT")
                    (cmd_set_primitive_topology_ext, "vkCmdSetPrimitiveTopologyEXT")
                    (cmd_set_viewport_with_count_ext, "vkCmdSetViewportWithCountEXT")
                    (cmd_set_scissor_with_count_ext, "vkCmdSetScissorWithCountEXT")
                    (cmd_bind_vertex_buffers_2_ext, "vkCmdBindVertexBuffers2EXT")
                    (cmd_set_depth_test_enable_ext, "vkCmdSetDepthTestEnableEXT")
                    (cmd_set_depth_write_enable_ext, "vkCmdSetDepthWriteEnableEXT")
                    (cmd_set_depth_compare_op_ext, "vkCmdSetDepthCompareOpEXT")
                    (cmd_set_depth_bounds_test_enable_ext,
                    "vkCmdSetDepthBoundsTestEnableEXT") (cmd_set_stencil_test_enable_ext,
                    "vkCmdSetStencilTestEnableEXT") (cmd_set_stencil_op_ext,
                    "vkCmdSetStencilOpEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_deferred_host_operations")) {
                load_fns! {
                    self, loader, (create_deferred_operation_khr,
                    "vkCreateDeferredOperationKHR") (destroy_deferred_operation_khr,
                    "vkDestroyDeferredOperationKHR")
                    (get_deferred_operation_max_concurrency_khr,
                    "vkGetDeferredOperationMaxConcurrencyKHR")
                    (get_deferred_operation_result_khr,
                    "vkGetDeferredOperationResultKHR") (deferred_operation_join_khr,
                    "vkDeferredOperationJoinKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_pipeline_executable_properties")) {
                load_fns! {
                    self, loader, (get_pipeline_executable_properties_khr,
                    "vkGetPipelineExecutablePropertiesKHR")
                    (get_pipeline_executable_statistics_khr,
                    "vkGetPipelineExecutableStatisticsKHR")
                    (get_pipeline_executable_internal_representations_khr,
                    "vkGetPipelineExecutableInternalRepresentationsKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_device_generated_commands")) {
                load_fns! {
                    self, loader, (cmd_execute_generated_commands_nv,
                    "vkCmdExecuteGeneratedCommandsNV")
                    (cmd_preprocess_generated_commands_nv,
                    "vkCmdPreprocessGeneratedCommandsNV")
                    (cmd_bind_pipeline_shader_group_nv, "vkCmdBindPipelineShaderGroupNV")
                    (get_generated_commands_memory_requirements_nv,
                    "vkGetGeneratedCommandsMemoryRequirementsNV")
                    (create_indirect_commands_layout_nv,
                    "vkCreateIndirectCommandsLayoutNV")
                    (destroy_indirect_commands_layout_nv,
                    "vkDestroyIndirectCommandsLayoutNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_private_data")) {
                load_fns! {
                    self, loader, (create_private_data_slot_ext,
                    "vkCreatePrivateDataSlotEXT") (destroy_private_data_slot_ext,
                    "vkDestroyPrivateDataSlotEXT") (set_private_data_ext,
                    "vkSetPrivateDataEXT") (get_private_data_ext, "vkGetPrivateDataEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_video_encode_queue")) {
                load_fns! {
                    self, loader, (cmd_encode_video_khr, "vkCmdEncodeVideoKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_metal_objects")) {
                load_fns! {
                    self, loader, (export_metal_objects_ext, "vkExportMetalObjectsEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_synchronization2")) {
                load_fns! {
                    self, loader, (cmd_set_event_2_khr, "vkCmdSetEvent2KHR")
                    (cmd_reset_event_2_khr, "vkCmdResetEvent2KHR")
                    (cmd_wait_events_2_khr, "vkCmdWaitEvents2KHR")
                    (cmd_pipeline_barrier_2_khr, "vkCmdPipelineBarrier2KHR")
                    (queue_submit_2_khr, "vkQueueSubmit2KHR") (cmd_write_timestamp_2_khr,
                    "vkCmdWriteTimestamp2KHR") (cmd_write_buffer_marker_2_amd,
                    "vkCmdWriteBufferMarker2AMD") (get_queue_checkpoint_data_2_nv,
                    "vkGetQueueCheckpointData2NV")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_fragment_shading_rate_enums")) {
                load_fns! {
                    self, loader, (cmd_set_fragment_shading_rate_enum_nv,
                    "vkCmdSetFragmentShadingRateEnumNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_mesh_shader")) {
                load_fns! {
                    self, loader, (cmd_draw_mesh_tasks_ext, "vkCmdDrawMeshTasksEXT")
                    (cmd_draw_mesh_tasks_indirect_ext, "vkCmdDrawMeshTasksIndirectEXT")
                    (cmd_draw_mesh_tasks_indirect_count_ext,
                    "vkCmdDrawMeshTasksIndirectCountEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_copy_commands2")) {
                load_fns! {
                    self, loader, (cmd_copy_buffer_2_khr, "vkCmdCopyBuffer2KHR")
                    (cmd_copy_image_2_khr, "vkCmdCopyImage2KHR") (cmd_blit_image_2_khr,
                    "vkCmdBlitImage2KHR") (cmd_copy_buffer_to_image_2_khr,
                    "vkCmdCopyBufferToImage2KHR") (cmd_copy_image_to_buffer_2_khr,
                    "vkCmdCopyImageToBuffer2KHR") (cmd_resolve_image_2_khr,
                    "vkCmdResolveImage2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_image_compression_control")) {
                load_fns! {
                    self, loader, (get_image_subresource_layout_2_ext,
                    "vkGetImageSubresourceLayout2EXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_device_fault")) {
                load_fns! {
                    self, loader, (get_device_fault_info_ext, "vkGetDeviceFaultInfoEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_vertex_input_dynamic_state")) {
                load_fns! {
                    self, loader, (cmd_set_vertex_input_ext, "vkCmdSetVertexInputEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_FUCHSIA_external_memory")) {
                load_fns! {
                    self, loader, (get_memory_zircon_handle_fuchsia,
                    "vkGetMemoryZirconHandleFUCHSIA")
                    (get_memory_zircon_handle_properties_fuchsia,
                    "vkGetMemoryZirconHandlePropertiesFUCHSIA")
                }
            }
            if conf.extension_enabled(cstr!("VK_FUCHSIA_external_semaphore")) {
                load_fns! {
                    self, loader, (get_semaphore_zircon_handle_fuchsia,
                    "vkGetSemaphoreZirconHandleFUCHSIA")
                    (import_semaphore_zircon_handle_fuchsia,
                    "vkImportSemaphoreZirconHandleFUCHSIA")
                }
            }
            if conf.extension_enabled(cstr!("VK_FUCHSIA_buffer_collection")) {
                load_fns! {
                    self, loader, (create_buffer_collection_fuchsia,
                    "vkCreateBufferCollectionFUCHSIA")
                    (set_buffer_collection_buffer_constraints_fuchsia,
                    "vkSetBufferCollectionBufferConstraintsFUCHSIA")
                    (set_buffer_collection_image_constraints_fuchsia,
                    "vkSetBufferCollectionImageConstraintsFUCHSIA")
                    (destroy_buffer_collection_fuchsia,
                    "vkDestroyBufferCollectionFUCHSIA")
                    (get_buffer_collection_properties_fuchsia,
                    "vkGetBufferCollectionPropertiesFUCHSIA")
                }
            }
            if conf.extension_enabled(cstr!("VK_HUAWEI_subpass_shading")) {
                load_fns! {
                    self, loader, (get_device_subpass_shading_max_workgroup_size_huawei,
                    "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")
                    (cmd_subpass_shading_huawei, "vkCmdSubpassShadingHUAWEI")
                }
            }
            if conf.extension_enabled(cstr!("VK_HUAWEI_invocation_mask")) {
                load_fns! {
                    self, loader, (cmd_bind_invocation_mask_huawei,
                    "vkCmdBindInvocationMaskHUAWEI")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_external_memory_rdma")) {
                load_fns! {
                    self, loader, (get_memory_remote_address_nv,
                    "vkGetMemoryRemoteAddressNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_pipeline_properties")) {
                load_fns! {
                    self, loader, (get_pipeline_properties_ext,
                    "vkGetPipelinePropertiesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_extended_dynamic_state2")) {
                load_fns! {
                    self, loader, (cmd_set_patch_control_points_ext,
                    "vkCmdSetPatchControlPointsEXT")
                    (cmd_set_rasterizer_discard_enable_ext,
                    "vkCmdSetRasterizerDiscardEnableEXT") (cmd_set_depth_bias_enable_ext,
                    "vkCmdSetDepthBiasEnableEXT") (cmd_set_logic_op_ext,
                    "vkCmdSetLogicOpEXT") (cmd_set_primitive_restart_enable_ext,
                    "vkCmdSetPrimitiveRestartEnableEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_color_write_enable")) {
                load_fns! {
                    self, loader, (cmd_set_color_write_enable_ext,
                    "vkCmdSetColorWriteEnableEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_ray_tracing_maintenance1")) {
                load_fns! {
                    self, loader, (cmd_trace_rays_indirect_2_khr,
                    "vkCmdTraceRaysIndirect2KHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_multi_draw")) {
                load_fns! {
                    self, loader, (cmd_draw_multi_ext, "vkCmdDrawMultiEXT")
                    (cmd_draw_multi_indexed_ext, "vkCmdDrawMultiIndexedEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_opacity_micromap")) {
                load_fns! {
                    self, loader, (create_micromap_ext, "vkCreateMicromapEXT")
                    (cmd_build_micromaps_ext, "vkCmdBuildMicromapsEXT")
                    (build_micromaps_ext, "vkBuildMicromapsEXT") (destroy_micromap_ext,
                    "vkDestroyMicromapEXT") (cmd_copy_micromap_ext,
                    "vkCmdCopyMicromapEXT") (copy_micromap_ext, "vkCopyMicromapEXT")
                    (cmd_copy_micromap_to_memory_ext, "vkCmdCopyMicromapToMemoryEXT")
                    (copy_micromap_to_memory_ext, "vkCopyMicromapToMemoryEXT")
                    (cmd_copy_memory_to_micromap_ext, "vkCmdCopyMemoryToMicromapEXT")
                    (copy_memory_to_micromap_ext, "vkCopyMemoryToMicromapEXT")
                    (cmd_write_micromaps_properties_ext,
                    "vkCmdWriteMicromapsPropertiesEXT") (write_micromaps_properties_ext,
                    "vkWriteMicromapsPropertiesEXT")
                    (get_device_micromap_compatibility_ext,
                    "vkGetDeviceMicromapCompatibilityEXT") (get_micromap_build_sizes_ext,
                    "vkGetMicromapBuildSizesEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_pageable_device_local_memory")) {
                load_fns! {
                    self, loader, (set_device_memory_priority_ext,
                    "vkSetDeviceMemoryPriorityEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_KHR_maintenance4")) {
                load_fns! {
                    self, loader, (get_device_buffer_memory_requirements_khr,
                    "vkGetDeviceBufferMemoryRequirementsKHR")
                    (get_device_image_memory_requirements_khr,
                    "vkGetDeviceImageMemoryRequirementsKHR")
                    (get_device_image_sparse_memory_requirements_khr,
                    "vkGetDeviceImageSparseMemoryRequirementsKHR")
                }
            }
            if conf.extension_enabled(cstr!("VK_VALVE_descriptor_set_host_mapping")) {
                load_fns! {
                    self, loader, (get_descriptor_set_layout_host_mapping_info_valve,
                    "vkGetDescriptorSetLayoutHostMappingInfoVALVE")
                    (get_descriptor_set_host_mapping_valve,
                    "vkGetDescriptorSetHostMappingVALVE")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_extended_dynamic_state3")) {
                load_fns! {
                    self, loader, (cmd_set_tessellation_domain_origin_ext,
                    "vkCmdSetTessellationDomainOriginEXT")
                    (cmd_set_depth_clamp_enable_ext, "vkCmdSetDepthClampEnableEXT")
                    (cmd_set_polygon_mode_ext, "vkCmdSetPolygonModeEXT")
                    (cmd_set_rasterization_samples_ext,
                    "vkCmdSetRasterizationSamplesEXT") (cmd_set_sample_mask_ext,
                    "vkCmdSetSampleMaskEXT") (cmd_set_alpha_to_coverage_enable_ext,
                    "vkCmdSetAlphaToCoverageEnableEXT") (cmd_set_alpha_to_one_enable_ext,
                    "vkCmdSetAlphaToOneEnableEXT") (cmd_set_logic_op_enable_ext,
                    "vkCmdSetLogicOpEnableEXT") (cmd_set_color_blend_enable_ext,
                    "vkCmdSetColorBlendEnableEXT") (cmd_set_color_blend_equation_ext,
                    "vkCmdSetColorBlendEquationEXT") (cmd_set_color_write_mask_ext,
                    "vkCmdSetColorWriteMaskEXT") (cmd_set_rasterization_stream_ext,
                    "vkCmdSetRasterizationStreamEXT")
                    (cmd_set_conservative_rasterization_mode_ext,
                    "vkCmdSetConservativeRasterizationModeEXT")
                    (cmd_set_extra_primitive_overestimation_size_ext,
                    "vkCmdSetExtraPrimitiveOverestimationSizeEXT")
                    (cmd_set_depth_clip_enable_ext, "vkCmdSetDepthClipEnableEXT")
                    (cmd_set_sample_locations_enable_ext,
                    "vkCmdSetSampleLocationsEnableEXT")
                    (cmd_set_color_blend_advanced_ext, "vkCmdSetColorBlendAdvancedEXT")
                    (cmd_set_provoking_vertex_mode_ext, "vkCmdSetProvokingVertexModeEXT")
                    (cmd_set_line_rasterization_mode_ext,
                    "vkCmdSetLineRasterizationModeEXT") (cmd_set_line_stipple_enable_ext,
                    "vkCmdSetLineStippleEnableEXT")
                    (cmd_set_depth_clip_negative_one_to_one_ext,
                    "vkCmdSetDepthClipNegativeOneToOneEXT")
                    (cmd_set_viewport_wscaling_enable_nv,
                    "vkCmdSetViewportWScalingEnableNV") (cmd_set_viewport_swizzle_nv,
                    "vkCmdSetViewportSwizzleNV") (cmd_set_coverage_to_color_enable_nv,
                    "vkCmdSetCoverageToColorEnableNV")
                    (cmd_set_coverage_to_color_location_nv,
                    "vkCmdSetCoverageToColorLocationNV")
                    (cmd_set_coverage_modulation_mode_nv,
                    "vkCmdSetCoverageModulationModeNV")
                    (cmd_set_coverage_modulation_table_enable_nv,
                    "vkCmdSetCoverageModulationTableEnableNV")
                    (cmd_set_coverage_modulation_table_nv,
                    "vkCmdSetCoverageModulationTableNV")
                    (cmd_set_shading_rate_image_enable_nv,
                    "vkCmdSetShadingRateImageEnableNV")
                    (cmd_set_coverage_reduction_mode_nv,
                    "vkCmdSetCoverageReductionModeNV")
                    (cmd_set_representative_fragment_test_enable_nv,
                    "vkCmdSetRepresentativeFragmentTestEnableNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_EXT_shader_module_identifier")) {
                load_fns! {
                    self, loader, (get_shader_module_identifier_ext,
                    "vkGetShaderModuleIdentifierEXT")
                    (get_shader_module_create_info_identifier_ext,
                    "vkGetShaderModuleCreateInfoIdentifierEXT")
                }
            }
            if conf.extension_enabled(cstr!("VK_NV_optical_flow")) {
                load_fns! {
                    self, loader, (create_optical_flow_session_nv,
                    "vkCreateOpticalFlowSessionNV") (destroy_optical_flow_session_nv,
                    "vkDestroyOpticalFlowSessionNV") (bind_optical_flow_session_image_nv,
                    "vkBindOpticalFlowSessionImageNV") (cmd_optical_flow_execute_nv,
                    "vkCmdOpticalFlowExecuteNV")
                }
            }
            if conf.extension_enabled(cstr!("VK_QCOM_tile_properties")) {
                load_fns! {
                    self, loader, (get_framebuffer_tile_properties_qcom,
                    "vkGetFramebufferTilePropertiesQCOM")
                    (get_dynamic_rendering_tile_properties_qcom,
                    "vkGetDynamicRenderingTilePropertiesQCOM")
                }
            }
        }
    }
}
#[cfg(feature = "raw")]
impl DeviceTable {
    #[track_caller]
    #[doc(alias = "vkGetDeviceProcAddr")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
    pub unsafe fn get_device_proc_addr(
        &self,
        device: crate::vk10::Device,
        p_name: *const std::os::raw::c_char,
    ) -> crate::vk10::PfnVoidFunction {
        (self.get_device_proc_addr.unwrap())(device, p_name)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDevice")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
    pub unsafe fn destroy_device(
        &self,
        device: crate::vk10::Device,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_device.unwrap())(device, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
    pub unsafe fn get_device_queue(
        &self,
        device: crate::vk10::Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut crate::vk10::Queue,
    ) {
        (self
            .get_device_queue
            .unwrap())(device, queue_family_index, queue_index, p_queue)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSubmit")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
    pub unsafe fn queue_submit(
        &self,
        queue: crate::vk10::Queue,
        submit_count: u32,
        p_submits: *const crate::vk10::SubmitInfo,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_submit.unwrap())(queue, submit_count, p_submits, fence)
    }
    #[track_caller]
    #[doc(alias = "vkQueueWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
    pub unsafe fn queue_wait_idle(
        &self,
        queue: crate::vk10::Queue,
    ) -> crate::vk10::Result {
        (self.queue_wait_idle.unwrap())(queue)
    }
    #[track_caller]
    #[doc(alias = "vkDeviceWaitIdle")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
    pub unsafe fn device_wait_idle(
        &self,
        device: crate::vk10::Device,
    ) -> crate::vk10::Result {
        (self.device_wait_idle.unwrap())(device)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
    pub unsafe fn allocate_memory(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::MemoryAllocateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_memory: *mut crate::vk10::DeviceMemory,
    ) -> crate::vk10::Result {
        (self.allocate_memory.unwrap())(device, p_allocate_info, p_allocator, p_memory)
    }
    #[track_caller]
    #[doc(alias = "vkFreeMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
    pub unsafe fn free_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.free_memory.unwrap())(device, memory, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkMapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
    pub unsafe fn map_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        offset: crate::vk10::DeviceSize,
        size: crate::vk10::DeviceSize,
        flags: crate::vk10::MemoryMapFlags,
        pp_data: *mut *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self.map_memory.unwrap())(device, memory, offset, size, flags, pp_data)
    }
    #[track_caller]
    #[doc(alias = "vkUnmapMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
    pub unsafe fn unmap_memory(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
    ) {
        (self.unmap_memory.unwrap())(device, memory)
    }
    #[track_caller]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: crate::vk10::Device,
        memory_range_count: u32,
        p_memory_ranges: *const crate::vk10::MappedMemoryRange,
    ) -> crate::vk10::Result {
        (self
            .flush_mapped_memory_ranges
            .unwrap())(device, memory_range_count, p_memory_ranges)
    }
    #[track_caller]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: crate::vk10::Device,
        memory_range_count: u32,
        p_memory_ranges: *const crate::vk10::MappedMemoryRange,
    ) -> crate::vk10::Result {
        (self
            .invalidate_mapped_memory_ranges
            .unwrap())(device, memory_range_count, p_memory_ranges)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        p_committed_memory_in_bytes: *mut crate::vk10::DeviceSize,
    ) {
        (self
            .get_device_memory_commitment
            .unwrap())(device, memory, p_committed_memory_in_bytes)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        p_memory_requirements: *mut crate::vk10::MemoryRequirements,
    ) {
        (self
            .get_buffer_memory_requirements
            .unwrap())(device, buffer, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkBindBufferMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
    pub unsafe fn bind_buffer_memory(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        memory: crate::vk10::DeviceMemory,
        memory_offset: crate::vk10::DeviceSize,
    ) -> crate::vk10::Result {
        (self.bind_buffer_memory.unwrap())(device, buffer, memory, memory_offset)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_memory_requirements: *mut crate::vk10::MemoryRequirements,
    ) {
        (self
            .get_image_memory_requirements
            .unwrap())(device, image, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkBindImageMemory")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
    pub unsafe fn bind_image_memory(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        memory: crate::vk10::DeviceMemory,
        memory_offset: crate::vk10::DeviceSize,
    ) -> crate::vk10::Result {
        (self.bind_image_memory.unwrap())(device, image, memory, memory_offset)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk10::SparseImageMemoryRequirements,
    ) {
        (self
            .get_image_sparse_memory_requirements
            .unwrap())(
            device,
            image,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkQueueBindSparse")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: crate::vk10::Queue,
        bind_info_count: u32,
        p_bind_info: *const crate::vk10::BindSparseInfo,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_bind_sparse.unwrap())(queue, bind_info_count, p_bind_info, fence)
    }
    #[track_caller]
    #[doc(alias = "vkCreateFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
    pub unsafe fn create_fence(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::FenceCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_fence: *mut crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.create_fence.unwrap())(device, p_create_info, p_allocator, p_fence)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyFence")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
    pub unsafe fn destroy_fence(
        &self,
        device: crate::vk10::Device,
        fence: crate::vk10::Fence,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_fence.unwrap())(device, fence, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
    pub unsafe fn reset_fences(
        &self,
        device: crate::vk10::Device,
        fence_count: u32,
        p_fences: *const crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.reset_fences.unwrap())(device, fence_count, p_fences)
    }
    #[track_caller]
    #[doc(alias = "vkGetFenceStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
    pub unsafe fn get_fence_status(
        &self,
        device: crate::vk10::Device,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.get_fence_status.unwrap())(device, fence)
    }
    #[track_caller]
    #[doc(alias = "vkWaitForFences")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
    pub unsafe fn wait_for_fences(
        &self,
        device: crate::vk10::Device,
        fence_count: u32,
        p_fences: *const crate::vk10::Fence,
        wait_all: crate::vk10::Bool32,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_for_fences.unwrap())(device, fence_count, p_fences, wait_all, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
    pub unsafe fn create_semaphore(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::SemaphoreCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_semaphore: *mut crate::vk10::Semaphore,
    ) -> crate::vk10::Result {
        (self.create_semaphore.unwrap())(device, p_create_info, p_allocator, p_semaphore)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
    pub unsafe fn destroy_semaphore(
        &self,
        device: crate::vk10::Device,
        semaphore: crate::vk10::Semaphore,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_semaphore.unwrap())(device, semaphore, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
    pub unsafe fn create_event(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::EventCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_event: *mut crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.create_event.unwrap())(device, p_create_info, p_allocator, p_event)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
    pub unsafe fn destroy_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_event.unwrap())(device, event, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetEventStatus")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
    pub unsafe fn get_event_status(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.get_event_status.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
    pub unsafe fn set_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.set_event.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
    pub unsafe fn reset_event(
        &self,
        device: crate::vk10::Device,
        event: crate::vk10::Event,
    ) -> crate::vk10::Result {
        (self.reset_event.unwrap())(device, event)
    }
    #[track_caller]
    #[doc(alias = "vkCreateQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
    pub unsafe fn create_query_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::QueryPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_query_pool: *mut crate::vk10::QueryPool,
    ) -> crate::vk10::Result {
        (self
            .create_query_pool
            .unwrap())(device, p_create_info, p_allocator, p_query_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
    pub unsafe fn destroy_query_pool(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_query_pool.unwrap())(device, query_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
    pub unsafe fn get_query_pool_results(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
        stride: crate::vk10::DeviceSize,
        flags: crate::vk10::QueryResultFlags,
    ) -> crate::vk10::Result {
        (self
            .get_query_pool_results
            .unwrap())(
            device,
            query_pool,
            first_query,
            query_count,
            data_size,
            p_data,
            stride,
            flags,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
    pub unsafe fn create_buffer(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::BufferCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_buffer: *mut crate::vk10::Buffer,
    ) -> crate::vk10::Result {
        (self.create_buffer.unwrap())(device, p_create_info, p_allocator, p_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
    pub unsafe fn destroy_buffer(
        &self,
        device: crate::vk10::Device,
        buffer: crate::vk10::Buffer,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_buffer.unwrap())(device, buffer, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
    pub unsafe fn create_buffer_view(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::BufferViewCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_view: *mut crate::vk10::BufferView,
    ) -> crate::vk10::Result {
        (self.create_buffer_view.unwrap())(device, p_create_info, p_allocator, p_view)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyBufferView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
    pub unsafe fn destroy_buffer_view(
        &self,
        device: crate::vk10::Device,
        buffer_view: crate::vk10::BufferView,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_buffer_view.unwrap())(device, buffer_view, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
    pub unsafe fn create_image(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ImageCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_image: *mut crate::vk10::Image,
    ) -> crate::vk10::Result {
        (self.create_image.unwrap())(device, p_create_info, p_allocator, p_image)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
    pub unsafe fn destroy_image(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_image.unwrap())(device, image, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_subresource: *const crate::vk10::ImageSubresource,
        p_layout: *mut crate::vk10::SubresourceLayout,
    ) {
        (self
            .get_image_subresource_layout
            .unwrap())(device, image, p_subresource, p_layout)
    }
    #[track_caller]
    #[doc(alias = "vkCreateImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
    pub unsafe fn create_image_view(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ImageViewCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_view: *mut crate::vk10::ImageView,
    ) -> crate::vk10::Result {
        (self.create_image_view.unwrap())(device, p_create_info, p_allocator, p_view)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyImageView")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
    pub unsafe fn destroy_image_view(
        &self,
        device: crate::vk10::Device,
        image_view: crate::vk10::ImageView,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_image_view.unwrap())(device, image_view, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
    pub unsafe fn create_shader_module(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_shader_module: *mut crate::vk10::ShaderModule,
    ) -> crate::vk10::Result {
        (self
            .create_shader_module
            .unwrap())(device, p_create_info, p_allocator, p_shader_module)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyShaderModule")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
    pub unsafe fn destroy_shader_module(
        &self,
        device: crate::vk10::Device,
        shader_module: crate::vk10::ShaderModule,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_shader_module.unwrap())(device, shader_module, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
    pub unsafe fn create_pipeline_cache(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::PipelineCacheCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipeline_cache: *mut crate::vk10::PipelineCache,
    ) -> crate::vk10::Result {
        (self
            .create_pipeline_cache
            .unwrap())(device, p_create_info, p_allocator, p_pipeline_cache)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineCache")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline_cache.unwrap())(device, pipeline_cache, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelineCacheData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_cache_data
            .unwrap())(device, pipeline_cache, p_data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkMergePipelineCaches")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: crate::vk10::Device,
        dst_cache: crate::vk10::PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const crate::vk10::PipelineCache,
    ) -> crate::vk10::Result {
        (self
            .merge_pipeline_caches
            .unwrap())(device, dst_cache, src_cache_count, p_src_caches)
    }
    #[track_caller]
    #[doc(alias = "vkCreateGraphicsPipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::vk10::GraphicsPipelineCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
            .create_graphics_pipelines
            .unwrap())(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateComputePipelines")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
    pub unsafe fn create_compute_pipelines(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::vk10::ComputePipelineCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
            .create_compute_pipelines
            .unwrap())(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
    pub unsafe fn destroy_pipeline(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline.unwrap())(device, pipeline, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
    pub unsafe fn create_pipeline_layout(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::PipelineLayoutCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipeline_layout: *mut crate::vk10::PipelineLayout,
    ) -> crate::vk10::Result {
        (self
            .create_pipeline_layout
            .unwrap())(device, p_create_info, p_allocator, p_pipeline_layout)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPipelineLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: crate::vk10::Device,
        pipeline_layout: crate::vk10::PipelineLayout,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_pipeline_layout.unwrap())(device, pipeline_layout, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
    pub unsafe fn create_sampler(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::SamplerCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_sampler: *mut crate::vk10::Sampler,
    ) -> crate::vk10::Result {
        (self.create_sampler.unwrap())(device, p_create_info, p_allocator, p_sampler)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySampler")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
    pub unsafe fn destroy_sampler(
        &self,
        device: crate::vk10::Device,
        sampler: crate::vk10::Sampler,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_sampler.unwrap())(device, sampler, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_set_layout: *mut crate::vk10::DescriptorSetLayout,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_set_layout
            .unwrap())(device, p_create_info, p_allocator, p_set_layout)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: crate::vk10::Device,
        descriptor_set_layout: crate::vk10::DescriptorSetLayout,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_descriptor_set_layout
            .unwrap())(device, descriptor_set_layout, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
    pub unsafe fn create_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_descriptor_pool: *mut crate::vk10::DescriptorPool,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_pool
            .unwrap())(device, p_create_info, p_allocator, p_descriptor_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_descriptor_pool.unwrap())(device, descriptor_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetDescriptorPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        flags: crate::vk10::DescriptorPoolResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_descriptor_pool.unwrap())(device, descriptor_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut crate::vk10::DescriptorSet,
    ) -> crate::vk10::Result {
        (self
            .allocate_descriptor_sets
            .unwrap())(device, p_allocate_info, p_descriptor_sets)
    }
    #[track_caller]
    #[doc(alias = "vkFreeDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
    pub unsafe fn free_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        descriptor_pool: crate::vk10::DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const crate::vk10::DescriptorSet,
    ) -> crate::vk10::Result {
        (self
            .free_descriptor_sets
            .unwrap())(device, descriptor_pool, descriptor_set_count, p_descriptor_sets)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
    pub unsafe fn update_descriptor_sets(
        &self,
        device: crate::vk10::Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const crate::vk10::CopyDescriptorSet,
    ) {
        (self
            .update_descriptor_sets
            .unwrap())(
            device,
            descriptor_write_count,
            p_descriptor_writes,
            descriptor_copy_count,
            p_descriptor_copies,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
    pub unsafe fn create_framebuffer(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::FramebufferCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_framebuffer: *mut crate::vk10::Framebuffer,
    ) -> crate::vk10::Result {
        (self
            .create_framebuffer
            .unwrap())(device, p_create_info, p_allocator, p_framebuffer)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyFramebuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
    pub unsafe fn destroy_framebuffer(
        &self,
        device: crate::vk10::Device,
        framebuffer: crate::vk10::Framebuffer,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_framebuffer.unwrap())(device, framebuffer, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
    pub unsafe fn create_render_pass(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::RenderPassCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_render_pass: *mut crate::vk10::RenderPass,
    ) -> crate::vk10::Result {
        (self
            .create_render_pass
            .unwrap())(device, p_create_info, p_allocator, p_render_pass)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
    pub unsafe fn destroy_render_pass(
        &self,
        device: crate::vk10::Device,
        render_pass: crate::vk10::RenderPass,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_render_pass.unwrap())(device, render_pass, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
    pub unsafe fn get_render_area_granularity(
        &self,
        device: crate::vk10::Device,
        render_pass: crate::vk10::RenderPass,
        p_granularity: *mut crate::vk10::Extent2D,
    ) {
        (self.get_render_area_granularity.unwrap())(device, render_pass, p_granularity)
    }
    #[track_caller]
    #[doc(alias = "vkCreateCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
    pub unsafe fn create_command_pool(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::CommandPoolCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_command_pool: *mut crate::vk10::CommandPool,
    ) -> crate::vk10::Result {
        (self
            .create_command_pool
            .unwrap())(device, p_create_info, p_allocator, p_command_pool)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
    pub unsafe fn destroy_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_command_pool.unwrap())(device, command_pool, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkResetCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
    pub unsafe fn reset_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        flags: crate::vk10::CommandPoolResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_command_pool.unwrap())(device, command_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkAllocateCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
    pub unsafe fn allocate_command_buffers(
        &self,
        device: crate::vk10::Device,
        p_allocate_info: *const crate::vk10::CommandBufferAllocateInfo,
        p_command_buffers: *mut crate::vk10::CommandBuffer,
    ) -> crate::vk10::Result {
        (self
            .allocate_command_buffers
            .unwrap())(device, p_allocate_info, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkFreeCommandBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
    pub unsafe fn free_command_buffers(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const crate::vk10::CommandBuffer,
    ) {
        (self
            .free_command_buffers
            .unwrap())(device, command_pool, command_buffer_count, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkBeginCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_begin_info: *const crate::vk10::CommandBufferBeginInfo,
    ) -> crate::vk10::Result {
        (self.begin_command_buffer.unwrap())(command_buffer, p_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkEndCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
    pub unsafe fn end_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) -> crate::vk10::Result {
        (self.end_command_buffer.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkResetCommandBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        flags: crate::vk10::CommandBufferResetFlags,
    ) -> crate::vk10::Result {
        (self.reset_command_buffer.unwrap())(command_buffer, flags)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindPipeline")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        pipeline: crate::vk10::Pipeline,
    ) {
        (self.cmd_bind_pipeline.unwrap())(command_buffer, pipeline_bind_point, pipeline)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const crate::vk10::Viewport,
    ) {
        (self
            .cmd_set_viewport
            .unwrap())(command_buffer, first_viewport, viewport_count, p_viewports)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetScissor")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_scissor
            .unwrap())(command_buffer, first_scissor, scissor_count, p_scissors)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLineWidth")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_width: std::os::raw::c_float,
    ) {
        (self.cmd_set_line_width.unwrap())(command_buffer, line_width)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBias")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_constant_factor: std::os::raw::c_float,
        depth_bias_clamp: std::os::raw::c_float,
        depth_bias_slope_factor: std::os::raw::c_float,
    ) {
        (self
            .cmd_set_depth_bias
            .unwrap())(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetBlendConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        blend_constants: *const [std::os::raw::c_float; 4],
    ) {
        (self.cmd_set_blend_constants.unwrap())(command_buffer, blend_constants)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBounds")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        min_depth_bounds: std::os::raw::c_float,
        max_depth_bounds: std::os::raw::c_float,
    ) {
        (self
            .cmd_set_depth_bounds
            .unwrap())(command_buffer, min_depth_bounds, max_depth_bounds)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self
            .cmd_set_stencil_compare_mask
            .unwrap())(command_buffer, face_mask, compare_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        write_mask: u32,
    ) {
        (self.cmd_set_stencil_write_mask.unwrap())(command_buffer, face_mask, write_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilReference")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        reference: u32,
    ) {
        (self.cmd_set_stencil_reference.unwrap())(command_buffer, face_mask, reference)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        layout: crate::vk10::PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const crate::vk10::DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        (self
            .cmd_bind_descriptor_sets
            .unwrap())(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_set_count,
            p_descriptor_sets,
            dynamic_offset_count,
            p_dynamic_offsets,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        index_type: crate::vk10::IndexType,
    ) {
        (self.cmd_bind_index_buffer.unwrap())(command_buffer, buffer, offset, index_type)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const crate::vk10::Buffer,
        p_offsets: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_bind_vertex_buffers
            .unwrap())(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDraw")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        (self
            .cmd_draw
            .unwrap())(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexed")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        (self
            .cmd_draw_indexed
            .unwrap())(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indirect
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indexed_indirect
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatch")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .cmd_dispatch
            .unwrap())(command_buffer, group_count_x, group_count_y, group_count_z)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatchIndirect")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
    ) {
        (self.cmd_dispatch_indirect.unwrap())(command_buffer, buffer, offset)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_buffer: crate::vk10::Buffer,
        dst_buffer: crate::vk10::Buffer,
        region_count: u32,
        p_regions: *const crate::vk10::BufferCopy,
    ) {
        (self
            .cmd_copy_buffer
            .unwrap())(command_buffer, src_buffer, dst_buffer, region_count, p_regions)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageCopy,
    ) {
        (self
            .cmd_copy_image
            .unwrap())(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageBlit,
        filter: crate::vk10::Filter,
    ) {
        (self
            .cmd_blit_image
            .unwrap())(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
            filter,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_buffer: crate::vk10::Buffer,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::BufferImageCopy,
    ) {
        (self
            .cmd_copy_buffer_to_image
            .unwrap())(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_buffer: crate::vk10::Buffer,
        region_count: u32,
        p_regions: *const crate::vk10::BufferImageCopy,
    ) {
        (self
            .cmd_copy_image_to_buffer
            .unwrap())(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            region_count,
            p_regions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdUpdateBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        data_size: crate::vk10::DeviceSize,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .cmd_update_buffer
            .unwrap())(command_buffer, dst_buffer, dst_offset, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdFillBuffer")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        size: crate::vk10::DeviceSize,
        data: u32,
    ) {
        (self
            .cmd_fill_buffer
            .unwrap())(command_buffer, dst_buffer, dst_offset, size, data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdClearColorImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image: crate::vk10::Image,
        image_layout: crate::vk10::ImageLayout,
        p_color: *const crate::vk10::ClearColorValue,
        range_count: u32,
        p_ranges: *const crate::vk10::ImageSubresourceRange,
    ) {
        (self
            .cmd_clear_color_image
            .unwrap())(
            command_buffer,
            image,
            image_layout,
            p_color,
            range_count,
            p_ranges,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image: crate::vk10::Image,
        image_layout: crate::vk10::ImageLayout,
        p_depth_stencil: *const crate::vk10::ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const crate::vk10::ImageSubresourceRange,
    ) {
        (self
            .cmd_clear_depth_stencil_image
            .unwrap())(
            command_buffer,
            image,
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdClearAttachments")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        attachment_count: u32,
        p_attachments: *const crate::vk10::ClearAttachment,
        rect_count: u32,
        p_rects: *const crate::vk10::ClearRect,
    ) {
        (self
            .cmd_clear_attachments
            .unwrap())(
            command_buffer,
            attachment_count,
            p_attachments,
            rect_count,
            p_rects,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_image: crate::vk10::Image,
        src_image_layout: crate::vk10::ImageLayout,
        dst_image: crate::vk10::Image,
        dst_image_layout: crate::vk10::ImageLayout,
        region_count: u32,
        p_regions: *const crate::vk10::ImageResolve,
    ) {
        (self
            .cmd_resolve_image
            .unwrap())(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk10::PipelineStageFlags,
    ) {
        (self.cmd_set_event.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk10::PipelineStageFlags,
    ) {
        (self.cmd_reset_event.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event_count: u32,
        p_events: *const crate::vk10::Event,
        src_stage_mask: crate::vk10::PipelineStageFlags,
        dst_stage_mask: crate::vk10::PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const crate::vk10::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
    ) {
        (self
            .cmd_wait_events
            .unwrap())(
            command_buffer,
            event_count,
            p_events,
            src_stage_mask,
            dst_stage_mask,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        src_stage_mask: crate::vk10::PipelineStageFlags,
        dst_stage_mask: crate::vk10::PipelineStageFlags,
        dependency_flags: crate::vk10::DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const crate::vk10::MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const crate::vk10::BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const crate::vk10::ImageMemoryBarrier,
    ) {
        (self
            .cmd_pipeline_barrier
            .unwrap())(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        flags: crate::vk10::QueryControlFlags,
    ) {
        (self.cmd_begin_query.unwrap())(command_buffer, query_pool, query, flags)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndQuery")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self.cmd_end_query.unwrap())(command_buffer, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .cmd_reset_query_pool
            .unwrap())(command_buffer, query_pool, first_query, query_count)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stage: crate::vk10::PipelineStageFlags,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self
            .cmd_write_timestamp
            .unwrap())(command_buffer, pipeline_stage, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        stride: crate::vk10::DeviceSize,
        flags: crate::vk10::QueryResultFlags,
    ) {
        (self
            .cmd_copy_query_pool_results
            .unwrap())(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdPushConstants")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        layout: crate::vk10::PipelineLayout,
        stage_flags: crate::vk10::ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const std::os::raw::c_void,
    ) {
        (self
            .cmd_push_constants
            .unwrap())(command_buffer, layout, stage_flags, offset, size, p_values)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
        contents: crate::vk10::SubpassContents,
    ) {
        (self
            .cmd_begin_render_pass
            .unwrap())(command_buffer, p_render_pass_begin, contents)
    }
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        contents: crate::vk10::SubpassContents,
    ) {
        (self.cmd_next_subpass.unwrap())(command_buffer, contents)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
    pub unsafe fn cmd_end_render_pass(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_render_pass.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdExecuteCommands")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const crate::vk10::CommandBuffer,
    ) {
        (self
            .cmd_execute_commands
            .unwrap())(command_buffer, command_buffer_count, p_command_buffers)
    }
    #[track_caller]
    #[doc(alias = "vkTrimCommandPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
    pub unsafe fn trim_command_pool(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        flags: crate::vk11::CommandPoolTrimFlags,
    ) {
        (self.trim_command_pool.unwrap())(device, command_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: crate::vk10::Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
    ) {
        (self
            .get_device_group_peer_memory_features
            .unwrap())(
            device,
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    #[track_caller]
    #[doc(alias = "vkBindBufferMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
    pub unsafe fn bind_buffer_memory_2(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_buffer_memory_2.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkBindImageMemory2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
    pub unsafe fn bind_image_memory_2(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_image_memory_2.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDeviceMask")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        device_mask: u32,
    ) {
        (self.cmd_set_device_mask.unwrap())(command_buffer, device_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatchBase")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html)
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .cmd_dispatch_base
            .unwrap())(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_update_template
            .unwrap())(device, p_create_info, p_allocator, p_descriptor_update_template)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: crate::vk10::Device,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_descriptor_update_template
            .unwrap())(device, descriptor_update_template, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: crate::vk10::Device,
        descriptor_set: crate::vk10::DescriptorSet,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .update_descriptor_set_with_template
            .unwrap())(device, descriptor_set, descriptor_update_template, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
    pub unsafe fn get_buffer_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_buffer_memory_requirements_2
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
    pub unsafe fn get_image_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_image_memory_requirements_2
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
    pub unsafe fn get_image_sparse_memory_requirements_2(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
    ) {
        (self
            .get_image_sparse_memory_requirements_2
            .unwrap())(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
    ) -> crate::vk10::Result {
        (self
            .create_sampler_ycbcr_conversion
            .unwrap())(device, p_create_info, p_allocator, p_ycbcr_conversion)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: crate::vk10::Device,
        ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_sampler_ycbcr_conversion
            .unwrap())(device, ycbcr_conversion, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceQueue2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
    pub unsafe fn get_device_queue_2(
        &self,
        device: crate::vk10::Device,
        p_queue_info: *const crate::vk11::DeviceQueueInfo2,
        p_queue: *mut crate::vk10::Queue,
    ) {
        (self.get_device_queue_2.unwrap())(device, p_queue_info, p_queue)
    }
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
        p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
    ) {
        (self
            .get_descriptor_set_layout_support
            .unwrap())(device, p_create_info, p_support)
    }
    #[track_caller]
    #[doc(alias = "vkResetQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)
    pub unsafe fn reset_query_pool(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self.reset_query_pool.unwrap())(device, query_pool, first_query, query_count)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html)
    pub unsafe fn create_render_pass_2(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk12::RenderPassCreateInfo2,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_render_pass: *mut crate::vk10::RenderPass,
    ) -> crate::vk10::Result {
        (self
            .create_render_pass_2
            .unwrap())(device, p_create_info, p_allocator, p_render_pass)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html)
    pub unsafe fn cmd_begin_render_pass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
        p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
    ) {
        (self
            .cmd_begin_render_pass_2
            .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html)
    pub unsafe fn cmd_next_subpass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
        p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
    ) {
        (self
            .cmd_next_subpass_2
            .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html)
    pub unsafe fn cmd_end_render_pass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
    ) {
        (self.cmd_end_render_pass_2.unwrap())(command_buffer, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html)
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: crate::vk10::Device,
        semaphore: crate::vk10::Semaphore,
        p_value: *mut u64,
    ) -> crate::vk10::Result {
        (self.get_semaphore_counter_value.unwrap())(device, semaphore, p_value)
    }
    #[track_caller]
    #[doc(alias = "vkWaitSemaphores")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)
    pub unsafe fn wait_semaphores(
        &self,
        device: crate::vk10::Device,
        p_wait_info: *const crate::vk12::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_semaphores.unwrap())(device, p_wait_info, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkSignalSemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html)
    pub unsafe fn signal_semaphore(
        &self,
        device: crate::vk10::Device,
        p_signal_info: *const crate::vk12::SemaphoreSignalInfo,
    ) -> crate::vk10::Result {
        (self.signal_semaphore.unwrap())(device, p_signal_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html)
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indirect_count
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirectCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html)
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indexed_indirect_count
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::BufferDeviceAddressInfo,
    ) -> u64 {
        (self.get_buffer_opaque_capture_address.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferDeviceAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)
    pub unsafe fn get_buffer_device_address(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::BufferDeviceAddressInfo,
    ) -> crate::vk10::DeviceAddress {
        (self.get_buffer_device_address.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        (self.get_device_memory_opaque_capture_address.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceBufferMemoryRequirements,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_device_buffer_memory_requirements
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceImageMemoryRequirements,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_device_image_memory_requirements
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html)
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceImageMemoryRequirements,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
    ) {
        (self
            .get_device_image_sparse_memory_requirements
            .unwrap())(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCullMode")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        cull_mode: crate::vk10::CullModeFlags,
    ) {
        (self.cmd_set_cull_mode.unwrap())(command_buffer, cull_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetFrontFace")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        front_face: crate::vk10::FrontFace,
    ) {
        (self.cmd_set_front_face.unwrap())(command_buffer, front_face)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html)
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_topology: crate::vk10::PrimitiveTopology,
    ) {
        (self.cmd_set_primitive_topology.unwrap())(command_buffer, primitive_topology)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWithCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewport_count: u32,
        p_viewports: *const crate::vk10::Viewport,
    ) {
        (self
            .cmd_set_viewport_with_count
            .unwrap())(command_buffer, viewport_count, p_viewports)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetScissorWithCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        scissor_count: u32,
        p_scissors: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_scissor_with_count
            .unwrap())(command_buffer, scissor_count, p_scissors)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html)
    pub unsafe fn cmd_bind_vertex_buffers_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const crate::vk10::Buffer,
        p_offsets: *const crate::vk10::DeviceSize,
        p_sizes: *const crate::vk10::DeviceSize,
        p_strides: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_bind_vertex_buffers_2
            .unwrap())(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
            p_strides,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_test_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_test_enable.unwrap())(command_buffer, depth_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_write_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_write_enable.unwrap())(command_buffer, depth_write_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_compare_op: crate::vk10::CompareOp,
    ) {
        (self.cmd_set_depth_compare_op.unwrap())(command_buffer, depth_compare_op)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html)
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bounds_test_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_depth_bounds_test_enable
            .unwrap())(command_buffer, depth_bounds_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html)
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stencil_test_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_stencil_test_enable.unwrap())(command_buffer, stencil_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilOp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html)
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        fail_op: crate::vk10::StencilOp,
        pass_op: crate::vk10::StencilOp,
        depth_fail_op: crate::vk10::StencilOp,
        compare_op: crate::vk10::CompareOp,
    ) {
        (self
            .cmd_set_stencil_op
            .unwrap())(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html)
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterizer_discard_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_rasterizer_discard_enable
            .unwrap())(command_buffer, rasterizer_discard_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html)
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_bias_enable.unwrap())(command_buffer, depth_bias_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html)
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_restart_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_primitive_restart_enable
            .unwrap())(command_buffer, primitive_restart_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePrivateDataSlot")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html)
    pub unsafe fn create_private_data_slot(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk13::PrivateDataSlotCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_private_data_slot: *mut crate::vk13::PrivateDataSlot,
    ) -> crate::vk10::Result {
        (self
            .create_private_data_slot
            .unwrap())(device, p_create_info, p_allocator, p_private_data_slot)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html)
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: crate::vk10::Device,
        private_data_slot: crate::vk13::PrivateDataSlot,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_private_data_slot.unwrap())(device, private_data_slot, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkSetPrivateData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html)
    pub unsafe fn set_private_data(
        &self,
        device: crate::vk10::Device,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
        data: u64,
    ) -> crate::vk10::Result {
        (self
            .set_private_data
            .unwrap())(device, object_type, object_handle, private_data_slot, data)
    }
    #[track_caller]
    #[doc(alias = "vkGetPrivateData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html)
    pub unsafe fn get_private_data(
        &self,
        device: crate::vk10::Device,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
        p_data: *mut u64,
    ) {
        (self
            .get_private_data
            .unwrap())(device, object_type, object_handle, private_data_slot, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html)
    pub unsafe fn cmd_copy_buffer_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_buffer_info: *const crate::vk13::CopyBufferInfo2,
    ) {
        (self.cmd_copy_buffer_2.unwrap())(command_buffer, p_copy_buffer_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html)
    pub unsafe fn cmd_copy_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_image_info: *const crate::vk13::CopyImageInfo2,
    ) {
        (self.cmd_copy_image_2.unwrap())(command_buffer, p_copy_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html)
    pub unsafe fn cmd_blit_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_blit_image_info: *const crate::vk13::BlitImageInfo2,
    ) {
        (self.cmd_blit_image_2.unwrap())(command_buffer, p_blit_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)
    pub unsafe fn cmd_copy_buffer_to_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_buffer_to_image_info: *const crate::vk13::CopyBufferToImageInfo2,
    ) {
        (self
            .cmd_copy_buffer_to_image_2
            .unwrap())(command_buffer, p_copy_buffer_to_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)
    pub unsafe fn cmd_copy_image_to_buffer_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_image_to_buffer_info: *const crate::vk13::CopyImageToBufferInfo2,
    ) {
        (self
            .cmd_copy_image_to_buffer_2
            .unwrap())(command_buffer, p_copy_image_to_buffer_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html)
    pub unsafe fn cmd_resolve_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_resolve_image_info: *const crate::vk13::ResolveImageInfo2,
    ) {
        (self.cmd_resolve_image_2.unwrap())(command_buffer, p_resolve_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html)
    pub unsafe fn cmd_set_event_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        p_dependency_info: *const crate::vk13::DependencyInfo,
    ) {
        (self.cmd_set_event_2.unwrap())(command_buffer, event, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html)
    pub unsafe fn cmd_reset_event_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk13::PipelineStageFlags2,
    ) {
        (self.cmd_reset_event_2.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html)
    pub unsafe fn cmd_wait_events_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event_count: u32,
        p_events: *const crate::vk10::Event,
        p_dependency_infos: *const crate::vk13::DependencyInfo,
    ) {
        (self
            .cmd_wait_events_2
            .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)
    pub unsafe fn cmd_pipeline_barrier_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_dependency_info: *const crate::vk13::DependencyInfo,
    ) {
        (self.cmd_pipeline_barrier_2.unwrap())(command_buffer, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSubmit2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)
    pub unsafe fn queue_submit_2(
        &self,
        queue: crate::vk10::Queue,
        submit_count: u32,
        p_submits: *const crate::vk13::SubmitInfo2,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_submit_2.unwrap())(queue, submit_count, p_submits, fence)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)
    pub unsafe fn cmd_write_timestamp_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: crate::vk13::PipelineStageFlags2,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self.cmd_write_timestamp_2.unwrap())(command_buffer, stage, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRendering")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_rendering_info: *const crate::vk13::RenderingInfo,
    ) {
        (self.cmd_begin_rendering.unwrap())(command_buffer, p_rendering_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRendering")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)
    pub unsafe fn cmd_end_rendering(&self, command_buffer: crate::vk10::CommandBuffer) {
        (self.cmd_end_rendering.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
    pub unsafe fn create_swapchain_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_swapchain: *mut crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self
            .create_swapchain_khr
            .unwrap())(device, p_create_info, p_allocator, p_swapchain)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySwapchainKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
    pub unsafe fn destroy_swapchain_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_swapchain_khr.unwrap())(device, swapchain, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
    pub unsafe fn get_swapchain_images_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut crate::vk10::Image,
    ) -> crate::vk10::Result {
        (self
            .get_swapchain_images_khr
            .unwrap())(device, swapchain, p_swapchain_image_count, p_swapchain_images)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireNextImageKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
    pub unsafe fn acquire_next_image_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        timeout: u64,
        semaphore: crate::vk10::Semaphore,
        fence: crate::vk10::Fence,
        p_image_index: *mut u32,
    ) -> crate::vk10::Result {
        (self
            .acquire_next_image_khr
            .unwrap())(device, swapchain, timeout, semaphore, fence, p_image_index)
    }
    #[track_caller]
    #[doc(alias = "vkQueuePresentKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
    pub unsafe fn queue_present_khr(
        &self,
        queue: crate::vk10::Queue,
        p_present_info: *const crate::extensions::khr_swapchain::PresentInfoKHR,
    ) -> crate::vk10::Result {
        (self.queue_present_khr.unwrap())(queue, p_present_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: crate::vk10::Device,
        p_device_group_present_capabilities: *mut crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_device_group_present_capabilities_khr
            .unwrap())(device, p_device_group_present_capabilities)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: crate::vk10::Device,
        surface: crate::extensions::khr_surface::SurfaceKHR,
        p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::vk10::Result {
        (self
            .get_device_group_surface_present_modes_khr
            .unwrap())(device, surface, p_modes)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
    pub unsafe fn acquire_next_image_2_khr(
        &self,
        device: crate::vk10::Device,
        p_acquire_info: *const crate::extensions::khr_swapchain::AcquireNextImageInfoKHR,
        p_image_index: *mut u32,
    ) -> crate::vk10::Result {
        (self.acquire_next_image_2_khr.unwrap())(device, p_acquire_info, p_image_index)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        device: crate::vk10::Device,
        swapchain_count: u32,
        p_create_infos: *const crate::extensions::khr_swapchain::SwapchainCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_swapchains: *mut crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self
            .create_shared_swapchains_khr
            .unwrap())(
            device,
            swapchain_count,
            p_create_infos,
            p_allocator,
            p_swapchains,
        )
    }
    #[track_caller]
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: crate::vk10::Device,
        p_name_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
    ) -> crate::vk10::Result {
        (self.debug_marker_set_object_name_ext.unwrap())(device, p_name_info)
    }
    #[track_caller]
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        device: crate::vk10::Device,
        p_tag_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
    ) -> crate::vk10::Result {
        (self.debug_marker_set_object_tag_ext.unwrap())(device, p_tag_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) {
        (self.cmd_debug_marker_begin_ext.unwrap())(command_buffer, p_marker_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
    pub unsafe fn cmd_debug_marker_end_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_debug_marker_end_ext.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) {
        (self.cmd_debug_marker_insert_ext.unwrap())(command_buffer, p_marker_info)
    }
    #[track_caller]
    #[doc(alias = "vkCreateVideoSessionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html)
    pub unsafe fn create_video_session_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_video_session: *mut crate::extensions::khr_video_queue::VideoSessionKHR,
    ) -> crate::vk10::Result {
        (self
            .create_video_session_khr
            .unwrap())(device, p_create_info, p_allocator, p_video_session)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyVideoSessionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html)
    pub unsafe fn destroy_video_session_khr(
        &self,
        device: crate::vk10::Device,
        video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_video_session_khr.unwrap())(device, video_session, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreateVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html)
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_video_session_parameters: *mut crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> crate::vk10::Result {
        (self
            .create_video_session_parameters_khr
            .unwrap())(device, p_create_info, p_allocator, p_video_session_parameters)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html)
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        device: crate::vk10::Device,
        video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
        p_update_info: *const crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .update_video_session_parameters_khr
            .unwrap())(device, video_session_parameters, p_update_info)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyVideoSessionParametersKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html)
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        device: crate::vk10::Device,
        video_session_parameters: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_video_session_parameters_khr
            .unwrap())(device, video_session_parameters, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetVideoSessionMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html)
    pub unsafe fn get_video_session_memory_requirements_khr(
        &self,
        device: crate::vk10::Device,
        video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut crate::extensions::khr_video_queue::VideoSessionMemoryRequirementsKHR,
    ) -> crate::vk10::Result {
        (self
            .get_video_session_memory_requirements_khr
            .unwrap())(
            device,
            video_session,
            p_memory_requirements_count,
            p_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkBindVideoSessionMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html)
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        device: crate::vk10::Device,
        video_session: crate::extensions::khr_video_queue::VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const crate::extensions::khr_video_queue::BindVideoSessionMemoryInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .bind_video_session_memory_khr
            .unwrap())(
            device,
            video_session,
            bind_session_memory_info_count,
            p_bind_session_memory_infos,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html)
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_begin_info: *const crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR,
    ) {
        (self.cmd_begin_video_coding_khr.unwrap())(command_buffer, p_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdControlVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html)
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_coding_control_info: *const crate::extensions::khr_video_queue::VideoCodingControlInfoKHR,
    ) {
        (self
            .cmd_control_video_coding_khr
            .unwrap())(command_buffer, p_coding_control_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndVideoCodingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html)
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_end_coding_info: *const crate::extensions::khr_video_queue::VideoEndCodingInfoKHR,
    ) {
        (self.cmd_end_video_coding_khr.unwrap())(command_buffer, p_end_coding_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html)
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_decode_info: *const crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR,
    ) {
        (self.cmd_decode_video_khr.unwrap())(command_buffer, p_decode_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const crate::vk10::Buffer,
        p_offsets: *const crate::vk10::DeviceSize,
        p_sizes: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_bind_transform_feedback_buffers_ext
            .unwrap())(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const crate::vk10::Buffer,
        p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_begin_transform_feedback_ext
            .unwrap())(
            command_buffer,
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers,
            p_counter_buffer_offsets,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const crate::vk10::Buffer,
        p_counter_buffer_offsets: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_end_transform_feedback_ext
            .unwrap())(
            command_buffer,
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers,
            p_counter_buffer_offsets,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        flags: crate::vk10::QueryControlFlags,
        index: u32,
    ) {
        (self
            .cmd_begin_query_indexed_ext
            .unwrap())(command_buffer, query_pool, query, flags, index)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        query_pool: crate::vk10::QueryPool,
        query: u32,
        index: u32,
    ) {
        (self
            .cmd_end_query_indexed_ext
            .unwrap())(command_buffer, query_pool, query, index)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: crate::vk10::Buffer,
        counter_buffer_offset: crate::vk10::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        (self
            .cmd_draw_indirect_byte_count_ext
            .unwrap())(
            command_buffer,
            instance_count,
            first_instance,
            counter_buffer,
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateCuModuleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html)
    pub unsafe fn create_cu_module_nvx(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_module: *mut crate::extensions::nvx_binary_import::CuModuleNVX,
    ) -> crate::vk10::Result {
        (self
            .create_cu_module_nvx
            .unwrap())(device, p_create_info, p_allocator, p_module)
    }
    #[track_caller]
    #[doc(alias = "vkCreateCuFunctionNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html)
    pub unsafe fn create_cu_function_nvx(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_function: *mut crate::extensions::nvx_binary_import::CuFunctionNVX,
    ) -> crate::vk10::Result {
        (self
            .create_cu_function_nvx
            .unwrap())(device, p_create_info, p_allocator, p_function)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyCuModuleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html)
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        device: crate::vk10::Device,
        module: crate::extensions::nvx_binary_import::CuModuleNVX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_cu_module_nvx.unwrap())(device, module, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html)
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        device: crate::vk10::Device,
        function: crate::extensions::nvx_binary_import::CuFunctionNVX,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_cu_function_nvx.unwrap())(device, function, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html)
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_launch_info: *const crate::extensions::nvx_binary_import::CuLaunchInfoNVX,
    ) {
        (self.cmd_cu_launch_kernel_nvx.unwrap())(command_buffer, p_launch_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageViewHandleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html)
    pub unsafe fn get_image_view_handle_nvx(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX,
    ) -> u32 {
        (self.get_image_view_handle_nvx.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageViewAddressNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html)
    pub unsafe fn get_image_view_address_nvx(
        &self,
        device: crate::vk10::Device,
        image_view: crate::vk10::ImageView,
        p_properties: *mut crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX,
    ) -> crate::vk10::Result {
        (self.get_image_view_address_nvx.unwrap())(device, image_view, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountAMD.html)
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indirect_count_amd
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html)
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indexed_indirect_count_amd
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetShaderInfoAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html)
    pub unsafe fn get_shader_info_amd(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        shader_stage: crate::vk10::ShaderStageFlags,
        info_type: crate::extensions::amd_shader_info::ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_shader_info_amd
            .unwrap())(device, pipeline, shader_stage, info_type, p_info_size, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html)
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_rendering_info: *const crate::vk13::RenderingInfo,
    ) {
        (self.cmd_begin_rendering_khr.unwrap())(command_buffer, p_rendering_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html)
    pub unsafe fn cmd_end_rendering_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_rendering_khr.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html)
    pub unsafe fn get_memory_win_32_handle_nv(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        handle_type: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
    ) -> crate::vk10::Result {
        (self
            .get_memory_win_32_handle_nv
            .unwrap())(device, memory, handle_type, p_handle)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        device: crate::vk10::Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut crate::vk11::PeerMemoryFeatureFlags,
    ) {
        (self
            .get_device_group_peer_memory_features_khr
            .unwrap())(
            device,
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html)
    pub unsafe fn cmd_set_device_mask_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        device_mask: u32,
    ) {
        (self.cmd_set_device_mask_khr.unwrap())(command_buffer, device_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html)
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .cmd_dispatch_base_khr
            .unwrap())(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    #[track_caller]
    #[doc(alias = "vkTrimCommandPoolKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html)
    pub unsafe fn trim_command_pool_khr(
        &self,
        device: crate::vk10::Device,
        command_pool: crate::vk10::CommandPool,
        flags: crate::vk11::CommandPoolTrimFlags,
    ) {
        (self.trim_command_pool_khr.unwrap())(device, command_pool, flags)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
    pub unsafe fn get_memory_win_32_handle_khr(
        &self,
        device: crate::vk10::Device,
        p_get_win_32_handle_info: *const crate::extensions::khr_external_memory_win32::MemoryGetWin32HandleInfoKHR,
        p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
    ) -> crate::vk10::Result {
        (self
            .get_memory_win_32_handle_khr
            .unwrap())(device, p_get_win_32_handle_info, p_handle)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
    pub unsafe fn get_memory_win_32_handle_properties_khr(
        &self,
        device: crate::vk10::Device,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        handle: crate::extensions::khr_win32_surface::HANDLE,
        p_memory_win_32_handle_properties: *mut crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_memory_win_32_handle_properties_khr
            .unwrap())(device, handle_type, handle, p_memory_win_32_handle_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html)
    pub unsafe fn get_memory_fd_khr(
        &self,
        device: crate::vk10::Device,
        p_get_fd_info: *const crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
        p_fd: *mut std::os::raw::c_int,
    ) -> crate::vk10::Result {
        (self.get_memory_fd_khr.unwrap())(device, p_get_fd_info, p_fd)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: crate::vk10::Device,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        fd: std::os::raw::c_int,
        p_memory_fd_properties: *mut crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_memory_fd_properties_khr
            .unwrap())(device, handle_type, fd, p_memory_fd_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
    pub unsafe fn get_semaphore_win_32_handle_khr(
        &self,
        device: crate::vk10::Device,
        p_get_win_32_handle_info: *const crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR,
        p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
    ) -> crate::vk10::Result {
        (self
            .get_semaphore_win_32_handle_khr
            .unwrap())(device, p_get_win_32_handle_info, p_handle)
    }
    #[track_caller]
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
    pub unsafe fn import_semaphore_win_32_handle_khr(
        &self,
        device: crate::vk10::Device,
        p_import_semaphore_win_32_handle_info: *const crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .import_semaphore_win_32_handle_khr
            .unwrap())(device, p_import_semaphore_win_32_handle_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html)
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        device: crate::vk10::Device,
        p_get_fd_info: *const crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR,
        p_fd: *mut std::os::raw::c_int,
    ) -> crate::vk10::Result {
        (self.get_semaphore_fd_khr.unwrap())(device, p_get_fd_info, p_fd)
    }
    #[track_caller]
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html)
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        device: crate::vk10::Device,
        p_import_semaphore_fd_info: *const crate::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR,
    ) -> crate::vk10::Result {
        (self.import_semaphore_fd_khr.unwrap())(device, p_import_semaphore_fd_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        layout: crate::vk10::PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const crate::vk10::WriteDescriptorSet,
    ) {
        (self
            .cmd_push_descriptor_set_khr
            .unwrap())(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_write_count,
            p_descriptor_writes,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        layout: crate::vk10::PipelineLayout,
        set: u32,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .cmd_push_descriptor_set_with_template_khr
            .unwrap())(command_buffer, descriptor_update_template, layout, set, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_conditional_rendering_begin: *const crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT,
    ) {
        (self
            .cmd_begin_conditional_rendering_ext
            .unwrap())(command_buffer, p_conditional_rendering_begin)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
    pub unsafe fn cmd_end_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_conditional_rendering_ext.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html)
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_descriptor_update_template: *mut crate::vk11::DescriptorUpdateTemplate,
    ) -> crate::vk10::Result {
        (self
            .create_descriptor_update_template_khr
            .unwrap())(device, p_create_info, p_allocator, p_descriptor_update_template)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html)
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        device: crate::vk10::Device,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_descriptor_update_template_khr
            .unwrap())(device, descriptor_update_template, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html)
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        device: crate::vk10::Device,
        descriptor_set: crate::vk10::DescriptorSet,
        descriptor_update_template: crate::vk11::DescriptorUpdateTemplate,
        p_data: *const std::os::raw::c_void,
    ) {
        (self
            .update_descriptor_set_with_template_khr
            .unwrap())(device, descriptor_set, descriptor_update_template, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html)
    pub unsafe fn cmd_set_viewport_wscaling_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_wscalings: *const crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV,
    ) {
        (self
            .cmd_set_viewport_wscaling_nv
            .unwrap())(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewport_wscalings,
        )
    }
    #[track_caller]
    #[doc(alias = "vkDisplayPowerControlEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html)
    pub unsafe fn display_power_control_ext(
        &self,
        device: crate::vk10::Device,
        display: crate::extensions::khr_display::DisplayKHR,
        p_display_power_info: *const crate::extensions::ext_display_control::DisplayPowerInfoEXT,
    ) -> crate::vk10::Result {
        (self.display_power_control_ext.unwrap())(device, display, p_display_power_info)
    }
    #[track_caller]
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html)
    pub unsafe fn register_device_event_ext(
        &self,
        device: crate::vk10::Device,
        p_device_event_info: *const crate::extensions::ext_display_control::DeviceEventInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_fence: *mut crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self
            .register_device_event_ext
            .unwrap())(device, p_device_event_info, p_allocator, p_fence)
    }
    #[track_caller]
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html)
    pub unsafe fn register_display_event_ext(
        &self,
        device: crate::vk10::Device,
        display: crate::extensions::khr_display::DisplayKHR,
        p_display_event_info: *const crate::extensions::ext_display_control::DisplayEventInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_fence: *mut crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self
            .register_display_event_ext
            .unwrap())(device, display, p_display_event_info, p_allocator, p_fence)
    }
    #[track_caller]
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html)
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        counter: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
        p_counter_value: *mut u64,
    ) -> crate::vk10::Result {
        (self
            .get_swapchain_counter_ext
            .unwrap())(device, swapchain, counter, p_counter_value)
    }
    #[track_caller]
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_display_timing_properties: *mut crate::extensions::google_display_timing::RefreshCycleDurationGOOGLE,
    ) -> crate::vk10::Result {
        (self
            .get_refresh_cycle_duration_google
            .unwrap())(device, swapchain, p_display_timing_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut crate::extensions::google_display_timing::PastPresentationTimingGOOGLE,
    ) -> crate::vk10::Result {
        (self
            .get_past_presentation_timing_google
            .unwrap())(
            device,
            swapchain,
            p_presentation_timing_count,
            p_presentation_timings,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_discard_rectangle_ext
            .unwrap())(
            command_buffer,
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles,
        )
    }
    #[track_caller]
    #[doc(alias = "vkSetHdrMetadataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html)
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        device: crate::vk10::Device,
        swapchain_count: u32,
        p_swapchains: *const crate::extensions::khr_swapchain::SwapchainKHR,
        p_metadata: *const crate::extensions::ext_hdr_metadata::HdrMetadataEXT,
    ) {
        (self
            .set_hdr_metadata_ext
            .unwrap())(device, swapchain_count, p_swapchains, p_metadata)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html)
    pub unsafe fn create_render_pass_2_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk12::RenderPassCreateInfo2,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_render_pass: *mut crate::vk10::RenderPass,
    ) -> crate::vk10::Result {
        (self
            .create_render_pass_2_khr
            .unwrap())(device, p_create_info, p_allocator, p_render_pass)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html)
    pub unsafe fn cmd_begin_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
        p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
    ) {
        (self
            .cmd_begin_render_pass_2_khr
            .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html)
    pub unsafe fn cmd_next_subpass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_begin_info: *const crate::vk12::SubpassBeginInfo,
        p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
    ) {
        (self
            .cmd_next_subpass_2_khr
            .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html)
    pub unsafe fn cmd_end_render_pass_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_subpass_end_info: *const crate::vk12::SubpassEndInfo,
    ) {
        (self.cmd_end_render_pass_2_khr.unwrap())(command_buffer, p_subpass_end_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html)
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self.get_swapchain_status_khr.unwrap())(device, swapchain)
    }
    #[track_caller]
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html)
    pub unsafe fn get_fence_win_32_handle_khr(
        &self,
        device: crate::vk10::Device,
        p_get_win_32_handle_info: *const crate::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR,
        p_handle: *mut crate::extensions::khr_win32_surface::HANDLE,
    ) -> crate::vk10::Result {
        (self
            .get_fence_win_32_handle_khr
            .unwrap())(device, p_get_win_32_handle_info, p_handle)
    }
    #[track_caller]
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html)
    pub unsafe fn import_fence_win_32_handle_khr(
        &self,
        device: crate::vk10::Device,
        p_import_fence_win_32_handle_info: *const crate::extensions::khr_external_fence_win32::ImportFenceWin32HandleInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .import_fence_win_32_handle_khr
            .unwrap())(device, p_import_fence_win_32_handle_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetFenceFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html)
    pub unsafe fn get_fence_fd_khr(
        &self,
        device: crate::vk10::Device,
        p_get_fd_info: *const crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR,
        p_fd: *mut std::os::raw::c_int,
    ) -> crate::vk10::Result {
        (self.get_fence_fd_khr.unwrap())(device, p_get_fd_info, p_fd)
    }
    #[track_caller]
    #[doc(alias = "vkImportFenceFdKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html)
    pub unsafe fn import_fence_fd_khr(
        &self,
        device: crate::vk10::Device,
        p_import_fence_fd_info: *const crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR,
    ) -> crate::vk10::Result {
        (self.import_fence_fd_khr.unwrap())(device, p_import_fence_fd_info)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html)
    pub unsafe fn acquire_profiling_lock_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR,
    ) -> crate::vk10::Result {
        (self.acquire_profiling_lock_khr.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html)
    pub unsafe fn release_profiling_lock_khr(&self, device: crate::vk10::Device) {
        (self.release_profiling_lock_khr.unwrap())(device)
    }
    #[track_caller]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: crate::vk10::Device,
        p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
    ) -> crate::vk10::Result {
        (self.set_debug_utils_object_name_ext.unwrap())(device, p_name_info)
    }
    #[track_caller]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: crate::vk10::Device,
        p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
    ) -> crate::vk10::Result {
        (self.set_debug_utils_object_tag_ext.unwrap())(device, p_tag_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.queue_begin_debug_utils_label_ext.unwrap())(queue, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk10::Queue) {
        (self.queue_end_debug_utils_label_ext.unwrap())(queue)
    }
    #[track_caller]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: crate::vk10::Queue,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.queue_insert_debug_utils_label_ext.unwrap())(queue, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.cmd_begin_debug_utils_label_ext.unwrap())(command_buffer, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_end_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_end_debug_utils_label_ext.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) {
        (self.cmd_insert_debug_utils_label_ext.unwrap())(command_buffer, p_label_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        device: crate::vk10::Device,
        buffer: *const crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
        p_properties: *mut crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID,
    ) -> crate::vk10::Result {
        (self
            .get_android_hardware_buffer_properties_android
            .unwrap())(device, buffer, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: *mut *mut crate::extensions::android_external_memory_android_hardware_buffer::AHardwareBuffer,
    ) -> crate::vk10::Result {
        (self
            .get_memory_android_hardware_buffer_android
            .unwrap())(device, p_info, p_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_sample_locations_info: *const crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) {
        (self
            .cmd_set_sample_locations_ext
            .unwrap())(command_buffer, p_sample_locations_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html)
    pub unsafe fn get_buffer_memory_requirements_2_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_buffer_memory_requirements_2_khr
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html)
    pub unsafe fn get_image_memory_requirements_2_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_image_memory_requirements_2_khr
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html)
    pub unsafe fn get_image_sparse_memory_requirements_2_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk11::ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
    ) {
        (self
            .get_image_sparse_memory_requirements_2_khr
            .unwrap())(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        device: crate::vk10::Device,
        acceleration_structure: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_acceleration_structure_khr
            .unwrap())(device, acceleration_structure, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR,
    ) {
        (self.cmd_copy_acceleration_structure_khr.unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html)
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .copy_acceleration_structure_khr
            .unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        (self
            .cmd_copy_acceleration_structure_to_memory_khr
            .unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .copy_acceleration_structure_to_memory_khr
            .unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        (self
            .cmd_copy_memory_to_acceleration_structure_khr
            .unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .copy_memory_to_acceleration_structure_khr
            .unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        (self
            .cmd_write_acceleration_structures_properties_khr
            .unwrap())(
            command_buffer,
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            query_pool,
            first_query,
        )
    }
    #[track_caller]
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
    pub unsafe fn write_acceleration_structures_properties_khr(
        &self,
        device: crate::vk10::Device,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
        query_type: crate::vk10::QueryType,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
        stride: usize,
    ) -> crate::vk10::Result {
        (self
            .write_acceleration_structures_properties_khr
            .unwrap())(
            device,
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            data_size,
            p_data,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        device: crate::vk10::Device,
        p_version_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR,
        p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
    ) {
        (self
            .get_device_acceleration_structure_compatibility_khr
            .unwrap())(device, p_version_info, p_compatibility)
    }
    #[track_caller]
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_acceleration_structure: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> crate::vk10::Result {
        (self
            .create_acceleration_structure_khr
            .unwrap())(device, p_create_info, p_allocator, p_acceleration_structure)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info_count: u32,
        p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
        pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
    ) {
        (self
            .cmd_build_acceleration_structures_khr
            .unwrap())(command_buffer, info_count, p_infos, pp_build_range_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info_count: u32,
        p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
        p_indirect_device_addresses: *const crate::vk10::DeviceAddress,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    ) {
        (self
            .cmd_build_acceleration_structures_indirect_khr
            .unwrap())(
            command_buffer,
            info_count,
            p_infos,
            p_indirect_device_addresses,
            p_indirect_strides,
            pp_max_primitive_counts,
        )
    }
    #[track_caller]
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info_count: u32,
        p_infos: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
        pp_build_range_infos: *const *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
    ) -> crate::vk10::Result {
        (self
            .build_acceleration_structures_khr
            .unwrap())(
            device,
            deferred_operation,
            info_count,
            p_infos,
            pp_build_range_infos,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR,
    ) -> crate::vk10::DeviceAddress {
        (self.get_acceleration_structure_device_address_khr.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        device: crate::vk10::Device,
        build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
        p_build_info: *const crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR,
    ) {
        (self
            .get_acceleration_structure_build_sizes_khr
            .unwrap())(
            device,
            build_type,
            p_build_info,
            p_max_primitive_counts,
            p_size_info,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self
            .cmd_trace_rays_khr
            .unwrap())(
            command_buffer,
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            width,
            height,
            depth,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_ray_tracing_shader_group_handles_khr
            .unwrap())(device, pipeline, first_group, group_count, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_ray_tracing_capture_replay_shader_group_handles_khr
            .unwrap())(device, pipeline, first_group, group_count, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
            .create_ray_tracing_pipelines_khr
            .unwrap())(
            device,
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_raygen_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const crate::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR,
        indirect_device_address: crate::vk10::DeviceAddress,
    ) {
        (self
            .cmd_trace_rays_indirect_khr
            .unwrap())(
            command_buffer,
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            indirect_device_address,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        group: u32,
        group_shader: crate::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR,
    ) -> crate::vk10::DeviceSize {
        (self
            .get_ray_tracing_shader_group_stack_size_khr
            .unwrap())(device, pipeline, group, group_shader)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        (self
            .cmd_set_ray_tracing_pipeline_stack_size_khr
            .unwrap())(command_buffer, pipeline_stack_size)
    }
    #[track_caller]
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html)
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
    ) -> crate::vk10::Result {
        (self
            .create_sampler_ycbcr_conversion_khr
            .unwrap())(device, p_create_info, p_allocator, p_ycbcr_conversion)
    }
    #[track_caller]
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html)
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        device: crate::vk10::Device,
        ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_sampler_ycbcr_conversion_khr
            .unwrap())(device, ycbcr_conversion, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkBindBufferMemory2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html)
    pub unsafe fn bind_buffer_memory_2_khr(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindBufferMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_buffer_memory_2_khr.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkBindImageMemory2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html)
    pub unsafe fn bind_image_memory_2_khr(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::vk11::BindImageMemoryInfo,
    ) -> crate::vk10::Result {
        (self.bind_image_memory_2_khr.unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_properties: *mut crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT,
    ) -> crate::vk10::Result {
        (self
            .get_image_drm_format_modifier_properties_ext
            .unwrap())(device, image, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCreateValidationCacheEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html)
    pub unsafe fn create_validation_cache_ext(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_validation_cache: *mut crate::extensions::ext_validation_cache::ValidationCacheEXT,
    ) -> crate::vk10::Result {
        (self
            .create_validation_cache_ext
            .unwrap())(device, p_create_info, p_allocator, p_validation_cache)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html)
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        device: crate::vk10::Device,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_validation_cache_ext
            .unwrap())(device, validation_cache, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html)
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: crate::vk10::Device,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_validation_cache_data_ext
            .unwrap())(device, validation_cache, p_data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkMergeValidationCachesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html)
    pub unsafe fn merge_validation_caches_ext(
        &self,
        device: crate::vk10::Device,
        dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const crate::extensions::ext_validation_cache::ValidationCacheEXT,
    ) -> crate::vk10::Result {
        (self
            .merge_validation_caches_ext
            .unwrap())(device, dst_cache, src_cache_count, p_src_caches)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html)
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image_view: crate::vk10::ImageView,
        image_layout: crate::vk10::ImageLayout,
    ) {
        (self
            .cmd_bind_shading_rate_image_nv
            .unwrap())(command_buffer, image_view, image_layout)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV,
    ) {
        (self
            .cmd_set_viewport_shading_rate_palette_nv
            .unwrap())(
            command_buffer,
            first_viewport,
            viewport_count,
            p_shading_rate_palettes,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        sample_order_type: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV,
    ) {
        (self
            .cmd_set_coarse_sample_order_nv
            .unwrap())(
            command_buffer,
            sample_order_type,
            custom_sample_order_count,
            p_custom_sample_orders,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCompileDeferredNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html)
    pub unsafe fn compile_deferred_nv(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        shader: u32,
    ) -> crate::vk10::Result {
        (self.compile_deferred_nv.unwrap())(device, pipeline, shader)
    }
    #[track_caller]
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html)
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_acceleration_structure: *mut crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    ) -> crate::vk10::Result {
        (self
            .create_acceleration_structure_nv
            .unwrap())(device, p_create_info, p_allocator, p_acceleration_structure)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html)
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        device: crate::vk10::Device,
        acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_acceleration_structure_nv
            .unwrap())(device, acceleration_structure, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: *mut crate::extensions::khr_get_memory_requirements2::MemoryRequirements2KHR,
    ) {
        (self
            .get_acceleration_structure_memory_requirements_nv
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        device: crate::vk10::Device,
        bind_info_count: u32,
        p_bind_infos: *const crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV,
    ) -> crate::vk10::Result {
        (self
            .bind_acceleration_structure_memory_nv
            .unwrap())(device, bind_info_count, p_bind_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        mode: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) {
        (self
            .cmd_copy_acceleration_structure_nv
            .unwrap())(command_buffer, dst, src, mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        (self
            .cmd_write_acceleration_structures_properties_nv
            .unwrap())(
            command_buffer,
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            query_pool,
            first_query,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV,
        instance_data: crate::vk10::Buffer,
        instance_offset: crate::vk10::DeviceSize,
        update: crate::vk10::Bool32,
        dst: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        src: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        scratch: crate::vk10::Buffer,
        scratch_offset: crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_build_acceleration_structure_nv
            .unwrap())(
            command_buffer,
            p_info,
            instance_data,
            instance_offset,
            update,
            dst,
            src,
            scratch,
            scratch_offset,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html)
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        raygen_shader_binding_table_buffer: crate::vk10::Buffer,
        raygen_shader_binding_offset: crate::vk10::DeviceSize,
        miss_shader_binding_table_buffer: crate::vk10::Buffer,
        miss_shader_binding_offset: crate::vk10::DeviceSize,
        miss_shader_binding_stride: crate::vk10::DeviceSize,
        hit_shader_binding_table_buffer: crate::vk10::Buffer,
        hit_shader_binding_offset: crate::vk10::DeviceSize,
        hit_shader_binding_stride: crate::vk10::DeviceSize,
        callable_shader_binding_table_buffer: crate::vk10::Buffer,
        callable_shader_binding_offset: crate::vk10::DeviceSize,
        callable_shader_binding_stride: crate::vk10::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self
            .cmd_trace_rays_nv
            .unwrap())(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html)
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        device: crate::vk10::Device,
        pipeline: crate::vk10::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_ray_tracing_shader_group_handles_nv
            .unwrap())(device, pipeline, first_group, group_count, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
    pub unsafe fn get_acceleration_structure_handle_nv(
        &self,
        device: crate::vk10::Device,
        acceleration_structure: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
    ) -> crate::vk10::Result {
        (self
            .get_acceleration_structure_handle_nv
            .unwrap())(device, acceleration_structure, data_size, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        device: crate::vk10::Device,
        pipeline_cache: crate::vk10::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_pipelines: *mut crate::vk10::Pipeline,
    ) -> crate::vk10::Result {
        (self
            .create_ray_tracing_pipelines_nv
            .unwrap())(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html)
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::DescriptorSetLayoutCreateInfo,
        p_support: *mut crate::vk11::DescriptorSetLayoutSupport,
    ) {
        (self
            .get_descriptor_set_layout_support_khr
            .unwrap())(device, p_create_info, p_support)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html)
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indirect_count_khr
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html)
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_indexed_indirect_count_khr
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        device: crate::vk10::Device,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const std::os::raw::c_void,
        p_memory_host_pointer_properties: *mut crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT,
    ) -> crate::vk10::Result {
        (self
            .get_memory_host_pointer_properties_ext
            .unwrap())(
            device,
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_stage: crate::vk10::PipelineStageFlags,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        marker: u32,
    ) {
        (self
            .cmd_write_buffer_marker_amd
            .unwrap())(command_buffer, pipeline_stage, dst_buffer, dst_offset, marker)
    }
    #[track_caller]
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        device: crate::vk10::Device,
        timestamp_count: u32,
        p_timestamp_infos: *const crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> crate::vk10::Result {
        (self
            .get_calibrated_timestamps_ext
            .unwrap())(
            device,
            timestamp_count,
            p_timestamp_infos,
            p_timestamps,
            p_max_deviation,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        (self.cmd_draw_mesh_tasks_nv.unwrap())(command_buffer, task_count, first_task)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_mesh_tasks_indirect_nv
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_mesh_tasks_indirect_count_nv
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_exclusive_scissor_nv
            .unwrap())(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissors,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCheckpointNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html)
    pub unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_checkpoint_marker: *const std::os::raw::c_void,
    ) {
        (self.cmd_set_checkpoint_nv.unwrap())(command_buffer, p_checkpoint_marker)
    }
    #[track_caller]
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html)
    pub unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: crate::vk10::Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV,
    ) {
        (self
            .get_queue_checkpoint_data_nv
            .unwrap())(queue, p_checkpoint_data_count, p_checkpoint_data)
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html)
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        device: crate::vk10::Device,
        semaphore: crate::vk10::Semaphore,
        p_value: *mut u64,
    ) -> crate::vk10::Result {
        (self.get_semaphore_counter_value_khr.unwrap())(device, semaphore, p_value)
    }
    #[track_caller]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html)
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: crate::vk10::Device,
        p_wait_info: *const crate::vk12::SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_semaphores_khr.unwrap())(device, p_wait_info, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html)
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: crate::vk10::Device,
        p_signal_info: *const crate::vk12::SemaphoreSignalInfo,
    ) -> crate::vk10::Result {
        (self.signal_semaphore_khr.unwrap())(device, p_signal_info)
    }
    #[track_caller]
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html)
    pub unsafe fn initialize_performance_api_intel(
        &self,
        device: crate::vk10::Device,
        p_initialize_info: *const crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL,
    ) -> crate::vk10::Result {
        (self.initialize_performance_api_intel.unwrap())(device, p_initialize_info)
    }
    #[track_caller]
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
    pub unsafe fn uninitialize_performance_api_intel(
        &self,
        device: crate::vk10::Device,
    ) {
        (self.uninitialize_performance_api_intel.unwrap())(device)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_marker_info: *const crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL,
    ) -> crate::vk10::Result {
        (self.cmd_set_performance_marker_intel.unwrap())(command_buffer, p_marker_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_marker_info: *const crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL,
    ) -> crate::vk10::Result {
        (self
            .cmd_set_performance_stream_marker_intel
            .unwrap())(command_buffer, p_marker_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_override_info: *const crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL,
    ) -> crate::vk10::Result {
        (self
            .cmd_set_performance_override_intel
            .unwrap())(command_buffer, p_override_info)
    }
    #[track_caller]
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        device: crate::vk10::Device,
        p_acquire_info: *const crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL,
        p_configuration: *mut crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::vk10::Result {
        (self
            .acquire_performance_configuration_intel
            .unwrap())(device, p_acquire_info, p_configuration)
    }
    #[track_caller]
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
    pub unsafe fn release_performance_configuration_intel(
        &self,
        device: crate::vk10::Device,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::vk10::Result {
        (self.release_performance_configuration_intel.unwrap())(device, configuration)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: crate::vk10::Queue,
        configuration: crate::extensions::intel_performance_query::PerformanceConfigurationINTEL,
    ) -> crate::vk10::Result {
        (self.queue_set_performance_configuration_intel.unwrap())(queue, configuration)
    }
    #[track_caller]
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html)
    pub unsafe fn get_performance_parameter_intel(
        &self,
        device: crate::vk10::Device,
        parameter: crate::extensions::intel_performance_query::PerformanceParameterTypeINTEL,
        p_value: *mut crate::extensions::intel_performance_query::PerformanceValueINTEL,
    ) -> crate::vk10::Result {
        (self.get_performance_parameter_intel.unwrap())(device, parameter, p_value)
    }
    #[track_caller]
    #[doc(alias = "vkSetLocalDimmingAMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html)
    pub unsafe fn set_local_dimming_amd(
        &self,
        device: crate::vk10::Device,
        swap_chain: crate::extensions::khr_swapchain::SwapchainKHR,
        local_dimming_enable: crate::vk10::Bool32,
    ) {
        (self.set_local_dimming_amd.unwrap())(device, swap_chain, local_dimming_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_fragment_size: *const crate::vk10::Extent2D,
        combiner_ops: *const [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        (self
            .cmd_set_fragment_shading_rate_khr
            .unwrap())(command_buffer, p_fragment_size, combiner_ops)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html)
    pub unsafe fn get_buffer_device_address_ext(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::BufferDeviceAddressInfo,
    ) -> crate::vk10::DeviceAddress {
        (self.get_buffer_device_address_ext.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkWaitForPresentKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html)
    pub unsafe fn wait_for_present_khr(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> crate::vk10::Result {
        (self.wait_for_present_khr.unwrap())(device, swapchain, present_id, timeout)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
    pub unsafe fn get_device_group_surface_present_modes_2_ext(
        &self,
        device: crate::vk10::Device,
        p_surface_info: *const crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR,
        p_modes: *mut crate::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR,
    ) -> crate::vk10::Result {
        (self
            .get_device_group_surface_present_modes_2_ext
            .unwrap())(device, p_surface_info, p_modes)
    }
    #[track_caller]
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self.acquire_full_screen_exclusive_mode_ext.unwrap())(device, swapchain)
    }
    #[track_caller]
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        device: crate::vk10::Device,
        swapchain: crate::extensions::khr_swapchain::SwapchainKHR,
    ) -> crate::vk10::Result {
        (self.release_full_screen_exclusive_mode_ext.unwrap())(device, swapchain)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html)
    pub unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::BufferDeviceAddressInfo,
    ) -> u64 {
        (self.get_buffer_opaque_capture_address_khr.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html)
    pub unsafe fn get_buffer_device_address_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::BufferDeviceAddressInfo,
    ) -> crate::vk10::DeviceAddress {
        (self.get_buffer_device_address_khr.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        (self.get_device_memory_opaque_capture_address_khr.unwrap())(device, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html)
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self
            .cmd_set_line_stipple_ext
            .unwrap())(command_buffer, line_stipple_factor, line_stipple_pattern)
    }
    #[track_caller]
    #[doc(alias = "vkResetQueryPoolEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPoolEXT.html)
    pub unsafe fn reset_query_pool_ext(
        &self,
        device: crate::vk10::Device,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .reset_query_pool_ext
            .unwrap())(device, query_pool, first_query, query_count)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCullModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html)
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        cull_mode: crate::vk10::CullModeFlags,
    ) {
        (self.cmd_set_cull_mode_ext.unwrap())(command_buffer, cull_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html)
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        front_face: crate::vk10::FrontFace,
    ) {
        (self.cmd_set_front_face_ext.unwrap())(command_buffer, front_face)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html)
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_topology: crate::vk10::PrimitiveTopology,
    ) {
        (self
            .cmd_set_primitive_topology_ext
            .unwrap())(command_buffer, primitive_topology)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html)
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewport_count: u32,
        p_viewports: *const crate::vk10::Viewport,
    ) {
        (self
            .cmd_set_viewport_with_count_ext
            .unwrap())(command_buffer, viewport_count, p_viewports)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html)
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        scissor_count: u32,
        p_scissors: *const crate::vk10::Rect2D,
    ) {
        (self
            .cmd_set_scissor_with_count_ext
            .unwrap())(command_buffer, scissor_count, p_scissors)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
    pub unsafe fn cmd_bind_vertex_buffers_2_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const crate::vk10::Buffer,
        p_offsets: *const crate::vk10::DeviceSize,
        p_sizes: *const crate::vk10::DeviceSize,
        p_strides: *const crate::vk10::DeviceSize,
    ) {
        (self
            .cmd_bind_vertex_buffers_2_ext
            .unwrap())(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
            p_strides,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html)
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_test_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_test_enable_ext.unwrap())(command_buffer, depth_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html)
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_write_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_depth_write_enable_ext
            .unwrap())(command_buffer, depth_write_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html)
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_compare_op: crate::vk10::CompareOp,
    ) {
        (self.cmd_set_depth_compare_op_ext.unwrap())(command_buffer, depth_compare_op)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html)
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bounds_test_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_depth_bounds_test_enable_ext
            .unwrap())(command_buffer, depth_bounds_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html)
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stencil_test_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_stencil_test_enable_ext
            .unwrap())(command_buffer, stencil_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html)
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        fail_op: crate::vk10::StencilOp,
        pass_op: crate::vk10::StencilOp,
        depth_fail_op: crate::vk10::StencilOp,
        compare_op: crate::vk10::CompareOp,
    ) {
        (self
            .cmd_set_stencil_op_ext
            .unwrap())(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html)
    pub unsafe fn create_deferred_operation_khr(
        &self,
        device: crate::vk10::Device,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_deferred_operation: *mut crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::vk10::Result {
        (self
            .create_deferred_operation_khr
            .unwrap())(device, p_allocator, p_deferred_operation)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html)
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        device: crate::vk10::Device,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_deferred_operation_khr.unwrap())(device, operation, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        device: crate::vk10::Device,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> u32 {
        (self.get_deferred_operation_max_concurrency_khr.unwrap())(device, operation)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html)
    pub unsafe fn get_deferred_operation_result_khr(
        &self,
        device: crate::vk10::Device,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::vk10::Result {
        (self.get_deferred_operation_result_khr.unwrap())(device, operation)
    }
    #[track_caller]
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html)
    pub unsafe fn deferred_operation_join_khr(
        &self,
        device: crate::vk10::Device,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::vk10::Result {
        (self.deferred_operation_join_khr.unwrap())(device, operation)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
    pub unsafe fn get_pipeline_executable_properties_khr(
        &self,
        device: crate::vk10::Device,
        p_pipeline_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR,
        p_executable_count: *mut u32,
        p_properties: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_executable_properties_khr
            .unwrap())(device, p_pipeline_info, p_executable_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
    pub unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        device: crate::vk10::Device,
        p_executable_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR,
        p_statistic_count: *mut u32,
        p_statistics: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_executable_statistics_khr
            .unwrap())(device, p_executable_info, p_statistic_count, p_statistics)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
    pub unsafe fn get_pipeline_executable_internal_representations_khr(
        &self,
        device: crate::vk10::Device,
        p_executable_info: *const crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_executable_internal_representations_khr
            .unwrap())(
            device,
            p_executable_info,
            p_internal_representation_count,
            p_internal_representations,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        is_preprocessed: crate::vk10::Bool32,
        p_generated_commands_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV,
    ) {
        (self
            .cmd_execute_generated_commands_nv
            .unwrap())(command_buffer, is_preprocessed, p_generated_commands_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_generated_commands_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV,
    ) {
        (self
            .cmd_preprocess_generated_commands_nv
            .unwrap())(command_buffer, p_generated_commands_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        pipeline_bind_point: crate::vk10::PipelineBindPoint,
        pipeline: crate::vk10::Pipeline,
        group_index: u32,
    ) {
        (self
            .cmd_bind_pipeline_shader_group_nv
            .unwrap())(command_buffer, pipeline_bind_point, pipeline, group_index)
    }
    #[track_caller]
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_generated_commands_memory_requirements_nv
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_indirect_commands_layout: *mut crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    ) -> crate::vk10::Result {
        (self
            .create_indirect_commands_layout_nv
            .unwrap())(device, p_create_info, p_allocator, p_indirect_commands_layout)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        device: crate::vk10::Device,
        indirect_commands_layout: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_indirect_commands_layout_nv
            .unwrap())(device, indirect_commands_layout, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlotEXT.html)
    pub unsafe fn create_private_data_slot_ext(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk13::PrivateDataSlotCreateInfo,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_private_data_slot: *mut crate::vk13::PrivateDataSlot,
    ) -> crate::vk10::Result {
        (self
            .create_private_data_slot_ext
            .unwrap())(device, p_create_info, p_allocator, p_private_data_slot)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlotEXT.html)
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        device: crate::vk10::Device,
        private_data_slot: crate::vk13::PrivateDataSlot,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_private_data_slot_ext
            .unwrap())(device, private_data_slot, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkSetPrivateDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateDataEXT.html)
    pub unsafe fn set_private_data_ext(
        &self,
        device: crate::vk10::Device,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
        data: u64,
    ) -> crate::vk10::Result {
        (self
            .set_private_data_ext
            .unwrap())(device, object_type, object_handle, private_data_slot, data)
    }
    #[track_caller]
    #[doc(alias = "vkGetPrivateDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateDataEXT.html)
    pub unsafe fn get_private_data_ext(
        &self,
        device: crate::vk10::Device,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: crate::vk13::PrivateDataSlot,
        p_data: *mut u64,
    ) {
        (self
            .get_private_data_ext
            .unwrap())(device, object_type, object_handle, private_data_slot, p_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html)
    pub unsafe fn cmd_encode_video_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_encode_info: *const crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR,
    ) {
        (self.cmd_encode_video_khr.unwrap())(command_buffer, p_encode_info)
    }
    #[track_caller]
    #[doc(alias = "vkExportMetalObjectsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html)
    pub unsafe fn export_metal_objects_ext(
        &self,
        device: crate::vk10::Device,
        p_metal_objects_info: *mut crate::extensions::ext_metal_objects::ExportMetalObjectsInfoEXT,
    ) {
        (self.export_metal_objects_ext.unwrap())(device, p_metal_objects_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html)
    pub unsafe fn cmd_set_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        p_dependency_info: *const crate::vk13::DependencyInfo,
    ) {
        (self.cmd_set_event_2_khr.unwrap())(command_buffer, event, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html)
    pub unsafe fn cmd_reset_event_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: crate::vk13::PipelineStageFlags2,
    ) {
        (self.cmd_reset_event_2_khr.unwrap())(command_buffer, event, stage_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html)
    pub unsafe fn cmd_wait_events_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event_count: u32,
        p_events: *const crate::vk10::Event,
        p_dependency_infos: *const crate::vk13::DependencyInfo,
    ) {
        (self
            .cmd_wait_events_2_khr
            .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
    }
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html)
    pub unsafe fn cmd_pipeline_barrier_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_dependency_info: *const crate::vk13::DependencyInfo,
    ) {
        (self.cmd_pipeline_barrier_2_khr.unwrap())(command_buffer, p_dependency_info)
    }
    #[track_caller]
    #[doc(alias = "vkQueueSubmit2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html)
    pub unsafe fn queue_submit_2_khr(
        &self,
        queue: crate::vk10::Queue,
        submit_count: u32,
        p_submits: *const crate::vk13::SubmitInfo2,
        fence: crate::vk10::Fence,
    ) -> crate::vk10::Result {
        (self.queue_submit_2_khr.unwrap())(queue, submit_count, p_submits, fence)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html)
    pub unsafe fn cmd_write_timestamp_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: crate::vk13::PipelineStageFlags2,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        (self
            .cmd_write_timestamp_2_khr
            .unwrap())(command_buffer, stage, query_pool, query)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html)
    pub unsafe fn cmd_write_buffer_marker_2_amd(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: crate::vk13::PipelineStageFlags2,
        dst_buffer: crate::vk10::Buffer,
        dst_offset: crate::vk10::DeviceSize,
        marker: u32,
    ) {
        (self
            .cmd_write_buffer_marker_2_amd
            .unwrap())(command_buffer, stage, dst_buffer, dst_offset, marker)
    }
    #[track_caller]
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html)
    pub unsafe fn get_queue_checkpoint_data_2_nv(
        &self,
        queue: crate::vk10::Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut crate::extensions::khr_synchronization2::CheckpointData2NV,
    ) {
        (self
            .get_queue_checkpoint_data_2_nv
            .unwrap())(queue, p_checkpoint_data_count, p_checkpoint_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        shading_rate: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
        combiner_ops: *const [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        (self
            .cmd_set_fragment_shading_rate_enum_nv
            .unwrap())(command_buffer, shading_rate, combiner_ops)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .cmd_draw_mesh_tasks_ext
            .unwrap())(command_buffer, group_count_x, group_count_y, group_count_z)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_mesh_tasks_indirect_ext
            .unwrap())(command_buffer, buffer, offset, draw_count, stride)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html)
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        buffer: crate::vk10::Buffer,
        offset: crate::vk10::DeviceSize,
        count_buffer: crate::vk10::Buffer,
        count_buffer_offset: crate::vk10::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_mesh_tasks_indirect_count_ext
            .unwrap())(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html)
    pub unsafe fn cmd_copy_buffer_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_buffer_info: *const crate::vk13::CopyBufferInfo2,
    ) {
        (self.cmd_copy_buffer_2_khr.unwrap())(command_buffer, p_copy_buffer_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html)
    pub unsafe fn cmd_copy_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_image_info: *const crate::vk13::CopyImageInfo2,
    ) {
        (self.cmd_copy_image_2_khr.unwrap())(command_buffer, p_copy_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html)
    pub unsafe fn cmd_blit_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_blit_image_info: *const crate::vk13::BlitImageInfo2,
    ) {
        (self.cmd_blit_image_2_khr.unwrap())(command_buffer, p_blit_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html)
    pub unsafe fn cmd_copy_buffer_to_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_buffer_to_image_info: *const crate::vk13::CopyBufferToImageInfo2,
    ) {
        (self
            .cmd_copy_buffer_to_image_2_khr
            .unwrap())(command_buffer, p_copy_buffer_to_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html)
    pub unsafe fn cmd_copy_image_to_buffer_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_copy_image_to_buffer_info: *const crate::vk13::CopyImageToBufferInfo2,
    ) {
        (self
            .cmd_copy_image_to_buffer_2_khr
            .unwrap())(command_buffer, p_copy_image_to_buffer_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html)
    pub unsafe fn cmd_resolve_image_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_resolve_image_info: *const crate::vk13::ResolveImageInfo2,
    ) {
        (self.cmd_resolve_image_2_khr.unwrap())(command_buffer, p_resolve_image_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html)
    pub unsafe fn get_image_subresource_layout_2_ext(
        &self,
        device: crate::vk10::Device,
        image: crate::vk10::Image,
        p_subresource: *const crate::extensions::ext_image_compression_control::ImageSubresource2EXT,
        p_layout: *mut crate::extensions::ext_image_compression_control::SubresourceLayout2EXT,
    ) {
        (self
            .get_image_subresource_layout_2_ext
            .unwrap())(device, image, p_subresource, p_layout)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceFaultInfoEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html)
    pub unsafe fn get_device_fault_info_ext(
        &self,
        device: crate::vk10::Device,
        p_fault_counts: *mut crate::extensions::ext_device_fault::DeviceFaultCountsEXT,
        p_fault_info: *mut crate::extensions::ext_device_fault::DeviceFaultInfoEXT,
    ) -> crate::vk10::Result {
        (self.get_device_fault_info_ext.unwrap())(device, p_fault_counts, p_fault_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html)
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT,
    ) {
        (self
            .cmd_set_vertex_input_ext
            .unwrap())(
            command_buffer,
            vertex_binding_description_count,
            p_vertex_binding_descriptions,
            vertex_attribute_description_count,
            p_vertex_attribute_descriptions,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html)
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        device: crate::vk10::Device,
        p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
    ) -> crate::vk10::Result {
        (self
            .get_memory_zircon_handle_fuchsia
            .unwrap())(device, p_get_zircon_handle_info, p_zircon_handle)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        device: crate::vk10::Device,
        handle_type: crate::vk11::ExternalMemoryHandleTypeFlags,
        zircon_handle: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
        p_memory_zircon_handle_properties: *mut crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .get_memory_zircon_handle_properties_fuchsia
            .unwrap())(
            device,
            handle_type,
            zircon_handle,
            p_memory_zircon_handle_properties,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html)
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        device: crate::vk10::Device,
        p_get_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA,
        p_zircon_handle: *mut crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
    ) -> crate::vk10::Result {
        (self
            .get_semaphore_zircon_handle_fuchsia
            .unwrap())(device, p_get_zircon_handle_info, p_zircon_handle)
    }
    #[track_caller]
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html)
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        device: crate::vk10::Device,
        p_import_semaphore_zircon_handle_info: *const crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .import_semaphore_zircon_handle_fuchsia
            .unwrap())(device, p_import_semaphore_zircon_handle_info)
    }
    #[track_caller]
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html)
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_collection: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .create_buffer_collection_fuchsia
            .unwrap())(device, p_create_info, p_allocator, p_collection)
    }
    #[track_caller]
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        device: crate::vk10::Device,
        collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .set_buffer_collection_buffer_constraints_fuchsia
            .unwrap())(device, collection, p_buffer_constraints_info)
    }
    #[track_caller]
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        device: crate::vk10::Device,
        collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
        p_image_constraints_info: *const crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .set_buffer_collection_image_constraints_fuchsia
            .unwrap())(device, collection, p_image_constraints_info)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html)
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        device: crate::vk10::Device,
        collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self
            .destroy_buffer_collection_fuchsia
            .unwrap())(device, collection, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html)
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        &self,
        device: crate::vk10::Device,
        collection: crate::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA,
        p_properties: *mut crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA,
    ) -> crate::vk10::Result {
        (self
            .get_buffer_collection_properties_fuchsia
            .unwrap())(device, collection, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        device: crate::vk10::Device,
        renderpass: crate::vk10::RenderPass,
        p_max_workgroup_size: *mut crate::vk10::Extent2D,
    ) -> crate::vk10::Result {
        (self
            .get_device_subpass_shading_max_workgroup_size_huawei
            .unwrap())(device, renderpass, p_max_workgroup_size)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html)
    pub unsafe fn cmd_subpass_shading_huawei(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        (self.cmd_subpass_shading_huawei.unwrap())(command_buffer)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html)
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image_view: crate::vk10::ImageView,
        image_layout: crate::vk10::ImageLayout,
    ) {
        (self
            .cmd_bind_invocation_mask_huawei
            .unwrap())(command_buffer, image_view, image_layout)
    }
    #[track_caller]
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html)
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        device: crate::vk10::Device,
        p_memory_get_remote_address_info: *const crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV,
        p_address: *mut crate::extensions::nv_external_memory_rdma::RemoteAddressNV,
    ) -> crate::vk10::Result {
        (self
            .get_memory_remote_address_nv
            .unwrap())(device, p_memory_get_remote_address_info, p_address)
    }
    #[track_caller]
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html)
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        device: crate::vk10::Device,
        p_pipeline_info: *const crate::extensions::ext_pipeline_properties::PipelineInfoEXT,
        p_pipeline_properties: *mut crate::vk10::BaseOutStructure,
    ) -> crate::vk10::Result {
        (self
            .get_pipeline_properties_ext
            .unwrap())(device, p_pipeline_info, p_pipeline_properties)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html)
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        patch_control_points: u32,
    ) {
        (self
            .cmd_set_patch_control_points_ext
            .unwrap())(command_buffer, patch_control_points)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html)
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterizer_discard_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_rasterizer_discard_enable_ext
            .unwrap())(command_buffer, rasterizer_discard_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html)
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_bias_enable_ext.unwrap())(command_buffer, depth_bias_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html)
    pub unsafe fn cmd_set_logic_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        logic_op: crate::vk10::LogicOp,
    ) {
        (self.cmd_set_logic_op_ext.unwrap())(command_buffer, logic_op)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html)
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_restart_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_primitive_restart_enable_ext
            .unwrap())(command_buffer, primitive_restart_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html)
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        attachment_count: u32,
        p_color_write_enables: *const crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_color_write_enable_ext
            .unwrap())(command_buffer, attachment_count, p_color_write_enables)
    }
    #[track_caller]
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html)
    pub unsafe fn cmd_trace_rays_indirect_2_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        indirect_device_address: crate::vk10::DeviceAddress,
    ) {
        (self
            .cmd_trace_rays_indirect_2_khr
            .unwrap())(command_buffer, indirect_device_address)
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMultiEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html)
    pub unsafe fn cmd_draw_multi_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const crate::extensions::ext_multi_draw::MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        (self
            .cmd_draw_multi_ext
            .unwrap())(
            command_buffer,
            draw_count,
            p_vertex_info,
            instance_count,
            first_instance,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html)
    pub unsafe fn cmd_draw_multi_indexed_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        draw_count: u32,
        p_index_info: *const crate::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ) {
        (self
            .cmd_draw_multi_indexed_ext
            .unwrap())(
            command_buffer,
            draw_count,
            p_index_info,
            instance_count,
            first_instance,
            stride,
            p_vertex_offset,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCreateMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html)
    pub unsafe fn create_micromap_ext(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::ext_opacity_micromap::MicromapCreateInfoEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_micromap: *mut crate::extensions::ext_opacity_micromap::MicromapEXT,
    ) -> crate::vk10::Result {
        (self
            .create_micromap_ext
            .unwrap())(device, p_create_info, p_allocator, p_micromap)
    }
    #[track_caller]
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildMicromapsEXT.html)
    pub unsafe fn cmd_build_micromaps_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info_count: u32,
        p_infos: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
    ) {
        (self.cmd_build_micromaps_ext.unwrap())(command_buffer, info_count, p_infos)
    }
    #[track_caller]
    #[doc(alias = "vkBuildMicromapsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildMicromapsEXT.html)
    pub unsafe fn build_micromaps_ext(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info_count: u32,
        p_infos: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
    ) -> crate::vk10::Result {
        (self
            .build_micromaps_ext
            .unwrap())(device, deferred_operation, info_count, p_infos)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html)
    pub unsafe fn destroy_micromap_ext(
        &self,
        device: crate::vk10::Device,
        micromap: crate::extensions::ext_opacity_micromap::MicromapEXT,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_micromap_ext.unwrap())(device, micromap, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html)
    pub unsafe fn cmd_copy_micromap_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT,
    ) {
        (self.cmd_copy_micromap_ext.unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html)
    pub unsafe fn copy_micromap_ext(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT,
    ) -> crate::vk10::Result {
        (self.copy_micromap_ext.unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapToMemoryEXT.html)
    pub unsafe fn cmd_copy_micromap_to_memory_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT,
    ) {
        (self.cmd_copy_micromap_to_memory_ext.unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapToMemoryEXT.html)
    pub unsafe fn copy_micromap_to_memory_ext(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT,
    ) -> crate::vk10::Result {
        (self.copy_micromap_to_memory_ext.unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToMicromapEXT.html)
    pub unsafe fn cmd_copy_memory_to_micromap_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT,
    ) {
        (self.cmd_copy_memory_to_micromap_ext.unwrap())(command_buffer, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToMicromapEXT.html)
    pub unsafe fn copy_memory_to_micromap_ext(
        &self,
        device: crate::vk10::Device,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        p_info: *const crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT,
    ) -> crate::vk10::Result {
        (self.copy_memory_to_micromap_ext.unwrap())(device, deferred_operation, p_info)
    }
    #[track_caller]
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html)
    pub unsafe fn cmd_write_micromaps_properties_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        micromap_count: u32,
        p_micromaps: *const crate::extensions::ext_opacity_micromap::MicromapEXT,
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        (self
            .cmd_write_micromaps_properties_ext
            .unwrap())(
            command_buffer,
            micromap_count,
            p_micromaps,
            query_type,
            query_pool,
            first_query,
        )
    }
    #[track_caller]
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html)
    pub unsafe fn write_micromaps_properties_ext(
        &self,
        device: crate::vk10::Device,
        micromap_count: u32,
        p_micromaps: *const crate::extensions::ext_opacity_micromap::MicromapEXT,
        query_type: crate::vk10::QueryType,
        data_size: usize,
        p_data: *mut std::os::raw::c_void,
        stride: usize,
    ) -> crate::vk10::Result {
        (self
            .write_micromaps_properties_ext
            .unwrap())(
            device,
            micromap_count,
            p_micromaps,
            query_type,
            data_size,
            p_data,
            stride,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html)
    pub unsafe fn get_device_micromap_compatibility_ext(
        &self,
        device: crate::vk10::Device,
        p_version_info: *const crate::extensions::ext_opacity_micromap::MicromapVersionInfoEXT,
        p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
    ) {
        (self
            .get_device_micromap_compatibility_ext
            .unwrap())(device, p_version_info, p_compatibility)
    }
    #[track_caller]
    #[doc(alias = "vkGetMicromapBuildSizesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMicromapBuildSizesEXT.html)
    pub unsafe fn get_micromap_build_sizes_ext(
        &self,
        device: crate::vk10::Device,
        build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
        p_build_info: *const crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT,
        p_size_info: *mut crate::extensions::ext_opacity_micromap::MicromapBuildSizesInfoEXT,
    ) {
        (self
            .get_micromap_build_sizes_ext
            .unwrap())(device, build_type, p_build_info, p_size_info)
    }
    #[track_caller]
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html)
    pub unsafe fn set_device_memory_priority_ext(
        &self,
        device: crate::vk10::Device,
        memory: crate::vk10::DeviceMemory,
        priority: std::os::raw::c_float,
    ) {
        (self.set_device_memory_priority_ext.unwrap())(device, memory, priority)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html)
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceBufferMemoryRequirements,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_device_buffer_memory_requirements_khr
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html)
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceImageMemoryRequirements,
        p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
    ) {
        (self
            .get_device_image_memory_requirements_khr
            .unwrap())(device, p_info, p_memory_requirements)
    }
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(
        &self,
        device: crate::vk10::Device,
        p_info: *const crate::vk13::DeviceImageMemoryRequirements,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
    ) {
        (self
            .get_device_image_sparse_memory_requirements_khr
            .unwrap())(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        device: crate::vk10::Device,
        p_binding_reference: *const crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetBindingReferenceVALVE,
        p_host_mapping: *mut crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetLayoutHostMappingInfoVALVE,
    ) {
        (self
            .get_descriptor_set_layout_host_mapping_info_valve
            .unwrap())(device, p_binding_reference, p_host_mapping)
    }
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html)
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        &self,
        device: crate::vk10::Device,
        descriptor_set: crate::vk10::DescriptorSet,
        pp_data: *mut *mut std::os::raw::c_void,
    ) {
        (self
            .get_descriptor_set_host_mapping_valve
            .unwrap())(device, descriptor_set, pp_data)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html)
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        domain_origin: crate::vk11::TessellationDomainOrigin,
    ) {
        (self
            .cmd_set_tessellation_domain_origin_ext
            .unwrap())(command_buffer, domain_origin)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html)
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_clamp_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_depth_clamp_enable_ext
            .unwrap())(command_buffer, depth_clamp_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html)
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        polygon_mode: crate::vk10::PolygonMode,
    ) {
        (self.cmd_set_polygon_mode_ext.unwrap())(command_buffer, polygon_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterization_samples: crate::vk10::SampleCountFlags,
    ) {
        (self
            .cmd_set_rasterization_samples_ext
            .unwrap())(command_buffer, rasterization_samples)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html)
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        samples: crate::vk10::SampleCountFlags,
        p_sample_mask: *const crate::vk10::SampleMask,
    ) {
        (self.cmd_set_sample_mask_ext.unwrap())(command_buffer, samples, p_sample_mask)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html)
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        alpha_to_coverage_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_alpha_to_coverage_enable_ext
            .unwrap())(command_buffer, alpha_to_coverage_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html)
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        alpha_to_one_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_alpha_to_one_enable_ext
            .unwrap())(command_buffer, alpha_to_one_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html)
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        logic_op_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_logic_op_enable_ext.unwrap())(command_buffer, logic_op_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)
    pub unsafe fn cmd_set_color_blend_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_color_blend_enable_ext
            .unwrap())(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_enables,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)
    pub unsafe fn cmd_set_color_blend_equation_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const crate::extensions::ext_extended_dynamic_state3::ColorBlendEquationEXT,
    ) {
        (self
            .cmd_set_color_blend_equation_ext
            .unwrap())(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_equations,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)
    pub unsafe fn cmd_set_color_write_mask_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const crate::vk10::ColorComponentFlags,
    ) {
        (self
            .cmd_set_color_write_mask_ext
            .unwrap())(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_write_masks,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html)
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterization_stream: u32,
    ) {
        (self
            .cmd_set_rasterization_stream_ext
            .unwrap())(command_buffer, rasterization_stream)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html)
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        conservative_rasterization_mode: crate::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT,
    ) {
        (self
            .cmd_set_conservative_rasterization_mode_ext
            .unwrap())(command_buffer, conservative_rasterization_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        extra_primitive_overestimation_size: std::os::raw::c_float,
    ) {
        (self
            .cmd_set_extra_primitive_overestimation_size_ext
            .unwrap())(command_buffer, extra_primitive_overestimation_size)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html)
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_clip_enable: crate::vk10::Bool32,
    ) {
        (self.cmd_set_depth_clip_enable_ext.unwrap())(command_buffer, depth_clip_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html)
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        sample_locations_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_sample_locations_enable_ext
            .unwrap())(command_buffer, sample_locations_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html)
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const crate::extensions::ext_extended_dynamic_state3::ColorBlendAdvancedEXT,
    ) {
        (self
            .cmd_set_color_blend_advanced_ext
            .unwrap())(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_advanced,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html)
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
    ) {
        (self
            .cmd_set_provoking_vertex_mode_ext
            .unwrap())(command_buffer, provoking_vertex_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html)
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
    ) {
        (self
            .cmd_set_line_rasterization_mode_ext
            .unwrap())(command_buffer, line_rasterization_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html)
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stippled_line_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_line_stipple_enable_ext
            .unwrap())(command_buffer, stippled_line_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html)
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        negative_one_to_one: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_depth_clip_negative_one_to_one_ext
            .unwrap())(command_buffer, negative_one_to_one)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html)
    pub unsafe fn cmd_set_viewport_wscaling_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewport_wscaling_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_viewport_wscaling_enable_nv
            .unwrap())(command_buffer, viewport_wscaling_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html)
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV,
    ) {
        (self
            .cmd_set_viewport_swizzle_nv
            .unwrap())(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewport_swizzles,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html)
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_to_color_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_coverage_to_color_enable_nv
            .unwrap())(command_buffer, coverage_to_color_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html)
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        (self
            .cmd_set_coverage_to_color_location_nv
            .unwrap())(command_buffer, coverage_to_color_location)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html)
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
    ) {
        (self
            .cmd_set_coverage_modulation_mode_nv
            .unwrap())(command_buffer, coverage_modulation_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html)
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_table_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_coverage_modulation_table_enable_nv
            .unwrap())(command_buffer, coverage_modulation_table_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html)
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const std::os::raw::c_float,
    ) {
        (self
            .cmd_set_coverage_modulation_table_nv
            .unwrap())(
            command_buffer,
            coverage_modulation_table_count,
            p_coverage_modulation_table,
        )
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html)
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        shading_rate_image_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_shading_rate_image_enable_nv
            .unwrap())(command_buffer, shading_rate_image_enable)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html)
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    ) {
        (self
            .cmd_set_coverage_reduction_mode_nv
            .unwrap())(command_buffer, coverage_reduction_mode)
    }
    #[track_caller]
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html)
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        representative_fragment_test_enable: crate::vk10::Bool32,
    ) {
        (self
            .cmd_set_representative_fragment_test_enable_nv
            .unwrap())(command_buffer, representative_fragment_test_enable)
    }
    #[track_caller]
    #[doc(alias = "vkGetShaderModuleIdentifierEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html)
    pub unsafe fn get_shader_module_identifier_ext(
        &self,
        device: crate::vk10::Device,
        shader_module: crate::vk10::ShaderModule,
        p_identifier: *mut crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT,
    ) {
        (self
            .get_shader_module_identifier_ext
            .unwrap())(device, shader_module, p_identifier)
    }
    #[track_caller]
    #[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html)
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
        p_identifier: *mut crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT,
    ) {
        (self
            .get_shader_module_create_info_identifier_ext
            .unwrap())(device, p_create_info, p_identifier)
    }
    #[track_caller]
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html)
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        device: crate::vk10::Device,
        p_create_info: *const crate::extensions::nv_optical_flow::OpticalFlowSessionCreateInfoNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
        p_session: *mut crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
    ) -> crate::vk10::Result {
        (self
            .create_optical_flow_session_nv
            .unwrap())(device, p_create_info, p_allocator, p_session)
    }
    #[track_caller]
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html)
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        device: crate::vk10::Device,
        session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
        p_allocator: *const crate::vk10::AllocationCallbacks,
    ) {
        (self.destroy_optical_flow_session_nv.unwrap())(device, session, p_allocator)
    }
    #[track_caller]
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html)
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        device: crate::vk10::Device,
        session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
        binding_point: crate::extensions::nv_optical_flow::OpticalFlowSessionBindingPointNV,
        view: crate::vk10::ImageView,
        layout: crate::vk10::ImageLayout,
    ) -> crate::vk10::Result {
        (self
            .bind_optical_flow_session_image_nv
            .unwrap())(device, session, binding_point, view, layout)
    }
    #[track_caller]
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html)
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        session: crate::extensions::nv_optical_flow::OpticalFlowSessionNV,
        p_execute_info: *const crate::extensions::nv_optical_flow::OpticalFlowExecuteInfoNV,
    ) {
        (self
            .cmd_optical_flow_execute_nv
            .unwrap())(command_buffer, session, p_execute_info)
    }
    #[track_caller]
    #[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html)
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        &self,
        device: crate::vk10::Device,
        framebuffer: crate::vk10::Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut crate::extensions::qcom_tile_properties::TilePropertiesQCOM,
    ) -> crate::vk10::Result {
        (self
            .get_framebuffer_tile_properties_qcom
            .unwrap())(device, framebuffer, p_properties_count, p_properties)
    }
    #[track_caller]
    #[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html)
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        device: crate::vk10::Device,
        p_rendering_info: *const crate::vk13::RenderingInfo,
        p_properties: *mut crate::extensions::qcom_tile_properties::TilePropertiesQCOM,
    ) -> crate::vk10::Result {
        (self
            .get_dynamic_rendering_tile_properties_qcom
            .unwrap())(device, p_rendering_info, p_properties)
    }
}
impl Default for DeviceTable {
    fn default() -> Self {
        Self::new_empty()
    }
}
