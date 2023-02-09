#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceTilePropertiesFeaturesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html)
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub tile_properties: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceTilePropertiesFeaturesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM,
            p_next: std::ptr::null_mut(),
            tile_properties: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkTilePropertiesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTilePropertiesQCOM.html)
pub struct TilePropertiesQCOM {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub tile_size: crate::vk10::Extent3D,
    pub apron_size: crate::vk10::Extent2D,
    pub origin: crate::vk10::Offset2D,
}
impl Default for TilePropertiesQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::TILE_PROPERTIES_QCOM,
            p_next: std::ptr::null_mut(),
            tile_size: Default::default(),
            apron_size: Default::default(),
            origin: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html)
pub unsafe fn get_framebuffer_tile_properties_qcom(
    device: crate::vk10::Device,
    framebuffer: crate::vk10::Framebuffer,
    p_properties_count: *mut u32,
    p_properties: *mut TilePropertiesQCOM,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_framebuffer_tile_properties_qcom
        .unwrap())(device, framebuffer, p_properties_count, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html)
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        &self,
        framebuffer: crate::vk10::Framebuffer,
        properties_count: Option<u32>,
        mut properties_callback: impl FnMut(&mut Vec<TilePropertiesQCOM>),
    ) -> crate::VulkanResult<(Vec<TilePropertiesQCOM>, crate::vk10::Result)> {
        let get_framebuffer_tile_properties_qcom = (*self.table)
            .get_framebuffer_tile_properties_qcom
            .unwrap();
        let mut properties_count = match properties_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_framebuffer_tile_properties_qcom(
                    self.handle,
                    framebuffer,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); properties_count as usize];
        properties_callback(&mut properties);
        let result = get_framebuffer_tile_properties_qcom(
            self.handle,
            framebuffer,
            &mut properties_count,
            properties.as_mut_ptr(),
        );
        crate::new_result((properties, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html)
pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
    device: crate::vk10::Device,
    p_rendering_info: *const crate::vk13::RenderingInfo,
    p_properties: *mut TilePropertiesQCOM,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_dynamic_rendering_tile_properties_qcom
        .unwrap())(device, p_rendering_info, p_properties)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html)
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        rendering_info: &crate::vk13::RenderingInfo,
    ) -> crate::VulkanResult<TilePropertiesQCOM> {
        let get_dynamic_rendering_tile_properties_qcom = (*self.table)
            .get_dynamic_rendering_tile_properties_qcom
            .unwrap();
        let mut properties = Default::default();
        let result = get_dynamic_rendering_tile_properties_qcom(
            self.handle,
            rendering_info as _,
            &mut properties,
        );
        crate::new_result(properties, result)
    }
}
pub const QCOM_TILE_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const QCOM_TILE_PROPERTIES_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_QCOM_tile_properties"
);
