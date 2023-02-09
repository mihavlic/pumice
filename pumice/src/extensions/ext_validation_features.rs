#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkValidationFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html)
pub struct ValidationFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
}
impl Default for ValidationFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VALIDATION_FEATURES_EXT,
            p_next: std::ptr::null(),
            enabled_validation_feature_count: Default::default(),
            p_enabled_validation_features: std::ptr::null(),
            disabled_validation_feature_count: Default::default(),
            p_disabled_validation_features: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkValidationFeatureEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureEnableEXT(pub i32);
impl ValidationFeatureEnableEXT {
    pub const GPU_ASSISTED: Self = Self(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    pub const BEST_PRACTICES: Self = Self(2);
    pub const DEBUG_PRINTF: Self = Self(3);
    pub const SYNCHRONIZATION_VALIDATION: Self = Self(4);
}
crate::enum_impl! {
    ValidationFeatureEnableEXT : i32, GPU_ASSISTED, GPU_ASSISTED_RESERVE_BINDING_SLOT,
    BEST_PRACTICES, DEBUG_PRINTF, SYNCHRONIZATION_VALIDATION
}
#[doc(alias = "VkValidationFeatureDisableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureDisableEXT(pub i32);
impl ValidationFeatureDisableEXT {
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
    pub const THREAD_SAFETY: Self = Self(2);
    pub const API_PARAMETERS: Self = Self(3);
    pub const OBJECT_LIFETIMES: Self = Self(4);
    pub const CORE_CHECKS: Self = Self(5);
    pub const UNIQUE_HANDLES: Self = Self(6);
    pub const SHADER_VALIDATION_CACHE: Self = Self(7);
}
crate::enum_impl! {
    ValidationFeatureDisableEXT : i32, ALL, SHADERS, THREAD_SAFETY, API_PARAMETERS,
    OBJECT_LIFETIMES, CORE_CHECKS, UNIQUE_HANDLES, SHADER_VALIDATION_CACHE
}
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 5;
pub const EXT_VALIDATION_FEATURES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_validation_features"
);
