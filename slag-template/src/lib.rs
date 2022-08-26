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
