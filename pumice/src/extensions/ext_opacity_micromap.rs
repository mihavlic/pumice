crate::dispatchable_handle!(
    MicromapEXT, MICROMAP_EXT, "VkMicromapEXT",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapEXT.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMicromapBuildInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapBuildInfoEXT.html)
pub struct MicromapBuildInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: MicromapTypeEXT,
    pub flags: BuildMicromapFlagsEXT,
    pub mode: BuildMicromapModeEXT,
    pub dst_micromap: MicromapEXT,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub scratch_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
    pub triangle_array: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub triangle_array_stride: crate::vk10::DeviceSize,
}
impl Default for MicromapBuildInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MICROMAP_BUILD_INFO_EXT,
            p_next: std::ptr::null(),
            kind: Default::default(),
            flags: Default::default(),
            mode: Default::default(),
            dst_micromap: Default::default(),
            usage_counts_count: Default::default(),
            p_usage_counts: std::ptr::null(),
            pp_usage_counts: std::ptr::null(),
            data: Default::default(),
            scratch_data: Default::default(),
            triangle_array: Default::default(),
            triangle_array_stride: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMicromapCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateInfoEXT.html)
pub struct MicromapCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub create_flags: MicromapCreateFlagsEXT,
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
    pub kind: MicromapTypeEXT,
    pub device_address: crate::vk10::DeviceAddress,
}
impl Default for MicromapCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MICROMAP_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            create_flags: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
            kind: Default::default(),
            device_address: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMicromapVersionInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapVersionInfoEXT.html)
pub struct MicromapVersionInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_version_data: *const u8,
}
impl Default for MicromapVersionInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MICROMAP_VERSION_INFO_EXT,
            p_next: std::ptr::null(),
            p_version_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyMicromapInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapInfoEXT.html)
pub struct CopyMicromapInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: MicromapEXT,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}
impl Default for CopyMicromapInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_MICROMAP_INFO_EXT,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyMicromapToMemoryInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapToMemoryInfoEXT.html)
pub struct CopyMicromapToMemoryInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: MicromapEXT,
    pub dst: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
    pub mode: CopyMicromapModeEXT,
}
impl Default for CopyMicromapToMemoryInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_MICROMAP_TO_MEMORY_INFO_EXT,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyMemoryToMicromapInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToMicromapInfoEXT.html)
pub struct CopyMemoryToMicromapInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}
impl Default for CopyMemoryToMicromapInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_MEMORY_TO_MICROMAP_INFO_EXT,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkMicromapBuildSizesInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapBuildSizesInfoEXT.html)
pub struct MicromapBuildSizesInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub micromap_size: crate::vk10::DeviceSize,
    pub build_scratch_size: crate::vk10::DeviceSize,
    pub discardable: crate::vk10::Bool32,
}
impl Default for MicromapBuildSizesInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::MICROMAP_BUILD_SIZES_INFO_EXT,
            p_next: std::ptr::null(),
            micromap_size: Default::default(),
            build_scratch_size: Default::default(),
            discardable: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMicromapUsageEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapUsageEXT.html)
