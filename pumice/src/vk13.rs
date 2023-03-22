pub const API_VERSION_1_3: u32 = crate::vk10::make_api_version(0, 1, 3, 0);
#[doc(alias = "VkFlags64")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags64.html)
pub type Flags64 = u64;
#[doc(alias = "VkPrivateDataSlotCreateFlags")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlags.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PrivateDataSlotCreateFlags(pub u32);
crate::bitflags_impl! {
    PrivateDataSlotCreateFlags : u32, 0x0,
}
crate::dispatchable_handle!(
    PrivateDataSlot, PRIVATE_DATA_SLOT, "VkPrivateDataSlot",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDevicePrivateDataCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfo.html)
pub struct DevicePrivateDataCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub private_data_slot_request_count: u32,
}
impl Default for DevicePrivateDataCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO,
            p_next: std::ptr::null(),
            private_data_slot_request_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPrivateDataSlotCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfo.html)
pub struct PrivateDataSlotCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PrivateDataSlotCreateFlags,
}
impl Default for PrivateDataSlotCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PRIVATE_DATA_SLOT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePrivateDataFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeatures.html)
pub struct PhysicalDevicePrivateDataFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub private_data: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePrivateDataFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES,
            p_next: std::ptr::null_mut(),
            private_data: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceBufferMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html)
pub struct DeviceBufferMemoryRequirements {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_create_info: *const crate::vk10::BufferCreateInfo,
}
impl Default for DeviceBufferMemoryRequirements {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS,
            p_next: std::ptr::null(),
            p_create_info: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDeviceImageMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html)
pub struct DeviceImageMemoryRequirements {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_create_info: *const crate::vk10::ImageCreateInfo,
    pub plane_aspect: crate::vk10::ImageAspectFlags,
}
impl Default for DeviceImageMemoryRequirements {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS,
            p_next: std::ptr::null(),
            p_create_info: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeatures.html)
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub inline_uniform_block: crate::vk10::Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceInlineUniformBlockFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
            p_next: std::ptr::null_mut(),
            inline_uniform_block: Default::default(),
            descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockProperties.html)
pub struct PhysicalDeviceInlineUniformBlockProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl Default for PhysicalDeviceInlineUniformBlockProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_inline_uniform_block_size: Default::default(),
            max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
            max_descriptor_set_inline_uniform_blocks: Default::default(),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlock")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetInlineUniformBlock.html)
pub struct WriteDescriptorSetInlineUniformBlock {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub data_size: u32,
    pub p_data: *const std::os::raw::c_void,
}
impl Default for WriteDescriptorSetInlineUniformBlock {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
            p_next: std::ptr::null(),
            data_size: Default::default(),
            p_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html)
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_inline_uniform_block_bindings: u32,
}
impl Default for DescriptorPoolInlineUniformBlockCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
            p_next: std::ptr::null(),
            max_inline_uniform_block_bindings: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMaintenance4Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Features.html)
pub struct PhysicalDeviceMaintenance4Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub maintenance_4: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMaintenance4Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES,
            p_next: std::ptr::null_mut(),
            maintenance_4: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMaintenance4Properties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Properties.html)
pub struct PhysicalDeviceMaintenance4Properties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_buffer_size: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceMaintenance4Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
            p_next: std::ptr::null_mut(),
            max_buffer_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTextureCompressionASTCHDRFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html)
pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub texture_compression_astc_hdr: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTextureCompressionASTCHDRFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES,
            p_next: std::ptr::null_mut(),
            texture_compression_astc_hdr: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkPipelineCreationFeedback")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html)
pub struct PipelineCreationFeedback {
    pub flags: PipelineCreationFeedbackFlags,
    pub duration: u64,
}
impl Default for PipelineCreationFeedback {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            duration: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCreationFeedbackCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackCreateInfo.html)
pub struct PipelineCreationFeedbackCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}
impl Default for PipelineCreationFeedbackCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
            p_next: std::ptr::null(),
            p_pipeline_creation_feedback: std::ptr::null_mut(),
            pipeline_stage_creation_feedback_count: Default::default(),
            p_pipeline_stage_creation_feedbacks: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html)
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_demote_to_helper_invocation: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_demote_to_helper_invocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html)
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub storage_texel_buffer_offset_alignment_bytes: crate::vk10::DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: crate::vk10::Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: crate::vk10::DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTexelBufferAlignmentProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            storage_texel_buffer_offset_alignment_bytes: Default::default(),
            storage_texel_buffer_offset_single_texel_alignment: Default::default(),
            uniform_texel_buffer_offset_alignment_bytes: Default::default(),
            uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeatures.html)
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subgroup_size_control: crate::vk10::Bool32,
    pub compute_full_subgroups: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSubgroupSizeControlFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
            p_next: std::ptr::null_mut(),
            subgroup_size_control: Default::default(),
            compute_full_subgroups: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html)
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: crate::vk10::ShaderStageFlags,
}
impl Default for PhysicalDeviceSubgroupSizeControlProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
            p_next: std::ptr::null_mut(),
            min_subgroup_size: Default::default(),
            max_subgroup_size: Default::default(),
            max_compute_workgroup_subgroups: Default::default(),
            required_subgroup_size_stages: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html)
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub required_subgroup_size: u32,
}
impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
            p_next: std::ptr::null_mut(),
            required_subgroup_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html)
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_creation_cache_control: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePipelineCreationCacheControlFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES,
            p_next: std::ptr::null_mut(),
            pipeline_creation_cache_control: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan13Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html)
pub struct PhysicalDeviceVulkan13Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub robust_image_access: crate::vk10::Bool32,
    pub inline_uniform_block: crate::vk10::Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: crate::vk10::Bool32,
    pub pipeline_creation_cache_control: crate::vk10::Bool32,
    pub private_data: crate::vk10::Bool32,
    pub shader_demote_to_helper_invocation: crate::vk10::Bool32,
    pub shader_terminate_invocation: crate::vk10::Bool32,
    pub subgroup_size_control: crate::vk10::Bool32,
    pub compute_full_subgroups: crate::vk10::Bool32,
    pub synchronization_2: crate::vk10::Bool32,
    pub texture_compression_astc_hdr: crate::vk10::Bool32,
    pub shader_zero_initialize_workgroup_memory: crate::vk10::Bool32,
    pub dynamic_rendering: crate::vk10::Bool32,
    pub shader_integer_dot_product: crate::vk10::Bool32,
    pub maintenance_4: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceVulkan13Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
            p_next: std::ptr::null_mut(),
            robust_image_access: Default::default(),
            inline_uniform_block: Default::default(),
            descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
            pipeline_creation_cache_control: Default::default(),
            private_data: Default::default(),
            shader_demote_to_helper_invocation: Default::default(),
            shader_terminate_invocation: Default::default(),
            subgroup_size_control: Default::default(),
            compute_full_subgroups: Default::default(),
            synchronization_2: Default::default(),
            texture_compression_astc_hdr: Default::default(),
            shader_zero_initialize_workgroup_memory: Default::default(),
            dynamic_rendering: Default::default(),
            shader_integer_dot_product: Default::default(),
            maintenance_4: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceVulkan13Properties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Properties.html)
