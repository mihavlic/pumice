#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT.html)
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_module_identifier: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_module_identifier: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT.html)
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub shader_module_identifier_algorithm_uuid: [u8; crate::vk10::UUID_SIZE as usize],
}
impl Default for PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            shader_module_identifier_algorithm_uuid: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineShaderStageModuleIdentifierCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageModuleIdentifierCreateInfoEXT.html)
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}
impl Default for PipelineShaderStageModuleIdentifierCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            identifier_size: Default::default(),
            p_identifier: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkShaderModuleIdentifierEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleIdentifierEXT.html)
pub struct ShaderModuleIdentifierEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub identifier_size: u32,
    pub identifier: [u8; MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
}
impl Default for ShaderModuleIdentifierEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SHADER_MODULE_IDENTIFIER_EXT,
            p_next: std::ptr::null_mut(),
            identifier_size: Default::default(),
            identifier: unsafe { std::mem::zeroed() },
        }
    }
}
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html)
pub unsafe fn get_shader_module_identifier_ext(
    device: crate::vk10::Device,
    shader_module: crate::vk10::ShaderModule,
    p_identifier: *mut ShaderModuleIdentifierEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_shader_module_identifier_ext
        .unwrap())(device, shader_module, p_identifier)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetShaderModuleIdentifierEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html)
    pub unsafe fn get_shader_module_identifier_ext(
        &self,
        shader_module: crate::vk10::ShaderModule,
    ) -> ShaderModuleIdentifierEXT {
        let get_shader_module_identifier_ext = (*self.table)
            .get_shader_module_identifier_ext
            .unwrap();
        let mut identifier = Default::default();
        get_shader_module_identifier_ext(self.handle, shader_module, &mut identifier);
        identifier
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html)
pub unsafe fn get_shader_module_create_info_identifier_ext(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk10::ShaderModuleCreateInfo,
    p_identifier: *mut ShaderModuleIdentifierEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_shader_module_create_info_identifier_ext
        .unwrap())(device, p_create_info, p_identifier)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html)
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        create_info: &crate::vk10::ShaderModuleCreateInfo,
    ) -> ShaderModuleIdentifierEXT {
        let get_shader_module_create_info_identifier_ext = (*self.table)
            .get_shader_module_create_info_identifier_ext
            .unwrap();
        let mut identifier = Default::default();
        get_shader_module_create_info_identifier_ext(
            self.handle,
            create_info as _,
            &mut identifier,
        );
        identifier
    }
}
pub const EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION: u32 = 1;
pub const EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_shader_module_identifier"
);
