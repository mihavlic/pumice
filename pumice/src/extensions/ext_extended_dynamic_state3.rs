#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState3FeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html)
pub struct PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub extended_dynamic_state_3_tessellation_domain_origin: crate::vk10::Bool32,
    pub extended_dynamic_state_3_depth_clamp_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_polygon_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_rasterization_samples: crate::vk10::Bool32,
    pub extended_dynamic_state_3_sample_mask: crate::vk10::Bool32,
    pub extended_dynamic_state_3_alpha_to_coverage_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_alpha_to_one_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_logic_op_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_color_blend_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_color_blend_equation: crate::vk10::Bool32,
    pub extended_dynamic_state_3_color_write_mask: crate::vk10::Bool32,
    pub extended_dynamic_state_3_rasterization_stream: crate::vk10::Bool32,
    pub extended_dynamic_state_3_conservative_rasterization_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_extra_primitive_overestimation_size: crate::vk10::Bool32,
    pub extended_dynamic_state_3_depth_clip_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_sample_locations_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_color_blend_advanced: crate::vk10::Bool32,
    pub extended_dynamic_state_3_provoking_vertex_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_line_rasterization_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_line_stipple_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_depth_clip_negative_one_to_one: crate::vk10::Bool32,
    pub extended_dynamic_state_3_viewport_wscaling_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_viewport_swizzle: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_to_color_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_to_color_location: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_modulation_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_modulation_table_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_modulation_table: crate::vk10::Bool32,
    pub extended_dynamic_state_3_coverage_reduction_mode: crate::vk10::Bool32,
    pub extended_dynamic_state_3_representative_fragment_test_enable: crate::vk10::Bool32,
    pub extended_dynamic_state_3_shading_rate_image_enable: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state_3_tessellation_domain_origin: Default::default(),
            extended_dynamic_state_3_depth_clamp_enable: Default::default(),
            extended_dynamic_state_3_polygon_mode: Default::default(),
            extended_dynamic_state_3_rasterization_samples: Default::default(),
            extended_dynamic_state_3_sample_mask: Default::default(),
            extended_dynamic_state_3_alpha_to_coverage_enable: Default::default(),
            extended_dynamic_state_3_alpha_to_one_enable: Default::default(),
            extended_dynamic_state_3_logic_op_enable: Default::default(),
            extended_dynamic_state_3_color_blend_enable: Default::default(),
            extended_dynamic_state_3_color_blend_equation: Default::default(),
            extended_dynamic_state_3_color_write_mask: Default::default(),
            extended_dynamic_state_3_rasterization_stream: Default::default(),
            extended_dynamic_state_3_conservative_rasterization_mode: Default::default(),
            extended_dynamic_state_3_extra_primitive_overestimation_size: Default::default(),
            extended_dynamic_state_3_depth_clip_enable: Default::default(),
            extended_dynamic_state_3_sample_locations_enable: Default::default(),
            extended_dynamic_state_3_color_blend_advanced: Default::default(),
            extended_dynamic_state_3_provoking_vertex_mode: Default::default(),
            extended_dynamic_state_3_line_rasterization_mode: Default::default(),
            extended_dynamic_state_3_line_stipple_enable: Default::default(),
            extended_dynamic_state_3_depth_clip_negative_one_to_one: Default::default(),
            extended_dynamic_state_3_viewport_wscaling_enable: Default::default(),
            extended_dynamic_state_3_viewport_swizzle: Default::default(),
            extended_dynamic_state_3_coverage_to_color_enable: Default::default(),
            extended_dynamic_state_3_coverage_to_color_location: Default::default(),
            extended_dynamic_state_3_coverage_modulation_mode: Default::default(),
            extended_dynamic_state_3_coverage_modulation_table_enable: Default::default(),
            extended_dynamic_state_3_coverage_modulation_table: Default::default(),
            extended_dynamic_state_3_coverage_reduction_mode: Default::default(),
            extended_dynamic_state_3_representative_fragment_test_enable: Default::default(),
            extended_dynamic_state_3_shading_rate_image_enable: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState3PropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3PropertiesEXT.html)