pub struct MicromapUsageEXT {
    pub count: u32,
    pub subdivision_level: u32,
    pub format: u32,
}
impl Default for MicromapUsageEXT {
    fn default() -> Self {
        Self {
            count: Default::default(),
            subdivision_level: Default::default(),
            format: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkMicromapTriangleEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapTriangleEXT.html)
pub struct MicromapTriangleEXT {
    pub data_offset: u32,
    pub subdivision_level: u16,
    pub format: u16,
}
impl Default for MicromapTriangleEXT {
    fn default() -> Self {
        Self {
            data_offset: Default::default(),
            subdivision_level: Default::default(),
            format: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceOpacityMicromapFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapFeaturesEXT.html)
pub struct PhysicalDeviceOpacityMicromapFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub micromap: crate::vk10::Bool32,
    pub micromap_capture_replay: crate::vk10::Bool32,
    pub micromap_host_commands: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceOpacityMicromapFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            micromap: Default::default(),
            micromap_capture_replay: Default::default(),
            micromap_host_commands: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceOpacityMicromapPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapPropertiesEXT.html)
pub struct PhysicalDeviceOpacityMicromapPropertiesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_opacity_2_state_subdivision_level: u32,
    pub max_opacity_4_state_subdivision_level: u32,
}
impl Default for PhysicalDeviceOpacityMicromapPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_opacity_2_state_subdivision_level: Default::default(),
            max_opacity_4_state_subdivision_level: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureTrianglesOpacityMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTrianglesOpacityMicromapEXT.html)
pub struct AccelerationStructureTrianglesOpacityMicromapEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub index_type: crate::vk10::IndexType,
    pub index_buffer: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    pub index_stride: crate::vk10::DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
}
impl Default for AccelerationStructureTrianglesOpacityMicromapEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT,
            p_next: std::ptr::null_mut(),
            index_type: Default::default(),
            index_buffer: Default::default(),
            index_stride: Default::default(),
            base_triangle: Default::default(),
            usage_counts_count: Default::default(),
            p_usage_counts: std::ptr::null(),
            pp_usage_counts: std::ptr::null(),
            micromap: Default::default(),
        }
    }
}
#[doc(alias = "VkMicromapTypeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapTypeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MicromapTypeEXT(pub i32);
impl MicromapTypeEXT {
    pub const OPACITY_MICROMAP: Self = Self(0);
}
crate::enum_impl! {
    MicromapTypeEXT : i32, OPACITY_MICROMAP
}
#[doc(alias = "VkBuildMicromapFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BuildMicromapFlagsEXT(pub u32);
impl BuildMicromapFlagsEXT {
    pub const PREFER_FAST_TRACE: Self = Self(1 << 0);
    pub const PREFER_FAST_BUILD: Self = Self(1 << 1);
    pub const ALLOW_COMPACTION: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    BuildMicromapFlagsEXT : u32, 0x7, PREFER_FAST_TRACE, PREFER_FAST_BUILD,
    ALLOW_COMPACTION
}
#[doc(alias = "VkMicromapCreateFlagsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateFlagsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MicromapCreateFlagsEXT(pub u32);
impl MicromapCreateFlagsEXT {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 0);
}
crate::bitflags_impl! {
    MicromapCreateFlagsEXT : u32, 0x1, DEVICE_ADDRESS_CAPTURE_REPLAY
}
#[doc(alias = "VkCopyMicromapModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CopyMicromapModeEXT(pub i32);
impl CopyMicromapModeEXT {
    pub const CLONE: Self = Self(0);
    pub const SERIALIZE: Self = Self(1);
    pub const DESERIALIZE: Self = Self(2);
    pub const COMPACT: Self = Self(3);
}
crate::enum_impl! {
    CopyMicromapModeEXT : i32, CLONE, SERIALIZE, DESERIALIZE, COMPACT
}
#[doc(alias = "VkBuildMicromapModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapModeEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BuildMicromapModeEXT(pub i32);
impl BuildMicromapModeEXT {
    pub const BUILD: Self = Self(0);
}
crate::enum_impl! {
    BuildMicromapModeEXT : i32, BUILD
}
#[doc(alias = "VkOpacityMicromapFormatEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapFormatEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpacityMicromapFormatEXT(pub i32);
impl OpacityMicromapFormatEXT {
    pub const F2_STATE: Self = Self(1);
    pub const F4_STATE: Self = Self(2);
}
crate::enum_impl! {
    OpacityMicromapFormatEXT : i32, F2_STATE, F4_STATE
}
#[doc(alias = "VkOpacityMicromapSpecialIndexEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapSpecialIndexEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct OpacityMicromapSpecialIndexEXT(pub i32);
impl OpacityMicromapSpecialIndexEXT {
    pub const FULLY_TRANSPARENT: Self = Self(-1);
    pub const FULLY_OPAQUE: Self = Self(-2);
    pub const FULLY_UNKNOWN_TRANSPARENT: Self = Self(-3);
    pub const FULLY_UNKNOWN_OPAQUE: Self = Self(-4);
}
crate::enum_impl! {
    OpacityMicromapSpecialIndexEXT : i32, FULLY_TRANSPARENT, FULLY_OPAQUE,
    FULLY_UNKNOWN_TRANSPARENT, FULLY_UNKNOWN_OPAQUE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html)