pub struct PhysicalDeviceVulkan13Properties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: crate::vk10::ShaderStageFlags,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    pub max_inline_uniform_total_size: u32,
    pub integer_dot_product_8_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_8_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_8_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub storage_texel_buffer_offset_alignment_bytes: crate::vk10::DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: crate::vk10::Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: crate::vk10::DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: crate::vk10::Bool32,
    pub max_buffer_size: crate::vk10::DeviceSize,
}
impl Default for PhysicalDeviceVulkan13Properties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES,
            p_next: std::ptr::null_mut(),
            min_subgroup_size: Default::default(),
            max_subgroup_size: Default::default(),
            max_compute_workgroup_subgroups: Default::default(),
            required_subgroup_size_stages: Default::default(),
            max_inline_uniform_block_size: Default::default(),
            max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
            max_descriptor_set_inline_uniform_blocks: Default::default(),
            max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
            max_inline_uniform_total_size: Default::default(),
            integer_dot_product_8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_8_bit_signed_accelerated: Default::default(),
            integer_dot_product_8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_16_bit_signed_accelerated: Default::default(),
            integer_dot_product_16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_32_bit_signed_accelerated: Default::default(),
            integer_dot_product_32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_64_bit_signed_accelerated: Default::default(),
            integer_dot_product_64_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Default::default(),
            storage_texel_buffer_offset_alignment_bytes: Default::default(),
            storage_texel_buffer_offset_single_texel_alignment: Default::default(),
            uniform_texel_buffer_offset_alignment_bytes: Default::default(),
            uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
            max_buffer_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceToolProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolProperties.html)
pub struct PhysicalDeviceToolProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub name: [std::os::raw::c_char; crate::vk10::MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [std::os::raw::c_char; crate::vk10::MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: ToolPurposeFlags,
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub layer: [std::os::raw::c_char; crate::vk10::MAX_EXTENSION_NAME_SIZE as usize],
}
impl Default for PhysicalDeviceToolProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            version: unsafe { std::mem::zeroed() },
            purposes: Default::default(),
            description: unsafe { std::mem::zeroed() },
            layer: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html)
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_zero_initialize_workgroup_memory: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_zero_initialize_workgroup_memory: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageRobustnessFeatures.html)
pub struct PhysicalDeviceImageRobustnessFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub robust_image_access: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImageRobustnessFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES,
            p_next: std::ptr::null_mut(),
            robust_image_access: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCopy2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2.html)
pub struct BufferCopy2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_offset: crate::vk10::DeviceSize,
    pub dst_offset: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
}
impl Default for BufferCopy2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COPY_2,
            p_next: std::ptr::null(),
            src_offset: Default::default(),
            dst_offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageCopy2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy2.html)
pub struct ImageCopy2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_subresource: crate::vk10::ImageSubresourceLayers,
    pub src_offset: crate::vk10::Offset3D,
    pub dst_subresource: crate::vk10::ImageSubresourceLayers,
    pub dst_offset: crate::vk10::Offset3D,
    pub extent: crate::vk10::Extent3D,
}
impl Default for ImageCopy2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_COPY_2,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageBlit2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit2.html)
pub struct ImageBlit2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_subresource: crate::vk10::ImageSubresourceLayers,
    pub src_offsets: [crate::vk10::Offset3D; 2],
    pub dst_subresource: crate::vk10::ImageSubresourceLayers,
    pub dst_offsets: [crate::vk10::Offset3D; 2],
}
impl Default for ImageBlit2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_BLIT_2,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offsets: unsafe { std::mem::zeroed() },
            dst_subresource: Default::default(),
            dst_offsets: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferImageCopy2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html)
pub struct BufferImageCopy2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer_offset: crate::vk10::DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: crate::vk10::ImageSubresourceLayers,
    pub image_offset: crate::vk10::Offset3D,
    pub image_extent: crate::vk10::Extent3D,
}
impl Default for BufferImageCopy2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_IMAGE_COPY_2,
            p_next: std::ptr::null(),
            buffer_offset: Default::default(),
            buffer_row_length: Default::default(),
            buffer_image_height: Default::default(),
            image_subresource: Default::default(),
            image_offset: Default::default(),
            image_extent: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageResolve2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageResolve2.html)
pub struct ImageResolve2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_subresource: crate::vk10::ImageSubresourceLayers,
    pub src_offset: crate::vk10::Offset3D,
    pub dst_subresource: crate::vk10::ImageSubresourceLayers,
    pub dst_offset: crate::vk10::Offset3D,
    pub extent: crate::vk10::Extent3D,
}
impl Default for ImageResolve2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_RESOLVE_2,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyBufferInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2.html)
pub struct CopyBufferInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_buffer: crate::vk10::Buffer,
    pub dst_buffer: crate::vk10::Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferCopy2,
}
impl Default for CopyBufferInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_BUFFER_INFO_2,
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_buffer: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyImageInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2.html)
pub struct CopyImageInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_image: crate::vk10::Image,
    pub src_image_layout: crate::vk10::ImageLayout,
    pub dst_image: crate::vk10::Image,
    pub dst_image_layout: crate::vk10::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageCopy2,
}
impl Default for CopyImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_IMAGE_INFO_2,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBlitImageInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2.html)
pub struct BlitImageInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_image: crate::vk10::Image,
    pub src_image_layout: crate::vk10::ImageLayout,
    pub dst_image: crate::vk10::Image,
    pub dst_image_layout: crate::vk10::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageBlit2,
    pub filter: crate::vk10::Filter,
}
impl Default for BlitImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BLIT_IMAGE_INFO_2,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
            filter: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyBufferToImageInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html)
pub struct CopyBufferToImageInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_buffer: crate::vk10::Buffer,
    pub dst_image: crate::vk10::Image,
    pub dst_image_layout: crate::vk10::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
impl Default for CopyBufferToImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_BUFFER_TO_IMAGE_INFO_2,
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyImageToBufferInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2.html)
pub struct CopyImageToBufferInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_image: crate::vk10::Image,
    pub src_image_layout: crate::vk10::ImageLayout,
    pub dst_buffer: crate::vk10::Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
