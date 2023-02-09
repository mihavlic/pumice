pub const API_VERSION_1_2: u32 = crate::vk10::make_api_version(0, 1, 2, 0);
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkConformanceVersion")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html)
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
impl Default for ConformanceVersion {
    fn default() -> Self {
        Self {
            major: Default::default(),
            minor: Default::default(),
            subminor: Default::default(),
            patch: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverProperties.html)
pub struct PhysicalDeviceDriverProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub driver_id: DriverId,
    pub driver_name: [std::os::raw::c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [std::os::raw::c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
}
impl Default for PhysicalDeviceDriverProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html)
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_subgroup_extended_types: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_subgroup_extended_types: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html)
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub filter_minmax_single_component_formats: crate::vk10::Bool32,
    pub filter_minmax_image_component_mapping: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSamplerFilterMinmaxProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES,
            p_next: std::ptr::null_mut(),
            filter_minmax_single_component_formats: Default::default(),
            filter_minmax_image_component_mapping: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeCreateInfo.html)
pub struct SamplerReductionModeCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub reduction_mode: SamplerReductionMode,
}
impl Default for SamplerReductionModeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO,
            p_next: std::ptr::null(),
            reduction_mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageFormatListCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html)
pub struct ImageFormatListCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk10::Format,
}
impl Default for ImageFormatListCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO,
            p_next: std::ptr::null(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html)
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_float_16: crate::vk10::Bool32,
    pub shader_int_8: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderFloat16Int8Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_float_16: Default::default(),
            shader_int_8: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html)
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: crate::vk10::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_32: crate::vk10::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_64: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_16: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_32: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_64: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_16: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_32: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_64: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_16: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_32: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_64: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_16: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_32: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_64: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFloatControlsProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
            p_next: std::ptr::null_mut(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_16: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_32: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_64: Default::default(),
            shader_denorm_preserve_float_16: Default::default(),
            shader_denorm_preserve_float_32: Default::default(),
            shader_denorm_preserve_float_64: Default::default(),
            shader_denorm_flush_to_zero_float_16: Default::default(),
            shader_denorm_flush_to_zero_float_32: Default::default(),
            shader_denorm_flush_to_zero_float_64: Default::default(),
            shader_rounding_mode_rtefloat_16: Default::default(),
            shader_rounding_mode_rtefloat_32: Default::default(),
            shader_rounding_mode_rtefloat_64: Default::default(),
            shader_rounding_mode_rtzfloat_16: Default::default(),
            shader_rounding_mode_rtzfloat_32: Default::default(),
            shader_rounding_mode_rtzfloat_64: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html)
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub host_query_reset: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceHostQueryResetFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
            p_next: std::ptr::null_mut(),
            host_query_reset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html)
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_input_attachment_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_image_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_image_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_update_unused_while_pending: crate::vk10::Bool32,
    pub descriptor_binding_partially_bound: crate::vk10::Bool32,
    pub descriptor_binding_variable_descriptor_count: crate::vk10::Bool32,
    pub runtime_descriptor_array: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDescriptorIndexingFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_input_attachment_array_dynamic_indexing: Default::default(),
            shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
            shader_sampled_image_array_non_uniform_indexing: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_image_array_non_uniform_indexing: Default::default(),
            shader_input_attachment_array_non_uniform_indexing: Default::default(),
            shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
            descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
            descriptor_binding_sampled_image_update_after_bind: Default::default(),
            descriptor_binding_storage_image_update_after_bind: Default::default(),
            descriptor_binding_storage_buffer_update_after_bind: Default::default(),
            descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_update_unused_while_pending: Default::default(),
            descriptor_binding_partially_bound: Default::default(),
            descriptor_binding_variable_descriptor_count: Default::default(),
            runtime_descriptor_array: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html)
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub robust_buffer_access_update_after_bind: crate::vk10::Bool32,
    pub quad_divergent_implicit_lod: crate::vk10::Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl Default for PhysicalDeviceDescriptorIndexingProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_update_after_bind_descriptors_in_all_pools: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_storage_image_array_non_uniform_indexing_native: Default::default(),
            shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
            robust_buffer_access_update_after_bind: Default::default(),
            quad_divergent_implicit_lod: Default::default(),
            max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
            max_per_stage_update_after_bind_resources: Default::default(),
            max_descriptor_set_update_after_bind_samplers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_sampled_images: Default::default(),
            max_descriptor_set_update_after_bind_storage_images: Default::default(),
            max_descriptor_set_update_after_bind_input_attachments: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html)
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlags,
}
impl Default for DescriptorSetLayoutBindingFlagsCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
            p_next: std::ptr::null(),
            binding_count: Default::default(),
            p_binding_flags: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html)
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
impl Default for DescriptorSetVariableDescriptorCountAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            descriptor_set_count: Default::default(),
            p_descriptor_counts: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html)
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_variable_descriptor_count: u32,
}
impl Default for DescriptorSetVariableDescriptorCountLayoutSupport {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
            p_next: std::ptr::null_mut(),
            max_variable_descriptor_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentDescription2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2.html)
