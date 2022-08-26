pub mod tables;

use crate::vk;
use std::os::raw::{c_char, c_void};

#[cfg(feature = "linked")]
extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const c_char) -> vk::PfnVoidFunction;
}

pub trait FunctionLoad {
    unsafe fn load(&self, name: *const c_char) -> vk::PfnVoidFunction;
}

pub trait EntryLoad: FunctionLoad {
    #[allow(non_snake_case)]
    unsafe fn get_vkGetInstanceProcAddr(
        &self,
    ) -> unsafe extern "system" fn(vk::Instance, name: *const c_char) -> vk::PfnVoidFunction;
}
pub trait InstanceLoad: FunctionLoad {}
pub trait DeviceLoad: FunctionLoad {}

#[cfg(feature = "linked")]
pub type EntryLoader = LinkedEntryLoader;
#[cfg(all(not(feature = "linked"), feature = "loaded"))]
pub type EntryLoader = LoadedEntryLoader;

#[cfg(feature = "linked")]
pub struct LinkedEntryLoader;

#[cfg(feature = "loaded")]
#[allow(non_snake_case)]
pub struct LoadedEntryLoader {
    _lib: libloading::Library,
    vkGetInstanceProcAddr:
        unsafe extern "system" fn(vk::Instance, name: *const c_char) -> vk::PfnVoidFunction,
}

#[allow(non_snake_case)]
pub struct InstanceLoader {
    vkGetInstanceProcAddr:
        unsafe extern "system" fn(vk::Instance, name: *const c_char) -> vk::PfnVoidFunction,
    instance: vk::Instance,
}

#[allow(non_snake_case)]
pub struct DeviceLoader {
    vkGetDeviceProcAddr:
        unsafe extern "system" fn(vk::Device, name: *const c_char) -> vk::PfnVoidFunction,
    device: vk::Device,
}

#[cfg(feature = "linked")]
impl EntryLoad for LinkedEntryLoader {
    unsafe fn get_vkGetInstanceProcAddr(
        &self,
    ) -> unsafe extern "system" fn(vk::Instance, name: *const c_char) -> vk::PfnVoidFunction {
        vkGetInstanceProcAddr
    }
}
#[cfg(feature = "linked")]
impl FunctionLoad for LinkedEntryLoader {
    unsafe fn load(&self, name: *const c_char) -> vk::PfnVoidFunction {
        // in this case it is correct to pass in null
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
        vkGetInstanceProcAddr(std::mem::transmute(0u64), name)
    }
}

#[cfg(feature = "loaded")]
impl EntryLoad for LoadedEntryLoader {
    unsafe fn get_vkGetInstanceProcAddr(
        &self,
    ) -> unsafe extern "system" fn(vk::Instance, name: *const c_char) -> vk::PfnVoidFunction {
        self.vkGetInstanceProcAddr
    }
}
#[cfg(feature = "loaded")]
impl FunctionLoad for LoadedEntryLoader {
    unsafe fn load(&self, name: *const c_char) -> vk::PfnVoidFunction {
        (self.vkGetInstanceProcAddr)(std::mem::transmute(0u64), name)
    }
}

impl InstanceLoad for InstanceLoader {}
impl DeviceLoad for InstanceLoader {}
impl FunctionLoad for InstanceLoader {
    unsafe fn load(&self, name: *const c_char) -> vk::PfnVoidFunction {
        (self.vkGetInstanceProcAddr)(self.instance, name)
    }
}

impl DeviceLoad for DeviceLoader {}
impl FunctionLoad for DeviceLoader {
    unsafe fn load(&self, name: *const c_char) -> vk::PfnVoidFunction {
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html
        (self.vkGetDeviceProcAddr)(self.device, name)
    }
}

#[derive(Debug)]
pub struct LinkedLoadError;

impl std::fmt::Display for LinkedLoadError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!("This type should never get constructed!")
    }
}

impl std::error::Error for LinkedLoadError {}

#[cfg(feature = "linked")]
impl LinkedEntryLoader {
    /// This function never fails, however we want to have it consistent with LoadedEntryLoader
    pub unsafe fn new() -> Result<Self, LinkedLoadError> {
        Ok(LinkedEntryLoader)
    }
}

#[cfg(feature = "loaded")]
impl LoadedEntryLoader {
    pub unsafe fn new() -> Result<Self, libloading::Error> {
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

        Self::new_with_path(LIB_PATH)
    }
    pub unsafe fn new_with_path(path: &str) -> Result<Self, libloading::Error> {
        let lib = libloading::Library::new(path)?;

        static ENTRY_POINT: &[u8] = b"vkGetInstanceProcAddr\0";

        let symbol = lib.get(ENTRY_POINT).unwrap();

        Ok(Self {
            vkGetInstanceProcAddr: *symbol,
            _lib: lib,
        })
    }
}

impl InstanceLoader {
    pub unsafe fn new(instance: vk::Instance, entry: &impl EntryLoad) -> Self {
        Self {
            vkGetInstanceProcAddr: entry.get_vkGetInstanceProcAddr(),
            instance,
        }
    }
}

impl DeviceLoader {
    pub fn new(device: vk::Device, entry: &impl DeviceLoad) -> Self {
        let ptr = unsafe { entry.load("vkGetDeviceProcAddr\0".as_ptr().cast()) };
        assert!(
            (ptr as *const c_void).is_null(),
            "vkGetInstanceProcAddr returned a null pointer, that's not good!"
        );
        let fun = unsafe { std::mem::transmute(ptr) };

        Self {
            vkGetDeviceProcAddr: fun,
            device,
        }
    }
}