impl Default for CopyImageToBufferInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_IMAGE_TO_BUFFER_INFO_2,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_buffer: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkResolveImageInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveImageInfo2.html)
pub struct ResolveImageInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_image: crate::vk10::Image,
    pub src_image_layout: crate::vk10::ImageLayout,
    pub dst_image: crate::vk10::Image,
    pub dst_image_layout: crate::vk10::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageResolve2,
}
impl Default for ResolveImageInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RESOLVE_IMAGE_INFO_2,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeatures.html)
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_terminate_invocation: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderTerminateInvocationFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_terminate_invocation: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryBarrier2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html)
pub struct MemoryBarrier2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
}
impl Default for MemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageMemoryBarrier2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html)
pub struct ImageMemoryBarrier2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub old_layout: crate::vk10::ImageLayout,
    pub new_layout: crate::vk10::ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: crate::vk10::Image,
    pub subresource_range: crate::vk10::ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            image: Default::default(),
            subresource_range: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferMemoryBarrier2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2.html)
pub struct BufferMemoryBarrier2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
}
impl Default for BufferMemoryBarrier2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            src_queue_family_index: Default::default(),
            dst_queue_family_index: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDependencyInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html)
pub struct DependencyInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub dependency_flags: crate::vk10::DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const MemoryBarrier2,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const BufferMemoryBarrier2,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const ImageMemoryBarrier2,
}
impl Default for DependencyInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DEPENDENCY_INFO,
            p_next: std::ptr::null(),
            dependency_flags: Default::default(),
            memory_barrier_count: Default::default(),
            p_memory_barriers: std::ptr::null(),
            buffer_memory_barrier_count: Default::default(),
            p_buffer_memory_barriers: std::ptr::null(),
            image_memory_barrier_count: Default::default(),
            p_image_memory_barriers: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSemaphoreSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfo.html)
pub struct SemaphoreSubmitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2,
    pub device_index: u32,
}
impl Default for SemaphoreSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SEMAPHORE_SUBMIT_INFO,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: Default::default(),
            stage_mask: Default::default(),
            device_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferSubmitInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html)
pub struct CommandBufferSubmitInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub command_buffer: crate::vk10::CommandBuffer,
    pub device_mask: u32,
}
impl Default for CommandBufferSubmitInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_SUBMIT_INFO,
            p_next: std::ptr::null(),
            command_buffer: Default::default(),
            device_mask: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubmitInfo2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html)
pub struct SubmitInfo2 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: SubmitFlags,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const CommandBufferSubmitInfo,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo,
}
impl Default for SubmitInfo2 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBMIT_INFO_2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            wait_semaphore_info_count: Default::default(),
            p_wait_semaphore_infos: std::ptr::null(),
            command_buffer_info_count: Default::default(),
            p_command_buffer_infos: std::ptr::null(),
            signal_semaphore_info_count: Default::default(),
            p_signal_semaphore_infos: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSynchronization2Features")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2Features.html)
pub struct PhysicalDeviceSynchronization2Features {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub synchronization_2: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSynchronization2Features {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
            p_next: std::ptr::null_mut(),
            synchronization_2: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeatures.html)
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_integer_dot_product: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderIntegerDotProductFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
            p_next: std::ptr::null_mut(),
            shader_integer_dot_product: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html)
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub integer_dot_product_8_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_8_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_8_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_16_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_32_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_64_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: crate::vk10::Bool32,
    pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderIntegerDotProductProperties {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES,
            p_next: std::ptr::null_mut(),
            integer_dot_product_8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_8_bit_signed_accelerated: Default::default(),
            integer_dot_product_8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_16_bit_signed_accelerated: Default::default(),
            integer_dot_product_16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_32_bit_signed_accelerated: Default::default(),
            integer_dot_product_32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_64_bit_signed_accelerated: Default::default(),
            integer_dot_product_64_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Default::default(),
            integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFormatProperties3")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties3.html)
pub struct FormatProperties3 {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub linear_tiling_features: FormatFeatureFlags2,
    pub optimal_tiling_features: FormatFeatureFlags2,
    pub buffer_features: FormatFeatureFlags2,
}
impl Default for FormatProperties3 {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FORMAT_PROPERTIES_3,
            p_next: std::ptr::null_mut(),
            linear_tiling_features: Default::default(),
            optimal_tiling_features: Default::default(),
            buffer_features: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineRenderingCreateInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfo.html)
pub struct PipelineRenderingCreateInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk10::Format,
    pub depth_attachment_format: crate::vk10::Format,
    pub stencil_attachment_format: crate::vk10::Format,
}
impl Default for PipelineRenderingCreateInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_RENDERING_CREATE_INFO,
            p_next: std::ptr::null(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingInfo.html)
pub struct RenderingInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: RenderingFlags,
    pub render_area: crate::vk10::Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const RenderingAttachmentInfo,
    pub p_depth_attachment: *const RenderingAttachmentInfo,
    pub p_stencil_attachment: *const RenderingAttachmentInfo,
}
impl Default for RenderingInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            render_area: Default::default(),
            layer_count: Default::default(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachments: std::ptr::null(),
            p_depth_attachment: std::ptr::null(),
            p_stencil_attachment: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderingAttachmentInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html)
pub struct RenderingAttachmentInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_view: crate::vk10::ImageView,
    pub image_layout: crate::vk10::ImageLayout,
    pub resolve_mode: crate::vk12::ResolveModeFlags,
    pub resolve_image_view: crate::vk10::ImageView,
    pub resolve_image_layout: crate::vk10::ImageLayout,
    pub load_op: crate::vk10::AttachmentLoadOp,
    pub store_op: crate::vk10::AttachmentStoreOp,
    pub clear_value: crate::vk10::ClearValue,
}
impl Default for RenderingAttachmentInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDERING_ATTACHMENT_INFO,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            resolve_mode: Default::default(),
            resolve_image_view: Default::default(),
            resolve_image_layout: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            clear_value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeatures")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeatures.html)
