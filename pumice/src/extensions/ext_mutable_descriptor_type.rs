#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT.html)
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub mutable_descriptor_type: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            mutable_descriptor_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMutableDescriptorTypeListEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListEXT.html)
pub struct MutableDescriptorTypeListEXT {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const crate::vk10::DescriptorType,
}
impl Default for MutableDescriptorTypeListEXT {
    fn default() -> Self {
        Self {
            descriptor_type_count: Default::default(),
            p_descriptor_types: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMutableDescriptorTypeCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoEXT.html)
pub struct MutableDescriptorTypeCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT,
}
impl Default for MutableDescriptorTypeCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            mutable_descriptor_type_list_count: Default::default(),
            p_mutable_descriptor_type_lists: std::ptr::null(),
        }
    }
}
pub const EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
pub const EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_mutable_descriptor_type"
);
