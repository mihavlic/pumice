use crate::vk;
use crate::{
    cstr, dispatchable_handle, non_dispatchable_handle,
    util::{char, double, float, int, void},
};

#[cfg(feature = "linked")]
extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const char) -> vk::PFN_vkVoidFunction;
}

#[cfg(feature = "linked")]
pub unsafe fn load_linked() -> fn(instance: vk::Instance, name: *const char) {
    vkGetInstanceProcAddr
}

#[cfg(feature = "loaded")]
pub unsafe fn load_dynamic(
) -> Result<fn(instance: vk::Instance, name: *const char), libloading::Error> {
    #[cfg(windows)]
    const LIB_PATH: &str = "vulkan-1.dll";

    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "ios", target_os = "android"))
    ))]
    const LIB_PATH: &str = "libvulkan.so.1";

    #[cfg(target_os = "android")]
    const LIB_PATH: &str = "libvulkan.so";

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const LIB_PATH: &str = "libvulkan.dylib";

    let lib = libloading::Library::new(LIB_PATH)?;

    static ENTRY_POINT: &[u8] = b"vkGetInstanceProcAddr\0";

    let ptr = *lib.get(ENTRY_POINT).unwrap();
    assert!(!(ptr as *const void).is_null());

    Ok(ptr)
}

pub trait FunctionLoad {
    unsafe fn load(&self, name: *const char) -> *const void;
}

pub trait EntryLoad: FunctionLoad {}

pub trait InstanceLoad: FunctionLoad {}

pub trait DeviceLoad: FunctionLoad {}

pub struct EntryLoader {
    vkGetInstanceProcAddr: extern "system" fn(vk::Instance, name: *const char) -> *const void,
}

impl FunctionLoad for EntryLoader {
    unsafe fn load(&self, name: *const char) -> *const void {
        // in this case it is correct to pass in null
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
        (self.vkGetInstanceProcAddr)(std::mem::transmute(0u64), name)
    }
}

pub struct InstanceLoader {
    vkGetInstanceProcAddr: extern "system" fn(vk::Instance, name: *const char) -> *const void,
    instance: vk::Instance,
}

pub struct Device {
    vkGetDeviceProcAddr: extern "system" fn(vk::Device, name: *const char) -> *const void,
    device: vk::Device,
}
