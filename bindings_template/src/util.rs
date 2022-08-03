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

#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast::<char>()
    };
}

pub trait ObjectHandle {
    const TYPE: crate::vk10::ObjectType;
    fn to_raw(self) -> u64;
    fn from_raw(raw: u64) -> Self;
}

// adapted from erupt
#[doc(hidden)]
#[macro_export]
macro_rules! non_dispatchable_handle {
    ($name:ident, $ty:ident, $doc:literal, $doc_alias:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        pub struct $name(pub u64);

        impl $crate::util::ObjectHandle for $name {
            const TYPE: $crate::vk10::ObjectType = $crate::vk10::ObjectType::$ty;

            fn to_raw(self) -> u64 {
                self.0
            }

            fn from_raw(raw: u64) -> Self {
                $name(raw)
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}

// adapted from erupt
#[doc(hidden)]
#[macro_export]
macro_rules! dispatchable_handle {
    ($name:ident, $ty:ident, $doc_alias:literal, $doc:literal) => {
        #[doc = $doc]
        #[doc(alias = $doc_alias)]
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(pub *mut ());

        impl $crate::util::ObjectHandle for $name {
            const TYPE: $crate::vk10::ObjectType = $crate::vk10::ObjectType::$ty;

            fn to_raw(self) -> u64 {
                self.0 as u64
            }

            fn from_raw(raw: u64) -> Self {
                $name(raw as _)
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            fn default() -> $name {
                $name(std::ptr::null_mut())
            }
        }

        impl std::fmt::Pointer for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Pointer::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}
