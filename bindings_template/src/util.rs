use crate::{extensions::metadata::get_metadata, vk};
use std::{
    collections::HashSet,
    error::Error,
    ffi::CStr,
    fmt::{self, Debug, Display},
    os::raw::c_char,
    slice,
};

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
#[inline]
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
        $crate::util::VulkanResult::new(raw, value)
    }};
    ($call:expr) => {{
        let raw = $call;
        $crate::util::VulkanResult::new(raw, ())
    }};
}

#[macro_export]
macro_rules! enum_impl {
    ($name:ident: $ty:ident, $($variant:ident),*) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(
                    match *self {
                        $(
                            Self::$variant => stringify!($variant),
                        )+
                        _ => return f.write_str("(unknown)")
                    }
                )
            }
        }
    };
}

#[macro_export]
macro_rules! bitflags_impl {
    ($name:ident: $ty:ident, $all:expr, $($variant:ident),*) => {
        impl $name {
            #[inline]
            pub const fn empty() -> $name {
                $name(0)
            }
            #[inline]
            pub const fn all() -> $name {
                $name($all)
            }
            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::empty()
            }
            #[inline]
            pub fn is_all(self) -> bool {
                self & $name::all() == $name::all()
            }
            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::empty()
            }
            #[inline]
            pub fn contains(self, other: $name) -> bool {
                self & other == other
            }
        }
        impl ::std::ops::BitOr for $name {
            type Output = $name;
            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name(self.0 | rhs.0)
            }
        }
        impl ::std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }
        impl ::std::ops::BitAnd for $name {
            type Output = $name;
            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name(self.0 & rhs.0)
            }
        }
        impl ::std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }
        impl ::std::ops::BitXor for $name {
            type Output = $name;
            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name(self.0 ^ rhs.0)
            }
        }
        impl ::std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }
        impl ::std::ops::Sub for $name {
            type Output = $name;
            #[inline]
            fn sub(self, rhs: $name) -> $name {
                self & !rhs
            }
        }
        impl ::std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs
            }
        }
        impl ::std::ops::Not for $name {
            type Output = $name;
            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut first = true;
                $(
                    if self.contains(Self::$variant) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(::std::stringify!($variant))?;
                    }
                )*
                let extra_bits = self.0 & !Self::all().0;
                if extra_bits != 0 {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    f.write_str("0x")?;
                    ::std::fmt::LowerHex::fmt(&extra_bits, f)?;
                }
                if first {
                    f.write_str("(empty)")?;
                }
                Ok(())
            }
        }
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

pub struct ApiLoadConfig<'a> {
    core_version: u32,
    extensions: HashSet<&'a CStr>,
}

impl<'a> ApiLoadConfig<'a> {
    pub fn new(core_version: u32) -> Self {
        Self {
            core_version,
            extensions: HashSet::new(),
        }
    }
    pub fn new_with_extensions(core_version: u32, extensions: &[&'a CStr]) -> Self {
        let mut s = Self {
            core_version,
            extensions: HashSet::new(),
        };
        for &e in extensions {
            s.add_extension(e);
        }
        s
    }
    #[cfg(feature = "surface")]
    pub fn enable_surface_extensions(
        &mut self,
        window_handle: &impl raw_window_handle::HasRawWindowHandle,
    ) {
        for &e in crate::surface::enumerate_required_extensions(window_handle).unwrap() {
            self.add_extension(e);
        }
    }
    pub fn enable_all_extensions(&mut self) {
        for meta in crate::extensions::metadata::EXTENSION_METADATA {
            self.add_extension(meta.name);
        }
    }
    pub fn get_instance_extensions(&self) -> Vec<*const c_char> {
        self.extensions
            .iter()
            .filter(|&&e| get_metadata(e).unwrap().instance == true)
            .map(|&e| e.as_ptr().cast())
            .collect()
    }
    pub fn get_device_extensions(&self) -> Vec<*const c_char> {
        self.extensions
            .iter()
            .filter(|&&e| get_metadata(e).unwrap().instance == false)
            .map(|&e| e.as_ptr().cast())
            .collect()
    }
    pub fn core_enabled(&self, version: u32) -> bool {
        self.core_version >= version
    }
    pub fn extension_enabled(&self, name: &CStr) -> bool {
        self.extensions.contains(&name)
    }
    #[track_caller]
    pub fn add_extension(&mut self, name: &'a CStr) {
        let meta = get_metadata(name).unwrap_or_else(|| {
            panic!(
                "'{}' is not a valid extension. Maybe it was not generated?",
                name.to_str().unwrap_or("Invalid UTF8!")
            )
        });
        if meta.core_version < self.core_version {
            panic!(
                "'{}' requires a higher core version than is configured",
                name.to_str().unwrap_or("Invalid UTF8!")
            )
        }
        self.extensions.insert(name);
    }
    /// Uses the embedded extention metadata to fill in all currently selected extensions' transient dependencies.
    pub fn fill_in_extensions(&mut self) -> Result<(), ApiLoadConfigErr> {
        let extensions = self.extensions.iter().cloned().collect::<Vec<_>>();
        self.extensions.clear();
        for e in extensions {
            self.foreach_extension_dependency(e)?;
        }
        Ok(())
    }
    fn foreach_extension_dependency(&mut self, name: &CStr) -> Result<(), ApiLoadConfigErr> {
        let extension = get_metadata(name).ok_or(ApiLoadConfigErr::ExtensionNotFound)?;

        if extension.core_version < self.core_version {
            return Err(ApiLoadConfigErr::ApiVersionMismatch);
        }

        if self.extensions.insert(extension.name) == false {
            return Ok(());
        }

        for &name in extension.requires_extensions {
            self.foreach_extension_dependency(name)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ApiLoadConfigErr {
    ApiVersionMismatch,
    ExtensionNotFound,
}

impl Display for ApiLoadConfigErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiLoadConfigErr::ApiVersionMismatch => {
                write!(f, "Not a valid extension. Maybe it was not generated?")
            }
            ApiLoadConfigErr::ExtensionNotFound => write!(
                f,
                "Extension requires a higher core version than is configured."
            ),
        }
    }
}

impl Error for ApiLoadConfigErr {}

/// Idiomatic wrapper around a Vulkan Result.
///
/// In order to use this type with the `?` operator, call the
/// [`.result()`](VulkanResult::result) method first.
/// Using the `?` operator directly on this type is blocked on
/// [rust-lang/rust#84277](https://github.com/rust-lang/rust/issues/84277).
#[must_use = "this `VulkanResult` may be an error, which should be handled"]
#[derive(Clone, Default)]
pub struct VulkanResult<T> {
    /// The raw result from Vulkan.
    pub raw: vk::Result,
    /// The value this wrapper type may be holding.
    pub value: Option<T>,
}

impl<T> VulkanResult<T> {
    /// Construct a new `VulkanResult` from `raw` and `value`.
    ///
    /// This will not populate `self.value`
    /// if the raw result is negative (Error).
    #[inline]
    pub fn new(raw: vk::Result, value: T) -> VulkanResult<T> {
        let value = if raw.0.is_negative() {
            None
        } else {
            Some(value)
        };

        VulkanResult { raw, value }
    }

