#[doc(alias = "VkDeviceBufferMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirementsKHR.html)
pub type DeviceBufferMemoryRequirementsKHR = crate::vk13::DeviceBufferMemoryRequirements;
#[doc(alias = "VkDeviceImageMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirementsKHR.html)
pub type DeviceImageMemoryRequirementsKHR = crate::vk13::DeviceImageMemoryRequirements;
#[doc(alias = "VkPhysicalDeviceMaintenance4FeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4FeaturesKHR.html)
pub type PhysicalDeviceMaintenance4FeaturesKHR = crate::vk13::PhysicalDeviceMaintenance4Features;
#[doc(alias = "VkPhysicalDeviceMaintenance4PropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4PropertiesKHR.html)
pub type PhysicalDeviceMaintenance4PropertiesKHR = crate::vk13::PhysicalDeviceMaintenance4Properties;
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html)
pub unsafe fn get_device_buffer_memory_requirements_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk13::DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_buffer_memory_requirements
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html)
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        info: &crate::vk13::DeviceBufferMemoryRequirements,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_device_buffer_memory_requirements_khr = (*self.table)
            .get_device_buffer_memory_requirements_khr
            .unwrap();
        let mut memory_requirements = Default::default();
        get_device_buffer_memory_requirements_khr(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html)
pub unsafe fn get_device_image_memory_requirements_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk13::DeviceImageMemoryRequirements,
    p_memory_requirements: *mut crate::vk11::MemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_image_memory_requirements
        .unwrap())(device, p_info, p_memory_requirements)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html)
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        info: &crate::vk13::DeviceImageMemoryRequirements,
    ) -> crate::vk11::MemoryRequirements2 {
        let get_device_image_memory_requirements_khr = (*self.table)
            .get_device_image_memory_requirements_khr
            .unwrap();
        let mut memory_requirements = Default::default();
        get_device_image_memory_requirements_khr(
            self.handle,
            info as _,
            &mut memory_requirements,
        );
        memory_requirements
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
pub unsafe fn get_device_image_sparse_memory_requirements_khr(
    device: crate::vk10::Device,
    p_info: *const crate::vk13::DeviceImageMemoryRequirements,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut crate::vk11::SparseImageMemoryRequirements2,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_image_sparse_memory_requirements
        .unwrap())(
        device,
        p_info,
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(
        &self,
        info: &crate::vk13::DeviceImageMemoryRequirements,
        sparse_memory_requirement_count: Option<u32>,
        mut sparse_memory_requirements_callback: impl FnMut(
            &mut Vec<crate::vk11::SparseImageMemoryRequirements2>,
        ),
    ) -> Vec<crate::vk11::SparseImageMemoryRequirements2> {
        let get_device_image_sparse_memory_requirements_khr = (*self.table)
            .get_device_image_sparse_memory_requirements_khr
            .unwrap();
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                get_device_image_sparse_memory_requirements_khr(
                    self.handle,
                    info as _,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut sparse_memory_requirements = vec![
            Default::default(); sparse_memory_requirement_count as usize
        ];
        sparse_memory_requirements_callback(&mut sparse_memory_requirements);
        get_device_image_sparse_memory_requirements_khr(
            self.handle,
            info as _,
            &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr(),
        );
        sparse_memory_requirements
    }
}
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;
pub const KHR_MAINTENANCE_4_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_maintenance4"
);