pub struct PhysicalDeviceDynamicRenderingFeatures {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub dynamic_rendering: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDynamicRenderingFeatures {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
            p_next: std::ptr::null_mut(),
            dynamic_rendering: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceRenderingInfo")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfo.html)
pub struct CommandBufferInheritanceRenderingInfo {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: RenderingFlags,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const crate::vk10::Format,
    pub depth_attachment_format: crate::vk10::Format,
    pub stencil_attachment_format: crate::vk10::Format,
    pub rasterization_samples: crate::vk10::SampleCountFlags,
}
impl Default for CommandBufferInheritanceRenderingInfo {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            view_mask: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
            rasterization_samples: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCreationFeedbackFlags(pub u32);
impl PipelineCreationFeedbackFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const VALID_EXT: Self = Self::VALID;
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(1 << 1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(1 << 2);
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
}
crate::bitflags_impl! {
    PipelineCreationFeedbackFlags : u32, 0x7, VALID, APPLICATION_PIPELINE_CACHE_HIT,
    BASE_PIPELINE_ACCELERATION
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ToolPurposeFlags(pub u32);
impl ToolPurposeFlags {
    pub const VALIDATION: Self = Self(1 << 0);
    pub const VALIDATION_EXT: Self = Self::VALIDATION;
    pub const PROFILING: Self = Self(1 << 1);
    pub const PROFILING_EXT: Self = Self::PROFILING;
    pub const TRACING: Self = Self(1 << 2);
    pub const TRACING_EXT: Self = Self::TRACING;
    pub const ADDITIONAL_FEATURES: Self = Self(1 << 3);
    pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
    pub const MODIFYING_FEATURES: Self = Self(1 << 4);
    pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
    /// ext_tooling_info
    pub const DEBUG_REPORTING_EXT: Self = Self(1 << 5);
    pub const DEBUG_MARKERS_EXT: Self = Self(1 << 6);
}
crate::bitflags_impl! {
    ToolPurposeFlags : u32, 0x7f, VALIDATION, PROFILING, TRACING, ADDITIONAL_FEATURES,
    MODIFYING_FEATURES, DEBUG_REPORTING_EXT, DEBUG_MARKERS_EXT
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccessFlags2(pub u64);
impl AccessFlags2 {
    pub const NONE: Self = Self(0);
    pub const NONE_KHR: Self = Self::NONE;
    pub const INDIRECT_COMMAND_READ: Self = Self(1 << 0);
    pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
    pub const INDEX_READ: Self = Self(1 << 1);
    pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(1 << 2);
    pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
    pub const UNIFORM_READ: Self = Self(1 << 3);
    pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
    pub const INPUT_ATTACHMENT_READ: Self = Self(1 << 4);
    pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
    pub const SHADER_READ: Self = Self(1 << 5);
    pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
    pub const SHADER_WRITE: Self = Self(1 << 6);
    pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
    pub const COLOR_ATTACHMENT_READ: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(1 << 8);
    pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(1 << 9);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1 << 10);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
    pub const TRANSFER_READ: Self = Self(1 << 11);
    pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
    pub const TRANSFER_WRITE: Self = Self(1 << 12);
    pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
    pub const HOST_READ: Self = Self(1 << 13);
    pub const HOST_READ_KHR: Self = Self::HOST_READ;
    pub const HOST_WRITE: Self = Self(1 << 14);
    pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
    pub const MEMORY_READ: Self = Self(1 << 15);
    pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
    pub const MEMORY_WRITE: Self = Self(1 << 16);
    pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
    pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
    pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
    pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
    pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
    pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
    pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
    /// khr_video_decode_queue
    pub const VIDEO_DECODE_READ_KHR: Self = Self(1 << 35);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(1 << 36);
    /// khr_video_encode_queue
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(1 << 37);
    pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(1 << 38);
    /// khr_synchronization2
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(1 << 25);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(1 << 26);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(1 << 27);
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1 << 20);
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(1 << 17);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(1 << 18);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(1 << 23);
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(1 << 21);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(1 << 22);
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(1 << 24);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(1 << 19);
    /// huawei_invocation_mask
    pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(1 << 39);
    /// ext_opacity_micromap
    pub const MICROMAP_READ_EXT: Self = Self(1 << 44);
    pub const MICROMAP_WRITE_EXT: Self = Self(1 << 45);
    /// nv_optical_flow
    pub const OPTICAL_FLOW_READ_NV: Self = Self(1 << 42);
    pub const OPTICAL_FLOW_WRITE_NV: Self = Self(1 << 43);
}
crate::bitflags_impl! {
    AccessFlags2 : u64, 0x3cff0fffffff, NONE, INDIRECT_COMMAND_READ, INDEX_READ,
    VERTEX_ATTRIBUTE_READ, UNIFORM_READ, INPUT_ATTACHMENT_READ, SHADER_READ,
    SHADER_WRITE, COLOR_ATTACHMENT_READ, COLOR_ATTACHMENT_WRITE,
    DEPTH_STENCIL_ATTACHMENT_READ, DEPTH_STENCIL_ATTACHMENT_WRITE, TRANSFER_READ,
    TRANSFER_WRITE, HOST_READ, HOST_WRITE, MEMORY_READ, MEMORY_WRITE,
    SHADER_SAMPLED_READ, SHADER_STORAGE_READ, SHADER_STORAGE_WRITE,
    VIDEO_DECODE_READ_KHR, VIDEO_DECODE_WRITE_KHR, VIDEO_ENCODE_READ_KHR,
    VIDEO_ENCODE_WRITE_KHR, TRANSFORM_FEEDBACK_WRITE_EXT,
    TRANSFORM_FEEDBACK_COUNTER_READ_EXT, TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT,
    CONDITIONAL_RENDERING_READ_EXT, COMMAND_PREPROCESS_READ_NV,
    COMMAND_PREPROCESS_WRITE_NV, FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR,
    ACCELERATION_STRUCTURE_READ_KHR, ACCELERATION_STRUCTURE_WRITE_KHR,
    FRAGMENT_DENSITY_MAP_READ_EXT, COLOR_ATTACHMENT_READ_NONCOHERENT_EXT,
    INVOCATION_MASK_READ_HUAWEI, MICROMAP_READ_EXT, MICROMAP_WRITE_EXT,
    OPTICAL_FLOW_READ_NV, OPTICAL_FLOW_WRITE_NV
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineStageFlags2(pub u64);
impl PipelineStageFlags2 {
    pub const NONE: Self = Self(0);
    pub const NONE_KHR: Self = Self::NONE;
    pub const TOP_OF_PIPE: Self = Self(1 << 0);
    pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
    pub const DRAW_INDIRECT: Self = Self(1 << 1);
    pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
    pub const VERTEX_INPUT: Self = Self(1 << 2);
    pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
    pub const VERTEX_SHADER: Self = Self(1 << 3);
    pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(1 << 4);
    pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(1 << 5);
    pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
    pub const GEOMETRY_SHADER: Self = Self(1 << 6);
    pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
    pub const FRAGMENT_SHADER: Self = Self(1 << 7);
    pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
    pub const EARLY_FRAGMENT_TESTS: Self = Self(1 << 8);
    pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
    pub const LATE_FRAGMENT_TESTS: Self = Self(1 << 9);
    pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1 << 10);
    pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
    pub const COMPUTE_SHADER: Self = Self(1 << 11);
    pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
    pub const ALL_TRANSFER: Self = Self(1 << 12);
    pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    pub const TRANSFER: Self = Self::ALL_TRANSFER_KHR;
    pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
    pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
    pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
    pub const HOST: Self = Self(1 << 14);
    pub const HOST_KHR: Self = Self::HOST;
    pub const ALL_GRAPHICS: Self = Self(1 << 15);
    pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
    pub const ALL_COMMANDS: Self = Self(1 << 16);
    pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
    pub const COPY: Self = Self(1 << 32);
    pub const COPY_KHR: Self = Self::COPY;
    pub const RESOLVE: Self = Self(1 << 33);
    pub const RESOLVE_KHR: Self = Self::RESOLVE;
    pub const BLIT: Self = Self(1 << 34);
    pub const BLIT_KHR: Self = Self::BLIT;
    pub const CLEAR: Self = Self(1 << 35);
    pub const CLEAR_KHR: Self = Self::CLEAR;
    pub const INDEX_INPUT: Self = Self(1 << 36);
    pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
    pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
    pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
    /// khr_video_decode_queue
    pub const VIDEO_DECODE_KHR: Self = Self(1 << 26);
    /// khr_video_encode_queue
    pub const VIDEO_ENCODE_KHR: Self = Self(1 << 27);
    /// khr_synchronization2
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
    pub const COMMAND_PREPROCESS_NV: Self = Self(1 << 17);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
    pub const TASK_SHADER_EXT: Self = Self(1 << 19);
    pub const MESH_SHADER_EXT: Self = Self(1 << 20);
    /// huawei_subpass_shading
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1 << 39);
    /// huawei_invocation_mask
    pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 40);
    /// khr_ray_tracing_maintenance1
    pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(1 << 28);
    /// ext_opacity_micromap
    pub const MICROMAP_BUILD_EXT: Self = Self(1 << 30);
    /// nv_optical_flow
    pub const OPTICAL_FLOW_NV: Self = Self(1 << 29);
}
crate::bitflags_impl! {
    PipelineStageFlags2 : u64, 0x1ff7fffffff, NONE, TOP_OF_PIPE, DRAW_INDIRECT,
    VERTEX_INPUT, VERTEX_SHADER, TESSELLATION_CONTROL_SHADER,
    TESSELLATION_EVALUATION_SHADER, GEOMETRY_SHADER, FRAGMENT_SHADER,
    EARLY_FRAGMENT_TESTS, LATE_FRAGMENT_TESTS, COLOR_ATTACHMENT_OUTPUT, COMPUTE_SHADER,
    ALL_TRANSFER, BOTTOM_OF_PIPE, HOST, ALL_GRAPHICS, ALL_COMMANDS, COPY, RESOLVE, BLIT,
    CLEAR, INDEX_INPUT, VERTEX_ATTRIBUTE_INPUT, PRE_RASTERIZATION_SHADERS,
    VIDEO_DECODE_KHR, VIDEO_ENCODE_KHR, TRANSFORM_FEEDBACK_EXT,
    CONDITIONAL_RENDERING_EXT, COMMAND_PREPROCESS_NV,
    FRAGMENT_SHADING_RATE_ATTACHMENT_KHR, ACCELERATION_STRUCTURE_BUILD_KHR,
    RAY_TRACING_SHADER_KHR, FRAGMENT_DENSITY_PROCESS_EXT, TASK_SHADER_EXT,
    MESH_SHADER_EXT, SUBPASS_SHADING_HUAWEI, INVOCATION_MASK_HUAWEI,
    ACCELERATION_STRUCTURE_COPY_KHR, MICROMAP_BUILD_EXT, OPTICAL_FLOW_NV
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubmitFlags(pub u32);
impl SubmitFlags {
    pub const PROTECTED: Self = Self(1 << 0);
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
}
crate::bitflags_impl! {
    SubmitFlags : u32, 0x1, PROTECTED
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FormatFeatureFlags2(pub u64);
impl FormatFeatureFlags2 {
    pub const SAMPLED_IMAGE: Self = Self(1 << 0);
    pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
    pub const STORAGE_IMAGE: Self = Self(1 << 1);
    pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(1 << 2);
    pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(1 << 5);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
    pub const VERTEX_BUFFER: Self = Self(1 << 6);
    pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    pub const COLOR_ATTACHMENT: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(1 << 8);
    pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 9);
    pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
    pub const BLIT_SRC: Self = Self(1 << 10);
    pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
    pub const BLIT_DST: Self = Self(1 << 11);
    pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(1 << 12);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(1 << 13);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
    pub const TRANSFER_SRC: Self = Self(1 << 14);
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST: Self = Self(1 << 15);
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(1 << 16);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(1 << 17);
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(1 << 18);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(
        1 << 19,
    );
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(
        1 << 20,
    );
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(
        1 << 21,
    );
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const DISJOINT: Self = Self(1 << 22);
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const COSITED_CHROMA_SAMPLES: Self = Self(1 << 23);
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(1 << 31);
    pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(1 << 32);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(1 << 33);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
    /// khr_video_decode_queue
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(1 << 25);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 26);
    /// khr_acceleration_structure
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(1 << 29);
    /// ext_fragment_density_map
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 24);
    /// khr_fragment_shading_rate
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 30);
    /// khr_video_encode_queue
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(1 << 27);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 28);
    /// nv_linear_color_attachment
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(1 << 38);
    /// qcom_image_processing
    pub const WEIGHT_IMAGE_QCOM: Self = Self(1 << 34);
    pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(1 << 35);
    pub const BLOCK_MATCHING_QCOM: Self = Self(1 << 36);
    pub const BOX_FILTER_SAMPLED_QCOM: Self = Self(1 << 37);
    /// nv_optical_flow
    pub const OPTICAL_FLOW_IMAGE_NV: Self = Self(1 << 40);
    pub const OPTICAL_FLOW_VECTOR_NV: Self = Self(1 << 41);
    pub const OPTICAL_FLOW_COST_NV: Self = Self(1 << 42);
}
crate::bitflags_impl! {
    FormatFeatureFlags2 : u64, 0x77fffffffff, SAMPLED_IMAGE, STORAGE_IMAGE,
    STORAGE_IMAGE_ATOMIC, UNIFORM_TEXEL_BUFFER, STORAGE_TEXEL_BUFFER,
    STORAGE_TEXEL_BUFFER_ATOMIC, VERTEX_BUFFER, COLOR_ATTACHMENT, COLOR_ATTACHMENT_BLEND,
    DEPTH_STENCIL_ATTACHMENT, BLIT_SRC, BLIT_DST, SAMPLED_IMAGE_FILTER_LINEAR,
    SAMPLED_IMAGE_FILTER_CUBIC, TRANSFER_SRC, TRANSFER_DST, SAMPLED_IMAGE_FILTER_MINMAX,
    MIDPOINT_CHROMA_SAMPLES, SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER,
    SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER,
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT,
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE, DISJOINT,
    COSITED_CHROMA_SAMPLES, STORAGE_READ_WITHOUT_FORMAT, STORAGE_WRITE_WITHOUT_FORMAT,
    SAMPLED_IMAGE_DEPTH_COMPARISON, VIDEO_DECODE_OUTPUT_KHR, VIDEO_DECODE_DPB_KHR,
    ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR, FRAGMENT_DENSITY_MAP_EXT,
    FRAGMENT_SHADING_RATE_ATTACHMENT_KHR, VIDEO_ENCODE_INPUT_KHR, VIDEO_ENCODE_DPB_KHR,
    LINEAR_COLOR_ATTACHMENT_NV, WEIGHT_IMAGE_QCOM, WEIGHT_SAMPLED_IMAGE_QCOM,
    BLOCK_MATCHING_QCOM, BOX_FILTER_SAMPLED_QCOM, OPTICAL_FLOW_IMAGE_NV,
    OPTICAL_FLOW_VECTOR_NV, OPTICAL_FLOW_COST_NV
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct RenderingFlags(pub u32);
impl RenderingFlags {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1 << 0);
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
    pub const SUSPENDING: Self = Self(1 << 1);
    pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    pub const RESUMING: Self = Self(1 << 2);
    pub const RESUMING_KHR: Self = Self::RESUMING;
    /// ext_legacy_dithering
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    RenderingFlags : u32, 0xf, CONTENTS_SECONDARY_COMMAND_BUFFERS, SUSPENDING, RESUMING,
    ENABLE_LEGACY_DITHERING_EXT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)
