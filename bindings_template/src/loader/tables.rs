use crate::{
    cstr,
    loader::{DeviceLoad, EntryLoad, InstanceLoad},
    util::{char, double, float, int, void, ApiLoadConfig},
};
use std::mem::MaybeUninit;

/// Oh, yes. Little Bobby Tables we call him.

macro_rules! load_fns {
    ($table:ident, $loader:ident, $(($name:ident, $str:literal))+) => {
        $($table.$name = Some(::std::mem::transmute($loader.load(concat!($str, "\0").as_ptr().cast::<std::os::raw::c_char>())));)+
    };
}

/// https://github.com/maxbla/const-zero#how-does-it-work
union ConstZeroedHack<T, const S: usize> {
    bytes: [u8; S],
    inner: std::mem::ManuallyDrop<T>,
}

impl<T, const S: usize> ConstZeroedHack<T, S> {
    const unsafe fn zero() -> T {
        std::mem::ManuallyDrop::into_inner(Self { bytes: [0; S] }.inner)
    }
}