pub struct AttachmentDescription2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::AttachmentDescriptionFlags,
    pub format: crate::vk10::Format,
    pub samples: crate::vk10::SampleCountFlags,
    pub load_op: crate::vk10::AttachmentLoadOp,
    pub store_op: crate::vk10::AttachmentStoreOp,
    pub stencil_load_op: crate::vk10::AttachmentLoadOp,
    pub stencil_store_op: crate::vk10::AttachmentStoreOp,
    pub initial_layout: crate::vk10::ImageLayout,
    pub final_layout: crate::vk10::ImageLayout,
}
impl Default for AttachmentDescription2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            format: Default::default(),
            samples: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            stencil_load_op: Default::default(),
            stencil_store_op: Default::default(),
            initial_layout: Default::default(),
            final_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentReference2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2.html)
pub struct AttachmentReference2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment: u32,
    pub layout: crate::vk10::ImageLayout,
    pub aspect_mask: crate::vk10::ImageAspectFlags,
}
impl Default for AttachmentReference2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_REFERENCE_2,
            p_next: std::ptr::null(),
            attachment: Default::default(),
            layout: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDescription2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2.html)
pub struct SubpassDescription2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::SubpassDescriptionFlags,
    pub pipeline_bind_point: crate::vk10::PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2,
    pub p_resolve_attachments: *const AttachmentReference2,
    pub p_depth_stencil_attachment: *const AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl Default for SubpassDescription2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DESCRIPTION_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            view_mask: Default::default(),
            input_attachment_count: Default::default(),
            p_input_attachments: std::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_resolve_attachments: std::ptr::null(),
            p_depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: Default::default(),
            p_preserve_attachments: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDependency2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2.html)
pub struct SubpassDependency2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: crate::vk10::PipelineStageFlags,
    pub dst_stage_mask: crate::vk10::PipelineStageFlags,
    pub src_access_mask: crate::vk10::AccessFlags,
    pub dst_access_mask: crate::vk10::AccessFlags,
    pub dependency_flags: crate::vk10::DependencyFlags,
    pub view_offset: i32,
}
impl Default for SubpassDependency2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DEPENDENCY_2,
            p_next: std::ptr::null(),
            src_subpass: Default::default(),
            dst_subpass: Default::default(),
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            dependency_flags: Default::default(),
            view_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreateInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2.html)
pub struct RenderPassCreateInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
impl Default for RenderPassCreateInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_CREATE_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
            subpass_count: Default::default(),
            p_subpasses: std::ptr::null(),
            dependency_count: Default::default(),
            p_dependencies: std::ptr::null(),
            correlated_view_mask_count: Default::default(),
            p_correlated_view_masks: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfo.html)
pub struct SubpassBeginInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub contents: crate::vk10::SubpassContents,
}
impl Default for SubpassBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            contents: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassEndInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html)
pub struct SubpassEndInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
}
impl Default for SubpassEndInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_END_INFO,
            p_next: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html)
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub timeline_semaphore: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTimelineSemaphoreFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
            p_next: std::ptr::null_mut(),
            timeline_semaphore: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html)
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
impl Default for PhysicalDeviceTimelineSemaphoreProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_timeline_semaphore_value_difference: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html)
pub struct SemaphoreTypeCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}
impl Default for SemaphoreTypeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_TYPE_CREATE_INFO,
            p_next: std::ptr::null(),
            semaphore_type: Default::default(),
            initial_value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html)
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl Default for TimelineSemaphoreSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_value_count: Default::default(),
            p_wait_semaphore_values: std::ptr::null(),
            signal_semaphore_value_count: Default::default(),
            p_signal_semaphore_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreWaitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html)
