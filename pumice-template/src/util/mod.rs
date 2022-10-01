pub mod config;
pub mod impl_macros;
pub mod pnext;
pub mod result;

use std::ffi::CStr;

#[doc(hidden)]
#[inline]
/// An implementation detail of the `cstr!` macro.
pub const fn __cstr_impl(str: &str) -> &CStr {
    // internally validated
    unsafe { CStr::from_bytes_with_nul_unchecked(str.as_bytes()) }
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        $crate::util::__cstr_impl(concat!($s, "\0"))
    };
}

#[macro_export]
macro_rules! vkcall {
    ($ok:ident, $call:expr) => {{
        let mut value = Default::default();
        let $ok = &mut value as *mut _;
        let raw = $call;
        $crate::util::result::VulkanResult::new(raw, value)
    }};
    ($call:expr) => {{
        let raw = $call;
        $crate::util::result::VulkanResult::new(raw, ())
    }};
}
