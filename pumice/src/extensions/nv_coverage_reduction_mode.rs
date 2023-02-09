#[doc(alias = "VkPipelineCoverageReductionStateCreateFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCoverageReductionStateCreateFlagsNV(pub u32);
crate::bitflags_impl! {
    PipelineCoverageReductionStateCreateFlagsNV : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html)
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub coverage_reduction_mode: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html)
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: CoverageReductionModeNV,
}
impl Default for PipelineCoverageReductionStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_reduction_mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html)
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub rasterization_samples: crate::vk10::SampleCountFlags,
    pub depth_stencil_samples: crate::vk10::SampleCountFlags,
    pub color_samples: crate::vk10::SampleCountFlags,
}
impl Default for FramebufferMixedSamplesCombinationNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
            rasterization_samples: Default::default(),
            depth_stencil_samples: Default::default(),
            color_samples: Default::default(),
        }
    }
}
#[doc(alias = "VkCoverageReductionModeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CoverageReductionModeNV(pub i32);
impl CoverageReductionModeNV {
    pub const MERGE: Self = Self(0);
    pub const TRUNCATE: Self = Self(1);
}
crate::enum_impl! {
    CoverageReductionModeNV : i32, MERGE, TRUNCATE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
    physical_device: crate::vk10::PhysicalDevice,
    p_combination_count: *mut u32,
    p_combinations: *mut FramebufferMixedSamplesCombinationNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
        .unwrap())(physical_device, p_combination_count, p_combinations)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        combination_count: Option<u32>,
        mut combinations_callback: impl FnMut(
            &mut Vec<FramebufferMixedSamplesCombinationNV>,
        ),
    ) -> crate::VulkanResult<
        (Vec<FramebufferMixedSamplesCombinationNV>, crate::vk10::Result),
    > {
        let get_physical_device_supported_framebuffer_mixed_samples_combinations_nv = (*self
            .table)
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
            .unwrap();
        let mut combination_count = match combination_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut combinations = vec![Default::default(); combination_count as usize];
        combinations_callback(&mut combinations);
        let result = get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
            physical_device,
            &mut combination_count,
            combinations.as_mut_ptr(),
        );
        crate::new_result((combinations, result), result)
    }
}
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_coverage_reduction_mode"
);
