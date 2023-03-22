#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageCompressionControlEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionControlEXT.html)
pub struct ImageCompressionControlEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ImageCompressionFlagsEXT,
    pub compression_control_plane_count: u32,
    pub p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
}
impl Default for ImageCompressionControlEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_COMPRESSION_CONTROL_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            compression_control_plane_count: Default::default(),
            p_fixed_rate_flags: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceImageCompressionControlFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlFeaturesEXT.html)
pub struct PhysicalDeviceImageCompressionControlFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_compression_control: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceImageCompressionControlFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            image_compression_control: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageCompressionPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionPropertiesEXT.html)
pub struct ImageCompressionPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_compression_flags: ImageCompressionFlagsEXT,
    pub image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
}
impl Default for ImageCompressionPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_COMPRESSION_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            image_compression_flags: Default::default(),
            image_compression_fixed_rate_flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageSubresource2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresource2EXT.html)
pub struct ImageSubresource2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub image_subresource: crate::vk10::ImageSubresource,
}
impl Default for ImageSubresource2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_SUBRESOURCE_2_EXT,
            p_next: std::ptr::null_mut(),
            image_subresource: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubresourceLayout2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout2EXT.html)
pub struct SubresourceLayout2EXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subresource_layout: crate::vk10::SubresourceLayout,
}
impl Default for SubresourceLayout2EXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBRESOURCE_LAYOUT_2_EXT,
            p_next: std::ptr::null_mut(),
            subresource_layout: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageCompressionFlagsEXT(pub u32);
impl ImageCompressionFlagsEXT {
    pub const DEFAULT: Self = Self(0);
    pub const FIXED_RATE_DEFAULT: Self = Self(1 << 0);
    pub const FIXED_RATE_EXPLICIT: Self = Self(1 << 1);
    pub const DISABLED: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    ImageCompressionFlagsEXT : u32, 0x7, DEFAULT, FIXED_RATE_DEFAULT,
    FIXED_RATE_EXPLICIT, DISABLED
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageCompressionFixedRateFlagsEXT(pub u32);
impl ImageCompressionFixedRateFlagsEXT {
    pub const NONE: Self = Self(0);
    pub const R1BPC: Self = Self(1 << 0);
    pub const R2BPC: Self = Self(1 << 1);
    pub const R3BPC: Self = Self(1 << 2);
    pub const R4BPC: Self = Self(1 << 3);
    pub const R5BPC: Self = Self(1 << 4);
    pub const R6BPC: Self = Self(1 << 5);
    pub const R7BPC: Self = Self(1 << 6);
    pub const R8BPC: Self = Self(1 << 7);
    pub const R9BPC: Self = Self(1 << 8);
    pub const R10BPC: Self = Self(1 << 9);
    pub const R11BPC: Self = Self(1 << 10);
    pub const R12BPC: Self = Self(1 << 11);
    pub const R13BPC: Self = Self(1 << 12);
    pub const R14BPC: Self = Self(1 << 13);
    pub const R15BPC: Self = Self(1 << 14);
    pub const R16BPC: Self = Self(1 << 15);
    pub const R17BPC: Self = Self(1 << 16);
    pub const R18BPC: Self = Self(1 << 17);
    pub const R19BPC: Self = Self(1 << 18);
    pub const R20BPC: Self = Self(1 << 19);
    pub const R21BPC: Self = Self(1 << 20);
    pub const R22BPC: Self = Self(1 << 21);
    pub const R23BPC: Self = Self(1 << 22);
    pub const R24BPC: Self = Self(1 << 23);
}
crate::bitflags_impl! {
    ImageCompressionFixedRateFlagsEXT : u32, 0xffffff, NONE, R1BPC, R2BPC, R3BPC, R4BPC,
    R5BPC, R6BPC, R7BPC, R8BPC, R9BPC, R10BPC, R11BPC, R12BPC, R13BPC, R14BPC, R15BPC,
    R16BPC, R17BPC, R18BPC, R19BPC, R20BPC, R21BPC, R22BPC, R23BPC, R24BPC
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html)
pub unsafe fn get_image_subresource_layout_2_ext(
    device: crate::vk10::Device,
    image: crate::vk10::Image,
    p_subresource: *const ImageSubresource2EXT,
    p_layout: *mut SubresourceLayout2EXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_subresource_layout_2_ext
        .unwrap())(device, image, p_subresource, p_layout)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html)
    pub unsafe fn get_image_subresource_layout_2_ext(
        &self,
        image: crate::vk10::Image,
        subresource: &ImageSubresource2EXT,
    ) -> SubresourceLayout2EXT {
        let get_image_subresource_layout_2_ext = (*self.table)
            .get_image_subresource_layout_2_ext
            .unwrap();
        let mut layout = Default::default();
        get_image_subresource_layout_2_ext(
            self.handle,
            image,
            subresource as _,
            &mut layout,
        );
        layout
    }
}
pub const EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION: u32 = 1;
pub const EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_image_compression_control"
);