pub unsafe fn create_micromap_ext(
    device: crate::vk10::Device,
    p_create_info: *const MicromapCreateInfoEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_micromap: *mut MicromapEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_micromap_ext
        .unwrap())(device, p_create_info, p_allocator, p_micromap)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html)
    pub unsafe fn create_micromap_ext(
        &self,
        create_info: &MicromapCreateInfoEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<MicromapEXT> {
        let create_micromap_ext = (*self.table).create_micromap_ext.unwrap();
        let mut micromap = Default::default();
        let result = create_micromap_ext(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut micromap,
        );
        crate::new_result(micromap, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBuildMicromapsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildMicromapsEXT.html)
pub unsafe fn cmd_build_micromaps_ext(
    command_buffer: crate::vk10::CommandBuffer,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_build_micromaps_ext
        .unwrap())(command_buffer, info_count, p_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildMicromapsEXT.html)
    pub unsafe fn cmd_build_micromaps_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        infos: &[MicromapBuildInfoEXT],
    ) {
        let cmd_build_micromaps_ext = (*self.table).cmd_build_micromaps_ext.unwrap();
        let info_count = infos.len();
        cmd_build_micromaps_ext(command_buffer, info_count as _, infos.as_ptr());
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBuildMicromapsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildMicromapsEXT.html)
pub unsafe fn build_micromaps_ext(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .build_micromaps_ext
        .unwrap())(device, deferred_operation, info_count, p_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBuildMicromapsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildMicromapsEXT.html)
    pub unsafe fn build_micromaps_ext(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT],
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let build_micromaps_ext = (*self.table).build_micromaps_ext.unwrap();
        let info_count = infos.len();
        let result = build_micromaps_ext(
            self.handle,
            deferred_operation,
            info_count as _,
            infos.as_ptr(),
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html)
pub unsafe fn destroy_micromap_ext(
    device: crate::vk10::Device,
    micromap: MicromapEXT,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_micromap_ext
        .unwrap())(device, micromap, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html)
    pub unsafe fn destroy_micromap_ext(
        &self,
        micromap: MicromapEXT,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_micromap_ext = (*self.table).destroy_micromap_ext.unwrap();
        destroy_micromap_ext(
            self.handle,
            micromap,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html)
pub unsafe fn cmd_copy_micromap_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyMicromapInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_micromap_ext
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html)
    pub unsafe fn cmd_copy_micromap_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyMicromapInfoEXT,
    ) {
        let cmd_copy_micromap_ext = (*self.table).cmd_copy_micromap_ext.unwrap();
        cmd_copy_micromap_ext(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html)
pub unsafe fn copy_micromap_ext(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyMicromapInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_micromap_ext
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html)
    pub unsafe fn copy_micromap_ext(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyMicromapInfoEXT,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_micromap_ext = (*self.table).copy_micromap_ext.unwrap();
        let result = copy_micromap_ext(self.handle, deferred_operation, info as _);
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapToMemoryEXT.html)
pub unsafe fn cmd_copy_micromap_to_memory_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyMicromapToMemoryInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_micromap_to_memory_ext
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapToMemoryEXT.html)
    pub unsafe fn cmd_copy_micromap_to_memory_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyMicromapToMemoryInfoEXT,
    ) {
        let cmd_copy_micromap_to_memory_ext = (*self.table)
            .cmd_copy_micromap_to_memory_ext
            .unwrap();
        cmd_copy_micromap_to_memory_ext(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyMicromapToMemoryEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapToMemoryEXT.html)
pub unsafe fn copy_micromap_to_memory_ext(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyMicromapToMemoryInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_micromap_to_memory_ext
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapToMemoryEXT.html)
    pub unsafe fn copy_micromap_to_memory_ext(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_micromap_to_memory_ext = (*self.table)
            .copy_micromap_to_memory_ext
            .unwrap();
        let result = copy_micromap_to_memory_ext(
            self.handle,
            deferred_operation,
            info as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToMicromapEXT.html)
pub unsafe fn cmd_copy_memory_to_micromap_ext(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyMemoryToMicromapInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_memory_to_micromap_ext
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToMicromapEXT.html)
    pub unsafe fn cmd_copy_memory_to_micromap_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyMemoryToMicromapInfoEXT,
    ) {
        let cmd_copy_memory_to_micromap_ext = (*self.table)
            .cmd_copy_memory_to_micromap_ext
            .unwrap();
        cmd_copy_memory_to_micromap_ext(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyMemoryToMicromapEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToMicromapEXT.html)
pub unsafe fn copy_memory_to_micromap_ext(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyMemoryToMicromapInfoEXT,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_memory_to_micromap_ext
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToMicromapEXT.html)
    pub unsafe fn copy_memory_to_micromap_ext(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_memory_to_micromap_ext = (*self.table)
            .copy_memory_to_micromap_ext
            .unwrap();
        let result = copy_memory_to_micromap_ext(
            self.handle,
            deferred_operation,
            info as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html)
pub unsafe fn cmd_write_micromaps_properties_ext(
    command_buffer: crate::vk10::CommandBuffer,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: crate::vk10::QueryType,
    query_pool: crate::vk10::QueryPool,
    first_query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_micromaps_properties_ext
        .unwrap())(
        command_buffer,
        micromap_count,
        p_micromaps,
        query_type,
        query_pool,
        first_query,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html)
    pub unsafe fn cmd_write_micromaps_properties_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        micromaps: &[MicromapEXT],
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        let cmd_write_micromaps_properties_ext = (*self.table)
            .cmd_write_micromaps_properties_ext
            .unwrap();
        let micromap_count = micromaps.len();
        cmd_write_micromaps_properties_ext(
            command_buffer,
            micromap_count as _,
            micromaps.as_ptr(),
            query_type as _,
            query_pool,
            first_query as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWriteMicromapsPropertiesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html)
pub unsafe fn write_micromaps_properties_ext(
    device: crate::vk10::Device,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: crate::vk10::QueryType,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
    stride: usize,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .write_micromaps_properties_ext
        .unwrap())(
        device,
        micromap_count,
        p_micromaps,
        query_type,
        data_size,
        p_data,
        stride,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html)
    pub unsafe fn write_micromaps_properties_ext(
        &self,
        micromaps: &[MicromapEXT],
        query_type: crate::vk10::QueryType,
        data_size: usize,
        data: *mut std::os::raw::c_void,
        stride: usize,
    ) -> crate::VulkanResult<()> {
        let write_micromaps_properties_ext = (*self.table)
            .write_micromaps_properties_ext
            .unwrap();
        let micromap_count = micromaps.len();
        let result = write_micromaps_properties_ext(
            self.handle,
            micromap_count as _,
            micromaps.as_ptr(),
            query_type as _,
            data_size,
            data,
            stride as _,
        );
        crate::new_result((), result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html)
pub unsafe fn get_device_micromap_compatibility_ext(
    device: crate::vk10::Device,
    p_version_info: *const MicromapVersionInfoEXT,
    p_compatibility: *mut crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_micromap_compatibility_ext
        .unwrap())(device, p_version_info, p_compatibility)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html)
    pub unsafe fn get_device_micromap_compatibility_ext(
        &self,
        version_info: &MicromapVersionInfoEXT,
    ) -> crate::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR {
        let get_device_micromap_compatibility_ext = (*self.table)
            .get_device_micromap_compatibility_ext
            .unwrap();
        let mut compatibility = Default::default();
        get_device_micromap_compatibility_ext(
            self.handle,
            version_info as _,
            &mut compatibility,
        );
        compatibility
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetMicromapBuildSizesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMicromapBuildSizesEXT.html)
pub unsafe fn get_micromap_build_sizes_ext(
    device: crate::vk10::Device,
    build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
    p_build_info: *const MicromapBuildInfoEXT,
    p_size_info: *mut MicromapBuildSizesInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_micromap_build_sizes_ext
        .unwrap())(device, build_type, p_build_info, p_size_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetMicromapBuildSizesEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMicromapBuildSizesEXT.html)
    pub unsafe fn get_micromap_build_sizes_ext(
        &self,
        build_type: crate::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
    ) -> MicromapBuildSizesInfoEXT {
        let get_micromap_build_sizes_ext = (*self.table)
            .get_micromap_build_sizes_ext
            .unwrap();
        let mut size_info = Default::default();
        get_micromap_build_sizes_ext(
            self.handle,
            build_type as _,
            build_info as _,
            &mut size_info,
        );
        size_info
    }
}
pub const EXT_OPACITY_MICROMAP_SPEC_VERSION: u32 = 2;
pub const EXT_OPACITY_MICROMAP_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_opacity_micromap"
);