pub unsafe fn get_device_buffer_memory_requirements(
    device: crate::vk10::Device,
    p_info: *const DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_buffer_memory_requirements
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_device_buffer_memory_requirements = (*self.table)
            .get_device_buffer_memory_requirements
            .unwrap();
        let mut memory_requirements = Default::default();
        get_device_buffer_memory_requirements(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)
pub unsafe fn get_device_image_memory_requirements(
    device: crate::vk10::Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_image_memory_requirements
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_device_image_memory_requirements = (*self.table)
            .get_device_image_memory_requirements
            .unwrap();
        let mut memory_requirements = Default::default();
        get_device_image_memory_requirements(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html)
pub unsafe fn get_device_image_sparse_memory_requirements(
    device: crate::vk10::Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_image_sparse_memory_requirements
        .unwrap())(
        device,
        p_info,
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html)
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirement_count: Option<u32>,
        mut sparse_memory_requirements_callback: impl FnMut(
            &mut Vec<crate::vk11::SparseImageMemoryRequirements2>,
        ),
    ) -> Vec<crate::vk11::SparseImageMemoryRequirements2> {
        let get_device_image_sparse_memory_requirements = (*self.table)
            .get_device_image_sparse_memory_requirements
            .unwrap();
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_device_image_sparse_memory_requirements(
                    self.handle,
                    info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut sparse_memory_requirements = vec![
            Default::default(); sparse_memory_requirement_count as usize
        ];
        sparse_memory_requirements_callback(&mut sparse_memory_requirements);
        get_device_image_sparse_memory_requirements(
            self.handle,
            info as _,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html)
pub unsafe fn get_physical_device_tool_properties(
    physical_device: crate::vk10::PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut PhysicalDeviceToolProperties,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_tool_properties
        .unwrap())(physical_device, p_tool_count, p_tool_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html)
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        tool_count: Option<u32>,
        mut tool_properties_callback: impl FnMut(&mut Vec<PhysicalDeviceToolProperties>),
    ) -> crate::VulkanResult<(Vec<PhysicalDeviceToolProperties>, crate::vk10::Result)> {
        let get_physical_device_tool_properties = (*self.table)
            .get_physical_device_tool_properties
            .unwrap();
        let mut tool_count = match tool_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_tool_properties(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut tool_properties = vec![Default::default(); tool_count as usize];
        tool_properties_callback(&mut tool_properties);
        let result = get_physical_device_tool_properties(
            physical_device,
            &mut tool_count,
            tool_properties.as_mut_ptr(),
        );
        crate::new_result((tool_properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCullMode")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)
pub unsafe fn cmd_set_cull_mode(
    command_buffer: crate::vk10::CommandBuffer,
    cull_mode: crate::vk10::CullModeFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_cull_mode
        .unwrap())(command_buffer, cull_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCullMode")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        cull_mode: Option<crate::vk10::CullModeFlags>,
    ) {
        let cmd_set_cull_mode = (*self.table).cmd_set_cull_mode.unwrap();
        cmd_set_cull_mode(
            command_buffer,
            match cull_mode {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetFrontFace")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)
pub unsafe fn cmd_set_front_face(
    command_buffer: crate::vk10::CommandBuffer,
    front_face: crate::vk10::FrontFace,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_front_face
        .unwrap())(command_buffer, front_face)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetFrontFace")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)
    pub unsafe fn cmd_set_front_face(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        front_face: crate::vk10::FrontFace,
    ) {
        let cmd_set_front_face = (*self.table).cmd_set_front_face.unwrap();
        cmd_set_front_face(command_buffer, front_face as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPrimitiveTopology")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html)
pub unsafe fn cmd_set_primitive_topology(
    command_buffer: crate::vk10::CommandBuffer,
    primitive_topology: crate::vk10::PrimitiveTopology,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_primitive_topology
        .unwrap())(command_buffer, primitive_topology)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html)
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_topology: crate::vk10::PrimitiveTopology,
    ) {
        let cmd_set_primitive_topology = (*self.table)
            .cmd_set_primitive_topology
            .unwrap();
        cmd_set_primitive_topology(command_buffer, primitive_topology as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportWithCount")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)
pub unsafe fn cmd_set_viewport_with_count(
    command_buffer: crate::vk10::CommandBuffer,
    viewport_count: u32,
    p_viewports: *const crate::vk10::Viewport,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_with_count
        .unwrap())(command_buffer, viewport_count, p_viewports)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWithCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewports: &[crate::vk10::Viewport],
    ) {
        let cmd_set_viewport_with_count = (*self.table)
            .cmd_set_viewport_with_count
            .unwrap();
        let viewport_count = viewports.len();
        cmd_set_viewport_with_count(
            command_buffer,
            viewport_count as _,
            viewports.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetScissorWithCount")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)
pub unsafe fn cmd_set_scissor_with_count(
    command_buffer: crate::vk10::CommandBuffer,
    scissor_count: u32,
    p_scissors: *const crate::vk10::Rect2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_scissor_with_count
        .unwrap())(command_buffer, scissor_count, p_scissors)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetScissorWithCount")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        scissors: &[crate::vk10::Rect2D],
    ) {
        let cmd_set_scissor_with_count = (*self.table)
            .cmd_set_scissor_with_count
            .unwrap();
        let scissor_count = scissors.len();
        cmd_set_scissor_with_count(
            command_buffer,
            scissor_count as _,
            scissors.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindVertexBuffers2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html)
pub unsafe fn cmd_bind_vertex_buffers_2(
    command_buffer: crate::vk10::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk10::Buffer,
    p_offsets: *const crate::vk10::DeviceSize,
    p_sizes: *const crate::vk10::DeviceSize,
    p_strides: *const crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
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
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html)
    pub unsafe fn cmd_bind_vertex_buffers_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk10::Buffer],
        offsets: &[crate::vk10::DeviceSize],
        sizes: &[crate::vk10::DeviceSize],
        strides: &[crate::vk10::DeviceSize],
    ) {
        let cmd_bind_vertex_buffers_2 = (*self.table).cmd_bind_vertex_buffers_2.unwrap();
        let binding_count = buffers
            .len()
            .min(offsets.len())
            .min(sizes.len())
            .min(strides.len());
        cmd_bind_vertex_buffers_2(
            command_buffer,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr(),
            offsets.as_ptr(),
            sizes.as_ptr(),
            strides.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthTestEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)
pub unsafe fn cmd_set_depth_test_enable(
    command_buffer: crate::vk10::CommandBuffer,
    depth_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_test_enable
        .unwrap())(command_buffer, depth_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_test_enable: bool,
    ) {
        let cmd_set_depth_test_enable = (*self.table).cmd_set_depth_test_enable.unwrap();
        cmd_set_depth_test_enable(command_buffer, depth_test_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthWriteEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)
pub unsafe fn cmd_set_depth_write_enable(
    command_buffer: crate::vk10::CommandBuffer,
    depth_write_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_write_enable
        .unwrap())(command_buffer, depth_write_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_write_enable: bool,
    ) {
        let cmd_set_depth_write_enable = (*self.table)
            .cmd_set_depth_write_enable
            .unwrap();
        cmd_set_depth_write_enable(command_buffer, depth_write_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthCompareOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)
pub unsafe fn cmd_set_depth_compare_op(
    command_buffer: crate::vk10::CommandBuffer,
    depth_compare_op: crate::vk10::CompareOp,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_compare_op
        .unwrap())(command_buffer, depth_compare_op)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_compare_op: crate::vk10::CompareOp,
    ) {
        let cmd_set_depth_compare_op = (*self.table).cmd_set_depth_compare_op.unwrap();
        cmd_set_depth_compare_op(command_buffer, depth_compare_op as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html)
pub unsafe fn cmd_set_depth_bounds_test_enable(
    command_buffer: crate::vk10::CommandBuffer,
    depth_bounds_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bounds_test_enable
        .unwrap())(command_buffer, depth_bounds_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html)
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        let cmd_set_depth_bounds_test_enable = (*self.table)
            .cmd_set_depth_bounds_test_enable
            .unwrap();
        cmd_set_depth_bounds_test_enable(command_buffer, depth_bounds_test_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilTestEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html)
pub unsafe fn cmd_set_stencil_test_enable(
    command_buffer: crate::vk10::CommandBuffer,
    stencil_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_test_enable
        .unwrap())(command_buffer, stencil_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html)
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stencil_test_enable: bool,
    ) {
        let cmd_set_stencil_test_enable = (*self.table)
            .cmd_set_stencil_test_enable
            .unwrap();
        cmd_set_stencil_test_enable(command_buffer, stencil_test_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilOp")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html)
pub unsafe fn cmd_set_stencil_op(
    command_buffer: crate::vk10::CommandBuffer,
    face_mask: crate::vk10::StencilFaceFlags,
    fail_op: crate::vk10::StencilOp,
    pass_op: crate::vk10::StencilOp,
    depth_fail_op: crate::vk10::StencilOp,
    compare_op: crate::vk10::CompareOp,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
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
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
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
        let cmd_set_stencil_op = (*self.table).cmd_set_stencil_op.unwrap();
        cmd_set_stencil_op(
            command_buffer,
            face_mask as _,
            fail_op as _,
            pass_op as _,
            depth_fail_op as _,
            compare_op as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html)
pub unsafe fn cmd_set_rasterizer_discard_enable(
    command_buffer: crate::vk10::CommandBuffer,
    rasterizer_discard_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_rasterizer_discard_enable
        .unwrap())(command_buffer, rasterizer_discard_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html)
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        let cmd_set_rasterizer_discard_enable = (*self.table)
            .cmd_set_rasterizer_discard_enable
            .unwrap();
        cmd_set_rasterizer_discard_enable(
            command_buffer,
            rasterizer_discard_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBiasEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html)
pub unsafe fn cmd_set_depth_bias_enable(
    command_buffer: crate::vk10::CommandBuffer,
    depth_bias_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bias_enable
        .unwrap())(command_buffer, depth_bias_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html)
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bias_enable: bool,
    ) {
        let cmd_set_depth_bias_enable = (*self.table).cmd_set_depth_bias_enable.unwrap();
        cmd_set_depth_bias_enable(command_buffer, depth_bias_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html)
pub unsafe fn cmd_set_primitive_restart_enable(
    command_buffer: crate::vk10::CommandBuffer,
    primitive_restart_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_primitive_restart_enable
        .unwrap())(command_buffer, primitive_restart_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html)
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        let cmd_set_primitive_restart_enable = (*self.table)
            .cmd_set_primitive_restart_enable
            .unwrap();
        cmd_set_primitive_restart_enable(command_buffer, primitive_restart_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreatePrivateDataSlot")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html)
pub unsafe fn create_private_data_slot(
    device: crate::vk10::Device,
    p_create_info: *const PrivateDataSlotCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_private_data_slot: *mut PrivateDataSlot,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_private_data_slot
        .unwrap())(device, p_create_info, p_allocator, p_private_data_slot)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreatePrivateDataSlot")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html)
    pub unsafe fn create_private_data_slot(
        &self,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<PrivateDataSlot> {
        let create_private_data_slot = (*self.table).create_private_data_slot.unwrap();
        let mut private_data_slot = Default::default();
        let result = create_private_data_slot(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut private_data_slot,
        );
        crate::new_result(private_data_slot, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyPrivateDataSlot")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html)
pub unsafe fn destroy_private_data_slot(
    device: crate::vk10::Device,
    private_data_slot: PrivateDataSlot,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_private_data_slot
        .unwrap())(device, private_data_slot, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html)
    pub unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_private_data_slot = (*self.table).destroy_private_data_slot.unwrap();
        destroy_private_data_slot(
            self.handle,
            private_data_slot,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetPrivateData")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html)
pub unsafe fn set_private_data(
    device: crate::vk10::Device,
    object_type: crate::vk10::ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_private_data
        .unwrap())(device, object_type, object_handle, private_data_slot, data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetPrivateData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html)
    pub unsafe fn set_private_data(
        &self,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::VulkanResult<()> {
        let set_private_data = (*self.table).set_private_data.unwrap();
        let result = set_private_data(
            self.handle,
            object_type as _,
            object_handle as _,
            private_data_slot,
            data as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPrivateData")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html)
pub unsafe fn get_private_data(
    device: crate::vk10::Device,
    object_type: crate::vk10::ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    p_data: *mut u64,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_private_data
        .unwrap())(device, object_type, object_handle, private_data_slot, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetPrivateData")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html)
    pub unsafe fn get_private_data(
        &self,
        object_type: crate::vk10::ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        let get_private_data = (*self.table).get_private_data.unwrap();
        let mut data = Default::default();
        get_private_data(
            self.handle,
            object_type as _,
            object_handle as _,
            private_data_slot,
            &mut data,
        );
        data
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBuffer2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html)
pub unsafe fn cmd_copy_buffer_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_buffer_info: *const CopyBufferInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer_2
        .unwrap())(command_buffer, p_copy_buffer_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBuffer2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html)
    pub unsafe fn cmd_copy_buffer_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) {
        let cmd_copy_buffer_2 = (*self.table).cmd_copy_buffer_2.unwrap();
        cmd_copy_buffer_2(command_buffer, copy_buffer_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImage2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html)
pub unsafe fn cmd_copy_image_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_image_info: *const CopyImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image_2
        .unwrap())(command_buffer, p_copy_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html)
    pub unsafe fn cmd_copy_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) {
        let cmd_copy_image_2 = (*self.table).cmd_copy_image_2.unwrap();
        cmd_copy_image_2(command_buffer, copy_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBlitImage2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html)
pub unsafe fn cmd_blit_image_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_blit_image_info: *const BlitImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_blit_image_2
        .unwrap())(command_buffer, p_blit_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBlitImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html)
    pub unsafe fn cmd_blit_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        blit_image_info: &BlitImageInfo2,
    ) {
        let cmd_blit_image_2 = (*self.table).cmd_blit_image_2.unwrap();
        cmd_blit_image_2(command_buffer, blit_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyBufferToImage2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)
pub unsafe fn cmd_copy_buffer_to_image_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_buffer_to_image_2
        .unwrap())(command_buffer, p_copy_buffer_to_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)
    pub unsafe fn cmd_copy_buffer_to_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) {
        let cmd_copy_buffer_to_image_2 = (*self.table)
            .cmd_copy_buffer_to_image_2
            .unwrap();
        cmd_copy_buffer_to_image_2(command_buffer, copy_buffer_to_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyImageToBuffer2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)
pub unsafe fn cmd_copy_image_to_buffer_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_image_to_buffer_2
        .unwrap())(command_buffer, p_copy_image_to_buffer_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)
    pub unsafe fn cmd_copy_image_to_buffer_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) {
        let cmd_copy_image_to_buffer_2 = (*self.table)
            .cmd_copy_image_to_buffer_2
            .unwrap();
        cmd_copy_image_to_buffer_2(command_buffer, copy_image_to_buffer_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResolveImage2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html)
pub unsafe fn cmd_resolve_image_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_resolve_image_info: *const ResolveImageInfo2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_resolve_image_2
        .unwrap())(command_buffer, p_resolve_image_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResolveImage2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html)
    pub unsafe fn cmd_resolve_image_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        resolve_image_info: &ResolveImageInfo2,
    ) {
        let cmd_resolve_image_2 = (*self.table).cmd_resolve_image_2.unwrap();
        cmd_resolve_image_2(command_buffer, resolve_image_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetEvent2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html)
pub unsafe fn cmd_set_event_2(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    p_dependency_info: *const DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_event_2
        .unwrap())(command_buffer, event, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetEvent2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html)
    pub unsafe fn cmd_set_event_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        dependency_info: &DependencyInfo,
    ) {
        let cmd_set_event_2 = (*self.table).cmd_set_event_2.unwrap();
        cmd_set_event_2(command_buffer, event, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdResetEvent2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html)
pub unsafe fn cmd_reset_event_2(
    command_buffer: crate::vk10::CommandBuffer,
    event: crate::vk10::Event,
    stage_mask: PipelineStageFlags2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_reset_event_2
        .unwrap())(command_buffer, event, stage_mask)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdResetEvent2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html)
    pub unsafe fn cmd_reset_event_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        event: crate::vk10::Event,
        stage_mask: Option<PipelineStageFlags2>,
    ) {
        let cmd_reset_event_2 = (*self.table).cmd_reset_event_2.unwrap();
        cmd_reset_event_2(
            command_buffer,
            event,
            match stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWaitEvents2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html)
pub unsafe fn cmd_wait_events_2(
    command_buffer: crate::vk10::CommandBuffer,
    event_count: u32,
    p_events: *const crate::vk10::Event,
    p_dependency_infos: *const DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_wait_events_2
        .unwrap())(command_buffer, event_count, p_events, p_dependency_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWaitEvents2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html)
    pub unsafe fn cmd_wait_events_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        events: &[crate::vk10::Event],
        dependency_infos: &[DependencyInfo],
    ) {
        let cmd_wait_events_2 = (*self.table).cmd_wait_events_2.unwrap();
        let event_count = events.len().min(dependency_infos.len());
        cmd_wait_events_2(
            command_buffer,
            event_count as _,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdPipelineBarrier2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)
pub unsafe fn cmd_pipeline_barrier_2(
    command_buffer: crate::vk10::CommandBuffer,
    p_dependency_info: *const DependencyInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_pipeline_barrier_2
        .unwrap())(command_buffer, p_dependency_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdPipelineBarrier2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)
    pub unsafe fn cmd_pipeline_barrier_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        dependency_info: &DependencyInfo,
    ) {
        let cmd_pipeline_barrier_2 = (*self.table).cmd_pipeline_barrier_2.unwrap();
        cmd_pipeline_barrier_2(command_buffer, dependency_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkQueueSubmit2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)
pub unsafe fn queue_submit_2(
    queue: crate::vk10::Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2,
    fence: crate::vk10::Fence,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .queue_submit_2
        .unwrap())(queue, submit_count, p_submits, fence)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkQueueSubmit2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)
    pub unsafe fn queue_submit_2(
        &self,
        queue: crate::vk10::Queue,
        submits: &[SubmitInfo2],
        fence: crate::vk10::Fence,
    ) -> crate::VulkanResult<()> {
        let queue_submit_2 = (*self.table).queue_submit_2.unwrap();
        let submit_count = submits.len();
        let result = queue_submit_2(queue, submit_count as _, submits.as_ptr(), fence);
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteTimestamp2")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)
pub unsafe fn cmd_write_timestamp_2(
    command_buffer: crate::vk10::CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: crate::vk10::QueryPool,
    query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_timestamp_2
        .unwrap())(command_buffer, stage, query_pool, query)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteTimestamp2")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)
    pub unsafe fn cmd_write_timestamp_2(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stage: Option<PipelineStageFlags2>,
        query_pool: crate::vk10::QueryPool,
        query: u32,
    ) {
        let cmd_write_timestamp_2 = (*self.table).cmd_write_timestamp_2.unwrap();
        cmd_write_timestamp_2(
            command_buffer,
            match stage {
                Some(v) => v,
                None => Default::default(),
            },
            query_pool,
            query as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBeginRendering")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)
pub unsafe fn cmd_begin_rendering(
    command_buffer: crate::vk10::CommandBuffer,
    p_rendering_info: *const RenderingInfo,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_begin_rendering
        .unwrap())(command_buffer, p_rendering_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBeginRendering")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        rendering_info: &RenderingInfo,
    ) {
        let cmd_begin_rendering = (*self.table).cmd_begin_rendering.unwrap();
        cmd_begin_rendering(command_buffer, rendering_info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdEndRendering")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)
pub unsafe fn cmd_end_rendering(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_end_rendering
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdEndRendering")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)
    pub unsafe fn cmd_end_rendering(&self, command_buffer: crate::vk10::CommandBuffer) {
        let cmd_end_rendering = (*self.table).cmd_end_rendering.unwrap();
        cmd_end_rendering(command_buffer);
    }
}
