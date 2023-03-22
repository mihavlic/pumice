crate::dispatchable_handle!(
    OpticalFlowSessionNV, OPTICAL_FLOW_SESSION_NV, "VkOpticalFlowSessionNV",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionNV.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceOpticalFlowFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowFeaturesNV.html)
pub struct PhysicalDeviceOpticalFlowFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub optical_flow: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceOpticalFlowFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            optical_flow: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceOpticalFlowPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowPropertiesNV.html)
pub struct PhysicalDeviceOpticalFlowPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub hint_supported: crate::vk10::Bool32,
    pub cost_supported: crate::vk10::Bool32,
    pub bidirectional_flow_supported: crate::vk10::Bool32,
    pub global_flow_supported: crate::vk10::Bool32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_num_regions_of_interest: u32,
}
impl Default for PhysicalDeviceOpticalFlowPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            supported_output_grid_sizes: Default::default(),
            supported_hint_grid_sizes: Default::default(),
            hint_supported: Default::default(),
            cost_supported: Default::default(),
            bidirectional_flow_supported: Default::default(),
            global_flow_supported: Default::default(),
            min_width: Default::default(),
            min_height: Default::default(),
            max_width: Default::default(),
            max_height: Default::default(),
            max_num_regions_of_interest: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkOpticalFlowImageFormatInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatInfoNV.html)
pub struct OpticalFlowImageFormatInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub usage: OpticalFlowUsageFlagsNV,
}
impl Default for OpticalFlowImageFormatInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV,
            p_next: std::ptr::null(),
            usage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkOpticalFlowImageFormatPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatPropertiesNV.html)
pub struct OpticalFlowImageFormatPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub format: crate::vk10::Format,
}
impl Default for OpticalFlowImageFormatPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
            p_next: std::ptr::null(),
            format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkOpticalFlowSessionCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateInfoNV.html)
pub struct OpticalFlowSessionCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub width: u32,
    pub height: u32,
    pub image_format: crate::vk10::Format,
    pub flow_vector_format: crate::vk10::Format,
    pub cost_format: crate::vk10::Format,
    pub output_grid_size: OpticalFlowGridSizeFlagsNV,
    pub hint_grid_size: OpticalFlowGridSizeFlagsNV,
    pub performance_level: OpticalFlowPerformanceLevelNV,
    pub flags: OpticalFlowSessionCreateFlagsNV,
}
impl Default for OpticalFlowSessionCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::OPTICAL_FLOW_SESSION_CREATE_INFO_NV,
            p_next: std::ptr::null_mut(),
            width: Default::default(),
            height: Default::default(),
            image_format: Default::default(),
            flow_vector_format: Default::default(),
            cost_format: Default::default(),
            output_grid_size: Default::default(),
            hint_grid_size: Default::default(),
            performance_level: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkOpticalFlowSessionCreatePrivateDataInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreatePrivateDataInfoNV.html)
pub struct OpticalFlowSessionCreatePrivateDataInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub id: u32,
    pub size: u32,
    pub p_private_data: *const std::os::raw::c_void,
}
impl Default for OpticalFlowSessionCreatePrivateDataInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
            p_next: std::ptr::null_mut(),
            id: Default::default(),
            size: Default::default(),
            p_private_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkOpticalFlowExecuteInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteInfoNV.html)
