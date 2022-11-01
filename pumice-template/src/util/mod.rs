pub mod config;
pub mod impl_macros;
pub mod pnext;
pub mod result;

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
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
