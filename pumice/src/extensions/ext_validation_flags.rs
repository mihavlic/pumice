#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkValidationFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFlagsEXT.html)
pub struct ValidationFlagsEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks: *const ValidationCheckEXT,
}
impl Default for ValidationFlagsEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VALIDATION_FLAGS_EXT,
            p_next: std::ptr::null(),
            disabled_validation_check_count: Default::default(),
            p_disabled_validation_checks: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkValidationCheckEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCheckEXT(pub i32);
impl ValidationCheckEXT {
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
}
crate::enum_impl! {
    ValidationCheckEXT : i32, ALL, SHADERS
}
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
pub const EXT_VALIDATION_FLAGS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_validation_flags"
);
