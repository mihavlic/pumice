#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewHandleInfoNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html)
pub struct ImageViewHandleInfoNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image_view: crate::vk10::ImageView,
    pub descriptor_type: crate::vk10::DescriptorType,
    pub sampler: crate::vk10::Sampler,
}
impl Default for ImageViewHandleInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            descriptor_type: Default::default(),
            sampler: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImageViewAddressPropertiesNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewAddressPropertiesNVX.html)
pub struct ImageViewAddressPropertiesNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub device_address: crate::vk10::DeviceAddress,
    pub size: crate::vk10::DeviceSize,
}
impl Default for ImageViewAddressPropertiesNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageViewHandleNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html)
pub unsafe fn get_image_view_handle_nvx(
    device: crate::vk10::Device,
    p_info: *const ImageViewHandleInfoNVX,
) -> u32 {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_view_handle_nvx
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetImageViewHandleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html)
    pub unsafe fn get_image_view_handle_nvx(&self, info: &ImageViewHandleInfoNVX) {
        let get_image_view_handle_nvx = (*self.table).get_image_view_handle_nvx.unwrap();
        get_image_view_handle_nvx(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetImageViewAddressNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html)
pub unsafe fn get_image_view_address_nvx(
    device: crate::vk10::Device,
    image_view: crate::vk10::ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_image_view_address_nvx
        .unwrap())(device, image_view, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetImageViewAddressNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html)
    pub unsafe fn get_image_view_address_nvx(
        &self,
        image_view: crate::vk10::ImageView,
    ) -> crate::VulkanResult<ImageViewAddressPropertiesNVX> {
        let get_image_view_address_nvx = (*self.table)
            .get_image_view_address_nvx
            .unwrap();
        let mut properties = Default::default();
        let result = get_image_view_address_nvx(
            self.handle,
            image_view,
            &mut properties,
        );
        crate::new_result(properties, result)
    }
}
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
pub const NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NVX_image_view_handle"
);
