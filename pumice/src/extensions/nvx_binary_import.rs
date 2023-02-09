crate::dispatchable_handle!(
    CuModuleNVX, CU_MODULE_NVX, "VkCuModuleNVX",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html)"
);
crate::dispatchable_handle!(
    CuFunctionNVX, CU_FUNCTION_NVX, "VkCuFunctionNVX",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCuModuleCreateInfoNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleCreateInfoNVX.html)
pub struct CuModuleCreateInfoNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub data_size: usize,
    pub p_data: *const std::os::raw::c_void,
}
impl Default for CuModuleCreateInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CU_MODULE_CREATE_INFO_NVX,
            p_next: std::ptr::null(),
            data_size: Default::default(),
            p_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html)
pub struct CuFunctionCreateInfoNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub module: CuModuleNVX,
    pub p_name: *const std::os::raw::c_char,
}
impl Default for CuFunctionCreateInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CU_FUNCTION_CREATE_INFO_NVX,
            p_next: std::ptr::null(),
            module: Default::default(),
            p_name: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCuLaunchInfoNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html)
pub struct CuLaunchInfoNVX {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub function: CuFunctionNVX,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const std::os::raw::c_void,
    pub extra_count: usize,
    pub p_extras: *const *const std::os::raw::c_void,
}
impl Default for CuLaunchInfoNVX {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::CU_LAUNCH_INFO_NVX,
            p_next: std::ptr::null(),
            function: Default::default(),
            grid_dim_x: Default::default(),
            grid_dim_y: Default::default(),
            grid_dim_z: Default::default(),
            block_dim_x: Default::default(),
            block_dim_y: Default::default(),
            block_dim_z: Default::default(),
            shared_mem_bytes: Default::default(),
            param_count: Default::default(),
            p_params: std::ptr::null(),
            extra_count: Default::default(),
            p_extras: std::ptr::null(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateCuModuleNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html)
pub unsafe fn create_cu_module_nvx(
    device: crate::vk10::Device,
    p_create_info: *const CuModuleCreateInfoNVX,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_module: *mut CuModuleNVX,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_cu_module_nvx
        .unwrap())(device, p_create_info, p_allocator, p_module)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateCuModuleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html)
    pub unsafe fn create_cu_module_nvx(
        &self,
        create_info: &CuModuleCreateInfoNVX,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<CuModuleNVX> {
        let create_cu_module_nvx = (*self.table).create_cu_module_nvx.unwrap();
        let mut module = Default::default();
        let result = create_cu_module_nvx(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut module,
        );
        crate::new_result(module, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateCuFunctionNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html)
pub unsafe fn create_cu_function_nvx(
    device: crate::vk10::Device,
    p_create_info: *const CuFunctionCreateInfoNVX,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_function: *mut CuFunctionNVX,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_cu_function_nvx
        .unwrap())(device, p_create_info, p_allocator, p_function)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateCuFunctionNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html)
    pub unsafe fn create_cu_function_nvx(
        &self,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<CuFunctionNVX> {
        let create_cu_function_nvx = (*self.table).create_cu_function_nvx.unwrap();
        let mut function = Default::default();
        let result = create_cu_function_nvx(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut function,
        );
        crate::new_result(function, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyCuModuleNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html)
pub unsafe fn destroy_cu_module_nvx(
    device: crate::vk10::Device,
    module: CuModuleNVX,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_cu_module_nvx
        .unwrap())(device, module, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyCuModuleNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html)
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        module: CuModuleNVX,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_cu_module_nvx = (*self.table).destroy_cu_module_nvx.unwrap();
        destroy_cu_module_nvx(
            self.handle,
            module,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyCuFunctionNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html)
pub unsafe fn destroy_cu_function_nvx(
    device: crate::vk10::Device,
    function: CuFunctionNVX,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_cu_function_nvx
        .unwrap())(device, function, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html)
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        function: CuFunctionNVX,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_cu_function_nvx = (*self.table).destroy_cu_function_nvx.unwrap();
        destroy_cu_function_nvx(
            self.handle,
            function,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html)
pub unsafe fn cmd_cu_launch_kernel_nvx(
    command_buffer: crate::vk10::CommandBuffer,
    p_launch_info: *const CuLaunchInfoNVX,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_cu_launch_kernel_nvx
        .unwrap())(command_buffer, p_launch_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html)
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        launch_info: &CuLaunchInfoNVX,
    ) {
        let cmd_cu_launch_kernel_nvx = (*self.table).cmd_cu_launch_kernel_nvx.unwrap();
        cmd_cu_launch_kernel_nvx(command_buffer, launch_info as _);
    }
}
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NVX_binary_import"
);
