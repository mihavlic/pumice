#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html)
pub struct FragmentShadingRateAttachmentInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_fragment_shading_rate_attachment: *const crate::vk12::AttachmentReference2,
    pub shading_rate_attachment_texel_size: crate::vk10::Extent2D,
}
impl Default for FragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
            p_next: std::ptr::null(),
            p_fragment_shading_rate_attachment: std::ptr::null(),
            shading_rate_attachment_texel_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html)
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub fragment_size: crate::vk10::Extent2D,
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}
impl Default for PipelineFragmentShadingRateStateCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            fragment_size: Default::default(),
            combiner_ops: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html)
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_fragment_shading_rate: crate::vk10::Bool32,
    pub primitive_fragment_shading_rate: crate::vk10::Bool32,
    pub attachment_fragment_shading_rate: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            pipeline_fragment_shading_rate: Default::default(),
            primitive_fragment_shading_rate: Default::default(),
            attachment_fragment_shading_rate: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html)
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub min_fragment_shading_rate_attachment_texel_size: crate::vk10::Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size: crate::vk10::Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    pub primitive_fragment_shading_rate_with_multiple_viewports: crate::vk10::Bool32,
    pub layered_shading_rate_attachments: crate::vk10::Bool32,
    pub fragment_shading_rate_non_trivial_combiner_ops: crate::vk10::Bool32,
    pub max_fragment_size: crate::vk10::Extent2D,
    pub max_fragment_size_aspect_ratio: u32,
    pub max_fragment_shading_rate_coverage_samples: u32,
    pub max_fragment_shading_rate_rasterization_samples: crate::vk10::SampleCountFlags,
    pub fragment_shading_rate_with_shader_depth_stencil_writes: crate::vk10::Bool32,
    pub fragment_shading_rate_with_sample_mask: crate::vk10::Bool32,
    pub fragment_shading_rate_with_shader_sample_mask: crate::vk10::Bool32,
    pub fragment_shading_rate_with_conservative_rasterization: crate::vk10::Bool32,
    pub fragment_shading_rate_with_fragment_shader_interlock: crate::vk10::Bool32,
    pub fragment_shading_rate_with_custom_sample_locations: crate::vk10::Bool32,
    pub fragment_shading_rate_strict_multiply_combiner: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            min_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: Default::default(),
            primitive_fragment_shading_rate_with_multiple_viewports: Default::default(),
            layered_shading_rate_attachments: Default::default(),
            fragment_shading_rate_non_trivial_combiner_ops: Default::default(),
            max_fragment_size: Default::default(),
            max_fragment_size_aspect_ratio: Default::default(),
            max_fragment_shading_rate_coverage_samples: Default::default(),
            max_fragment_shading_rate_rasterization_samples: Default::default(),
            fragment_shading_rate_with_shader_depth_stencil_writes: Default::default(),
            fragment_shading_rate_with_sample_mask: Default::default(),
            fragment_shading_rate_with_shader_sample_mask: Default::default(),
            fragment_shading_rate_with_conservative_rasterization: Default::default(),
            fragment_shading_rate_with_fragment_shader_interlock: Default::default(),
            fragment_shading_rate_with_custom_sample_locations: Default::default(),
            fragment_shading_rate_strict_multiply_combiner: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html)
pub struct PhysicalDeviceFragmentShadingRateKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub sample_counts: crate::vk10::SampleCountFlags,
    pub fragment_size: crate::vk10::Extent2D,
}
impl Default for PhysicalDeviceFragmentShadingRateKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR,
            p_next: std::ptr::null_mut(),
            sample_counts: Default::default(),
            fragment_size: Default::default(),
        }
    }
}
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FragmentShadingRateCombinerOpKHR(pub i32);
impl FragmentShadingRateCombinerOpKHR {
    pub const KEEP: Self = Self(0);
    pub const REPLACE: Self = Self(1);
    pub const MIN: Self = Self(2);
    pub const MAX: Self = Self(3);
    pub const MUL: Self = Self(4);
}
crate::enum_impl! {
    FragmentShadingRateCombinerOpKHR : i32, KEEP, REPLACE, MIN, MAX, MUL
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
pub unsafe fn cmd_set_fragment_shading_rate_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_fragment_size: *const crate::vk10::Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_fragment_shading_rate_khr
        .unwrap())(command_buffer, p_fragment_size, combiner_ops)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        fragment_size: &crate::vk10::Extent2D,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let cmd_set_fragment_shading_rate_khr = (*self.table)
            .cmd_set_fragment_shading_rate_khr
            .unwrap();
        cmd_set_fragment_shading_rate_khr(
            command_buffer,
            fragment_size as _,
            combiner_ops as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
pub unsafe fn get_physical_device_fragment_shading_rates_khr(
    physical_device: crate::vk10::PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_fragment_shading_rates_khr
        .unwrap())(
        physical_device,
        p_fragment_shading_rate_count,
        p_fragment_shading_rates,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        fragment_shading_rate_count: Option<u32>,
        mut fragment_shading_rates_callback: impl FnMut(
            &mut Vec<PhysicalDeviceFragmentShadingRateKHR>,
        ),
    ) -> crate::VulkanResult<
        (Vec<PhysicalDeviceFragmentShadingRateKHR>, crate::vk10::Result),
    > {
        let get_physical_device_fragment_shading_rates_khr = (*self.table)
            .get_physical_device_fragment_shading_rates_khr
            .unwrap();
        let mut fragment_shading_rate_count = match fragment_shading_rate_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_fragment_shading_rates_khr(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut fragment_shading_rates = vec![
            Default::default(); fragment_shading_rate_count as usize
        ];
        fragment_shading_rates_callback(&mut fragment_shading_rates);
        let result = get_physical_device_fragment_shading_rates_khr(
            physical_device,
            &mut fragment_shading_rate_count,
            fragment_shading_rates.as_mut_ptr(),
        );
        crate::new_result((fragment_shading_rates, result), result)
    }
}
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_fragment_shading_rate"
);
