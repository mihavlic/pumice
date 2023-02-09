#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreationControlEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationControlEXT.html)
pub struct RenderPassCreationControlEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub disallow_merging: crate::vk10::Bool32,
}
impl Default for RenderPassCreationControlEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_CREATION_CONTROL_EXT,
            p_next: std::ptr::null(),
            disallow_merging: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreationFeedbackInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackInfoEXT.html)
pub struct RenderPassCreationFeedbackInfoEXT {
    pub post_merge_subpass_count: u32,
}
impl Default for RenderPassCreationFeedbackInfoEXT {
    fn default() -> Self {
        Self {
            post_merge_subpass_count: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassCreationFeedbackCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackCreateInfoEXT.html)
pub struct RenderPassCreationFeedbackCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
}
impl Default for RenderPassCreationFeedbackCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            p_render_pass_feedback: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkRenderPassSubpassFeedbackInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackInfoEXT.html)
pub struct RenderPassSubpassFeedbackInfoEXT {
    pub subpass_merge_status: SubpassMergeStatusEXT,
    pub description: [std::os::raw::c_char; crate::vk10::MAX_DESCRIPTION_SIZE as usize],
    pub post_merge_index: u32,
}
impl Default for RenderPassSubpassFeedbackInfoEXT {
    fn default() -> Self {
        Self {
            subpass_merge_status: Default::default(),
            description: unsafe { std::mem::zeroed() },
            post_merge_index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkRenderPassSubpassFeedbackCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackCreateInfoEXT.html)
pub struct RenderPassSubpassFeedbackCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
}
impl Default for RenderPassSubpassFeedbackCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            p_subpass_feedback: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT.html)
pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subpass_merge_feedback: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            subpass_merge_feedback: Default::default(),
        }
    }
}
#[doc(alias = "VkSubpassMergeStatusEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassMergeStatusEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct SubpassMergeStatusEXT(pub i32);
impl SubpassMergeStatusEXT {
    pub const MERGED: Self = Self(0);
    pub const DISALLOWED: Self = Self(1);
    pub const NOT_MERGED_SIDE_EFFECTS: Self = Self(2);
    pub const NOT_MERGED_SAMPLES_MISMATCH: Self = Self(3);
    pub const NOT_MERGED_VIEWS_MISMATCH: Self = Self(4);
    pub const NOT_MERGED_ALIASING: Self = Self(5);
    pub const NOT_MERGED_DEPENDENCIES: Self = Self(6);
    pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT: Self = Self(7);
    pub const NOT_MERGED_TOO_MANY_ATTACHMENTS: Self = Self(8);
    pub const NOT_MERGED_INSUFFICIENT_STORAGE: Self = Self(9);
    pub const NOT_MERGED_DEPTH_STENCIL_COUNT: Self = Self(10);
    pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE: Self = Self(11);
    pub const NOT_MERGED_SINGLE_SUBPASS: Self = Self(12);
    pub const NOT_MERGED_UNSPECIFIED: Self = Self(13);
}
crate::enum_impl! {
    SubpassMergeStatusEXT : i32, MERGED, DISALLOWED, NOT_MERGED_SIDE_EFFECTS,
    NOT_MERGED_SAMPLES_MISMATCH, NOT_MERGED_VIEWS_MISMATCH, NOT_MERGED_ALIASING,
    NOT_MERGED_DEPENDENCIES, NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT,
    NOT_MERGED_TOO_MANY_ATTACHMENTS, NOT_MERGED_INSUFFICIENT_STORAGE,
    NOT_MERGED_DEPTH_STENCIL_COUNT, NOT_MERGED_RESOLVE_ATTACHMENT_REUSE,
    NOT_MERGED_SINGLE_SUBPASS, NOT_MERGED_UNSPECIFIED
}
pub const EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION: u32 = 2;
pub const EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_subpass_merge_feedback"
);