pub struct PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub dynamic_primitive_topology_unrestricted: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            dynamic_primitive_topology_unrestricted: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkColorBlendEquationEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorBlendEquationEXT.html)
pub struct ColorBlendEquationEXT {
    pub src_color_blend_factor: crate::vk10::BlendFactor,
    pub dst_color_blend_factor: crate::vk10::BlendFactor,
    pub color_blend_op: crate::vk10::BlendOp,
    pub src_alpha_blend_factor: crate::vk10::BlendFactor,
    pub dst_alpha_blend_factor: crate::vk10::BlendFactor,
    pub alpha_blend_op: crate::vk10::BlendOp,
}
impl Default for ColorBlendEquationEXT {
    fn default() -> Self {
        Self {
            src_color_blend_factor: Default::default(),
            dst_color_blend_factor: Default::default(),
            color_blend_op: Default::default(),
            src_alpha_blend_factor: Default::default(),
            dst_alpha_blend_factor: Default::default(),
            alpha_blend_op: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkColorBlendAdvancedEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorBlendAdvancedEXT.html)
pub struct ColorBlendAdvancedEXT {
    pub advanced_blend_op: crate::vk10::BlendOp,
    pub src_premultiplied: crate::vk10::Bool32,
    pub dst_premultiplied: crate::vk10::Bool32,
    pub blend_overlap: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
    pub clamp_results: crate::vk10::Bool32,
}
impl Default for ColorBlendAdvancedEXT {
    fn default() -> Self {
        Self {
            advanced_blend_op: Default::default(),
            src_premultiplied: Default::default(),
            dst_premultiplied: Default::default(),
            blend_overlap: Default::default(),
            clamp_results: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html)
pub unsafe fn cmd_set_tessellation_domain_origin_ext(
    command_buffer: crate::vk10::CommandBuffer,
    domain_origin: crate::vk11::TessellationDomainOrigin,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_tessellation_domain_origin_ext
        .unwrap())(command_buffer, domain_origin)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html)
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        domain_origin: crate::vk11::TessellationDomainOrigin,
    ) {
        let cmd_set_tessellation_domain_origin_ext = (*self.table)
            .cmd_set_tessellation_domain_origin_ext
            .unwrap();
        cmd_set_tessellation_domain_origin_ext(command_buffer, domain_origin as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html)
pub unsafe fn cmd_set_depth_clamp_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_clamp_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_clamp_enable_ext
        .unwrap())(command_buffer, depth_clamp_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html)
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_clamp_enable: bool,
    ) {
        let cmd_set_depth_clamp_enable_ext = (*self.table)
            .cmd_set_depth_clamp_enable_ext
            .unwrap();
        cmd_set_depth_clamp_enable_ext(command_buffer, depth_clamp_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPolygonModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html)
pub unsafe fn cmd_set_polygon_mode_ext(
    command_buffer: crate::vk10::CommandBuffer,
    polygon_mode: crate::vk10::PolygonMode,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_polygon_mode_ext
        .unwrap())(command_buffer, polygon_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html)
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        polygon_mode: crate::vk10::PolygonMode,
    ) {
        let cmd_set_polygon_mode_ext = (*self.table).cmd_set_polygon_mode_ext.unwrap();
        cmd_set_polygon_mode_ext(command_buffer, polygon_mode as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)
pub unsafe fn cmd_set_rasterization_samples_ext(
    command_buffer: crate::vk10::CommandBuffer,
    rasterization_samples: crate::vk10::SampleCountFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_rasterization_samples_ext
        .unwrap())(command_buffer, rasterization_samples)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterization_samples: crate::vk10::SampleCountFlags,
    ) {
        let cmd_set_rasterization_samples_ext = (*self.table)
            .cmd_set_rasterization_samples_ext
            .unwrap();
        cmd_set_rasterization_samples_ext(command_buffer, rasterization_samples as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetSampleMaskEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html)
pub unsafe fn cmd_set_sample_mask_ext(
    command_buffer: crate::vk10::CommandBuffer,
    samples: crate::vk10::SampleCountFlags,
    p_sample_mask: *const crate::vk10::SampleMask,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_sample_mask_ext
        .unwrap())(command_buffer, samples, p_sample_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html)
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        samples: crate::vk10::SampleCountFlags,
        sample_mask: &[crate::vk10::SampleMask],
    ) {
        let cmd_set_sample_mask_ext = (*self.table).cmd_set_sample_mask_ext.unwrap();
        cmd_set_sample_mask_ext(command_buffer, samples as _, sample_mask.as_ptr());
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html)
pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    alpha_to_coverage_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_alpha_to_coverage_enable_ext
        .unwrap())(command_buffer, alpha_to_coverage_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html)
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) {
        let cmd_set_alpha_to_coverage_enable_ext = (*self.table)
            .cmd_set_alpha_to_coverage_enable_ext
            .unwrap();
        cmd_set_alpha_to_coverage_enable_ext(
            command_buffer,
            alpha_to_coverage_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html)
pub unsafe fn cmd_set_alpha_to_one_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    alpha_to_one_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_alpha_to_one_enable_ext
        .unwrap())(command_buffer, alpha_to_one_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html)
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        alpha_to_one_enable: bool,
    ) {
        let cmd_set_alpha_to_one_enable_ext = (*self.table)
            .cmd_set_alpha_to_one_enable_ext
            .unwrap();
        cmd_set_alpha_to_one_enable_ext(command_buffer, alpha_to_one_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html)
pub unsafe fn cmd_set_logic_op_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    logic_op_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_logic_op_enable_ext
        .unwrap())(command_buffer, logic_op_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html)
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        logic_op_enable: bool,
    ) {
        let cmd_set_logic_op_enable_ext = (*self.table)
            .cmd_set_logic_op_enable_ext
            .unwrap();
        cmd_set_logic_op_enable_ext(command_buffer, logic_op_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)
pub unsafe fn cmd_set_color_blend_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_enables: *const crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_color_blend_enable_ext
        .unwrap())(
        command_buffer,
        first_attachment,
        attachment_count,
        p_color_blend_enables,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)
    pub unsafe fn cmd_set_color_blend_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        color_blend_enables: &[crate::vk10::Bool32],
    ) {
        let cmd_set_color_blend_enable_ext = (*self.table)
            .cmd_set_color_blend_enable_ext
            .unwrap();
        let attachment_count = color_blend_enables.len();
        cmd_set_color_blend_enable_ext(
            command_buffer,
            first_attachment as _,
            attachment_count as _,
            color_blend_enables.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)
pub unsafe fn cmd_set_color_blend_equation_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_equations: *const ColorBlendEquationEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_color_blend_equation_ext
        .unwrap())(
        command_buffer,
        first_attachment,
        attachment_count,
        p_color_blend_equations,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)
    pub unsafe fn cmd_set_color_blend_equation_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) {
        let cmd_set_color_blend_equation_ext = (*self.table)
            .cmd_set_color_blend_equation_ext
            .unwrap();
        let attachment_count = color_blend_equations.len();
        cmd_set_color_blend_equation_ext(
            command_buffer,
            first_attachment as _,
            attachment_count as _,
            color_blend_equations.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)
pub unsafe fn cmd_set_color_write_mask_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_write_masks: *const crate::vk10::ColorComponentFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_color_write_mask_ext
        .unwrap())(
        command_buffer,
        first_attachment,
        attachment_count,
        p_color_write_masks,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)
    pub unsafe fn cmd_set_color_write_mask_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        color_write_masks: &[crate::vk10::ColorComponentFlags],
    ) {
        let cmd_set_color_write_mask_ext = (*self.table)
            .cmd_set_color_write_mask_ext
            .unwrap();
        let attachment_count = color_write_masks.len();
        cmd_set_color_write_mask_ext(
            command_buffer,
            first_attachment as _,
            attachment_count as _,
            color_write_masks.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRasterizationStreamEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html)
pub unsafe fn cmd_set_rasterization_stream_ext(
    command_buffer: crate::vk10::CommandBuffer,
    rasterization_stream: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_rasterization_stream_ext
        .unwrap())(command_buffer, rasterization_stream)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html)
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterization_stream: u32,
    ) {
        let cmd_set_rasterization_stream_ext = (*self.table)
            .cmd_set_rasterization_stream_ext
            .unwrap();
        cmd_set_rasterization_stream_ext(command_buffer, rasterization_stream as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html)
pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
    command_buffer: crate::vk10::CommandBuffer,
    conservative_rasterization_mode: crate::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_conservative_rasterization_mode_ext
        .unwrap())(command_buffer, conservative_rasterization_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html)
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        conservative_rasterization_mode: crate::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT,
    ) {
        let cmd_set_conservative_rasterization_mode_ext = (*self.table)
            .cmd_set_conservative_rasterization_mode_ext
            .unwrap();
        cmd_set_conservative_rasterization_mode_ext(
            command_buffer,
            conservative_rasterization_mode as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
    command_buffer: crate::vk10::CommandBuffer,
    extra_primitive_overestimation_size: std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_extra_primitive_overestimation_size_ext
        .unwrap())(command_buffer, extra_primitive_overestimation_size)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        extra_primitive_overestimation_size: std::os::raw::c_float,
    ) {
        let cmd_set_extra_primitive_overestimation_size_ext = (*self.table)
            .cmd_set_extra_primitive_overestimation_size_ext
            .unwrap();
        cmd_set_extra_primitive_overestimation_size_ext(
            command_buffer,
            extra_primitive_overestimation_size as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthClipEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html)
pub unsafe fn cmd_set_depth_clip_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_clip_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_clip_enable_ext
        .unwrap())(command_buffer, depth_clip_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html)
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_clip_enable: bool,
    ) {
        let cmd_set_depth_clip_enable_ext = (*self.table)
            .cmd_set_depth_clip_enable_ext
            .unwrap();
        cmd_set_depth_clip_enable_ext(command_buffer, depth_clip_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html)
pub unsafe fn cmd_set_sample_locations_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    sample_locations_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_sample_locations_enable_ext
        .unwrap())(command_buffer, sample_locations_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html)
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        sample_locations_enable: bool,
    ) {
        let cmd_set_sample_locations_enable_ext = (*self.table)
            .cmd_set_sample_locations_enable_ext
            .unwrap();
        cmd_set_sample_locations_enable_ext(
            command_buffer,
            sample_locations_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html)
pub unsafe fn cmd_set_color_blend_advanced_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_advanced: *const ColorBlendAdvancedEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_color_blend_advanced_ext
        .unwrap())(
        command_buffer,
        first_attachment,
        attachment_count,
        p_color_blend_advanced,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html)
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) {
        let cmd_set_color_blend_advanced_ext = (*self.table)
            .cmd_set_color_blend_advanced_ext
            .unwrap();
        let attachment_count = color_blend_advanced.len();
        cmd_set_color_blend_advanced_ext(
            command_buffer,
            first_attachment as _,
            attachment_count as _,
            color_blend_advanced.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html)
pub unsafe fn cmd_set_provoking_vertex_mode_ext(
    command_buffer: crate::vk10::CommandBuffer,
    provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_provoking_vertex_mode_ext
        .unwrap())(command_buffer, provoking_vertex_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html)
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        provoking_vertex_mode: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
    ) {
        let cmd_set_provoking_vertex_mode_ext = (*self.table)
            .cmd_set_provoking_vertex_mode_ext
            .unwrap();
        cmd_set_provoking_vertex_mode_ext(command_buffer, provoking_vertex_mode as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html)
pub unsafe fn cmd_set_line_rasterization_mode_ext(
    command_buffer: crate::vk10::CommandBuffer,
    line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_line_rasterization_mode_ext
        .unwrap())(command_buffer, line_rasterization_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html)
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        line_rasterization_mode: crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
    ) {
        let cmd_set_line_rasterization_mode_ext = (*self.table)
            .cmd_set_line_rasterization_mode_ext
            .unwrap();
        cmd_set_line_rasterization_mode_ext(
            command_buffer,
            line_rasterization_mode as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html)
pub unsafe fn cmd_set_line_stipple_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    stippled_line_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_line_stipple_enable_ext
        .unwrap())(command_buffer, stippled_line_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html)
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stippled_line_enable: bool,
    ) {
        let cmd_set_line_stipple_enable_ext = (*self.table)
            .cmd_set_line_stipple_enable_ext
            .unwrap();
        cmd_set_line_stipple_enable_ext(command_buffer, stippled_line_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html)
pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
    command_buffer: crate::vk10::CommandBuffer,
    negative_one_to_one: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_clip_negative_one_to_one_ext
        .unwrap())(command_buffer, negative_one_to_one)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html)
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        negative_one_to_one: bool,
    ) {
        let cmd_set_depth_clip_negative_one_to_one_ext = (*self.table)
            .cmd_set_depth_clip_negative_one_to_one_ext
            .unwrap();
        cmd_set_depth_clip_negative_one_to_one_ext(
            command_buffer,
            negative_one_to_one as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html)
pub unsafe fn cmd_set_viewport_wscaling_enable_nv(
    command_buffer: crate::vk10::CommandBuffer,
    viewport_wscaling_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_wscaling_enable_nv
        .unwrap())(command_buffer, viewport_wscaling_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html)
    pub unsafe fn cmd_set_viewport_wscaling_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewport_wscaling_enable: bool,
    ) {
        let cmd_set_viewport_wscaling_enable_nv = (*self.table)
            .cmd_set_viewport_wscaling_enable_nv
            .unwrap();
        cmd_set_viewport_wscaling_enable_nv(
            command_buffer,
            viewport_wscaling_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportSwizzleNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html)
pub unsafe fn cmd_set_viewport_swizzle_nv(
    command_buffer: crate::vk10::CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_swizzles: *const crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_swizzle_nv
        .unwrap())(command_buffer, first_viewport, viewport_count, p_viewport_swizzles)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html)
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_viewport: u32,
        viewport_swizzles: &[crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV],
    ) {
        let cmd_set_viewport_swizzle_nv = (*self.table)
            .cmd_set_viewport_swizzle_nv
            .unwrap();
        let viewport_count = viewport_swizzles.len();
        cmd_set_viewport_swizzle_nv(
            command_buffer,
            first_viewport as _,
            viewport_count as _,
            viewport_swizzles.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html)
pub unsafe fn cmd_set_coverage_to_color_enable_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_to_color_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_to_color_enable_nv
        .unwrap())(command_buffer, coverage_to_color_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html)
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_to_color_enable: bool,
    ) {
        let cmd_set_coverage_to_color_enable_nv = (*self.table)
            .cmd_set_coverage_to_color_enable_nv
            .unwrap();
        cmd_set_coverage_to_color_enable_nv(
            command_buffer,
            coverage_to_color_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html)
pub unsafe fn cmd_set_coverage_to_color_location_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_to_color_location: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_to_color_location_nv
        .unwrap())(command_buffer, coverage_to_color_location)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html)
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        let cmd_set_coverage_to_color_location_nv = (*self.table)
            .cmd_set_coverage_to_color_location_nv
            .unwrap();
        cmd_set_coverage_to_color_location_nv(
            command_buffer,
            coverage_to_color_location as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageModulationModeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html)
