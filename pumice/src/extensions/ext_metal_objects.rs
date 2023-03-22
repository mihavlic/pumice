pub type MTLDevice_id = *mut std::os::raw::c_void;
pub type MTLCommandQueue_id = *mut std::os::raw::c_void;
pub type MTLBuffer_id = *mut std::os::raw::c_void;
pub type MTLTexture_id = *mut std::os::raw::c_void;
pub type MTLSharedEvent_id = *mut std::os::raw::c_void;
pub type IOSurfaceRef = *mut std::os::raw::c_void;
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalObjectCreateInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectCreateInfoEXT.html)
pub struct ExportMetalObjectCreateInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub export_object_type: ExportMetalObjectTypeFlagsEXT,
}
impl Default for ExportMetalObjectCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            export_object_type: Default::default(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalObjectsInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectsInfoEXT.html)
pub struct ExportMetalObjectsInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
}
impl Default for ExportMetalObjectsInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_OBJECTS_INFO_EXT,
            p_next: std::ptr::null(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalDeviceInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalDeviceInfoEXT.html)
pub struct ExportMetalDeviceInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mtl_device: MTLDevice_id,
}
impl Default for ExportMetalDeviceInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_DEVICE_INFO_EXT,
            p_next: std::ptr::null(),
            mtl_device: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalCommandQueueInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalCommandQueueInfoEXT.html)
pub struct ExportMetalCommandQueueInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub queue: crate::vk10::Queue,
    pub mtl_command_queue: MTLCommandQueue_id,
}
impl Default for ExportMetalCommandQueueInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT,
            p_next: std::ptr::null(),
            queue: Default::default(),
            mtl_command_queue: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalBufferInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalBufferInfoEXT.html)
pub struct ExportMetalBufferInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub memory: crate::vk10::DeviceMemory,
    pub mtl_buffer: MTLBuffer_id,
}
impl Default for ExportMetalBufferInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_BUFFER_INFO_EXT,
            p_next: std::ptr::null(),
            memory: Default::default(),
            mtl_buffer: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMetalBufferInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMetalBufferInfoEXT.html)
pub struct ImportMetalBufferInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mtl_buffer: MTLBuffer_id,
}
impl Default for ImportMetalBufferInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_METAL_BUFFER_INFO_EXT,
            p_next: std::ptr::null(),
            mtl_buffer: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalTextureInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalTextureInfoEXT.html)
pub struct ExportMetalTextureInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
    pub image_view: crate::vk10::ImageView,
    pub buffer_view: crate::vk10::BufferView,
    pub plane: crate::vk10::ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}
impl Default for ExportMetalTextureInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_TEXTURE_INFO_EXT,
            p_next: std::ptr::null(),
            image: Default::default(),
            image_view: Default::default(),
            buffer_view: Default::default(),
            plane: Default::default(),
            mtl_texture: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMetalTextureInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMetalTextureInfoEXT.html)
pub struct ImportMetalTextureInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub plane: crate::vk10::ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}
impl Default for ImportMetalTextureInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_METAL_TEXTURE_INFO_EXT,
            p_next: std::ptr::null(),
            plane: Default::default(),
            mtl_texture: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalIOSurfaceInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalIOSurfaceInfoEXT.html)
pub struct ExportMetalIOSurfaceInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub image: crate::vk10::Image,
    pub io_surface: IOSurfaceRef,
}
impl Default for ExportMetalIOSurfaceInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_IO_SURFACE_INFO_EXT,
            p_next: std::ptr::null(),
            image: Default::default(),
            io_surface: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMetalIOSurfaceInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMetalIOSurfaceInfoEXT.html)
pub struct ImportMetalIOSurfaceInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub io_surface: IOSurfaceRef,
}
impl Default for ImportMetalIOSurfaceInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT,
            p_next: std::ptr::null(),
            io_surface: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkExportMetalSharedEventInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalSharedEventInfoEXT.html)
pub struct ExportMetalSharedEventInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub semaphore: crate::vk10::Semaphore,
    pub event: crate::vk10::Event,
    pub mtl_shared_event: MTLSharedEvent_id,
}
impl Default for ExportMetalSharedEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::EXPORT_METAL_SHARED_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            event: Default::default(),
            mtl_shared_event: std::ptr::null_mut(),
        }
    }
}
#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkImportMetalSharedEventInfoEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMetalSharedEventInfoEXT.html)
pub struct ImportMetalSharedEventInfoEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub mtl_shared_event: MTLSharedEvent_id,
}
impl Default for ImportMetalSharedEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::IMPORT_METAL_SHARED_EVENT_INFO_EXT,
            p_next: std::ptr::null(),
            mtl_shared_event: std::ptr::null_mut(),
        }
    }
}
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExportMetalObjectTypeFlagsEXT(pub u32);
impl ExportMetalObjectTypeFlagsEXT {
    pub const METAL_DEVICE: Self = Self(1 << 0);
    pub const METAL_COMMAND_QUEUE: Self = Self(1 << 1);
    pub const METAL_BUFFER: Self = Self(1 << 2);
    pub const METAL_TEXTURE: Self = Self(1 << 3);
    pub const METAL_IOSURFACE: Self = Self(1 << 4);
    pub const METAL_SHARED_EVENT: Self = Self(1 << 5);
}
crate::bitflags_impl! {
    ExportMetalObjectTypeFlagsEXT : u32, 0x3f, METAL_DEVICE, METAL_COMMAND_QUEUE,
    METAL_BUFFER, METAL_TEXTURE, METAL_IOSURFACE, METAL_SHARED_EVENT
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkExportMetalObjectsEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html)
pub unsafe fn export_metal_objects_ext(
    device: crate::vk10::Device,
    p_metal_objects_info: *mut ExportMetalObjectsInfoEXT,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .export_metal_objects_ext
        .unwrap())(device, p_metal_objects_info)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkExportMetalObjectsEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html)
    pub unsafe fn export_metal_objects_ext(&self) -> ExportMetalObjectsInfoEXT {
        let export_metal_objects_ext = (*self.table).export_metal_objects_ext.unwrap();
        let mut metal_objects_info = Default::default();
        export_metal_objects_ext(self.handle, &mut metal_objects_info);
        metal_objects_info
    }
}
pub const EXT_METAL_OBJECTS_SPEC_VERSION: u32 = 1;
pub const EXT_METAL_OBJECTS_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_metal_objects"
);
