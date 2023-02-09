#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html)
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub pipeline_executable_info: crate::vk10::Bool32,
}
impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            pipeline_executable_info: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html)
pub struct PipelineInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub pipeline: crate::vk10::Pipeline,
}
impl Default for PipelineInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html)
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub stages: crate::vk10::ShaderStageFlags,
    pub name: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
}
impl Default for PipelineExecutablePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            stages: Default::default(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            subgroup_size: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineExecutableInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html)
pub struct PipelineExecutableInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub pipeline: crate::vk10::Pipeline,
    pub executable_index: u32,
}
impl Default for PipelineExecutableInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_EXECUTABLE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
            executable_index: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html)
pub union PipelineExecutableStatisticValueKHR {
    pub b_32: crate::vk10::Bool32,
    pub i_64: i64,
    pub u_64: u64,
    pub f_64: std::os::raw::c_double,
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html)
pub struct PipelineExecutableStatisticKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub name: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}
impl Default for PipelineExecutableStatisticKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            format: Default::default(),
            value: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html)
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub name: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub is_text: crate::vk10::Bool32,
    pub data_size: usize,
    pub p_data: *mut std::os::raw::c_void,
}
impl Default for PipelineExecutableInternalRepresentationKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
            p_next: std::ptr::null_mut(),
            name: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
            is_text: Default::default(),
            data_size: Default::default(),
            p_data: std::ptr::null_mut(),
        }
    }
}
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineExecutableStatisticFormatKHR(pub i32);
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT64: Self = Self(2);
    pub const FLOAT64: Self = Self(3);
}
crate::enum_impl! {
    PipelineExecutableStatisticFormatKHR : i32, BOOL32, INT64, UINT64, FLOAT64
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
pub unsafe fn get_pipeline_executable_properties_khr(
    device: crate::vk10::Device,
    p_pipeline_info: *const PipelineInfoKHR,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_pipeline_executable_properties_khr
        .unwrap())(device, p_pipeline_info, p_executable_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
    pub unsafe fn get_pipeline_executable_properties_khr(
        &self,
        pipeline_info: &PipelineInfoKHR,
        executable_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<PipelineExecutablePropertiesKHR>),
    ) -> crate::VulkanResult<
        (Vec<PipelineExecutablePropertiesKHR>, crate::vk10::Result),
    > {
        let get_pipeline_executable_properties_khr = (*self.table)
            .get_pipeline_executable_properties_khr
            .unwrap();
        let mut executable_count = match executable_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_pipeline_executable_properties_khr(
                    self.handle,
                    pipeline_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); executable_count as usize];
        properties_callback(&mut properties);
        let result = get_pipeline_executable_properties_khr(
            self.handle,
            pipeline_info as _,
            &mut executable_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
pub unsafe fn get_pipeline_executable_statistics_khr(
    device: crate::vk10::Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_pipeline_executable_statistics_khr
        .unwrap())(device, p_executable_info, p_statistic_count, p_statistics)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
    pub unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistic_count: Option<u32>,
        mut statistics_callback: impl FnMut(&mut Vec<PipelineExecutableStatisticKHR>),
    ) -> crate::VulkanResult<
        (Vec<PipelineExecutableStatisticKHR>, crate::vk10::Result),
    > {
        let get_pipeline_executable_statistics_khr = (*self.table)
            .get_pipeline_executable_statistics_khr
            .unwrap();
        let mut statistic_count = match statistic_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_pipeline_executable_statistics_khr(
                    self.handle,
                    executable_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut statistics = vec![Default::default(); statistic_count as usize];
        statistics_callback(&mut statistics);
        let result = get_pipeline_executable_statistics_khr(
            self.handle,
            executable_info as _,
            &mut statistic_count,
            statistics.as_mut_ptr(),
        );
        crate::new_result((statistics, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
pub unsafe fn get_pipeline_executable_internal_representations_khr(
    device: crate::vk10::Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_internal_representation_count: *mut u32,
    p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_pipeline_executable_internal_representations_khr
        .unwrap())(
        device,
        p_executable_info,
        p_internal_representation_count,
        p_internal_representations,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
    pub unsafe fn get_pipeline_executable_internal_representations_khr(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representation_count: Option<u32>,
        mut internal_representations_callback: impl FnMut(
            &mut Vec<PipelineExecutableInternalRepresentationKHR>,
        ),
    ) -> crate::VulkanResult<
        (Vec<PipelineExecutableInternalRepresentationKHR>, crate::vk10::Result),
    > {
        let get_pipeline_executable_internal_representations_khr = (*self.table)
            .get_pipeline_executable_internal_representations_khr
            .unwrap();
        let mut internal_representation_count = match internal_representation_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_pipeline_executable_internal_representations_khr(
                    self.handle,
                    executable_info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut internal_representations = vec![
            Default::default(); internal_representation_count as usize
        ];
        internal_representations_callback(&mut internal_representations);
        let result = get_pipeline_executable_internal_representations_khr(
            self.handle,
            executable_info as _,
            &mut internal_representation_count,
            internal_representations.as_mut_ptr(),
        );
        crate::new_result((internal_representations, result), result)
    }
}
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_pipeline_executable_properties"
);