    /// Returns the contained value, consuming `self`.
    ///
    /// Panics with the name of `self.raw` if `self.value` is `None`.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}", self.raw),
        }
    }

    /// Returns the contained value, consuming `self`.
    ///
    /// Panics with `msg` and the name of `self.raw` if `self.value` is `None`.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: impl Display) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}: {}", self.raw, msg),
        }
    }

    /// Converts from `&VulkanResult<T>` to `VulkanResult<&T>`.
    ///
    /// Clones `self.raw`.
    #[inline]
    pub fn as_ref(&self) -> VulkanResult<&T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_ref(),
        }
    }

    /// Converts from `&mut VulkanResult<T>` to `VulkanResult<&mut T>`.
    ///
    /// Clones `self.raw`.
    #[inline]
    pub fn as_mut(&mut self) -> VulkanResult<&mut T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_mut(),
        }
    }

    /// Constructs a new `VulkanResult` from `value`.
    ///
    /// This will always set `self.raw` to `vk::Result::SUCCESS`.
    #[inline]
    pub fn new_ok(value: T) -> VulkanResult<T> {
        VulkanResult::new(vk::Result::SUCCESS, value)
    }

    /// Constructs a new `VulkanResult` from `raw`.
    ///
    /// This will always set `self.value` to `None`.
    #[inline]
    pub fn new_err(raw: vk::Result) -> VulkanResult<T> {
        VulkanResult { raw, value: None }
    }

    /// Returns `self.value`, consuming `self` and dropping `self.raw`.
    #[inline]
    pub fn ok(self) -> Option<T> {
        self.value
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(self.raw)`.
    #[inline]
    pub fn result(self) -> Result<T, vk::Result> {
        self.value.ok_or(self.raw)
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(op(self.raw))`.
    #[inline]
    pub fn map_err<F, O: FnOnce(vk::Result) -> F>(self, op: O) -> Result<T, F> {
        let raw = self.raw;
        self.value.ok_or_else(move || op(raw))
    }

    /// Returns `true` if `self.value` is `Some`.
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
    }

    /// Returns `true` if `self.raw` is positive.
    #[inline]
    pub fn raw_is_ok(&self) -> bool {
        self.raw.0.is_positive()
    }

    /// Returns `true` if `self.value` is `None`.
    #[inline]
    pub fn is_err(&self) -> bool {
        self.value.is_none()
    }

    /// Returns `true` if `self.raw` is negative.
    #[inline]
    pub fn raw_is_err(&self) -> bool {
        self.raw.0.is_negative()
    }
}

impl Display for vk::Result {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

impl Error for vk::Result {}

impl<T> Debug for VulkanResult<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => {
                write!(fmt, "{:?}: ", self.raw)?;
                Debug::fmt(value, fmt)
            }
            None => write!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}

impl<T> Display for VulkanResult<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => {
                write!(fmt, "{:?}: ", self.raw)?;
                Display::fmt(value, fmt)
            }
            None => write!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}
