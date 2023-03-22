#[doc(alias = "VkImageFormatConstraintsFlagsFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageFormatConstraintsFlagsFUCHSIA(pub u32);
crate::bitflags_impl! {
    ImageFormatConstraintsFlagsFUCHSIA : u32, 0x0,
}
crate::dispatchable_handle!(
    BufferCollectionFUCHSIA, BUFFER_COLLECTION_FUCHSIA, "VkBufferCollectionFUCHSIA",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMemoryBufferCollectionFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html)
pub struct ImportMemoryBufferCollectionFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for ImportMemoryBufferCollectionFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCollectionImageCreateInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html)
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for BufferCollectionImageCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCollectionBufferCreateInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html)
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
impl Default for BufferCollectionBufferCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCollectionCreateInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html)
pub struct BufferCollectionCreateInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub collection_token: crate::extensions::fuchsia_imagepipe_surface::zx_handle_t,
}
impl Default for BufferCollectionCreateInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            collection_token: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCollectionPropertiesFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html)
pub struct BufferCollectionPropertiesFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub memory_type_bits: u32,
    pub buffer_count: u32,
    pub create_info_index: u32,
    pub sysmem_pixel_format: u64,
    pub format_features: crate::vk10::FormatFeatureFlags,
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
    pub sampler_ycbcr_conversion_components: crate::vk10::ComponentMapping,
    pub suggested_ycbcr_model: crate::vk11::SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: crate::vk11::SamplerYcbcrRange,
    pub suggested_xchroma_offset: crate::vk11::ChromaLocation,
    pub suggested_ychroma_offset: crate::vk11::ChromaLocation,
}
impl Default for BufferCollectionPropertiesFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
            buffer_count: Default::default(),
            create_info_index: Default::default(),
            sysmem_pixel_format: Default::default(),
            format_features: Default::default(),
            sysmem_color_space_index: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_xchroma_offset: Default::default(),
            suggested_ychroma_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferConstraintsInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html)
pub struct BufferConstraintsInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub create_info: crate::vk10::BufferCreateInfo,
    pub required_format_features: crate::vk10::FormatFeatureFlags,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}
impl Default for BufferConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            create_info: Default::default(),
            required_format_features: Default::default(),
            buffer_collection_constraints: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSysmemColorSpaceFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html)
pub struct SysmemColorSpaceFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub color_space: u32,
}
impl Default for SysmemColorSpaceFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SYSMEM_COLOR_SPACE_FUCHSIA,
            p_next: std::ptr::null(),
            color_space: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageFormatConstraintsInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html)
pub struct ImageFormatConstraintsInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_create_info: crate::vk10::ImageCreateInfo,
    pub required_format_features: crate::vk10::FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    pub sysmem_pixel_format: u64,
    pub color_space_count: u32,
    pub p_color_spaces: *const SysmemColorSpaceFUCHSIA,
}
impl Default for ImageFormatConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            image_create_info: Default::default(),
            required_format_features: Default::default(),
            flags: Default::default(),
            sysmem_pixel_format: Default::default(),
            color_space_count: Default::default(),
            p_color_spaces: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageConstraintsInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html)
pub struct ImageConstraintsInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}
impl Default for ImageConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            format_constraints_count: Default::default(),
            p_format_constraints: std::ptr::null(),
            buffer_collection_constraints: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkBufferCollectionConstraintsInfoFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html)
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub min_buffer_count: u32,
    pub max_buffer_count: u32,
    pub min_buffer_count_for_camping: u32,
    pub min_buffer_count_for_dedicated_slack: u32,
    pub min_buffer_count_for_shared_slack: u32,
}
impl Default for BufferCollectionConstraintsInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            min_buffer_count: Default::default(),
            max_buffer_count: Default::default(),
            min_buffer_count_for_camping: Default::default(),
            min_buffer_count_for_dedicated_slack: Default::default(),
            min_buffer_count_for_shared_slack: Default::default(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ImageConstraintsInfoFlagsFUCHSIA(pub u32);
impl ImageConstraintsInfoFlagsFUCHSIA {
    pub const CPU_READ_RARELY: Self = Self(1 << 0);
    pub const CPU_READ_OFTEN: Self = Self(1 << 1);
    pub const CPU_WRITE_RARELY: Self = Self(1 << 2);
    pub const CPU_WRITE_OFTEN: Self = Self(1 << 3);
    pub const PROTECTED_OPTIONAL: Self = Self(1 << 4);
}
crate::bitflags_impl! {
    ImageConstraintsInfoFlagsFUCHSIA : u32, 0x1f, CPU_READ_RARELY, CPU_READ_OFTEN,
    CPU_WRITE_RARELY, CPU_WRITE_OFTEN, PROTECTED_OPTIONAL
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html)
pub unsafe fn create_buffer_collection_fuchsia(
    device: crate::vk10::Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_buffer_collection_fuchsia
        .unwrap())(device, p_create_info, p_allocator, p_collection)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html)
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<BufferCollectionFUCHSIA> {
        let create_buffer_collection_fuchsia = (*self.table)
            .create_buffer_collection_fuchsia
            .unwrap();
        let mut collection = Default::default();
        let result = create_buffer_collection_fuchsia(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut collection,
        );
        crate::new_result(collection, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
    device: crate::vk10::Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_buffer_collection_buffer_constraints_fuchsia
        .unwrap())(device, collection, p_buffer_constraints_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> crate::VulkanResult<()> {
        let set_buffer_collection_buffer_constraints_fuchsia = (*self.table)
            .set_buffer_collection_buffer_constraints_fuchsia
            .unwrap();
        let result = set_buffer_collection_buffer_constraints_fuchsia(
            self.handle,
            collection,
            buffer_constraints_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
    device: crate::vk10::Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .set_buffer_collection_image_constraints_fuchsia
        .unwrap())(device, collection, p_image_constraints_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> crate::VulkanResult<()> {
        let set_buffer_collection_image_constraints_fuchsia = (*self.table)
            .set_buffer_collection_image_constraints_fuchsia
            .unwrap();
        let result = set_buffer_collection_image_constraints_fuchsia(
            self.handle,
            collection,
            image_constraints_info as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html)
pub unsafe fn destroy_buffer_collection_fuchsia(
    device: crate::vk10::Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_buffer_collection_fuchsia
        .unwrap())(device, collection, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html)
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_buffer_collection_fuchsia = (*self.table)
            .destroy_buffer_collection_fuchsia
            .unwrap();
        destroy_buffer_collection_fuchsia(
            self.handle,
            collection,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html)
pub unsafe fn get_buffer_collection_properties_fuchsia(
    device: crate::vk10::Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_buffer_collection_properties_fuchsia
        .unwrap())(device, collection, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html)
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
    ) -> crate::VulkanResult<BufferCollectionPropertiesFUCHSIA> {
        let get_buffer_collection_properties_fuchsia = (*self.table)
            .get_buffer_collection_properties_fuchsia
            .unwrap();
        let mut properties = Default::default();
        let result = get_buffer_collection_properties_fuchsia(
            self.handle,
            collection,
            &mut properties,
        );
        crate::new_result(properties, result)
    }
}
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
pub const FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_FUCHSIA_buffer_collection"
);