pub unsafe fn cmd_set_coverage_modulation_mode_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_modulation_mode_nv
        .unwrap())(command_buffer, coverage_modulation_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html)
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_mode: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
    ) {
        let cmd_set_coverage_modulation_mode_nv = (*self.table)
            .cmd_set_coverage_modulation_mode_nv
            .unwrap();
        cmd_set_coverage_modulation_mode_nv(
            command_buffer,
            coverage_modulation_mode as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html)
pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_modulation_table_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_modulation_table_enable_nv
        .unwrap())(command_buffer, coverage_modulation_table_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html)
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) {
        let cmd_set_coverage_modulation_table_enable_nv = (*self.table)
            .cmd_set_coverage_modulation_table_enable_nv
            .unwrap();
        cmd_set_coverage_modulation_table_enable_nv(
            command_buffer,
            coverage_modulation_table_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageModulationTableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html)
pub unsafe fn cmd_set_coverage_modulation_table_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_modulation_table_count: u32,
    p_coverage_modulation_table: *const std::os::raw::c_float,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_modulation_table_nv
        .unwrap())(
        command_buffer,
        coverage_modulation_table_count,
        p_coverage_modulation_table,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html)
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_modulation_table: &[std::os::raw::c_float],
    ) {
        let cmd_set_coverage_modulation_table_nv = (*self.table)
            .cmd_set_coverage_modulation_table_nv
            .unwrap();
        let coverage_modulation_table_count = coverage_modulation_table.len();
        cmd_set_coverage_modulation_table_nv(
            command_buffer,
            coverage_modulation_table_count as _,
            coverage_modulation_table.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html)
pub unsafe fn cmd_set_shading_rate_image_enable_nv(
    command_buffer: crate::vk10::CommandBuffer,
    shading_rate_image_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_shading_rate_image_enable_nv
        .unwrap())(command_buffer, shading_rate_image_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html)
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        shading_rate_image_enable: bool,
    ) {
        let cmd_set_shading_rate_image_enable_nv = (*self.table)
            .cmd_set_shading_rate_image_enable_nv
            .unwrap();
        cmd_set_shading_rate_image_enable_nv(
            command_buffer,
            shading_rate_image_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCoverageReductionModeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html)
pub unsafe fn cmd_set_coverage_reduction_mode_nv(
    command_buffer: crate::vk10::CommandBuffer,
    coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_coverage_reduction_mode_nv
        .unwrap())(command_buffer, coverage_reduction_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html)
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        coverage_reduction_mode: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    ) {
        let cmd_set_coverage_reduction_mode_nv = (*self.table)
            .cmd_set_coverage_reduction_mode_nv
            .unwrap();
        cmd_set_coverage_reduction_mode_nv(command_buffer, coverage_reduction_mode as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html)
pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
    command_buffer: crate::vk10::CommandBuffer,
    representative_fragment_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_representative_fragment_test_enable_nv
        .unwrap())(command_buffer, representative_fragment_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html)
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        representative_fragment_test_enable: bool,
    ) {
        let cmd_set_representative_fragment_test_enable_nv = (*self.table)
            .cmd_set_representative_fragment_test_enable_nv
            .unwrap();
        cmd_set_representative_fragment_test_enable_nv(
            command_buffer,
            representative_fragment_test_enable as _,
        );
    }
}
pub const EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION: u32 = 2;
pub const EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_extended_dynamic_state3"
);
