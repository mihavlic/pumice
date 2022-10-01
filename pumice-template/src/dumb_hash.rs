use std::{
    borrow::Borrow,
    ffi::CStr,
    hash::{self, Hasher},
    os::raw::{c_char, c_void},
};

pub trait DumbHash {
    fn hash<H: Hasher>(&self, state: &mut H);
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for element in data {
            DumbHash::hash(element, state);
        }
    }
    unsafe fn wrap(self) -> DumbWrap<Self>
    where
        Self: Sized,
    {
        DumbWrap(self)
    }
    unsafe fn wrap_ref(&self) -> &DumbWrap<Self> {
        // ok because we have repr(transparent)
        // https://stackoverflow.com/questions/61441303/is-it-safe-to-cast-references-to-unsized-transparent-types
        std::mem::transmute(self)
    }
}

#[repr(transparent)]
pub struct DumbWrap<T: DumbHash + ?Sized>(T);

impl<T: DumbHash> DumbWrap<T> {
    pub unsafe fn new(what: T) -> Self {
        Self(what)
    }
}

impl<T: DumbHash + ?Sized> DumbWrap<T> {
    pub unsafe fn from_ref(what: &T) -> &Self {
        // ok because we have repr(transparent)
        std::mem::transmute(what)
    }
}

impl<T: DumbHash> hash::Hash for DumbWrap<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash(&self.0, state);
    }
}

unsafe fn hash_ptr<T: DumbHash, H: Hasher>(what: *const T, state: &mut H) {
    if let Some(what) = what.as_ref() {
        what.hash(state);
    }
}

unsafe fn hash_raw_arr<T: DumbHash, H: Hasher>(what: *const T, len: usize, state: &mut H) {
    if !what.is_null() {
        let slice = std::slice::from_raw_parts(what, len);
        DumbHash::hash_slice(slice, state);
    }
}

unsafe fn hash_cstr<H: Hasher>(ptr: *const c_char, state: &mut H) {
    if !ptr.is_null() {
        hash::Hash::hash(&CStr::from_ptr(ptr), state);
    }
}

#[track_caller]
unsafe fn hash_pnext<H: Hasher>(mut pnext: *const c_void, state: &mut H) {
    while !pnext.is_null() {
        #[rustfmt::skip]
        crate::pnext_visit!(
            pnext, stype, object,
            {
                // the usefulness of hashing the stype is dubious, however following the same logic as `Hasher::write_length_prefix` it eliminates a class of hash collisions
                hash::Hash::hash(&stype, state);
                (&&*object).get_or_die().hash(state);
            }
        );
    }
}

// hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

struct NeverInstantiated;

impl DumbHash for NeverInstantiated {
    fn hash<H: Hasher>(&self, _: &mut H) {
        unreachable!()
    }
}

trait ViaPanic {
    fn get_or_die(&self) -> &NeverInstantiated {
        panic!("{:?} does not implement DumbHash, this is likely because it contains a void pointer without any size information which cannot be manipulated and thus no implementation could be generated.", std::any::type_name::<Self>());
    }
}
impl<T> ViaPanic for &&T {}

trait ViaActual<T>: Borrow<T> {
    fn get_or_die(&self) -> &T {
        self.borrow()
    }
}
impl<T: DumbHash> ViaActual<T> for &T {}

impl DumbHash for f32 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.to_bits(), state);
    }
}

impl DumbHash for f64 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.to_bits(), state);
    }
}

impl<T: DumbHash, const S: usize> DumbHash for [T; S] {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash_slice(self, state);
    }
}

impl<T: DumbHash> DumbHash for [T] {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DumbHash::hash_slice(self, state);
    }
}

impl<T> DumbHash for *const T {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state)
    }
}

impl<T> DumbHash for *mut T {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state)
    }
}

macro_rules! dumb_hash_passthrough_impl {
    ($($name:path),+) => {
        $(
            impl DumbHash for $name {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    hash::Hash::hash(self, state);
                }
            }
        )+
    };
}
