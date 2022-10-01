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

        impl $crate::util::impl_macros::ObjectHandle for $name {
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

        impl $crate::util::impl_macros::ObjectHandle for $name {
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
