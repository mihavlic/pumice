#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkShadingRatePaletteNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html)
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}
impl Default for ShadingRatePaletteNV {
    fn default() -> Self {
        Self {
            shading_rate_palette_entry_count: Default::default(),
            p_shading_rate_palette_entries: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html)
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub shading_rate_image_enable: crate::vk10::Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const ShadingRatePaletteNV,
}
impl Default for PipelineViewportShadingRateImageStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            shading_rate_image_enable: Default::default(),
            viewport_count: Default::default(),
            p_shading_rate_palettes: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html)
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shading_rate_image: crate::vk10::Bool32,
    pub shading_rate_coarse_sample_order: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShadingRateImageFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            shading_rate_image: Default::default(),
            shading_rate_coarse_sample_order: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html)
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shading_rate_texel_size: crate::vk10::Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}
impl Default for PhysicalDeviceShadingRateImagePropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shading_rate_texel_size: Default::default(),
            shading_rate_palette_size: Default::default(),
            shading_rate_max_coarse_samples: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkCoarseSampleLocationNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html)
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}
impl Default for CoarseSampleLocationNV {
    fn default() -> Self {
        Self {
            pixel_x: Default::default(),
            pixel_y: Default::default(),
            sample: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderCustomNV.html)
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const CoarseSampleLocationNV,
}
impl Default for CoarseSampleOrderCustomNV {
    fn default() -> Self {
        Self {
            shading_rate: Default::default(),
            sample_count: Default::default(),
            sample_location_count: Default::default(),
            p_sample_locations: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html)
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub sample_order_type: CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
}
impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            sample_order_type: Default::default(),
            custom_sample_order_count: Default::default(),
            p_custom_sample_orders: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkShadingRatePaletteEntryNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteEntryNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShadingRatePaletteEntryNV(pub i32);
impl ShadingRatePaletteEntryNV {
    pub const NO_INVOCATIONS: Self = Self(0);
    pub const E16_INVOCATIONS_PER_PIXEL: Self = Self(1);
    pub const E8_INVOCATIONS_PER_PIXEL: Self = Self(2);
    pub const E4_INVOCATIONS_PER_PIXEL: Self = Self(3);
    pub const E2_INVOCATIONS_PER_PIXEL: Self = Self(4);
    pub const E1_INVOCATION_PER_PIXEL: Self = Self(5);
    pub const E1_INVOCATION_PER_2X1_PIXELS: Self = Self(6);
    pub const E1_INVOCATION_PER_1X2_PIXELS: Self = Self(7);
    pub const E1_INVOCATION_PER_2X2_PIXELS: Self = Self(8);
    pub const E1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    pub const E1_INVOCATION_PER_2X4_PIXELS: Self = Self(10);
    pub const E1_INVOCATION_PER_4X4_PIXELS: Self = Self(11);
}
crate::enum_impl! {
    ShadingRatePaletteEntryNV : i32, NO_INVOCATIONS, E16_INVOCATIONS_PER_PIXEL,
    E8_INVOCATIONS_PER_PIXEL, E4_INVOCATIONS_PER_PIXEL, E2_INVOCATIONS_PER_PIXEL,
    E1_INVOCATION_PER_PIXEL, E1_INVOCATION_PER_2X1_PIXELS, E1_INVOCATION_PER_1X2_PIXELS,
    E1_INVOCATION_PER_2X2_PIXELS, E1_INVOCATION_PER_4X2_PIXELS,
    E1_INVOCATION_PER_2X4_PIXELS, E1_INVOCATION_PER_4X4_PIXELS
}
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoarseSampleOrderTypeNV(pub i32);
impl CoarseSampleOrderTypeNV {
    pub const DEFAULT: Self = Self(0);
    pub const CUSTOM: Self = Self(1);
    pub const PIXEL_MAJOR: Self = Self(2);
    pub const SAMPLE_MAJOR: Self = Self(3);
}
crate::enum_impl! {
    CoarseSampleOrderTypeNV : i32, DEFAULT, CUSTOM, PIXEL_MAJOR, SAMPLE_MAJOR
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindShadingRateImageNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html)
pub unsafe fn cmd_bind_shading_rate_image_nv(
    command_buffer: crate::vk10::CommandBuffer,
    image_view: crate::vk10::ImageView,
    image_layout: crate::vk10::ImageLayout,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_shading_rate_image_nv
        .unwrap())(command_buffer, image_view, image_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html)
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        image_view: crate::vk10::ImageView,
        image_layout: crate::vk10::ImageLayout,
    ) {
        let cmd_bind_shading_rate_image_nv = (*self.table)
            .cmd_bind_shading_rate_image_nv
            .unwrap();
        cmd_bind_shading_rate_image_nv(command_buffer, image_view, image_layout as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
    command_buffer: crate::vk10::CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const ShadingRatePaletteNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_shading_rate_palette_nv
        .unwrap())(
        command_buffer,
        first_viewport,
        viewport_count,
        p_shading_rate_palettes,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    ) {
        let cmd_set_viewport_shading_rate_palette_nv = (*self.table)
            .cmd_set_viewport_shading_rate_palette_nv
            .unwrap();
        let viewport_count = shading_rate_palettes.len();
        cmd_set_viewport_shading_rate_palette_nv(
            command_buffer,
            first_viewport as _,
            viewport_count as _,
            shading_rate_palettes.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
pub unsafe fn cmd_set_coarse_sample_order_nv(
    command_buffer: crate::vk10::CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coarse_sample_order_nv
        .unwrap())(
        command_buffer,
        sample_order_type,
        custom_sample_order_count,
        p_custom_sample_orders,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    ) {
        let cmd_set_coarse_sample_order_nv = (*self.table)
            .cmd_set_coarse_sample_order_nv
            .unwrap();
        let custom_sample_order_count = custom_sample_orders.len();
        cmd_set_coarse_sample_order_nv(
            command_buffer,
            sample_order_type as _,
            custom_sample_order_count as _,
            custom_sample_orders.as_ptr(),
        );
    }
}
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_shading_rate_image"
);
