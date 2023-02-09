#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html)
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub inherited_viewport_scissor_2_d: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            inherited_viewport_scissor_2_d: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html)
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub viewport_scissor_2_d: crate::vk10::Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const crate::vk10::Viewport,
}
impl Default for CommandBufferInheritanceViewportScissorInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
            p_next: std::ptr::null(),
            viewport_scissor_2_d: Default::default(),
            viewport_depth_count: Default::default(),
            p_viewport_depths: std::ptr::null(),
        }
    }
}
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
pub const NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_inherited_viewport_scissor"
);
