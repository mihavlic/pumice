crate::dispatchable_handle!(
    AccelerationStructureKHR, ACCELERATION_STRUCTURE_KHR, "VkAccelerationStructureKHR",
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html)"
);
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html)
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureKHR,
}
impl Default for WriteDescriptorSetAccelerationStructureKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
            p_next: std::ptr::null(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html)
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub acceleration_structure: crate::vk10::Bool32,
    pub acceleration_structure_capture_replay: crate::vk10::Bool32,
    pub acceleration_structure_indirect_build: crate::vk10::Bool32,
    pub acceleration_structure_host_commands: crate::vk10::Bool32,
    pub descriptor_binding_acceleration_structure_update_after_bind: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            acceleration_structure: Default::default(),
            acceleration_structure_capture_replay: Default::default(),
            acceleration_structure_indirect_build: Default::default(),
            acceleration_structure_host_commands: Default::default(),
            descriptor_binding_acceleration_structure_update_after_bind: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html)
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_per_stage_descriptor_acceleration_structures: u32,
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    pub max_descriptor_set_acceleration_structures: u32,
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}
impl Default for PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            max_geometry_count: Default::default(),
            max_instance_count: Default::default(),
            max_primitive_count: Default::default(),
            max_per_stage_descriptor_acceleration_structures: Default::default(),
            max_per_stage_descriptor_update_after_bind_acceleration_structures: Default::default(),
            max_descriptor_set_acceleration_structures: Default::default(),
            max_descriptor_set_update_after_bind_acceleration_structures: Default::default(),
            min_acceleration_structure_scratch_offset_alignment: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkDeviceOrHostAddressKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html)
pub union DeviceOrHostAddressKHR {
    pub device_address: crate::vk10::DeviceAddress,
    pub host_address: *mut std::os::raw::c_void,
}
impl Default for DeviceOrHostAddressKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html)
pub union DeviceOrHostAddressConstKHR {
    pub device_address: crate::vk10::DeviceAddress,
    pub host_address: *const std::os::raw::c_void,
}
impl Default for DeviceOrHostAddressConstKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html)
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub vertex_format: crate::vk10::Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: crate::vk10::DeviceSize,
    pub max_vertex: u32,
    pub index_type: crate::vk10::IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub transform_data: DeviceOrHostAddressConstKHR,
}
impl Default for AccelerationStructureGeometryTrianglesDataKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
            p_next: std::ptr::null(),
            vertex_format: Default::default(),
            vertex_data: Default::default(),
            vertex_stride: Default::default(),
            max_vertex: Default::default(),
            index_type: Default::default(),
            index_data: Default::default(),
            transform_data: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html)
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: crate::vk10::DeviceSize,
}
impl Default for AccelerationStructureGeometryAabbsDataKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR,
            p_next: std::ptr::null(),
            data: Default::default(),
            stride: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html)
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub array_of_pointers: crate::vk10::Bool32,
    pub data: DeviceOrHostAddressConstKHR,
}
impl Default for AccelerationStructureGeometryInstancesDataKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
            p_next: std::ptr::null(),
            array_of_pointers: Default::default(),
            data: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html)
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}
impl Default for AccelerationStructureGeometryDataKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html)
pub struct AccelerationStructureGeometryKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}
impl Default for AccelerationStructureGeometryKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html)
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub kind: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub mode: BuildAccelerationStructureModeKHR,
    pub src_acceleration_structure: AccelerationStructureKHR,
    pub dst_acceleration_structure: AccelerationStructureKHR,
    pub geometry_count: u32,
    pub p_geometries: *const AccelerationStructureGeometryKHR,
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
}
impl Default for AccelerationStructureBuildGeometryInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            p_next: std::ptr::null(),
            kind: Default::default(),
            flags: Default::default(),
            mode: Default::default(),
            src_acceleration_structure: Default::default(),
            dst_acceleration_structure: Default::default(),
            geometry_count: Default::default(),
            p_geometries: std::ptr::null(),
            pp_geometries: std::ptr::null(),
            scratch_data: Default::default(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html)
pub struct AccelerationStructureBuildRangeInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}
impl Default for AccelerationStructureBuildRangeInfoKHR {
    fn default() -> Self {
        Self {
            primitive_count: Default::default(),
            primitive_offset: Default::default(),
            first_vertex: Default::default(),
            transform_offset: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html)
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    pub buffer: crate::vk10::Buffer,
    pub offset: crate::vk10::DeviceSize,
    pub size: crate::vk10::DeviceSize,
    pub kind: AccelerationStructureTypeKHR,
    pub device_address: crate::vk10::DeviceAddress,
}
impl Default for AccelerationStructureCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
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
#[doc(alias = "VkAabbPositionsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html)
pub struct AabbPositionsKHR {
    pub min_x: std::os::raw::c_float,
    pub min_y: std::os::raw::c_float,
    pub min_z: std::os::raw::c_float,
    pub max_x: std::os::raw::c_float,
    pub max_y: std::os::raw::c_float,
    pub max_z: std::os::raw::c_float,
}
impl Default for AabbPositionsKHR {
    fn default() -> Self {
        Self {
            min_x: Default::default(),
            min_y: Default::default(),
            min_z: Default::default(),
            max_x: Default::default(),
            max_y: Default::default(),
            max_z: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkTransformMatrixKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html)
pub struct TransformMatrixKHR {
    pub matrix: [[std::os::raw::c_float; 4]; 3],
}
impl Default for TransformMatrixKHR {
    fn default() -> Self {
        Self {
            matrix: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html)
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    pub instance_custom_index_mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureInstanceKHR {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            instance_custom_index_mask: Default::default(),
            instance_shader_binding_table_record_offset: Default::default(),
            flags: Default::default(),
            acceleration_structure_reference: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html)
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acceleration_structure: AccelerationStructureKHR,
}
impl Default for AccelerationStructureDeviceAddressInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html)
pub struct AccelerationStructureVersionInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub p_version_data: *const u8,
}
impl Default for AccelerationStructureVersionInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR,
            p_next: std::ptr::null(),
            p_version_data: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html)
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html)
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyAccelerationStructureToMemoryInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html)
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyMemoryToAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR,
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html)
pub struct AccelerationStructureBuildSizesInfoKHR {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acceleration_structure_size: crate::vk10::DeviceSize,
    pub update_scratch_size: crate::vk10::DeviceSize,
    pub build_scratch_size: crate::vk10::DeviceSize,
}
impl Default for AccelerationStructureBuildSizesInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
            p_next: std::ptr::null(),
            acceleration_structure_size: Default::default(),
            update_scratch_size: Default::default(),
            build_scratch_size: Default::default(),
        }
    }
}
#[doc(alias = "VkGeometryInstanceFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryInstanceFlagsKHR(pub u32);
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1 << 0);
    pub const TRIANGLE_FLIP_FACING: Self = Self(1 << 1);
    pub const FORCE_OPAQUE: Self = Self(1 << 2);
    pub const FORCE_NO_OPAQUE: Self = Self(1 << 3);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
    /// nv_ray_tracing
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
    /// ext_opacity_micromap
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(1 << 4);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 5);
}
crate::bitflags_impl! {
    GeometryInstanceFlagsKHR : u32, 0x3f, TRIANGLE_FACING_CULL_DISABLE,
    TRIANGLE_FLIP_FACING, FORCE_OPAQUE, FORCE_NO_OPAQUE,
    FORCE_OPACITY_MICROMAP_2_STATE_EXT, DISABLE_OPACITY_MICROMAPS_EXT
}
#[doc(alias = "VkGeometryFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryFlagsKHR(pub u32);
impl GeometryFlagsKHR {
    pub const OPAQUE: Self = Self(1 << 0);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(1 << 1);
    /// nv_ray_tracing
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
}
crate::bitflags_impl! {
    GeometryFlagsKHR : u32, 0x3, OPAQUE, NO_DUPLICATE_ANY_HIT_INVOCATION
}
#[doc(alias = "VkBuildAccelerationStructureFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BuildAccelerationStructureFlagsKHR(pub u32);
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE: Self = Self(1 << 0);
    pub const ALLOW_COMPACTION: Self = Self(1 << 1);
    pub const PREFER_FAST_TRACE: Self = Self(1 << 2);
    pub const PREFER_FAST_BUILD: Self = Self(1 << 3);
    pub const LOW_MEMORY: Self = Self(1 << 4);
    /// nv_ray_tracing
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
    /// nv_ray_tracing_motion_blur
    pub const MOTION_NV: Self = Self(1 << 5);
    /// ext_opacity_micromap
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(1 << 6);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 7);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(1 << 8);
}
crate::bitflags_impl! {
    BuildAccelerationStructureFlagsKHR : u32, 0x1ff, ALLOW_UPDATE, ALLOW_COMPACTION,
    PREFER_FAST_TRACE, PREFER_FAST_BUILD, LOW_MEMORY, MOTION_NV,
    ALLOW_OPACITY_MICROMAP_UPDATE_EXT, ALLOW_DISABLE_OPACITY_MICROMAPS_EXT,
    ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT
}
#[doc(alias = "VkAccelerationStructureCreateFlagsKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagsKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureCreateFlagsKHR(pub u32);
impl AccelerationStructureCreateFlagsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 0);
    /// nv_ray_tracing_motion_blur
    pub const MOTION_NV: Self = Self(1 << 2);
}
crate::bitflags_impl! {
    AccelerationStructureCreateFlagsKHR : u32, 0x5, DEVICE_ADDRESS_CAPTURE_REPLAY,
    MOTION_NV
}
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct CopyAccelerationStructureModeKHR(pub i32);
impl CopyAccelerationStructureModeKHR {
    pub const CLONE: Self = Self(0);
    pub const COMPACT: Self = Self(1);
    pub const SERIALIZE: Self = Self(2);
    pub const DESERIALIZE: Self = Self(3);
    /// nv_ray_tracing
    pub const CLONE_NV: Self = Self::CLONE;
    pub const COMPACT_NV: Self = Self::COMPACT;
}
crate::enum_impl! {
    CopyAccelerationStructureModeKHR : i32, CLONE, COMPACT, SERIALIZE, DESERIALIZE
}
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BuildAccelerationStructureModeKHR(pub i32);
impl BuildAccelerationStructureModeKHR {
    pub const BUILD: Self = Self(0);
    pub const UPDATE: Self = Self(1);
}
crate::enum_impl! {
    BuildAccelerationStructureModeKHR : i32, BUILD, UPDATE
}
#[doc(alias = "VkAccelerationStructureTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureTypeKHR(pub i32);
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL: Self = Self(0);
    pub const BOTTOM_LEVEL: Self = Self(1);
    pub const GENERIC: Self = Self(2);
    /// nv_ray_tracing
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL;
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL;
}
crate::enum_impl! {
    AccelerationStructureTypeKHR : i32, TOP_LEVEL, BOTTOM_LEVEL, GENERIC
}
#[doc(alias = "VkGeometryTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GeometryTypeKHR(pub i32);
impl GeometryTypeKHR {
    pub const TRIANGLES: Self = Self(0);
    pub const AABBS: Self = Self(1);
    pub const INSTANCES: Self = Self(2);
    /// nv_ray_tracing
    pub const TRIANGLES_NV: Self = Self::TRIANGLES;
    pub const AABBS_NV: Self = Self::AABBS;
}
crate::enum_impl! {
    GeometryTypeKHR : i32, TRIANGLES, AABBS, INSTANCES
}
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureBuildTypeKHR(pub i32);
impl AccelerationStructureBuildTypeKHR {
    pub const HOST: Self = Self(0);
    pub const DEVICE: Self = Self(1);
    pub const HOST_OR_DEVICE: Self = Self(2);
}
crate::enum_impl! {
    AccelerationStructureBuildTypeKHR : i32, HOST, DEVICE, HOST_OR_DEVICE
}
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureCompatibilityKHR(pub i32);
impl AccelerationStructureCompatibilityKHR {
    pub const COMPATIBLE: Self = Self(0);
    pub const INCOMPATIBLE: Self = Self(1);
}
crate::enum_impl! {
    AccelerationStructureCompatibilityKHR : i32, COMPATIBLE, INCOMPATIBLE
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
pub unsafe fn destroy_acceleration_structure_khr(
    device: crate::vk10::Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .destroy_acceleration_structure_khr
        .unwrap())(device, acceleration_structure, p_allocator)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) {
        let destroy_acceleration_structure_khr = (*self.table)
            .destroy_acceleration_structure_khr
            .unwrap();
        destroy_acceleration_structure_khr(
            self.handle,
            acceleration_structure,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
pub unsafe fn cmd_copy_acceleration_structure_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyAccelerationStructureInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_acceleration_structure_khr
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR,
    ) {
        let cmd_copy_acceleration_structure_khr = (*self.table)
            .cmd_copy_acceleration_structure_khr
            .unwrap();
        cmd_copy_acceleration_structure_khr(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html)
pub unsafe fn copy_acceleration_structure_khr(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_acceleration_structure_khr
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html)
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_acceleration_structure_khr = (*self.table)
            .copy_acceleration_structure_khr
            .unwrap();
        let result = copy_acceleration_structure_khr(
            self.handle,
            deferred_operation,
            info as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_acceleration_structure_to_memory_khr
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        let cmd_copy_acceleration_structure_to_memory_khr = (*self.table)
            .cmd_copy_acceleration_structure_to_memory_khr
            .unwrap();
        cmd_copy_acceleration_structure_to_memory_khr(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
pub unsafe fn copy_acceleration_structure_to_memory_khr(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_acceleration_structure_to_memory_khr
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_acceleration_structure_to_memory_khr = (*self.table)
            .copy_acceleration_structure_to_memory_khr
            .unwrap();
        let result = copy_acceleration_structure_to_memory_khr(
            self.handle,
            deferred_operation,
            info as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
    command_buffer: crate::vk10::CommandBuffer,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_copy_memory_to_acceleration_structure_khr
        .unwrap())(command_buffer, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        let cmd_copy_memory_to_acceleration_structure_khr = (*self.table)
            .cmd_copy_memory_to_acceleration_structure_khr
            .unwrap();
        cmd_copy_memory_to_acceleration_structure_khr(command_buffer, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
pub unsafe fn copy_memory_to_acceleration_structure_khr(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .copy_memory_to_acceleration_structure_khr
        .unwrap())(device, deferred_operation, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let copy_memory_to_acceleration_structure_khr = (*self.table)
            .copy_memory_to_acceleration_structure_khr
            .unwrap();
        let result = copy_memory_to_acceleration_structure_khr(
            self.handle,
            deferred_operation,
            info as _,
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
pub unsafe fn cmd_write_acceleration_structures_properties_khr(
    command_buffer: crate::vk10::CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: crate::vk10::QueryType,
    query_pool: crate::vk10::QueryPool,
    first_query: u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_write_acceleration_structures_properties_khr
        .unwrap())(
        command_buffer,
        acceleration_structure_count,
        p_acceleration_structures,
        query_type,
        query_pool,
        first_query,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: crate::vk10::QueryType,
        query_pool: crate::vk10::QueryPool,
        first_query: u32,
    ) {
        let cmd_write_acceleration_structures_properties_khr = (*self.table)
            .cmd_write_acceleration_structures_properties_khr
            .unwrap();
        let acceleration_structure_count = acceleration_structures.len();
        cmd_write_acceleration_structures_properties_khr(
            command_buffer,
            acceleration_structure_count as _,
            acceleration_structures.as_ptr(),
            query_type as _,
            query_pool,
            first_query as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
pub unsafe fn write_acceleration_structures_properties_khr(
    device: crate::vk10::Device,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: crate::vk10::QueryType,
    data_size: usize,
    p_data: *mut std::os::raw::c_void,
    stride: usize,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .write_acceleration_structures_properties_khr
        .unwrap())(
        device,
        acceleration_structure_count,
        p_acceleration_structures,
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
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
    pub unsafe fn write_acceleration_structures_properties_khr(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: crate::vk10::QueryType,
        data_size: usize,
        data: *mut std::os::raw::c_void,
        stride: usize,
    ) -> crate::VulkanResult<()> {
        let write_acceleration_structures_properties_khr = (*self.table)
            .write_acceleration_structures_properties_khr
            .unwrap();
        let acceleration_structure_count = acceleration_structures.len();
        let result = write_acceleration_structures_properties_khr(
            self.handle,
            acceleration_structure_count as _,
            acceleration_structures.as_ptr(),
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
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
pub unsafe fn get_device_acceleration_structure_compatibility_khr(
    device: crate::vk10::Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_device_acceleration_structure_compatibility_khr
        .unwrap())(device, p_version_info, p_compatibility)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        let get_device_acceleration_structure_compatibility_khr = (*self.table)
            .get_device_acceleration_structure_compatibility_khr
            .unwrap();
        let mut compatibility = Default::default();
        get_device_acceleration_structure_compatibility_khr(
            self.handle,
            version_info as _,
            &mut compatibility,
        );
        compatibility
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCreateAccelerationStructureKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)
pub unsafe fn create_acceleration_structure_khr(
    device: crate::vk10::Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR,
    p_allocator: *const crate::vk10::AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .create_acceleration_structure_khr
        .unwrap())(device, p_create_info, p_allocator, p_acceleration_structure)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&crate::vk10::AllocationCallbacks>,
    ) -> crate::VulkanResult<AccelerationStructureKHR> {
        let create_acceleration_structure_khr = (*self.table)
            .create_acceleration_structure_khr
            .unwrap();
        let mut acceleration_structure = Default::default();
        let result = create_acceleration_structure_khr(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut acceleration_structure,
        );
        crate::new_result(acceleration_structure, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
pub unsafe fn cmd_build_acceleration_structures_khr(
    command_buffer: crate::vk10::CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_build_acceleration_structures_khr
        .unwrap())(command_buffer, info_count, p_infos, pp_build_range_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) {
        let cmd_build_acceleration_structures_khr = (*self.table)
            .cmd_build_acceleration_structures_khr
            .unwrap();
        let info_count = infos.len().min(build_range_infos.len());
        cmd_build_acceleration_structures_khr(
            command_buffer,
            info_count as _,
            infos.as_ptr(),
            build_range_infos.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
    command_buffer: crate::vk10::CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    p_indirect_device_addresses: *const crate::vk10::DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_build_acceleration_structures_indirect_khr
        .unwrap())(
        command_buffer,
        info_count,
        p_infos,
        p_indirect_device_addresses,
        p_indirect_strides,
        pp_max_primitive_counts,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[crate::vk10::DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[*const u32],
    ) {
        let cmd_build_acceleration_structures_indirect_khr = (*self.table)
            .cmd_build_acceleration_structures_indirect_khr
            .unwrap();
        let info_count = infos
            .len()
            .min(indirect_device_addresses.len())
            .min(indirect_strides.len())
            .min(max_primitive_counts.len());
        cmd_build_acceleration_structures_indirect_khr(
            command_buffer,
            info_count as _,
            infos.as_ptr(),
            indirect_device_addresses.as_ptr(),
            indirect_strides.as_ptr(),
            max_primitive_counts.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
pub unsafe fn build_acceleration_structures_khr(
    device: crate::vk10::Device,
    deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> crate::vk10::Result {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .build_acceleration_structures_khr
        .unwrap())(device, deferred_operation, info_count, p_infos, pp_build_range_infos)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[must_use]
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        deferred_operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> crate::VulkanResult<crate::vk10::Result> {
        let build_acceleration_structures_khr = (*self.table)
            .build_acceleration_structures_khr
            .unwrap();
        let info_count = infos.len().min(build_range_infos.len());
        let result = build_acceleration_structures_khr(
            self.handle,
            deferred_operation,
            info_count as _,
            infos.as_ptr(),
            build_range_infos.as_ptr(),
        );
        crate::new_result(result, result)
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
pub unsafe fn get_acceleration_structure_device_address_khr(
    device: crate::vk10::Device,
    p_info: *const AccelerationStructureDeviceAddressInfoKHR,
) -> crate::vk10::DeviceAddress {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_acceleration_structure_device_address_khr
        .unwrap())(device, p_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) {
        let get_acceleration_structure_device_address_khr = (*self.table)
            .get_acceleration_structure_device_address_khr
            .unwrap();
        get_acceleration_structure_device_address_khr(self.handle, info as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
pub unsafe fn get_acceleration_structure_build_sizes_khr(
    device: crate::vk10::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .get_acceleration_structure_build_sizes_khr
        .unwrap())(device, build_type, p_build_info, p_max_primitive_counts, p_size_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: &[u32],
    ) -> AccelerationStructureBuildSizesInfoKHR {
        let get_acceleration_structure_build_sizes_khr = (*self.table)
            .get_acceleration_structure_build_sizes_khr
            .unwrap();
        let mut size_info = Default::default();
        get_acceleration_structure_build_sizes_khr(
            self.handle,
            build_type as _,
            build_info as _,
            max_primitive_counts.as_ptr(),
            &mut size_info,
        );
        size_info
    }
}
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_KHR_acceleration_structure"
);