pub struct SemaphoreWaitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const crate::vk10::Semaphore,
    pub p_values: *const u64,
}
impl Default for SemaphoreWaitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_WAIT_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            semaphore_count: Default::default(),
            p_semaphores: std::ptr::null(),
            p_values: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreSignalInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfo.html)
pub struct SemaphoreSignalInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub value: u64,
}
impl Default for SemaphoreSignalInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_SIGNAL_INFO,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html)
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub storage_buffer_8_bit_access: crate::vk10::Bool32,
    pub uniform_and_storage_buffer_8_bit_access: crate::vk10::Bool32,
    pub storage_push_constant_8: crate::vk10::Bool32,
}
impl Default for PhysicalDevice8BitStorageFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer_8_bit_access: Default::default(),
            uniform_and_storage_buffer_8_bit_access: Default::default(),
            storage_push_constant_8: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html)
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub vulkan_memory_model: crate::vk10::Bool32,
    pub vulkan_memory_model_device_scope: crate::vk10::Bool32,
    pub vulkan_memory_model_availability_visibility_chains: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVulkanMemoryModelFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
            p_next: std::ptr::null_mut(),
            vulkan_memory_model: Default::default(),
            vulkan_memory_model_device_scope: Default::default(),
            vulkan_memory_model_availability_visibility_chains: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html)
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_buffer_int_64_atomics: crate::vk10::Bool32,
    pub shader_shared_int_64_atomics: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderAtomicInt64Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_buffer_int_64_atomics: Default::default(),
            shader_shared_int_64_atomics: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html)
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: crate::vk10::Bool32,
    pub independent_resolve: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDepthStencilResolveProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
            p_next: std::ptr::null_mut(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: Default::default(),
            independent_resolve: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html)
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub depth_resolve_mode: ResolveModeFlags,
    pub stencil_resolve_mode: ResolveModeFlags,
    pub p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}
impl Default for SubpassDescriptionDepthStencilResolve {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
            p_next: std::ptr::null(),
            depth_resolve_mode: Default::default(),
            stencil_resolve_mode: Default::default(),
            p_depth_stencil_resolve_attachment: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageStencilUsageCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageStencilUsageCreateInfo.html)
pub struct ImageStencilUsageCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub stencil_usage: crate::vk10::ImageUsageFlags,
}
impl Default for ImageStencilUsageCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_STENCIL_USAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            stencil_usage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html)
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub scalar_block_layout: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceScalarBlockLayoutFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
            p_next: std::ptr::null_mut(),
            scalar_block_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html)
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub uniform_buffer_standard_layout: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
            p_next: std::ptr::null_mut(),
            uniform_buffer_standard_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html)
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub buffer_device_address: crate::vk10::Bool32,
    pub buffer_device_address_capture_replay: crate::vk10::Bool32,
    pub buffer_device_address_multi_device: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceBufferDeviceAddressFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
            p_next: std::ptr::null_mut(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferDeviceAddressInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html)
pub struct BufferDeviceAddressInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer: crate::vk10::Buffer,
}
impl Default for BufferDeviceAddressInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_DEVICE_ADDRESS_INFO,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html)
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub opaque_capture_address: u64,
}
impl Default for BufferOpaqueCaptureAddressCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
            p_next: std::ptr::null(),
            opaque_capture_address: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html)
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub imageless_framebuffer: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImagelessFramebufferFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
            p_next: std::ptr::null_mut(),
            imageless_framebuffer: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html)
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
impl Default for FramebufferAttachmentsCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
            p_next: std::ptr::null(),
            attachment_image_info_count: Default::default(),
            p_attachment_image_infos: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfo.html)
pub struct FramebufferAttachmentImageInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: crate::vk10::ImageCreateFlags,
    pub usage: crate::vk10::ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const crate::vk10::Format,
}
impl Default for FramebufferAttachmentImageInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            width: Default::default(),
            height: Default::default(),
            layer_count: Default::default(),
            view_format_count: Default::default(),
            p_view_formats: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfo.html)
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk10::ImageView,
}
impl Default for RenderPassAttachmentBeginInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO,
            p_next: std::ptr::null(),
            attachment_count: Default::default(),
            p_attachments: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html)
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub separate_depth_stencil_layouts: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
            p_next: std::ptr::null_mut(),
            separate_depth_stencil_layouts: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReferenceStencilLayout.html)
pub struct AttachmentReferenceStencilLayout {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub stencil_layout: crate::vk10::ImageLayout,
}
impl Default for AttachmentReferenceStencilLayout {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT,
            p_next: std::ptr::null_mut(),
            stencil_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionStencilLayout.html)
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub stencil_initial_layout: crate::vk10::ImageLayout,
    pub stencil_final_layout: crate::vk10::ImageLayout,
}
impl Default for AttachmentDescriptionStencilLayout {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
            p_next: std::ptr::null_mut(),
            stencil_initial_layout: Default::default(),
            stencil_final_layout: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html)
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub opaque_capture_address: u64,
}
impl Default for MemoryOpaqueCaptureAddressAllocateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            opaque_capture_address: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html)
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
}
impl Default for DeviceMemoryOpaqueCaptureAddressInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub storage_buffer_16_bit_access: crate::vk10::Bool32,
    pub uniform_and_storage_buffer_16_bit_access: crate::vk10::Bool32,
    pub storage_push_constant_16: crate::vk10::Bool32,
    pub storage_input_output_16: crate::vk10::Bool32,
    pub multiview: crate::vk10::Bool32,
    pub multiview_geometry_shader: crate::vk10::Bool32,
    pub multiview_tessellation_shader: crate::vk10::Bool32,
    pub variable_pointers_storage_buffer: crate::vk10::Bool32,
    pub variable_pointers: crate::vk10::Bool32,
    pub protected_memory: crate::vk10::Bool32,
    pub sampler_ycbcr_conversion: crate::vk10::Bool32,
    pub shader_draw_parameters: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVulkan11Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
            p_next: std::ptr::null_mut(),
            storage_buffer_16_bit_access: Default::default(),
            uniform_and_storage_buffer_16_bit_access: Default::default(),
            storage_push_constant_16: Default::default(),
            storage_input_output_16: Default::default(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
            protected_memory: Default::default(),
            sampler_ycbcr_conversion: Default::default(),
            shader_draw_parameters: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html)
