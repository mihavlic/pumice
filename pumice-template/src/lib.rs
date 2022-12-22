#![allow(unused_unsafe)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod deep_copy;
pub mod dumb_hash;
pub mod loader;
#[cfg(feature = "surface")]
pub mod surface;
pub mod util;

pub mod extensions;

pub type VulkanResult<T> = Result<T, vk::Result>;
#[inline]
pub fn new_result<T>(value: T, result: vk::Result) -> VulkanResult<T> {
    if result.0.is_negative() {
        VulkanResult::Err(result)
    } else {
        VulkanResult::Ok(value)
    }
}
impl From<vk::Result> for VulkanResult<()> {
    fn from(value: vk::Result) -> Self {
        new_result((), value)
    }
}
