use std::ffi::c_void;
use std::os::raw::c_char;

mod vk {
    pub type Instance = u64;
}

#[cfg(feature = "linked")]
extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const c_char)
        -> vk::PFN_vkVoidFunction;
}

#[cfg(feature = "linked")]
pub unsafe fn load_linked() -> fn(instance: vk::Instance, name: *const c_char) {
    vkGetInstanceProcAddr
}

#[cfg(feature = "loaded")]
pub unsafe fn load_dynamic(
) -> Result<fn(instance: vk::Instance, name: *const c_char), libloading::Error> {
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
    assert!(!(ptr as *const c_void).is_null());

    Ok(ptr)
}

pub struct Entry {}
