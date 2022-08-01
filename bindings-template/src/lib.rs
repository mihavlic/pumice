#![allow(unused_imports)]

pub mod loader;

// aliases providing definitions for some types from vk_platform.h
#[allow(non_camel_case_types)]
pub type void = std::os::raw::c_void;
#[allow(non_camel_case_types)]
pub type char = std::os::raw::c_char;
#[allow(non_camel_case_types)]
pub type float = std::os::raw::c_float;
#[allow(non_camel_case_types)]
pub type double = std::os::raw::c_double;
#[allow(non_camel_case_types)]
pub type int = std::os::raw::c_int;

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast::<char>()
    };
}
