/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/AHardwareBuffer.html)
pub type AHardwareBuffer = std::os::raw::c_void;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportAndroidHardwareBufferInfoANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html)
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub buffer: *mut AHardwareBuffer,
}
impl Default for ImportAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            buffer: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAndroidHardwareBufferUsageANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html)
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub android_hardware_buffer_usage: u64,
}
impl Default for AndroidHardwareBufferUsageANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: std::ptr::null_mut(),
            android_hardware_buffer_usage: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAndroidHardwareBufferPropertiesANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html)
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub allocation_size: crate::vk10::DeviceSize,
    pub memory_type_bits: u32,
}
impl Default for AndroidHardwareBufferPropertiesANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMemoryGetAndroidHardwareBufferInfoANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html)
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
}
impl Default for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAndroidHardwareBufferFormatPropertiesANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html)
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub external_format: u64,
    pub format_features: crate::vk10::FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: crate::vk10::ComponentMapping,
    pub suggested_ycbcr_model: crate::vk11::SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: crate::vk11::SamplerYcbcrRange,
    pub suggested_xchroma_offset: crate::vk11::ChromaLocation,
    pub suggested_ychroma_offset: crate::vk11::ChromaLocation,
}
impl Default for AndroidHardwareBufferFormatPropertiesANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            format_features: Default::default(),
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
#[doc(alias = "VkExternalFormatANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFormatANDROID.html)
pub struct ExternalFormatANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub external_format: u64,
}
impl Default for ExternalFormatANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: std::ptr::null_mut(),
            external_format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAndroidHardwareBufferFormatProperties2ANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html)
pub struct AndroidHardwareBufferFormatProperties2ANDROID {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub format: crate::vk10::Format,
    pub external_format: u64,
    pub format_features: crate::vk13::FormatFeatureFlags2,
    pub sampler_ycbcr_conversion_components: crate::vk10::ComponentMapping,
    pub suggested_ycbcr_model: crate::vk11::SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: crate::vk11::SamplerYcbcrRange,
    pub suggested_xchroma_offset: crate::vk11::ChromaLocation,
    pub suggested_ychroma_offset: crate::vk11::ChromaLocation,
}
impl Default for AndroidHardwareBufferFormatProperties2ANDROID {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID,
            p_next: std::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_xchroma_offset: Default::default(),
            suggested_ychroma_offset: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
pub unsafe fn get_android_hardware_buffer_properties_android(
    device: crate::vk10::Device,
    buffer: *const AHardwareBuffer,
    p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_android_hardware_buffer_properties_android
        .unwrap())(device, buffer, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: *const AHardwareBuffer,
    ) -> crate::VulkanResult<AndroidHardwareBufferPropertiesANDROID> {
        let get_android_hardware_buffer_properties_android = (*self.table)
            .get_android_hardware_buffer_properties_android
            .unwrap();
        let mut properties = Default::default();
        let result = get_android_hardware_buffer_properties_android(
            self.handle,
            buffer,
            &mut properties,
        );
        crate::new_result(properties, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
pub unsafe fn get_memory_android_hardware_buffer_android(
    device: crate::vk10::Device,
    p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: *mut *mut AHardwareBuffer,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_memory_android_hardware_buffer_android
        .unwrap())(device, p_info, p_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> crate::VulkanResult<*mut AHardwareBuffer> {
        let get_memory_android_hardware_buffer_android = (*self.table)
            .get_memory_android_hardware_buffer_android
            .unwrap();
        let mut buffer = std::ptr::null_mut();
        let result = get_memory_android_hardware_buffer_android(
            self.handle,
            info as _,
            &mut buffer,
        );
        crate::new_result(buffer, result)
    }
}
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_ANDROID_external_memory_android_hardware_buffer"
);
