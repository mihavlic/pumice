#![allow(unused)]

use std::{
    alloc::Layout,
    borrow::Borrow,
    ffi::CStr,
    ops::{Deref, DerefMut},
    os::raw::{c_char, c_void},
    ptr::{self, NonNull},
};

pub struct DeepCopyBox<T> {
    copy: NonNull<T>,
    measure: CopyMeasure,
}

impl<T> Deref for DeepCopyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.copy.as_ref() }
    }
}

impl<T> DerefMut for DeepCopyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.copy.as_mut() }
    }
}

impl<T: DeepCopy> Clone for DeepCopyBox<T> {
    fn clone(&self) -> Self {
        unsafe {
            let mut writer = CopyWriter::new(&self.measure);
            writer.write_ptr(self.copy.as_ptr());

            Self {
                copy: NonNull::new(writer.finish().cast()).unwrap(),
                measure: self.measure.clone(),
            }
        }
    }
}

impl<T> Drop for DeepCopyBox<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.copy.as_ptr().cast(), self.measure.layout());
        }
    }
}

pub unsafe trait DeepCopy: Sized {
    fn measure(&self, measure: &mut CopyMeasure);
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter);
    unsafe fn deep_copy(&self) -> DeepCopyBox<Self> {
        let mut measure = CopyMeasure::new();
        measure.alloc::<Self>();
        self.measure(&mut measure);

        let mut writer = CopyWriter::new(&measure);
        writer.write_ptr(self);

        DeepCopyBox {
            copy: NonNull::new(writer.finish().cast()).unwrap(),
            measure,
        }
    }
}

#[derive(Clone)]
pub struct CopyMeasure {
    layout: Layout,
}

impl CopyMeasure {
    fn new() -> Self {
        Self {
            layout: Layout::new::<()>(),
        }
    }
    pub fn layout(&self) -> Layout {
        self.layout
    }
    pub unsafe fn measure_ptr<T: DeepCopy>(&mut self, what: *const T) {
        if let Some(what) = what.as_ref() {
            self.alloc::<T>();
            what.measure(self);
        }
    }
    pub unsafe fn measure_arr_ptr<T: DeepCopy>(&mut self, ptr: *const T, len: usize) {
        if !ptr.is_null() && len > 0 {
            self.alloc_arr::<T>(len);

            let slice = std::slice::from_raw_parts(ptr, len);
            for element in slice {
                element.measure(self);
            }
        }
    }
    pub unsafe fn measure_cstr(&mut self, ptr: *const c_char) {
        if !ptr.is_null() {
            let len = CStr::from_ptr(ptr).to_bytes_with_nul().len();
            self.alloc_arr::<c_char>(len);
        }
    }
    #[track_caller]
    pub unsafe fn measure_pnext(&mut self, mut p_next: *const c_void) {
        while !p_next.is_null() {
            #[rustfmt::skip]
            crate::pnext_visit!(
                p_next, stype, object,
                // hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                // I am very sorry
                self.measure_ptr((&&*object).get_or_die())
            );
        }
    }
    pub unsafe fn alloc<T>(&mut self) {
        layout_extend::<T>(&mut self.layout);
    }
    pub unsafe fn alloc_arr<T>(&mut self, len: usize) {
        layout_extend_arr::<T>(&mut self.layout, len);
    }
}

pub struct CopyWriter {
    buf: *mut u8,
    layout: Layout,
}

impl CopyWriter {
    pub unsafe fn new(measure: &CopyMeasure) -> Self {
        Self {
            buf: std::alloc::alloc(measure.layout),
            layout: Layout::from_size_align(0, measure.layout.align()).unwrap(),
        }
    }
    pub unsafe fn write_ptr<T: DeepCopy>(&mut self, what: *const T) -> *mut T {
        if let Some(what_ref) = what.as_ref() {
            let ptr = self.alloc::<T>();

            ptr::copy_nonoverlapping(what, ptr, 1);
            what_ref.copy(ptr, self);
            ptr
        } else {
            ptr::null_mut()
        }
    }
    pub unsafe fn write_arr_ptr<T: DeepCopy>(&mut self, ptr: *const T, len: usize) -> *mut T {
        if !ptr.is_null() && len > 0 {
            let slice = std::slice::from_raw_parts(ptr, len);
            let ptr = self.alloc_arr::<T>(len);

            ptr::copy_nonoverlapping(slice.as_ptr(), ptr, len);
            for (i, element) in slice.iter().enumerate() {
                element.copy(ptr.add(i), self);
            }

            ptr
        } else {
            ptr::null_mut()
        }
    }
    pub unsafe fn write_cstr(&mut self, what: *const c_char) -> *const c_char {
        if !what.is_null() {
            let len = CStr::from_ptr(what).to_bytes_with_nul().len();
            let ptr = self.alloc_arr::<c_char>(len);

            ptr::copy_nonoverlapping(what, ptr, len);
            ptr
        } else {
            ptr::null()
        }
    }
    #[track_caller]
    pub unsafe fn write_pnext(&mut self, mut p_next: *const c_void) -> *mut c_void {
        let mut first = None;
        let mut prev: *mut *const c_void = ptr::null_mut();
        while !p_next.is_null() {
            #[rustfmt::skip]
            crate::pnext_visit!(
                p_next, stype, object,
                // hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
                // I am very sorry
                {
                    let ptr = self.write_ptr((&&*object).get_or_die());
                    if first.is_none() {
                        first = Some(ptr.cast::<c_void>());
                    } else {
                        *prev = ptr as *const _;
                    }
                    prev = std::mem::transmute(std::ptr::addr_of!((*ptr).p_next));
                }
            );
        }
        first.unwrap_or(ptr::null_mut())
    }
    pub unsafe fn alloc<T>(&mut self) -> *mut T {
        let offset = layout_extend::<T>(&mut self.layout);
        let ptr = self.buf.add(offset).cast();
        ptr
    }
    pub unsafe fn alloc_arr<T>(&mut self, len: usize) -> *mut T {
        let offset = layout_extend_arr::<T>(&mut self.layout, len);
        let ptr = self.buf.add(offset).cast();
        ptr
    }
    pub unsafe fn finish(self) -> *mut u8 {
        self.buf
    }
}

