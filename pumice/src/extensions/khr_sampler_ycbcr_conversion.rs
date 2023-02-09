#[doc(alias = "VkSamplerYcbcrConversionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionKHR.html)
pub type SamplerYcbcrConversionKHR = crate::vk11::SamplerYcbcrConversion;
#[doc(alias = "VkSamplerYcbcrModelConversionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversionKHR.html)
pub type SamplerYcbcrModelConversionKHR = crate::vk11::SamplerYcbcrModelConversion;
#[doc(alias = "VkSamplerYcbcrRangeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRangeKHR.html)
pub type SamplerYcbcrRangeKHR = crate::vk11::SamplerYcbcrRange;
#[doc(alias = "VkChromaLocationKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocationKHR.html)
pub type ChromaLocationKHR = crate::vk11::ChromaLocation;
#[doc(alias = "VkSamplerYcbcrConversionInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfoKHR.html)
pub type SamplerYcbcrConversionInfoKHR = crate::vk11::SamplerYcbcrConversionInfo;
#[doc(alias = "VkSamplerYcbcrConversionCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfoKHR.html)
pub type SamplerYcbcrConversionCreateInfoKHR = crate::vk11::SamplerYcbcrConversionCreateInfo;
#[doc(alias = "VkBindImagePlaneMemoryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfoKHR.html)
pub type BindImagePlaneMemoryInfoKHR = crate::vk11::BindImagePlaneMemoryInfo;
#[doc(alias = "VkImagePlaneMemoryRequirementsInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfoKHR.html)
pub type ImagePlaneMemoryRequirementsInfoKHR = crate::vk11::ImagePlaneMemoryRequirementsInfo;
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR.html)
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR = crate::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures;
#[doc(alias = "VkSamplerYcbcrConversionImageFormatPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatPropertiesKHR.html)
pub type SamplerYcbcrConversionImageFormatPropertiesKHR = crate::vk11::SamplerYcbcrConversionImageFormatProperties;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html)
pub unsafe fn create_sampler_ycbcr_conversion_khr(
    device: crate::vk10::Device,
    p_create_info: *const crate::vk11::SamplerYcbcrConversionCreateInfo,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_ycbcr_conversion: *mut crate::vk11::SamplerYcbcrConversion,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_sampler_ycbcr_conversion
        .unwrap())(device, p_create_info, p_allocator, p_ycbcr_conversion)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html)
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        create_info: &crate::vk11::SamplerYcbcrConversionCreateInfo,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<crate::vk11::SamplerYcbcrConversion> {
        let create_sampler_ycbcr_conversion_khr = (*self.table)
            .create_sampler_ycbcr_conversion_khr
            .unwrap();
        let mut ycbcr_conversion = Default::default();
        let result = create_sampler_ycbcr_conversion_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut ycbcr_conversion,
        );
        crate::new_result(ycbcr_conversion, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html)
pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
    device: crate::vk10::Device,
    ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_sampler_ycbcr_conversion
        .unwrap())(device, ycbcr_conversion, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html)
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: crate::vk11::SamplerYcbcrConversion,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_sampler_ycbcr_conversion_khr = (*self.table)
            .destroy_sampler_ycbcr_conversion_khr
            .unwrap();
        destroy_sampler_ycbcr_conversion_khr(
            self.handle,
            ycbcr_conversion,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
pub const KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_sampler_ycbcr_conversion"
);
