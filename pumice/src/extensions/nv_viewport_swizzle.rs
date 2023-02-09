#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(pub u32);
crate::bitflags_impl! {
    PipelineViewportSwizzleStateCreateFlagsNV : u32, 0x0,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkViewportSwizzleNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html)
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
impl Default for ViewportSwizzleNV {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html)
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}
impl Default for PipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: Default::default(),
            p_viewport_swizzles: std::ptr::null(),
        }
    }
}
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X: Self = Self(0);
    pub const NEGATIVE_X: Self = Self(1);
    pub const POSITIVE_Y: Self = Self(2);
    pub const NEGATIVE_Y: Self = Self(3);
    pub const POSITIVE_Z: Self = Self(4);
    pub const NEGATIVE_Z: Self = Self(5);
    pub const POSITIVE_W: Self = Self(6);
    pub const NEGATIVE_W: Self = Self(7);
}
crate::enum_impl! {
    ViewportCoordinateSwizzleNV : i32, POSITIVE_X, NEGATIVE_X, POSITIVE_Y, NEGATIVE_Y,
    POSITIVE_Z, NEGATIVE_Z, POSITIVE_W, NEGATIVE_W
}
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_viewport_swizzle"
);