// hack for specialization: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

struct NeverInstantiated {
    p_next: *const c_void,
}

unsafe impl DeepCopy for NeverInstantiated {
    fn measure(&self, _: &mut CopyMeasure) {
        unreachable!()
    }
    unsafe fn copy(&self, _: *mut Self, _: &mut CopyWriter) {
        unreachable!()
    }
}

trait ViaPanic {
    fn get_or_die(&self) -> &NeverInstantiated {
        panic!("{:?} does not implement DeepCopy, this is likely because it contains a void pointer without any size information which cannot be manipulated and thus no implementation could be generated.", std::any::type_name::<Self>());
    }
}
impl<T> ViaPanic for &&T {}

trait ViaActual<T>: Borrow<T> {
    fn get_or_die(&self) -> &T {
        self.borrow()
    }
}
impl<T: DeepCopy> ViaActual<T> for &T {}

#[inline]
#[track_caller]
fn layout_extend<T>(layout: &mut Layout) -> usize {
    let (new_layout, offset) = layout.extend(Layout::new::<T>()).unwrap();
    *layout = new_layout;
    offset
}

#[inline]
#[track_caller]
fn layout_extend_arr<T>(layout: &mut Layout, len: usize) -> usize {
    let (new_layout, offset) = layout.extend(Layout::array::<T>(len).unwrap()).unwrap();
    *layout = new_layout;
    offset
}

/// All memory directly contained in a struct is always already accounted for by the encasing
/// type's write() call, further calls are required only when indirectly referenced memory needs to be copied
macro_rules! value_type_copy_impl {
    ($($name:path),+) => {
        $(
            unsafe impl DeepCopy for $name {
                fn measure(&self, _measure: &mut CopyMeasure) {
                    {}
                }
                unsafe fn copy(&self, _copy: *mut Self, _writer: &mut CopyWriter) {
                    {}
                }
            }
        )+
    };
}

#[test]
fn test() {
    use crate::vk::{
        DeviceCreateInfo, GraphicsPipelineCreateInfo, PipelineInputAssemblyStateCreateInfo,
        PrimitiveTopology,
    };

    let a = PipelineInputAssemblyStateCreateInfo {
        topology: PrimitiveTopology::TRIANGLE_FAN,
        ..Default::default()
    };
    let a = GraphicsPipelineCreateInfo {
        p_next: &a as *const _ as *const _,
        ..Default::default()
    };
    unsafe {
        assert_eq!(
            (*a.p_next.cast::<PipelineInputAssemblyStateCreateInfo>()).topology,
            PrimitiveTopology::TRIANGLE_FAN
        );
        let copy = a.deep_copy();
        assert_eq!(
            (*copy.p_next.cast::<PipelineInputAssemblyStateCreateInfo>()).topology,
            PrimitiveTopology::TRIANGLE_FAN
        );

        let names = [
            crate::cstr!("Aaaa").as_ptr(),
            crate::cstr!("Baaa").as_ptr(),
            crate::cstr!("Aaaa").as_ptr(),
            crate::cstr!("Aaaa").as_ptr(),
        ];

        let a = DeviceCreateInfo {
            enabled_layer_count: 4,
            pp_enabled_layer_names: names.as_ptr(),
            ..Default::default()
        };

        let copy = a.deep_copy();

        assert_ne!(copy.pp_enabled_layer_names, a.pp_enabled_layer_names);

        assert_eq!(
            CStr::from_ptr(*copy.pp_enabled_layer_names.add(1)),
            crate::cstr!("Baaa"),
        )
    }
}

// CODEGEN START
