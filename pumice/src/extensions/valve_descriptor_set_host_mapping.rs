#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html)
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub descriptor_set_host_mapping: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
            p_next: std::ptr::null_mut(),
            descriptor_set_host_mapping: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetBindingReferenceVALVE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html)
pub struct DescriptorSetBindingReferenceVALVE {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub descriptor_set_layout: crate::vk10::DescriptorSetLayout,
    pub binding: u32,
}
impl Default for DescriptorSetBindingReferenceVALVE {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
            p_next: std::ptr::null(),
            descriptor_set_layout: Default::default(),
            binding: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkDescriptorSetLayoutHostMappingInfoVALVE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html)
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub descriptor_offset: usize,
    pub descriptor_size: u32,
}
impl Default for DescriptorSetLayoutHostMappingInfoVALVE {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
            p_next: std::ptr::null_mut(),
            descriptor_offset: Default::default(),
            descriptor_size: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
    device: crate::vk10::Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_descriptor_set_layout_host_mapping_info_valve
        .unwrap())(device, p_binding_reference, p_host_mapping)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
    ) -> DescriptorSetLayoutHostMappingInfoVALVE {
        let get_descriptor_set_layout_host_mapping_info_valve = (*self.table)
            .get_descriptor_set_layout_host_mapping_info_valve
            .unwrap();
        let mut host_mapping = Default::default();
        get_descriptor_set_layout_host_mapping_info_valve(
            self.handle,
            binding_reference as _,
            &mut host_mapping,
        );
        host_mapping
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html)
pub unsafe fn get_descriptor_set_host_mapping_valve(
    device: crate::vk10::Device,
    descriptor_set: crate::vk10::DescriptorSet,
    pp_data: *mut *mut std::os::raw::c_void,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_descriptor_set_host_mapping_valve
        .unwrap())(device, descriptor_set, pp_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html)
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        &self,
        descriptor_set: crate::vk10::DescriptorSet,
    ) -> *mut std::os::raw::c_void {
        let get_descriptor_set_host_mapping_valve = (*self.table)
            .get_descriptor_set_host_mapping_valve
            .unwrap();
        let mut data = std::ptr::null_mut();
        get_descriptor_set_host_mapping_valve(self.handle, descriptor_set, &mut data);
        data
    }
}
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_VALVE_descriptor_set_host_mapping"
);
