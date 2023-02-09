#[doc(alias = "VkAccelerationStructureMotionInfoFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInfoFlagsNV(pub u32);
crate::bitflags_impl! {
    AccelerationStructureMotionInfoFlagsNV : u32, 0x0,
}
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagsNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceFlagsNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceFlagsNV(pub u32);
crate::bitflags_impl! {
    AccelerationStructureMotionInstanceFlagsNV : u32, 0x0,
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html)
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub ray_tracing_motion_blur: crate::vk10::Bool32,
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            ray_tracing_motion_blur: Default::default(),
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html)
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub vertex_data: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
}
impl Default for AccelerationStructureGeometryMotionTrianglesDataNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV,
            p_next: std::ptr::null(),
            vertex_data: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoNV.html)
pub struct AccelerationStructureMotionInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}
impl Default for AccelerationStructureMotionInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV,
            p_next: std::ptr::null(),
            max_instances: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkSRTDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSRTDataNV.html)
pub struct SRTDataNV {
    pub sx: std::os::raw::c_float,
    pub a: std::os::raw::c_float,
    pub b: std::os::raw::c_float,
    pub pvx: std::os::raw::c_float,
    pub sy: std::os::raw::c_float,
    pub c: std::os::raw::c_float,
    pub pvy: std::os::raw::c_float,
    pub sz: std::os::raw::c_float,
    pub pvz: std::os::raw::c_float,
    pub qx: std::os::raw::c_float,
    pub qy: std::os::raw::c_float,
    pub qz: std::os::raw::c_float,
    pub qw: std::os::raw::c_float,
    pub tx: std::os::raw::c_float,
    pub ty: std::os::raw::c_float,
    pub tz: std::os::raw::c_float,
}
impl Default for SRTDataNV {
    fn default() -> Self {
        Self {
            sx: Default::default(),
            a: Default::default(),
            b: Default::default(),
            pvx: Default::default(),
            sy: Default::default(),
            c: Default::default(),
            pvy: Default::default(),
            sz: Default::default(),
            pvz: Default::default(),
            qx: Default::default(),
            qy: Default::default(),
            qz: Default::default(),
            qw: Default::default(),
            tx: Default::default(),
            ty: Default::default(),
            tz: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureSRTMotionInstanceNV.html)
pub struct AccelerationStructureSRTMotionInstanceNV {
    pub transform_t_0: SRTDataNV,
    pub transform_t_1: SRTDataNV,
    pub instance_custom_index_mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureSRTMotionInstanceNV {
    fn default() -> Self {
        Self {
            transform_t_0: Default::default(),
            transform_t_1: Default::default(),
            instance_custom_index_mask: Default::default(),
            instance_shader_binding_table_record_offset: Default::default(),
            flags: Default::default(),
            acceleration_structure_reference: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMatrixMotionInstanceNV.html)
pub struct AccelerationStructureMatrixMotionInstanceNV {
    pub transform_t_0: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    pub transform_t_1: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    pub instance_custom_index_mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureMatrixMotionInstanceNV {
    fn default() -> Self {
        Self {
            transform_t_0: Default::default(),
            transform_t_1: Default::default(),
            instance_custom_index_mask: Default::default(),
            instance_shader_binding_table_record_offset: Default::default(),
            flags: Default::default(),
            acceleration_structure_reference: Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html)
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}
impl Default for AccelerationStructureMotionInstanceDataNV {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html)
pub struct AccelerationStructureMotionInstanceNV {
    pub kind: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}
impl Default for AccelerationStructureMotionInstanceNV {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            flags: Default::default(),
            data: Default::default(),
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMotionInstanceTypeNV(pub i32);
impl AccelerationStructureMotionInstanceTypeNV {
    pub const STATIC: Self = Self(0);
    pub const MATRIX_MOTION: Self = Self(1);
    pub const SRT_MOTION: Self = Self(2);
}
crate::enum_impl! {
    AccelerationStructureMotionInstanceTypeNV : i32, STATIC, MATRIX_MOTION, SRT_MOTION
}
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
pub const NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_ray_tracing_motion_blur"
);
