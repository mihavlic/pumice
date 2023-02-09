#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html)
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub fragment_shading_rate_enums: crate::vk10::Bool32,
    pub supersample_fragment_shading_rates: crate::vk10::Bool32,
    pub no_invocation_fragment_shading_rates: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            fragment_shading_rate_enums: Default::default(),
            supersample_fragment_shading_rates: Default::default(),
            no_invocation_fragment_shading_rates: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html)
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_fragment_shading_rate_invocation_count: crate::vk10::SampleCountFlags,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_fragment_shading_rate_invocation_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html)
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub shading_rate_type: FragmentShadingRateTypeNV,
    pub shading_rate: FragmentShadingRateNV,
    pub combiner_ops: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
}
impl Default for PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            shading_rate_type: Default::default(),
            shading_rate: Default::default(),
            combiner_ops: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFragmentShadingRateNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FragmentShadingRateNV(pub i32);
impl FragmentShadingRateNV {
    pub const R1_INVOCATION_PER_PIXEL: Self = Self(0);
    pub const R1_INVOCATION_PER_1X2_PIXELS: Self = Self(1);
    pub const R1_INVOCATION_PER_2X1_PIXELS: Self = Self(4);
    pub const R1_INVOCATION_PER_2X2_PIXELS: Self = Self(5);
    pub const R1_INVOCATION_PER_2X4_PIXELS: Self = Self(6);
    pub const R1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    pub const R1_INVOCATION_PER_4X4_PIXELS: Self = Self(10);
    pub const R2_INVOCATIONS_PER_PIXEL: Self = Self(11);
    pub const R4_INVOCATIONS_PER_PIXEL: Self = Self(12);
    pub const R8_INVOCATIONS_PER_PIXEL: Self = Self(13);
    pub const R16_INVOCATIONS_PER_PIXEL: Self = Self(14);
    pub const NO_INVOCATIONS: Self = Self(15);
}
crate::enum_impl! {
    FragmentShadingRateNV : i32, R1_INVOCATION_PER_PIXEL, R1_INVOCATION_PER_1X2_PIXELS,
    R1_INVOCATION_PER_2X1_PIXELS, R1_INVOCATION_PER_2X2_PIXELS,
    R1_INVOCATION_PER_2X4_PIXELS, R1_INVOCATION_PER_4X2_PIXELS,
    R1_INVOCATION_PER_4X4_PIXELS, R2_INVOCATIONS_PER_PIXEL, R4_INVOCATIONS_PER_PIXEL,
    R8_INVOCATIONS_PER_PIXEL, R16_INVOCATIONS_PER_PIXEL, NO_INVOCATIONS
}
#[doc(alias = "VkFragmentShadingRateTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FragmentShadingRateTypeNV(pub i32);
impl FragmentShadingRateTypeNV {
    pub const FRAGMENT_SIZE: Self = Self(0);
    pub const ENUMS: Self = Self(1);
}
crate::enum_impl! {
    FragmentShadingRateTypeNV : i32, FRAGMENT_SIZE, ENUMS
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
    command_buffer: crate::vk10::CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_fragment_shading_rate_enum_nv
        .unwrap())(command_buffer, shading_rate, combiner_ops)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: &[crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        let cmd_set_fragment_shading_rate_enum_nv = (*self.table)
            .cmd_set_fragment_shading_rate_enum_nv
            .unwrap();
        cmd_set_fragment_shading_rate_enum_nv(
            command_buffer,
            shading_rate as _,
            combiner_ops as _,
        );
    }
}
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_fragment_shading_rate_enums"
);