pub struct OpticalFlowExecuteInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub flags: OpticalFlowExecuteFlagsNV,
    pub region_count: u32,
    pub p_regions: *const crate::vk10::Rect2D,
}
impl Default for OpticalFlowExecuteInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::OPTICAL_FLOW_EXECUTE_INFO_NV,
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowGridSizeFlagsNV(pub u32);
impl OpticalFlowGridSizeFlagsNV {
    pub const UNKNOWN: Self = Self(0);
    pub const S1X1: Self = Self(1 << 0);
    pub const S2X2: Self = Self(1 << 1);
    pub const S4X4: Self = Self(1 << 2);
    pub const S8X8: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    OpticalFlowGridSizeFlagsNV : u32, 0xf, UNKNOWN, S1X1, S2X2, S4X4, S8X8
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowUsageFlagsNV(pub u32);
impl OpticalFlowUsageFlagsNV {
    pub const UNKNOWN: Self = Self(0);
    pub const INPUT: Self = Self(1 << 0);
    pub const OUTPUT: Self = Self(1 << 1);
    pub const HINT: Self = Self(1 << 2);
    pub const COST: Self = Self(1 << 3);
    pub const GLOBAL_FLOW: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    OpticalFlowUsageFlagsNV : u32, 0x1f, UNKNOWN, INPUT, OUTPUT, HINT, COST, GLOBAL_FLOW
}
#[doc(alias = "VkOpticalFlowPerformanceLevelNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowPerformanceLevelNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowPerformanceLevelNV(pub i32);
impl OpticalFlowPerformanceLevelNV {
    pub const UNKNOWN: Self = Self(0);
    pub const SLOW: Self = Self(1);
    pub const MEDIUM: Self = Self(2);
    pub const FAST: Self = Self(3);
}
crate::enum_impl! {
    OpticalFlowPerformanceLevelNV : i32, UNKNOWN, SLOW, MEDIUM, FAST
}
#[doc(alias = "VkOpticalFlowSessionBindingPointNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionBindingPointNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionBindingPointNV(pub i32);
impl OpticalFlowSessionBindingPointNV {
    pub const UNKNOWN: Self = Self(0);
    pub const INPUT: Self = Self(1);
    pub const REFERENCE: Self = Self(2);
    pub const HINT: Self = Self(3);
    pub const FLOW_VECTOR: Self = Self(4);
    pub const BACKWARD_FLOW_VECTOR: Self = Self(5);
    pub const COST: Self = Self(6);
    pub const BACKWARD_COST: Self = Self(7);
    pub const GLOBAL_FLOW: Self = Self(8);
}
crate::enum_impl! {
    OpticalFlowSessionBindingPointNV : i32, UNKNOWN, INPUT, REFERENCE, HINT, FLOW_VECTOR,
    BACKWARD_FLOW_VECTOR, COST, BACKWARD_COST, GLOBAL_FLOW
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowSessionCreateFlagsNV(pub u32);
impl OpticalFlowSessionCreateFlagsNV {
    pub const ENABLE_HINT: Self = Self(1 << 0);
    pub const ENABLE_COST: Self = Self(1 << 1);
    pub const ENABLE_GLOBAL_FLOW: Self = Self(1 << 2);
    pub const ALLOW_REGIONS: Self = Self(1 << 3);
    pub const BOTH_DIRECTIONS: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    OpticalFlowSessionCreateFlagsNV : u32, 0x1f, ENABLE_HINT, ENABLE_COST,
    ENABLE_GLOBAL_FLOW, ALLOW_REGIONS, BOTH_DIRECTIONS
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpticalFlowExecuteFlagsNV(pub u32);
impl OpticalFlowExecuteFlagsNV {
    pub const DISABLE_TEMPORAL_HINTS: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    OpticalFlowExecuteFlagsNV : u32, 0x1, DISABLE_TEMPORAL_HINTS
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
    physical_device: crate::vk10::PhysicalDevice,
    p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
    p_format_count: *mut u32,
    p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_optical_flow_image_formats_nv
        .unwrap())(
        physical_device,
        p_optical_flow_image_format_info,
        p_format_count,
        p_image_format_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        format_count: Option<u32>,
    ) -> crate::VulkanResult<
        (Vec<OpticalFlowImageFormatPropertiesNV>, crate::vk10::Result),
    > {
        let get_physical_device_optical_flow_image_formats_nv = (*self.table)
            .get_physical_device_optical_flow_image_formats_nv
            .unwrap();
        let mut format_count = match format_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_optical_flow_image_formats_nv(
                    physical_device,
                    optical_flow_image_format_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut image_format_properties = vec![
            Default::default(); format_count as usize
        ];
        let result = get_physical_device_optical_flow_image_formats_nv(
            physical_device,
            optical_flow_image_format_info as _,
            &mut format_count,
            image_format_properties.as_mut_ptr(),
        );
        crate::new_result((image_format_properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateOpticalFlowSessionNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html)
pub unsafe fn create_optical_flow_session_nv(
    device: crate::vk10::Device,
    p_create_info: *const OpticalFlowSessionCreateInfoNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_session: *mut OpticalFlowSessionNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_optical_flow_session_nv
        .unwrap())(device, p_create_info, p_allocator, p_session)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html)
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        create_info: &OpticalFlowSessionCreateInfoNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<OpticalFlowSessionNV> {
        let create_optical_flow_session_nv = (*self.table)
            .create_optical_flow_session_nv
            .unwrap();
        let mut session = Default::default();
        let result = create_optical_flow_session_nv(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut session,
        );
        crate::new_result(session, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html)
pub unsafe fn destroy_optical_flow_session_nv(
    device: crate::vk10::Device,
    session: OpticalFlowSessionNV,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_optical_flow_session_nv
        .unwrap())(device, session, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html)
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        session: OpticalFlowSessionNV,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_optical_flow_session_nv = (*self.table)
            .destroy_optical_flow_session_nv
            .unwrap();
        destroy_optical_flow_session_nv(
            self.handle,
            session,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBindOpticalFlowSessionImageNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html)
pub unsafe fn bind_optical_flow_session_image_nv(
    device: crate::vk10::Device,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: crate::vk10::ImageView,
    layout: crate::vk10::ImageLayout,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .bind_optical_flow_session_image_nv
        .unwrap())(device, session, binding_point, view, layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html)
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: crate::vk10::ImageView,
        layout: crate::vk10::ImageLayout,
    ) -> crate::VulkanResult<()> {
        let bind_optical_flow_session_image_nv = (*self.table)
            .bind_optical_flow_session_image_nv
            .unwrap();
        let result = bind_optical_flow_session_image_nv(
            self.handle,
            session,
            binding_point as _,
            view,
            layout as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html)
pub unsafe fn cmd_optical_flow_execute_nv(
    command_buffer: crate::vk10::CommandBuffer,
    session: OpticalFlowSessionNV,
    p_execute_info: *const OpticalFlowExecuteInfoNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_optical_flow_execute_nv
        .unwrap())(command_buffer, session, p_execute_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html)
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    ) {
        let cmd_optical_flow_execute_nv = (*self.table)
            .cmd_optical_flow_execute_nv
            .unwrap();
        cmd_optical_flow_execute_nv(command_buffer, session, execute_info as _);
    }
}
pub const NV_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;
pub const NV_OPTICAL_FLOW_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_optical_flow"
);
