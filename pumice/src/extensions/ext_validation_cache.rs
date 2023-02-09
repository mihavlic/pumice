#[doc(alias = "VkValidationCacheCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCacheCreateFlagsEXT(pub u32);
crate::bitflags_impl! {
    ValidationCacheCreateFlagsEXT : u32, 0x0,
}
crate::dispatchable_handle!(
    ValidationCacheEXT, VALIDATION_CACHE_EXT, "VkValidationCacheEXT",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html)
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const std::os::raw::c_void,
}
impl Default for ValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            initial_data_size: Default::default(),
            p_initial_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html)
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub validation_cache: ValidationCacheEXT,
}
impl Default for ShaderModuleValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            validation_cache: Default::default(),
        }
    }
}
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
impl ValidationCacheHeaderVersionEXT {
    pub const ONE: Self = Self(1);
}
crate::enum_impl! {
    ValidationCacheHeaderVersionEXT : i32, ONE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateValidationCacheEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html)
pub unsafe fn create_validation_cache_ext(
    device: crate::vk10::Device,
    p_create_info: *const ValidationCacheCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_validation_cache: *mut ValidationCacheEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_validation_cache_ext
        .unwrap())(device, p_create_info, p_allocator, p_validation_cache)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateValidationCacheEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html)
    pub unsafe fn create_validation_cache_ext(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<ValidationCacheEXT> {
        let create_validation_cache_ext = (*self.table)
            .create_validation_cache_ext
            .unwrap();
        let mut validation_cache = Default::default();
        let result = create_validation_cache_ext(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut validation_cache,
        );
        crate::new_result(validation_cache, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyValidationCacheEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html)
pub unsafe fn destroy_validation_cache_ext(
    device: crate::vk10::Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_validation_cache_ext
        .unwrap())(device, validation_cache, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html)
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_validation_cache_ext = (*self.table)
            .destroy_validation_cache_ext
            .unwrap();
        destroy_validation_cache_ext(
            self.handle,
            validation_cache,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetValidationCacheDataEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html)
pub unsafe fn get_validation_cache_data_ext(
    device: crate::vk10::Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut std::os::raw::c_void,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_validation_cache_data_ext
        .unwrap())(device, validation_cache, p_data_size, p_data)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html)
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: ValidationCacheEXT,
        data_size: *mut usize,
        data: *mut std::os::raw::c_void,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let get_validation_cache_data_ext = (*self.table)
            .get_validation_cache_data_ext
            .unwrap();
        let result = get_validation_cache_data_ext(
            self.handle,
            validation_cache,
            data_size,
            data,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkMergeValidationCachesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html)
pub unsafe fn merge_validation_caches_ext(
    device: crate::vk10::Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .merge_validation_caches_ext
        .unwrap())(device, dst_cache, src_cache_count, p_src_caches)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkMergeValidationCachesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html)
    pub unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> crate::VulkanResult<()> {
        let merge_validation_caches_ext = (*self.table)
            .merge_validation_caches_ext
            .unwrap();
        let src_cache_count = src_caches.len();
        let result = merge_validation_caches_ext(
            self.handle,
            dst_cache,
            src_cache_count as _,
            src_caches.as_ptr(),
        );
        crate::new_result((), result)
    }
}
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_validation_cache"
);
