#![allow(unused, non_camel_case_types)]
mod loader;

// aliases providing definitions for some types from vk_platform.h
pub type void = std::os::raw::c_void;
pub type char = std::os::raw::c_char;
pub type float = std::os::raw::c_float;
pub type double = std::os::raw::c_double;
pub type int = std::os::raw::c_int;

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast::<char>()
    };
}
