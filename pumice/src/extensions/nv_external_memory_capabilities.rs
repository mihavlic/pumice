#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkExternalImageFormatPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html)
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: crate::vk10::ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalImageFormatPropertiesNV {
    fn default() -> Self {
        Self {
            image_format_properties: Default::default(),
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub u32);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const OPAQUE_WIN32: Self = Self(1 << 0);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 1);
    pub const D3D11_IMAGE: Self = Self(1 << 2);
    pub const D3D11_IMAGE_KMT: Self = Self(1 << 3);
}
crate::bitflags_impl! {
    ExternalMemoryHandleTypeFlagsNV : u32, 0xf, OPAQUE_WIN32, OPAQUE_WIN32_KMT,
    D3D11_IMAGE, D3D11_IMAGE_KMT
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagsNV(pub u32);
impl ExternalMemoryFeatureFlagsNV {
    pub const DEDICATED_ONLY: Self = Self(1 << 0);
    pub const EXPORTABLE: Self = Self(1 << 1);
    pub const IMPORTABLE: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    ExternalMemoryFeatureFlagsNV : u32, 0x7, DEDICATED_ONLY, EXPORTABLE, IMPORTABLE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
pub unsafe fn get_physical_device_external_image_format_properties_nv(
    physical_device: crate::vk10::PhysicalDevice,
    format: crate::vk10::Format,
    kind: crate::vk10::ImageType,
    tiling: crate::vk10::ImageTiling,
    usage: crate::vk10::ImageUsageFlags,
    flags: crate::vk10::ImageCreateFlags,
    external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_INSTANCE_TABLE
        .get_physical_device_external_image_format_properties_nv
        .unwrap())(
        physical_device,
        format,
        kind,
        tiling,
        usage,
        flags,
        external_handle_type,
        p_external_image_format_properties,
    )
}
#[cfg(feature = "wrappers")]
impl crate::InstanceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: crate::vk10::PhysicalDevice,
        format: crate::vk10::Format,
        kind: crate::vk10::ImageType,
        tiling: crate::vk10::ImageTiling,
        usage: crate::vk10::ImageUsageFlags,
        flags: Option<crate::vk10::ImageCreateFlags>,
        external_handle_type: Option<ExternalMemoryHandleTypeFlagsNV>,
    ) -> crate::VulkanResult<ExternalImageFormatPropertiesNV> {
        let get_physical_device_external_image_format_properties_nv = (*self.table)
            .get_physical_device_external_image_format_properties_nv
            .unwrap();
        let mut external_image_format_properties = Default::default();
        let result = get_physical_device_external_image_format_properties_nv(
            physical_device,
            format as _,
            kind as _,
            tiling as _,
            usage as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            match external_handle_type {
                Some(v) => v,
                None => Default::default(),
            },
            &mut external_image_format_properties,
        );
        crate::new_result(external_image_format_properties, result)
    }
}
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_external_memory_capabilities"
);
