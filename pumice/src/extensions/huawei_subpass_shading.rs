#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html)
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub render_pass: crate::vk10::RenderPass,
    pub subpass: u32,
}
impl Default for SubpassShadingPipelineCreateInfoHUAWEI {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
            p_next: std::ptr::null_mut(),
            render_pass: Default::default(),
            subpass: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html)
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
            p_next: std::ptr::null_mut(),
            max_subpass_shading_workgroup_size_aspect_ratio: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html)
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub subpass_shading: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
            p_next: std::ptr::null_mut(),
            subpass_shading: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
    device: crate::vk10::Device,
    renderpass: crate::vk10::RenderPass,
    p_max_workgroup_size: *mut crate::vk10::Extent2D,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_subpass_shading_max_workgroup_size_huawei
        .unwrap())(device, renderpass, p_max_workgroup_size)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        renderpass: crate::vk10::RenderPass,
    ) -> crate::VulkanResult<(crate::vk10::Extent2D, crate::vk10::Result)> {
        let get_device_subpass_shading_max_workgroup_size_huawei = (*self.table)
            .get_device_subpass_shading_max_workgroup_size_huawei
            .unwrap();
        let mut max_workgroup_size = Default::default();
        let result = get_device_subpass_shading_max_workgroup_size_huawei(
            self.handle,
            renderpass,
            &mut max_workgroup_size,
        );
        crate::new_result((max_workgroup_size, result), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html)
pub unsafe fn cmd_subpass_shading_huawei(command_buffer: crate::vk10::CommandBuffer) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_subpass_shading_huawei
        .unwrap())(command_buffer)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html)
    pub unsafe fn cmd_subpass_shading_huawei(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
    ) {
        let cmd_subpass_shading_huawei = (*self.table)
            .cmd_subpass_shading_huawei
            .unwrap();
        cmd_subpass_shading_huawei(command_buffer);
    }
}
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_HUAWEI_subpass_shading"
);
