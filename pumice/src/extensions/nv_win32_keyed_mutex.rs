#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html)
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *const std::os::raw::c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const crate::vk10::DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeout_milliseconds: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const crate::vk10::DeviceMemory,
    pub p_release_keys: *const u64,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
            p_next: std::ptr::null(),
            acquire_count: Default::default(),
            p_acquire_syncs: std::ptr::null(),
            p_acquire_keys: std::ptr::null(),
            p_acquire_timeout_milliseconds: std::ptr::null(),
            release_count: Default::default(),
            p_release_syncs: std::ptr::null(),
            p_release_keys: std::ptr::null(),
        }
    }
}
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
pub const NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_NV_win32_keyed_mutex"
);