pub struct PhysicalDeviceVulkan11Properties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_uuid: [u8; crate::vk10::UUID_SIZE as usize],
    pub driver_uuid: [u8; crate::vk10::UUID_SIZE as usize],
    pub device_luid: [u8; crate::vk11::LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luidvalid: crate::vk10::Bool32,
    pub subgroup_size: u32,
    pub subgroup_supported_stages: crate::vk10::ShaderStageFlags,
    pub subgroup_supported_operations: crate::vk11::SubgroupFeatureFlags,
    pub subgroup_quad_operations_in_all_stages: crate::vk10::Bool32,
    pub point_clipping_behavior: crate::vk11::PointClippingBehavior,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub protected_no_fault: crate::vk10::Bool32,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceVulkan11Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES,
            p_next: std::ptr::null_mut(),
            device_uuid: unsafe { std::mem::zeroed() },
            driver_uuid: unsafe { std::mem::zeroed() },
            device_luid: unsafe { std::mem::zeroed() },
            device_node_mask: Default::default(),
            device_luidvalid: Default::default(),
            subgroup_size: Default::default(),
            subgroup_supported_stages: Default::default(),
            subgroup_supported_operations: Default::default(),
            subgroup_quad_operations_in_all_stages: Default::default(),
            point_clipping_behavior: Default::default(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
            protected_no_fault: Default::default(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub sampler_mirror_clamp_to_edge: crate::vk10::Bool32,
    pub draw_indirect_count: crate::vk10::Bool32,
    pub storage_buffer_8_bit_access: crate::vk10::Bool32,
    pub uniform_and_storage_buffer_8_bit_access: crate::vk10::Bool32,
    pub storage_push_constant_8: crate::vk10::Bool32,
    pub shader_buffer_int_64_atomics: crate::vk10::Bool32,
    pub shader_shared_int_64_atomics: crate::vk10::Bool32,
    pub shader_float_16: crate::vk10::Bool32,
    pub shader_int_8: crate::vk10::Bool32,
    pub descriptor_indexing: crate::vk10::Bool32,
    pub shader_input_attachment_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: crate::vk10::Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_image_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: crate::vk10::Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_image_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: crate::vk10::Bool32,
    pub descriptor_binding_update_unused_while_pending: crate::vk10::Bool32,
    pub descriptor_binding_partially_bound: crate::vk10::Bool32,
    pub descriptor_binding_variable_descriptor_count: crate::vk10::Bool32,
    pub runtime_descriptor_array: crate::vk10::Bool32,
    pub sampler_filter_minmax: crate::vk10::Bool32,
    pub scalar_block_layout: crate::vk10::Bool32,
    pub imageless_framebuffer: crate::vk10::Bool32,
    pub uniform_buffer_standard_layout: crate::vk10::Bool32,
    pub shader_subgroup_extended_types: crate::vk10::Bool32,
    pub separate_depth_stencil_layouts: crate::vk10::Bool32,
    pub host_query_reset: crate::vk10::Bool32,
    pub timeline_semaphore: crate::vk10::Bool32,
    pub buffer_device_address: crate::vk10::Bool32,
    pub buffer_device_address_capture_replay: crate::vk10::Bool32,
    pub buffer_device_address_multi_device: crate::vk10::Bool32,
    pub vulkan_memory_model: crate::vk10::Bool32,
    pub vulkan_memory_model_device_scope: crate::vk10::Bool32,
    pub vulkan_memory_model_availability_visibility_chains: crate::vk10::Bool32,
    pub shader_output_viewport_index: crate::vk10::Bool32,
    pub shader_output_layer: crate::vk10::Bool32,
    pub subgroup_broadcast_dynamic_id: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVulkan12Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
            p_next: std::ptr::null_mut(),
            sampler_mirror_clamp_to_edge: Default::default(),
            draw_indirect_count: Default::default(),
            storage_buffer_8_bit_access: Default::default(),
            uniform_and_storage_buffer_8_bit_access: Default::default(),
            storage_push_constant_8: Default::default(),
            shader_buffer_int_64_atomics: Default::default(),
            shader_shared_int_64_atomics: Default::default(),
            shader_float_16: Default::default(),
            shader_int_8: Default::default(),
            descriptor_indexing: Default::default(),
            shader_input_attachment_array_dynamic_indexing: Default::default(),
            shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
            shader_sampled_image_array_non_uniform_indexing: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_image_array_non_uniform_indexing: Default::default(),
            shader_input_attachment_array_non_uniform_indexing: Default::default(),
            shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
            shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
            descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
            descriptor_binding_sampled_image_update_after_bind: Default::default(),
            descriptor_binding_storage_image_update_after_bind: Default::default(),
            descriptor_binding_storage_buffer_update_after_bind: Default::default(),
            descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
            descriptor_binding_update_unused_while_pending: Default::default(),
            descriptor_binding_partially_bound: Default::default(),
            descriptor_binding_variable_descriptor_count: Default::default(),
            runtime_descriptor_array: Default::default(),
            sampler_filter_minmax: Default::default(),
            scalar_block_layout: Default::default(),
            imageless_framebuffer: Default::default(),
            uniform_buffer_standard_layout: Default::default(),
            shader_subgroup_extended_types: Default::default(),
            separate_depth_stencil_layouts: Default::default(),
            host_query_reset: Default::default(),
            timeline_semaphore: Default::default(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
            vulkan_memory_model: Default::default(),
            vulkan_memory_model_device_scope: Default::default(),
            vulkan_memory_model_availability_visibility_chains: Default::default(),
            shader_output_viewport_index: Default::default(),
            shader_output_layer: Default::default(),
            subgroup_broadcast_dynamic_id: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html)
pub struct PhysicalDeviceVulkan12Properties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub driver_id: DriverId,
    pub driver_name: [std::os::raw::c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [std::os::raw::c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float_16: crate::vk10::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_32: crate::vk10::Bool32,
    pub shader_signed_zero_inf_nan_preserve_float_64: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_16: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_32: crate::vk10::Bool32,
    pub shader_denorm_preserve_float_64: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_16: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_32: crate::vk10::Bool32,
    pub shader_denorm_flush_to_zero_float_64: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_16: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_32: crate::vk10::Bool32,
    pub shader_rounding_mode_rtefloat_64: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_16: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_32: crate::vk10::Bool32,
    pub shader_rounding_mode_rtzfloat_64: crate::vk10::Bool32,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: crate::vk10::Bool32,
    pub robust_buffer_access_update_after_bind: crate::vk10::Bool32,
    pub quad_divergent_implicit_lod: crate::vk10::Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: crate::vk10::Bool32,
    pub independent_resolve: crate::vk10::Bool32,
    pub filter_minmax_single_component_formats: crate::vk10::Bool32,
    pub filter_minmax_image_component_mapping: crate::vk10::Bool32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: crate::vk10::SampleCountFlags,
}
impl Default for PhysicalDeviceVulkan12Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: unsafe { std::mem::zeroed() },
            driver_info: unsafe { std::mem::zeroed() },
            conformance_version: Default::default(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_16: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_32: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_64: Default::default(),
            shader_denorm_preserve_float_16: Default::default(),
            shader_denorm_preserve_float_32: Default::default(),
            shader_denorm_preserve_float_64: Default::default(),
            shader_denorm_flush_to_zero_float_16: Default::default(),
            shader_denorm_flush_to_zero_float_32: Default::default(),
            shader_denorm_flush_to_zero_float_64: Default::default(),
            shader_rounding_mode_rtefloat_16: Default::default(),
            shader_rounding_mode_rtefloat_32: Default::default(),
            shader_rounding_mode_rtefloat_64: Default::default(),
            shader_rounding_mode_rtzfloat_16: Default::default(),
            shader_rounding_mode_rtzfloat_32: Default::default(),
            shader_rounding_mode_rtzfloat_64: Default::default(),
            max_update_after_bind_descriptors_in_all_pools: Default::default(),
            shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
            shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
            shader_storage_image_array_non_uniform_indexing_native: Default::default(),
            shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
            robust_buffer_access_update_after_bind: Default::default(),
            quad_divergent_implicit_lod: Default::default(),
            max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
            max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
            max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
            max_per_stage_update_after_bind_resources: Default::default(),
            max_descriptor_set_update_after_bind_samplers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_sampled_images: Default::default(),
            max_descriptor_set_update_after_bind_storage_images: Default::default(),
            max_descriptor_set_update_after_bind_input_attachments: Default::default(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: Default::default(),
            independent_resolve: Default::default(),
            filter_minmax_single_component_formats: Default::default(),
            filter_minmax_image_component_mapping: Default::default(),
            max_timeline_semaphore_value_difference: Default::default(),
            framebuffer_integer_color_sample_counts: Default::default(),
        }
    }
}
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
#[doc(alias = "VkSemaphoreType")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreType.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreType(pub i32);
impl SemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
crate::enum_impl! {
    SemaphoreType : i32, BINARY, TIMELINE
}
#[doc(alias = "VkSemaphoreWaitFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreWaitFlags(pub u32);
impl SemaphoreWaitFlags {
    pub const ANY: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    SemaphoreWaitFlags : u32, 0x1, ANY
}
#[doc(alias = "VkSamplerReductionMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionMode.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SamplerReductionMode(pub i32);
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}
crate::enum_impl! {
    SamplerReductionMode : i32, WEIGHTED_AVERAGE, MIN, MAX
}
#[doc(alias = "VkDescriptorBindingFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorBindingFlags(pub u32);
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(1 << 0);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(1 << 1);
    pub const PARTIALLY_BOUND: Self = Self(1 << 2);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    DescriptorBindingFlags : u32, 0xf, UPDATE_AFTER_BIND, UPDATE_UNUSED_WHILE_PENDING,
    PARTIALLY_BOUND, VARIABLE_DESCRIPTOR_COUNT
}
#[doc(alias = "VkDriverId")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverId.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DriverId(pub i32);
impl DriverId {
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    pub const MESA_LLVMPIPE: Self = Self(13);
    pub const MOLTENVK: Self = Self(14);
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    pub const JUICE_PROPRIETARY: Self = Self(16);
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    pub const MESA_TURNIP: Self = Self(18);
    pub const MESA_V3DV: Self = Self(19);
    pub const MESA_PANVK: Self = Self(20);
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const MESA_VENUS: Self = Self(22);
    pub const MESA_DOZEN: Self = Self(23);
}
crate::enum_impl! {
    DriverId : i32, AMD_PROPRIETARY, AMD_OPEN_SOURCE, MESA_RADV, NVIDIA_PROPRIETARY,
    INTEL_PROPRIETARY_WINDOWS, INTEL_OPEN_SOURCE_MESA, IMAGINATION_PROPRIETARY,
    QUALCOMM_PROPRIETARY, ARM_PROPRIETARY, GOOGLE_SWIFTSHADER, GGP_PROPRIETARY,
    BROADCOM_PROPRIETARY, MESA_LLVMPIPE, MOLTENVK, COREAVI_PROPRIETARY,
    JUICE_PROPRIETARY, VERISILICON_PROPRIETARY, MESA_TURNIP, MESA_V3DV, MESA_PANVK,
    SAMSUNG_PROPRIETARY, MESA_VENUS, MESA_DOZEN
}
#[doc(alias = "VkResolveModeFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ResolveModeFlags(pub u32);
impl ResolveModeFlags {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(1 << 0);
    pub const AVERAGE: Self = Self(1 << 1);
    pub const MIN: Self = Self(1 << 2);
    pub const MAX: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    ResolveModeFlags : u32, 0xf, NONE, SAMPLE_ZERO, AVERAGE, MIN, MAX
}
#[doc(alias = "VkShaderFloatControlsIndependence")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderFloatControlsIndependence(pub i32);
impl ShaderFloatControlsIndependence {
    pub const I32_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}
crate::enum_impl! {
    ShaderFloatControlsIndependence : i32, I32_ONLY, ALL, NONE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkResetQueryPool")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)
pub unsafe fn reset_query_pool(
    device: crate::vk10::Device,
    query_pool: crate::vk10::QueryPool,
    first_query: u32,
    query_count: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .reset_query_pool
        .unwrap())(device, query_pool, first_query, query_count)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkResetQueryPool")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)
    pub unsafe fn reset_query_pool(
        &self,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        let reset_query_pool = (*self.table).reset_query_pool.unwrap();
        reset_query_pool(self.handle, query_pool, first_query as _, query_count as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateRenderPass2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html)
pub unsafe fn create_render_pass_2(
    device: crate::vk10::Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_render_pass: *mut crate::vk10::RenderPass,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_render_pass_2
        .unwrap())(device, p_create_info, p_allocator, p_render_pass)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html)
    pub unsafe fn create_render_pass_2(
        &self,
        create_info: &RenderPassCreateInfo2,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk10::RenderPass> {
        let create_render_pass_2 = (*self.table).create_render_pass_2.unwrap();
        let mut render_pass = Default::default();
        let result = create_render_pass_2(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut render_pass,
        );
        crate::new_result(render_pass, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRenderPass2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html)
pub unsafe fn cmd_begin_render_pass_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_render_pass_begin: *const crate::vk10::RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_render_pass_2
        .unwrap())(command_buffer, p_render_pass_begin, p_subpass_begin_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html)
    pub unsafe fn cmd_begin_render_pass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        render_pass_begin: &crate::vk10::RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) {
        let cmd_begin_render_pass_2 = (*self.table).cmd_begin_render_pass_2.unwrap();
        cmd_begin_render_pass_2(
            command_buffer,
            render_pass_begin as _,
            subpass_begin_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdNextSubpass2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html)
pub unsafe fn cmd_next_subpass_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_next_subpass_2
        .unwrap())(command_buffer, p_subpass_begin_info, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdNextSubpass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html)
    pub unsafe fn cmd_next_subpass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) {
        let cmd_next_subpass_2 = (*self.table).cmd_next_subpass_2.unwrap();
        cmd_next_subpass_2(
            command_buffer,
            subpass_begin_info as _,
            subpass_end_info as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRenderPass2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html)
pub unsafe fn cmd_end_render_pass_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_render_pass_2
        .unwrap())(command_buffer, p_subpass_end_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRenderPass2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html)
    pub unsafe fn cmd_end_render_pass_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) {
        let cmd_end_render_pass_2 = (*self.table).cmd_end_render_pass_2.unwrap();
        cmd_end_render_pass_2(command_buffer, subpass_end_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetSemaphoreCounterValue")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html)
pub unsafe fn get_semaphore_counter_value(
    device: crate::vk10::Device,
    semaphore: crate::vk10::Semaphore,
    p_value: *mut u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_semaphore_counter_value
        .unwrap())(device, semaphore, p_value)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html)
    pub unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: crate::vk10::Semaphore,
    ) -> crate::VulkanResult<u64> {
        let get_semaphore_counter_value = (*self.table)
            .get_semaphore_counter_value
            .unwrap();
        let mut value = Default::default();
        let result = get_semaphore_counter_value(self.handle, semaphore, &mut value);
        crate::new_result(value, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWaitSemaphores")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)
pub unsafe fn wait_semaphores(
    device: crate::vk10::Device,
    p_wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .wait_semaphores
        .unwrap())(device, p_wait_info, timeout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWaitSemaphores")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let wait_semaphores = (*self.table).wait_semaphores.unwrap();
        let result = wait_semaphores(self.handle, wait_info as _, timeout as _);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSignalSemaphore")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html)
pub unsafe fn signal_semaphore(
    device: crate::vk10::Device,
    p_signal_info: *const SemaphoreSignalInfo,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .signal_semaphore
        .unwrap())(device, p_signal_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSignalSemaphore")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html)
    pub unsafe fn signal_semaphore(
        &self,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::VulkanResult<()> {
        let signal_semaphore = (*self.table).signal_semaphore.unwrap();
        let result = signal_semaphore(self.handle, signal_info as _);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndirectCount")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html)
pub unsafe fn cmd_draw_indirect_count(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
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
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
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
        let cmd_draw_indirect_count = (*self.table).cmd_draw_indirect_count.unwrap();
        cmd_draw_indirect_count(
            command_buffer,
            buffer,
            offset as _,
            count_buffer,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html)
pub unsafe fn cmd_draw_indexed_indirect_count(
    command_buffer: crate::vk10::CommandBuffer,
    buffer: crate::vk10::Buffer,
    offset: crate::vk10::DeviceSize,
    count_buffer: crate::vk10::Buffer,
    count_buffer_offset: crate::vk10::DeviceSize,
    max_draw_count: u32,
    stride: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
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
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
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
        let cmd_draw_indexed_indirect_count = (*self.table)
            .cmd_draw_indexed_indirect_count
            .unwrap();
        cmd_draw_indexed_indirect_count(
            command_buffer,
            buffer,
            offset as _,
            count_buffer,
            count_buffer_offset as _,
            max_draw_count as _,
            stride as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
pub unsafe fn get_buffer_opaque_capture_address(
    device: crate::vk10::Device,
    p_info: *const BufferDeviceAddressInfo,
) -> u64 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_opaque_capture_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &BufferDeviceAddressInfo,
    ) {
        let get_buffer_opaque_capture_address = (*self.table)
            .get_buffer_opaque_capture_address
            .unwrap();
        get_buffer_opaque_capture_address(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferDeviceAddress")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)
pub unsafe fn get_buffer_device_address(
    device: crate::vk10::Device,
    p_info: *const BufferDeviceAddressInfo,
) -> crate::vk10::DeviceAddress {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_device_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetBufferDeviceAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)
    pub unsafe fn get_buffer_device_address(&self, info: &BufferDeviceAddressInfo) {
        let get_buffer_device_address = (*self.table).get_buffer_device_address.unwrap();
        get_buffer_device_address(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
pub unsafe fn get_device_memory_opaque_capture_address(
    device: crate::vk10::Device,
    p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_memory_opaque_capture_address
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) {
        let get_device_memory_opaque_capture_address = (*self.table)
            .get_device_memory_opaque_capture_address
            .unwrap();
        get_device_memory_opaque_capture_address(self.handle, info as _);
    }
}
