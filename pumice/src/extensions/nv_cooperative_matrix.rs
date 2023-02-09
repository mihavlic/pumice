#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html)
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub cooperative_matrix: crate::vk10::Bool32,
    pub cooperative_matrix_robust_buffer_access: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            cooperative_matrix: Default::default(),
            cooperative_matrix_robust_buffer_access: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html)
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub cooperative_matrix_supported_stages: crate::vk10::ShaderStageFlags,
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            cooperative_matrix_supported_stages: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html)
pub struct CooperativeMatrixPropertiesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub msize: u32,
    pub nsize: u32,
    pub ksize: u32,
    pub atype: ComponentTypeNV,
    pub btype: ComponentTypeNV,
    pub ctype: ComponentTypeNV,
    pub dtype: ComponentTypeNV,
    pub scope: ScopeNV,
}
impl Default for CooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            msize: Default::default(),
            nsize: Default::default(),
            ksize: Default::default(),
            atype: Default::default(),
            btype: Default::default(),
            ctype: Default::default(),
            dtype: Default::default(),
            scope: Default::default(),
        }
    }
}
#[doc(alias = "VkScopeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScopeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ScopeNV(pub i32);
impl ScopeNV {
    pub const DEVICE: Self = Self(1);
    pub const WORKGROUP: Self = Self(2);
    pub const SUBGROUP: Self = Self(3);
    pub const QUEUE_FAMILY: Self = Self(5);
}
crate::enum_impl! {
    ScopeNV : i32, DEVICE, WORKGROUP, SUBGROUP, QUEUE_FAMILY
}
#[doc(alias = "VkComponentTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ComponentTypeNV(pub i32);
impl ComponentTypeNV {
    pub const FLOAT16: Self = Self(0);
    pub const FLOAT32: Self = Self(1);
    pub const FLOAT64: Self = Self(2);
    pub const SINT8: Self = Self(3);
    pub const SINT16: Self = Self(4);
    pub const SINT32: Self = Self(5);
    pub const SINT64: Self = Self(6);
    pub const UINT8: Self = Self(7);
    pub const UINT16: Self = Self(8);
    pub const UINT32: Self = Self(9);
    pub const UINT64: Self = Self(10);
}
crate::enum_impl! {
    ComponentTypeNV : i32, FLOAT16, FLOAT32, FLOAT64, SINT8, SINT16, SINT32, SINT64,
    UINT8, UINT16, UINT32, UINT64
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
    physical_device: crate::vk10::PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_cooperative_matrix_properties_nv
        .unwrap())(physical_device, p_property_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        property_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<CooperativeMatrixPropertiesNV>),
    ) -> crate::VulkanResult<(Vec<CooperativeMatrixPropertiesNV>, crate::vk10::Result)> {
        let get_physical_device_cooperative_matrix_properties_nv = (*self.table)
            .get_physical_device_cooperative_matrix_properties_nv
            .unwrap();
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_physical_device_cooperative_matrix_properties_nv(
                    physical_device,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as usize];
        properties_callback(&mut properties);
        let result = get_physical_device_cooperative_matrix_properties_nv(
            physical_device,
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_cooperative_matrix"
);
