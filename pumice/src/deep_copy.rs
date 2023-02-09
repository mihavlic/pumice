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
unsafe impl DeepCopy for crate::vk10::BaseOutStructure {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_ptr(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk10::BaseInStructure {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_ptr(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk10::ApplicationInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_application_name);
            measure.measure_cstr(self.p_engine_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_application_name = writer.write_cstr(self.p_application_name);
        (*copy).p_engine_name = writer.write_cstr(self.p_engine_name);
    }
}
unsafe impl DeepCopy for crate::vk10::AllocationCallbacks {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {}
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {}
}
unsafe impl DeepCopy for crate::vk10::DeviceQueueCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_queue_priorities, (self.queue_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_priorities = writer
            .write_arr_ptr(self.p_queue_priorities, (self.queue_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DeviceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_create_infos,
                    (self.queue_create_info_count) as usize,
                );
            let len = (self.enabled_layer_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                measure.measure_cstr(ptr);
            }
            let len = (self.enabled_extension_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                measure.measure_cstr(ptr);
            }
            measure.measure_ptr(self.p_enabled_features);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_create_infos = writer
            .write_arr_ptr(
                self.p_queue_create_infos,
                (self.queue_create_info_count) as usize,
            );
        let len = (self.enabled_layer_count) as usize;
        (*copy)
            .pp_enabled_layer_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_layer_names.add(i);
            let copy = (*copy).pp_enabled_layer_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        let len = (self.enabled_extension_count) as usize;
        (*copy)
            .pp_enabled_extension_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_extension_names.add(i);
            let copy = (*copy).pp_enabled_extension_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        (*copy).p_enabled_features = writer.write_ptr(self.p_enabled_features);
    }
}
unsafe impl DeepCopy for crate::vk10::InstanceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_application_info);
            let len = (self.enabled_layer_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                measure.measure_cstr(ptr);
            }
            let len = (self.enabled_extension_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_char>(len);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                measure.measure_cstr(ptr);
            }
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_application_info = writer.write_ptr(self.p_application_info);
        let len = (self.enabled_layer_count) as usize;
        (*copy)
            .pp_enabled_layer_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_layer_names.add(i);
            let copy = (*copy).pp_enabled_layer_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
        let len = (self.enabled_extension_count) as usize;
        (*copy)
            .pp_enabled_extension_names = writer
            .alloc_arr::<*const std::os::raw::c_char>(len);
        for i in 0..len {
            let ptr = *self.pp_enabled_extension_names.add(i);
            let copy = (*copy).pp_enabled_extension_names.add(i).cast_mut();
            *copy = writer.write_cstr(ptr);
        }
    }
}
unsafe impl DeepCopy for crate::vk10::MemoryAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::MappedMemoryRange {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::WriteDescriptorSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_image_info, (self.descriptor_count) as usize);
            measure
                .measure_arr_ptr(self.p_buffer_info, (self.descriptor_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_texel_buffer_view,
                    (self.descriptor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_image_info = writer
            .write_arr_ptr(self.p_image_info, (self.descriptor_count) as usize);
        (*copy)
            .p_buffer_info = writer
            .write_arr_ptr(self.p_buffer_info, (self.descriptor_count) as usize);
        (*copy)
            .p_texel_buffer_view = writer
            .write_arr_ptr(self.p_texel_buffer_view, (self.descriptor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::CopyDescriptorSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::BufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::BufferViewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::MemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::BufferMemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::ImageMemoryBarrier {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::ImageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::ImageViewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::SparseBufferMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SparseImageOpaqueMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SparseImageMemoryBindInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_binds, (self.bind_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_binds = writer.write_arr_ptr(self.p_binds, (self.bind_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::BindSparseInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(self.p_buffer_binds, (self.buffer_bind_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_image_opaque_binds,
                    (self.image_opaque_bind_count) as usize,
                );
            measure
                .measure_arr_ptr(self.p_image_binds, (self.image_bind_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphores,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_buffer_binds = writer
            .write_arr_ptr(self.p_buffer_binds, (self.buffer_bind_count) as usize);
        (*copy)
            .p_image_opaque_binds = writer
            .write_arr_ptr(
                self.p_image_opaque_binds,
                (self.image_opaque_bind_count) as usize,
            );
        (*copy)
            .p_image_binds = writer
            .write_arr_ptr(self.p_image_binds, (self.image_bind_count) as usize);
        (*copy)
            .p_signal_semaphores = writer
            .write_arr_ptr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::ShaderModuleCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_code, (self.code_size / 4) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_code = writer.write_arr_ptr(self.p_code, (self.code_size / 4) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetLayoutBinding {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_immutable_samplers,
                    (self.descriptor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_immutable_samplers = writer
            .write_arr_ptr(self.p_immutable_samplers, (self.descriptor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetLayoutCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_bindings, (self.binding_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_bindings = writer
            .write_arr_ptr(self.p_bindings, (self.binding_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_pool_sizes, (self.pool_size_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_pool_sizes = writer
            .write_arr_ptr(self.p_pool_sizes, (self.pool_size_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::DescriptorSetAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_set_layouts,
                    (self.descriptor_set_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_set_layouts = writer
            .write_arr_ptr(self.p_set_layouts, (self.descriptor_set_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SpecializationInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_map_entries, (self.map_entry_count) as usize);
            measure.measure_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_map_entries = writer
            .write_arr_ptr(self.p_map_entries, (self.map_entry_count) as usize);
        (*copy)
            .p_data = writer
            .write_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineShaderStageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_name);
            measure.measure_ptr(self.p_specialization_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_name = writer.write_cstr(self.p_name);
        (*copy).p_specialization_info = writer.write_ptr(self.p_specialization_info);
    }
}
unsafe impl DeepCopy for crate::vk10::ComputePipelineCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.stage);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.stage.copy(&mut (*copy).stage, writer);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineVertexInputStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_vertex_binding_descriptions,
                    (self.vertex_binding_description_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_vertex_attribute_descriptions,
                    (self.vertex_attribute_description_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_vertex_binding_descriptions = writer
            .write_arr_ptr(
                self.p_vertex_binding_descriptions,
                (self.vertex_binding_description_count) as usize,
            );
        (*copy)
            .p_vertex_attribute_descriptions = writer
            .write_arr_ptr(
                self.p_vertex_attribute_descriptions,
                (self.vertex_attribute_description_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineInputAssemblyStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineTessellationStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineViewportStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_viewports, (self.viewport_count) as usize);
            measure.measure_arr_ptr(self.p_scissors, (self.scissor_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_viewports = writer
            .write_arr_ptr(self.p_viewports, (self.viewport_count) as usize);
        (*copy)
            .p_scissors = writer
            .write_arr_ptr(self.p_scissors, (self.scissor_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineRasterizationStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineMultisampleStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_sample_mask,
                    ((self.rasterization_samples.0 + 31) / 32) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_sample_mask = writer
            .write_arr_ptr(
                self.p_sample_mask,
                ((self.rasterization_samples.0 + 31) / 32) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineColorBlendStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineDynamicStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_dynamic_states,
                    (self.dynamic_state_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_dynamic_states = writer
            .write_arr_ptr(self.p_dynamic_states, (self.dynamic_state_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineDepthStencilStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::GraphicsPipelineCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_stages, (self.stage_count) as usize);
            measure.measure_ptr(self.p_vertex_input_state);
            measure.measure_ptr(self.p_input_assembly_state);
            measure.measure_ptr(self.p_tessellation_state);
            measure.measure_ptr(self.p_viewport_state);
            measure.measure_ptr(self.p_rasterization_state);
            measure.measure_ptr(self.p_multisample_state);
            measure.measure_ptr(self.p_depth_stencil_state);
            measure.measure_ptr(self.p_color_blend_state);
            measure.measure_ptr(self.p_dynamic_state);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_stages = writer.write_arr_ptr(self.p_stages, (self.stage_count) as usize);
        (*copy).p_vertex_input_state = writer.write_ptr(self.p_vertex_input_state);
        (*copy).p_input_assembly_state = writer.write_ptr(self.p_input_assembly_state);
        (*copy).p_tessellation_state = writer.write_ptr(self.p_tessellation_state);
        (*copy).p_viewport_state = writer.write_ptr(self.p_viewport_state);
        (*copy).p_rasterization_state = writer.write_ptr(self.p_rasterization_state);
        (*copy).p_multisample_state = writer.write_ptr(self.p_multisample_state);
        (*copy).p_depth_stencil_state = writer.write_ptr(self.p_depth_stencil_state);
        (*copy).p_color_blend_state = writer.write_ptr(self.p_color_blend_state);
        (*copy).p_dynamic_state = writer.write_ptr(self.p_dynamic_state);
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineCacheCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_initial_data.cast::<u8>(),
                    (self.initial_data_size) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_initial_data = writer
            .write_arr_ptr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
            )
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::vk10::PipelineLayoutCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_set_layouts, (self.set_layout_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_push_constant_ranges,
                    (self.push_constant_range_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_set_layouts = writer
            .write_arr_ptr(self.p_set_layouts, (self.set_layout_count) as usize);
        (*copy)
            .p_push_constant_ranges = writer
            .write_arr_ptr(
                self.p_push_constant_ranges,
                (self.push_constant_range_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::SamplerCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferInheritanceInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::CommandBufferBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_inheritance_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_inheritance_info = writer.write_ptr(self.p_inheritance_info);
    }
}
unsafe impl DeepCopy for crate::vk10::RenderPassBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_clear_values, (self.clear_value_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_clear_values = writer
            .write_arr_ptr(self.p_clear_values, (self.clear_value_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SubpassDescription {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_input_attachments,
                    (self.input_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_resolve_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_stencil_attachment);
            measure
                .measure_arr_ptr(
                    self.p_preserve_attachments,
                    (self.preserve_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_input_attachments = writer
            .write_arr_ptr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
            );
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_resolve_attachments = writer
            .write_arr_ptr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_depth_stencil_attachment = writer
            .write_ptr(self.p_depth_stencil_attachment);
        (*copy)
            .p_preserve_attachments = writer
            .write_arr_ptr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk10::RenderPassCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
            measure.measure_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        (*copy)
            .p_subpasses = writer
            .write_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
        (*copy)
            .p_dependencies = writer
            .write_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::EventCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::FenceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::SemaphoreCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::QueryPoolCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk10::FramebufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk10::SubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_wait_dst_stage_mask,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffers,
                    (self.command_buffer_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphores,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_wait_dst_stage_mask = writer
            .write_arr_ptr(
                self.p_wait_dst_stage_mask,
                (self.wait_semaphore_count) as usize,
            );
        (*copy)
            .p_command_buffers = writer
            .write_arr_ptr(self.p_command_buffers, (self.command_buffer_count) as usize);
        (*copy)
            .p_signal_semaphores = writer
            .write_arr_ptr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::khr_display::DisplayPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_cstr(self.display_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).display_name = writer.write_cstr(self.display_name);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_display::DisplayModeCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_display_swapchain::DisplayPresentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::PresentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphores,
                    (self.wait_semaphore_count) as usize,
                );
            measure.measure_arr_ptr(self.p_swapchains, (self.swapchain_count) as usize);
            measure
                .measure_arr_ptr(self.p_image_indices, (self.swapchain_count) as usize);
            measure.measure_arr_ptr(self.p_results, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphores = writer
            .write_arr_ptr(self.p_wait_semaphores, (self.wait_semaphore_count) as usize);
        (*copy)
            .p_swapchains = writer
            .write_arr_ptr(self.p_swapchains, (self.swapchain_count) as usize);
        (*copy)
            .p_image_indices = writer
            .write_arr_ptr(self.p_image_indices, (self.swapchain_count) as usize);
        (*copy)
            .p_results = writer
            .write_arr_ptr(self.p_results, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_validation_flags::ValidationFlagsEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_disabled_validation_checks,
                    (self.disabled_validation_check_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_disabled_validation_checks = writer
            .write_arr_ptr(
                self.p_disabled_validation_checks,
                (self.disabled_validation_check_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_validation_features::ValidationFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_enabled_validation_features,
                    (self.enabled_validation_feature_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_disabled_validation_features,
                    (self.disabled_validation_feature_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_enabled_validation_features = writer
            .write_arr_ptr(
                self.p_enabled_validation_features,
                (self.enabled_validation_feature_count) as usize,
            );
        (*copy)
            .p_disabled_validation_features = writer
            .write_arr_ptr(
                self.p_disabled_validation_features,
                (self.disabled_validation_feature_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_rasterization_order::PipelineRasterizationStateRasterizationOrderAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_object_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_object_name = writer.write_cstr(self.p_object_name);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_tag = writer
            .write_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_marker_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_marker_name = writer.write_cstr(self.p_marker_name);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationImageCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationBufferCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationMemoryAllocateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory::ExternalMemoryImageCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory::ExportMemoryAllocateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory_win32::ImportMemoryWin32HandleInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory_win32::ExportMemoryWin32HandleInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_acquire_syncs, (self.acquire_count) as usize);
            measure.measure_arr_ptr(self.p_acquire_keys, (self.acquire_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_acquire_timeout_milliseconds,
                    (self.acquire_count) as usize,
                );
            measure.measure_arr_ptr(self.p_release_syncs, (self.release_count) as usize);
            measure.measure_arr_ptr(self.p_release_keys, (self.release_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_acquire_syncs = writer
            .write_arr_ptr(self.p_acquire_syncs, (self.acquire_count) as usize);
        (*copy)
            .p_acquire_keys = writer
            .write_arr_ptr(self.p_acquire_keys, (self.acquire_count) as usize);
        (*copy)
            .p_acquire_timeout_milliseconds = writer
            .write_arr_ptr(
                self.p_acquire_timeout_milliseconds,
                (self.acquire_count) as usize,
            );
        (*copy)
            .p_release_syncs = writer
            .write_arr_ptr(self.p_release_syncs, (self.release_count) as usize);
        (*copy)
            .p_release_keys = writer
            .write_arr_ptr(self.p_release_keys, (self.release_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::DevicePrivateDataCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::PrivateDataSlotCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDevicePrivateDataFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_stages, (self.stage_count) as usize);
            measure.measure_ptr(self.p_vertex_input_state);
            measure.measure_ptr(self.p_tessellation_state);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_stages = writer.write_arr_ptr(self.p_stages, (self.stage_count) as usize);
        (*copy).p_vertex_input_state = writer.write_ptr(self.p_vertex_input_state);
        (*copy).p_tessellation_state = writer.write_ptr(self.p_tessellation_state);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::GraphicsPipelineShaderGroupsCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_groups, (self.group_count) as usize);
            measure.measure_arr_ptr(self.p_pipelines, (self.pipeline_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_groups = writer.write_arr_ptr(self.p_groups, (self.group_count) as usize);
        (*copy)
            .p_pipelines = writer
            .write_arr_ptr(self.p_pipelines, (self.pipeline_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_index_types, (self.index_type_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_index_type_values,
                    (self.index_type_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_index_types = writer
            .write_arr_ptr(self.p_index_types, (self.index_type_count) as usize);
        (*copy)
            .p_index_type_values = writer
            .write_arr_ptr(self.p_index_type_values, (self.index_type_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_tokens, (self.token_count) as usize);
            measure.measure_arr_ptr(self.p_stream_strides, (self.stream_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_tokens = writer.write_arr_ptr(self.p_tokens, (self.token_count) as usize);
        (*copy)
            .p_stream_strides = writer
            .write_arr_ptr(self.p_stream_strides, (self.stream_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_streams, (self.stream_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_streams = writer
            .write_arr_ptr(self.p_streams, (self.stream_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceFeatures2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::FormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ImageFormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceImageFormatInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::QueueFamilyProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMemoryProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SparseImageFormatProperties2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSparseImageFormatInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceDriverProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_incremental_present::PresentRegionsKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_incremental_present::PresentRegionKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_arr_ptr(self.p_rectangles, (self.rectangle_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_rectangles = writer
            .write_arr_ptr(self.p_rectangles, (self.rectangle_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceVariablePointersFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalImageFormatInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalImageFormatProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalBufferInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalBufferProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceIDProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalMemoryImageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalMemoryBufferCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExportMemoryAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_win32::ImportMemoryWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_win32::ExportMemoryWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_external_memory::ImportMemoryZirconHandleInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_win32::MemoryGetWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_fd::ImportMemoryFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_acquire_syncs, (self.acquire_count) as usize);
            measure.measure_arr_ptr(self.p_acquire_keys, (self.acquire_count) as usize);
            measure
                .measure_arr_ptr(self.p_acquire_timeouts, (self.acquire_count) as usize);
            measure.measure_arr_ptr(self.p_release_syncs, (self.release_count) as usize);
            measure.measure_arr_ptr(self.p_release_keys, (self.release_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_acquire_syncs = writer
            .write_arr_ptr(self.p_acquire_syncs, (self.acquire_count) as usize);
        (*copy)
            .p_acquire_keys = writer
            .write_arr_ptr(self.p_acquire_keys, (self.acquire_count) as usize);
        (*copy)
            .p_acquire_timeouts = writer
            .write_arr_ptr(self.p_acquire_timeouts, (self.acquire_count) as usize);
        (*copy)
            .p_release_syncs = writer
            .write_arr_ptr(self.p_release_syncs, (self.release_count) as usize);
        (*copy)
            .p_release_keys = writer
            .write_arr_ptr(self.p_release_keys, (self.release_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalSemaphoreInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalSemaphoreProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExportSemaphoreCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_win32::ExportSemaphoreWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_win32::D3D12FenceSubmitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_values,
                    (self.wait_semaphore_values_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_values,
                    (self.signal_semaphore_values_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_values = writer
            .write_arr_ptr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_values_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_values = writer
            .write_arr_ptr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_values_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceExternalFenceInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ExternalFenceProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::ExportFenceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_fence_win32::ImportFenceWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_fence_win32::ExportFenceWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).name = writer.write_ptr(self.name);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMultiviewFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMultiviewProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::RenderPassMultiviewCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_view_masks, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_view_offsets, (self.dependency_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_correlation_masks,
                    (self.correlation_mask_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_masks = writer
            .write_arr_ptr(self.p_view_masks, (self.subpass_count) as usize);
        (*copy)
            .p_view_offsets = writer
            .write_arr_ptr(self.p_view_offsets, (self.dependency_count) as usize);
        (*copy)
            .p_correlation_masks = writer
            .write_arr_ptr(
                self.p_correlation_masks,
                (self.correlation_mask_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_display_control::DisplayPowerInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_display_control::DeviceEventInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_display_control::DisplayEventInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceGroupProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryAllocateFlagsInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindBufferMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindBufferMemoryDeviceGroupInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_indices,
                    (self.device_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_indices = writer
            .write_arr_ptr(self.p_device_indices, (self.device_index_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::BindImageMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindImageMemoryDeviceGroupInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_indices,
                    (self.device_index_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_split_instance_bind_regions,
                    (self.split_instance_bind_region_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_indices = writer
            .write_arr_ptr(self.p_device_indices, (self.device_index_count) as usize);
        (*copy)
            .p_split_instance_bind_regions = writer
            .write_arr_ptr(
                self.p_split_instance_bind_regions,
                (self.split_instance_bind_region_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupRenderPassBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_render_areas,
                    (self.device_render_area_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_render_areas = writer
            .write_arr_ptr(
                self.p_device_render_areas,
                (self.device_render_area_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupCommandBufferBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_device_indices,
                    (self.wait_semaphore_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffer_device_masks,
                    (self.command_buffer_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_device_indices,
                    (self.signal_semaphore_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_device_indices = writer
            .write_arr_ptr(
                self.p_wait_semaphore_device_indices,
                (self.wait_semaphore_count) as usize,
            );
        (*copy)
            .p_command_buffer_device_masks = writer
            .write_arr_ptr(
                self.p_command_buffer_device_masks,
                (self.command_buffer_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_device_indices = writer
            .write_arr_ptr(
                self.p_signal_semaphore_device_indices,
                (self.signal_semaphore_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupBindSparseInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::AcquireNextImageInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_device_masks, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_masks = writer
            .write_arr_ptr(self.p_device_masks, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceGroupDeviceCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_physical_devices,
                    (self.physical_device_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_physical_devices = writer
            .write_arr_ptr(
                self.p_physical_devices,
                (self.physical_device_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::DescriptorUpdateTemplateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_descriptor_update_entries,
                    (self.descriptor_update_entry_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_descriptor_update_entries = writer
            .write_arr_ptr(
                self.p_descriptor_update_entries,
                (self.descriptor_update_entry_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_present_id::PresentIdKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_present_ids, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_present_ids = writer
            .write_arr_ptr(self.p_present_ids, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_hdr_metadata::HdrMetadataEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::google_display_timing::PresentTimesInfoGOOGLE {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_times, (self.swapchain_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_times = writer
            .write_arr_ptr(self.p_times, (self.swapchain_count) as usize);
    }
}
unsafe impl DeepCopy for crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_clip_space_w_scaling::PipelineViewportWScalingStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_viewport_wscalings,
                    (self.viewport_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_viewport_wscalings = writer
            .write_arr_ptr(self.p_viewport_wscalings, (self.viewport_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_viewport_swizzles,
                    (self.viewport_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_viewport_swizzles = writer
            .write_arr_ptr(self.p_viewport_swizzles, (self.viewport_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_discard_rectangles,
                    (self.discard_rectangle_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_discard_rectangles = writer
            .write_arr_ptr(
                self.p_discard_rectangles,
                (self.discard_rectangle_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::RenderPassInputAttachmentAspectCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_aspect_references,
                    (self.aspect_reference_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_aspect_references = writer
            .write_arr_ptr(
                self.p_aspect_references,
                (self.aspect_reference_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_display_properties2::DisplayProperties2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.display_properties);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        self.display_properties.copy(&mut (*copy).display_properties, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDevice16BitStorageFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSubgroupProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::BufferMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::DeviceBufferMemoryRequirements {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_create_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_create_info = writer.write_ptr(self.p_create_info);
    }
}
unsafe impl DeepCopy for crate::vk11::ImageMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImageSparseMemoryRequirementsInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::DeviceImageMemoryRequirements {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_create_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_create_info = writer.write_ptr(self.p_create_info);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryRequirements2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SparseImageMemoryRequirements2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDevicePointClippingProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryDedicatedRequirements {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::MemoryDedicatedAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImageViewUsageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PipelineTessellationDomainOriginStateCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::BindImagePlaneMemoryInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ImagePlaneMemoryRequirementsInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::SamplerYcbcrConversionImageFormatProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_texture_gather_bias_lod::TextureLODGatherFormatPropertiesAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::ProtectedSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProtectedMemoryFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceProtectedMemoryProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::DeviceQueueInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceSamplerFilterMinmaxProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::SampleLocationsInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_sample_locations,
                    (self.sample_locations_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_sample_locations = writer
            .write_arr_ptr(
                self.p_sample_locations,
                (self.sample_locations_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::AttachmentSampleLocationsEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(&self.sample_locations_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        self.sample_locations_info.copy(&mut (*copy).sample_locations_info, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::SubpassSampleLocationsEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(&self.sample_locations_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        self.sample_locations_info.copy(&mut (*copy).sample_locations_info, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::RenderPassSampleLocationsBeginInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_attachment_initial_sample_locations,
                    (self.attachment_initial_sample_locations_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_post_subpass_sample_locations,
                    (self.post_subpass_sample_locations_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachment_initial_sample_locations = writer
            .write_arr_ptr(
                self.p_attachment_initial_sample_locations,
                (self.attachment_initial_sample_locations_count) as usize,
            );
        (*copy)
            .p_post_subpass_sample_locations = writer
            .write_arr_ptr(
                self.p_post_subpass_sample_locations,
                (self.post_subpass_sample_locations_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::PipelineSampleLocationsStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.sample_locations_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.sample_locations_info.copy(&mut (*copy).sample_locations_info, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_sample_locations::MultisamplePropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::SamplerReductionModeCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_blend_operation_advanced::PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceInlineUniformBlockFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceInlineUniformBlockProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::WriteDescriptorSetInlineUniformBlock {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_data = writer
            .write_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::vk13::DescriptorPoolInlineUniformBlockCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_coverage_modulation_table,
                    (self.coverage_modulation_table_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_coverage_modulation_table = writer
            .write_arr_ptr(
                self.p_coverage_modulation_table,
                (self.coverage_modulation_table_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::ImageFormatListCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_formats = writer
            .write_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_initial_data.cast::<u8>(),
                    (self.initial_data_size) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_initial_data = writer
            .write_arr_ptr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
            )
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_validation_cache::ShaderModuleValidationCacheCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceMaintenance3Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceMaintenance4Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceMaintenance4Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::DescriptorSetLayoutSupport {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk11::PhysicalDeviceShaderDrawParametersFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceShaderFloat16Int8Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceFloatControlsProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceHostQueryResetFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_object_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_object_name = writer.write_cstr(self.p_object_name);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_tag = writer
            .write_arr_ptr(self.p_tag.cast::<u8>(), (self.tag_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::extensions::ext_debug_utils::DebugUtilsLabelEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_label_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_label_name = writer.write_cstr(self.p_label_name);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_message_id_name);
            measure.measure_cstr(self.p_message);
            measure
                .measure_arr_ptr(self.p_queue_labels, (self.queue_label_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_cmd_buf_labels,
                    (self.cmd_buf_label_count) as usize,
                );
            measure.measure_arr_ptr(self.p_objects, (self.object_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_message_id_name = writer.write_cstr(self.p_message_id_name);
        (*copy).p_message = writer.write_cstr(self.p_message);
        (*copy)
            .p_queue_labels = writer
            .write_arr_ptr(self.p_queue_labels, (self.queue_label_count) as usize);
        (*copy)
            .p_cmd_buf_labels = writer
            .write_arr_ptr(self.p_cmd_buf_labels, (self.cmd_buf_label_count) as usize);
        (*copy)
            .p_objects = writer
            .write_arr_ptr(self.p_objects, (self.object_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_memory_report::DeviceDeviceMemoryReportCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_external_memory_host::ImportMemoryHostPointerInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceDescriptorIndexingFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceDescriptorIndexingProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::DescriptorSetLayoutBindingFlagsCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_binding_flags, (self.binding_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_binding_flags = writer
            .write_arr_ptr(self.p_binding_flags, (self.binding_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk12::DescriptorSetVariableDescriptorCountAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_descriptor_counts,
                    (self.descriptor_set_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_descriptor_counts = writer
            .write_arr_ptr(
                self.p_descriptor_counts,
                (self.descriptor_set_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::DescriptorSetVariableDescriptorCountLayoutSupport {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::AttachmentDescription2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::AttachmentReference2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::SubpassDescription2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_input_attachments,
                    (self.input_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_resolve_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_stencil_attachment);
            measure
                .measure_arr_ptr(
                    self.p_preserve_attachments,
                    (self.preserve_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_input_attachments = writer
            .write_arr_ptr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
            );
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_resolve_attachments = writer
            .write_arr_ptr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy)
            .p_depth_stencil_attachment = writer
            .write_ptr(self.p_depth_stencil_attachment);
        (*copy)
            .p_preserve_attachments = writer
            .write_arr_ptr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::SubpassDependency2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::RenderPassCreateInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
            measure.measure_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
            measure
                .measure_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
            measure
                .measure_arr_ptr(
                    self.p_correlated_view_masks,
                    (self.correlated_view_mask_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        (*copy)
            .p_subpasses = writer
            .write_arr_ptr(self.p_subpasses, (self.subpass_count) as usize);
        (*copy)
            .p_dependencies = writer
            .write_arr_ptr(self.p_dependencies, (self.dependency_count) as usize);
        (*copy)
            .p_correlated_view_masks = writer
            .write_arr_ptr(
                self.p_correlated_view_masks,
                (self.correlated_view_mask_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::SubpassBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::SubpassEndInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceTimelineSemaphoreFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceTimelineSemaphoreProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::SemaphoreTypeCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::TimelineSemaphoreSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_values,
                    (self.wait_semaphore_value_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_values,
                    (self.signal_semaphore_value_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_values = writer
            .write_arr_ptr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_value_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_values = writer
            .write_arr_ptr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_value_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::SemaphoreWaitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_semaphores, (self.semaphore_count) as usize);
            measure.measure_arr_ptr(self.p_values, (self.semaphore_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_semaphores = writer
            .write_arr_ptr(self.p_semaphores, (self.semaphore_count) as usize);
        (*copy)
            .p_values = writer
            .write_arr_ptr(self.p_values, (self.semaphore_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk12::SemaphoreSignalInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_attribute_divisor::PipelineVertexInputDivisorStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_vertex_binding_divisors,
                    (self.vertex_binding_divisor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_vertex_binding_divisors = writer
            .write_arr_ptr(
                self.p_vertex_binding_divisors,
                (self.vertex_binding_divisor_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pci_bus_info::PhysicalDevicePCIBusInfoPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::ImportAndroidHardwareBufferInfoANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatPropertiesANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_conditional_rendering::CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDevice8BitStorageFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceVulkanMemoryModelFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceShaderAtomicInt64Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceDepthStencilResolveProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::SubpassDescriptionDepthStencilResolve {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_depth_stencil_resolve_attachment);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_depth_stencil_resolve_attachment = writer
            .write_ptr(self.p_depth_stencil_resolve_attachment);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_astc_decode_mode::ImageViewASTCDecodeModeEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_astc_decode_mode::PhysicalDeviceASTCDecodeFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_representative_fragment_test::PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_scissor_exclusive::PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_exclusive_scissors,
                    (self.exclusive_scissor_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_exclusive_scissors = writer
            .write_arr_ptr(
                self.p_exclusive_scissors,
                (self.exclusive_scissor_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_shading_rate_palette_entries,
                    (self.shading_rate_palette_entry_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_shading_rate_palette_entries = writer
            .write_arr_ptr(
                self.p_shading_rate_palette_entries,
                (self.shading_rate_palette_entry_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shading_rate_image::PipelineViewportShadingRateImageStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_shading_rate_palettes,
                    (self.viewport_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_shading_rate_palettes = writer
            .write_arr_ptr(self.p_shading_rate_palettes, (self.viewport_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_sample_locations,
                    (self.sample_location_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_sample_locations = writer
            .write_arr_ptr(
                self.p_sample_locations,
                (self.sample_location_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shading_rate_image::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_custom_sample_orders,
                    (self.custom_sample_order_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_custom_sample_orders = writer
            .write_arr_ptr(
                self.p_custom_sample_orders,
                (self.custom_sample_order_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_stages, (self.stage_count) as usize);
            measure.measure_arr_ptr(self.p_groups, (self.group_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_stages = writer.write_arr_ptr(self.p_stages, (self.stage_count) as usize);
        (*copy)
            .p_groups = writer.write_arr_ptr(self.p_groups, (self.group_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_stages, (self.stage_count) as usize);
            measure.measure_arr_ptr(self.p_groups, (self.group_count) as usize);
            measure.measure_ptr(self.p_library_info);
            measure.measure_ptr(self.p_library_interface);
            measure.measure_ptr(self.p_dynamic_state);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_stages = writer.write_arr_ptr(self.p_stages, (self.stage_count) as usize);
        (*copy)
            .p_groups = writer.write_arr_ptr(self.p_groups, (self.group_count) as usize);
        (*copy).p_library_info = writer.write_ptr(self.p_library_info);
        (*copy).p_library_interface = writer.write_ptr(self.p_library_interface);
        (*copy).p_dynamic_state = writer.write_ptr(self.p_dynamic_state);
    }
}
unsafe impl DeepCopy for crate::extensions::nv_ray_tracing::GeometryTrianglesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::nv_ray_tracing::GeometryAABBNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::nv_ray_tracing::GeometryDataNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(&self.triangles);
            measure.measure_ptr(&self.aabbs);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        self.triangles.copy(&mut (*copy).triangles, writer);
        self.aabbs.copy(&mut (*copy).aabbs, writer);
    }
}
unsafe impl DeepCopy for crate::extensions::nv_ray_tracing::GeometryNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.geometry);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.geometry.copy(&mut (*copy).geometry, writer);
    }
}
unsafe impl DeepCopy for crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_geometries, (self.geometry_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_geometries = writer
            .write_arr_ptr(self.p_geometries, (self.geometry_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.info.copy(&mut (*copy).info, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_device_indices,
                    (self.device_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_device_indices = writer
            .write_arr_ptr(self.p_device_indices, (self.device_index_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::WriteDescriptorSetAccelerationStructureKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_acceleration_structures,
                    (self.acceleration_structure_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_acceleration_structures = writer
            .write_arr_ptr(
                self.p_acceleration_structures,
                (self.acceleration_structure_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::WriteDescriptorSetAccelerationStructureNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_acceleration_structures,
                    (self.acceleration_structure_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_acceleration_structures = writer
            .write_arr_ptr(
                self.p_acceleration_structures,
                (self.acceleration_structure_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_maintenance1::PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_drm_format_modifier_properties,
                    (self.drm_format_modifier_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        (*copy)
            .p_drm_format_modifier_properties = writer
            .write_arr_ptr(
                self.p_drm_format_modifier_properties,
                (self.drm_format_modifier_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_queue_family_indices,
                    (self.queue_family_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_queue_family_indices = writer
            .write_arr_ptr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierListCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_drm_format_modifiers,
                    (self.drm_format_modifier_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_drm_format_modifiers = writer
            .write_arr_ptr(
                self.p_drm_format_modifiers,
                (self.drm_format_modifier_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_plane_layouts,
                    (self.drm_format_modifier_plane_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_plane_layouts = writer
            .write_arr_ptr(
                self.p_plane_layouts,
                (self.drm_format_modifier_plane_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::ImageStencilUsageCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_memory_overallocation_behavior::DeviceMemoryOverallocationCreateInfoAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_fragment_density_map_offset::SubpassFragmentDensityMapOffsetEndInfoQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_fragment_density_offsets,
                    (self.fragment_density_offset_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_fragment_density_offsets = writer
            .write_arr_ptr(
                self.p_fragment_density_offsets,
                (self.fragment_density_offset_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceScalarBlockLayoutFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_memory_priority::MemoryPriorityAllocateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pageable_device_local_memory::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceBufferDeviceAddressFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::BufferDeviceAddressInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::BufferOpaqueCaptureAddressCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_buffer_device_address::BufferDeviceAddressCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceImagelessFramebufferFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::FramebufferAttachmentsCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_attachment_image_infos,
                    (self.attachment_image_info_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachment_image_infos = writer
            .write_arr_ptr(
                self.p_attachment_image_infos,
                (self.attachment_image_info_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk12::FramebufferAttachmentImageInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_view_formats = writer
            .write_arr_ptr(self.p_view_formats, (self.view_format_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk12::RenderPassAttachmentBeginInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_attachments = writer
            .write_arr_ptr(self.p_attachments, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceTextureCompressionASTCHDRFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ggp_frame_token::PresentFrameTokenGGP {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::PipelineCreationFeedbackCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_pipeline_creation_feedback);
            measure
                .measure_arr_ptr(
                    self.p_pipeline_stage_creation_feedbacks,
                    (self.pipeline_stage_creation_feedback_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_pipeline_creation_feedback = writer
            .write_ptr(self.p_pipeline_creation_feedback);
        (*copy)
            .p_pipeline_stage_creation_feedbacks = writer
            .write_arr_ptr(
                self.p_pipeline_stage_creation_feedbacks,
                (self.pipeline_stage_creation_feedback_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_present_barrier::PhysicalDevicePresentBarrierFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_present_barrier::SurfaceCapabilitiesPresentBarrierNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_present_barrier::SwapchainPresentBarrierCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::PerformanceCounterKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_counter_indices,
                    (self.counter_index_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_counter_indices = writer
            .write_arr_ptr(self.p_counter_indices, (self.counter_index_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::PerformanceValueINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(&self.data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        self.data.copy(&mut (*copy).data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::AttachmentReferenceStencilLayout {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_primitive_topology_list_restart::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::AttachmentDescriptionStencilLayout {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        (*copy)
            .p_data = writer
            .write_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy
for crate::vk13::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceTexelBufferAlignmentProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceSubgroupSizeControlFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceSubgroupSizeControlProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::huawei_subpass_shading::SubpassShadingPipelineCreateInfoHUAWEI {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::MemoryOpaqueCaptureAddressAllocateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_line_rasterization::PipelineRasterizationLineStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::vk13::PhysicalDevicePipelineCreationCacheControlFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceVulkan11Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceVulkan11Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceVulkan12Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk12::PhysicalDeviceVulkan12Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceVulkan13Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceVulkan13Properties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlCreateInfoAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceToolProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_custom_border_color::SamplerCustomBorderColorCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_border_color_swizzle::SamplerBorderColorComponentMappingCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.vertex_data);
            measure.measure_ptr(&self.index_data);
            measure.measure_ptr(&self.transform_data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.vertex_data.copy(&mut (*copy).vertex_data, writer);
        self.index_data.copy(&mut (*copy).index_data, writer);
        self.transform_data.copy(&mut (*copy).transform_data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.data.copy(&mut (*copy).data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.data.copy(&mut (*copy).data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.geometry);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.geometry.copy(&mut (*copy).geometry, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_geometries, (self.geometry_count) as usize);
            let len = (self.geometry_count) as usize;
            measure
                .alloc_arr::<
                    *const crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR,
                >(len);
            for i in 0..len {
                let ptr = *self.pp_geometries.add(i);
                measure.measure_arr_ptr(ptr, (1) as usize);
            }
            measure.measure_ptr(&self.scratch_data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_geometries = writer
            .write_arr_ptr(self.p_geometries, (self.geometry_count) as usize);
        let len = (self.geometry_count) as usize;
        (*copy)
            .pp_geometries = writer
            .alloc_arr::<
                *const crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR,
            >(len);
        for i in 0..len {
            let ptr = *self.pp_geometries.add(i);
            let copy = (*copy).pp_geometries.add(i).cast_mut();
            *copy = writer.write_arr_ptr(ptr, (1) as usize);
        }
        self.scratch_data.copy(&mut (*copy).scratch_data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_version_data,
                    (2 * crate::vk10::UUID_SIZE) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_version_data = writer
            .write_arr_ptr(self.p_version_data, (2 * crate::vk10::UUID_SIZE) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.dst);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.dst.copy(&mut (*copy).dst, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.src);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.src.copy(&mut (*copy).src, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_libraries, (self.library_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_libraries = writer
            .write_arr_ptr(self.p_libraries, (self.library_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_render_pass_transform::RenderPassTransformBeginInfoQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_render_pass_transform::CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::vk13::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_shader_subgroup_uniform_control_flow::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceImageRobustnessFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::BufferCopy2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::ImageCopy2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::ImageBlit2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::BufferImageCopy2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::ImageResolve2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::CopyBufferInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::CopyImageInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::BlitImageInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::CopyBufferToImageInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::CopyImageToBufferInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::ResolveImageInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_fragment_shading_rate_attachment);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_fragment_shading_rate_attachment = writer
            .write_ptr(self.p_fragment_shading_rate_attachment);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shading_rate::PipelineFragmentShadingRateStateCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceShaderTerminateInvocationFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_fragment_shading_rate_enums::PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_2d_view_of_3d::PhysicalDeviceImage2DViewOf3DFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_mutable_descriptor_type::MutableDescriptorTypeListEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure
                .measure_arr_ptr(
                    self.p_descriptor_types,
                    (self.descriptor_type_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_descriptor_types = writer
            .write_arr_ptr(
                self.p_descriptor_types,
                (self.descriptor_type_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_mutable_descriptor_type::MutableDescriptorTypeCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_mutable_descriptor_type_lists,
                    (self.mutable_descriptor_type_list_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_mutable_descriptor_type_lists = writer
            .write_arr_ptr(
                self.p_mutable_descriptor_type_lists,
                (self.mutable_descriptor_type_list_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_depth_clip_control::PipelineViewportDepthClipControlCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_color_write_enable::PipelineColorWriteCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_write_enables,
                    (self.attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_write_enables = writer
            .write_arr_ptr(self.p_color_write_enables, (self.attachment_count) as usize);
    }
}
unsafe impl DeepCopy for crate::vk13::MemoryBarrier2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::ImageMemoryBarrier2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::BufferMemoryBarrier2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::DependencyInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_memory_barriers,
                    (self.memory_barrier_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_buffer_memory_barriers,
                    (self.buffer_memory_barrier_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_image_memory_barriers,
                    (self.image_memory_barrier_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_memory_barriers = writer
            .write_arr_ptr(self.p_memory_barriers, (self.memory_barrier_count) as usize);
        (*copy)
            .p_buffer_memory_barriers = writer
            .write_arr_ptr(
                self.p_buffer_memory_barriers,
                (self.buffer_memory_barrier_count) as usize,
            );
        (*copy)
            .p_image_memory_barriers = writer
            .write_arr_ptr(
                self.p_image_memory_barriers,
                (self.image_memory_barrier_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk13::SemaphoreSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::CommandBufferSubmitInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::SubmitInfo2 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_wait_semaphore_infos,
                    (self.wait_semaphore_info_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_command_buffer_infos,
                    (self.command_buffer_info_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_signal_semaphore_infos,
                    (self.signal_semaphore_info_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_wait_semaphore_infos = writer
            .write_arr_ptr(
                self.p_wait_semaphore_infos,
                (self.wait_semaphore_info_count) as usize,
            );
        (*copy)
            .p_command_buffer_infos = writer
            .write_arr_ptr(
                self.p_command_buffer_infos,
                (self.command_buffer_info_count) as usize,
            );
        (*copy)
            .p_signal_semaphore_infos = writer
            .write_arr_ptr(
                self.p_signal_semaphore_infos,
                (self.signal_semaphore_info_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_synchronization2::CheckpointData2NV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceSynchronization2Features {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_primitives_generated_query::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_legacy_dithering::PhysicalDeviceLegacyDitheringFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_multisampled_render_to_single_sampled::PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_multisampled_render_to_single_sampled::SubpassResolvePerformanceQueryEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_multisampled_render_to_single_sampled::MultisampledRenderToSingleSampledInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_protected_access::PhysicalDevicePipelineProtectedAccessFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::QueueFamilyVideoPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::QueueFamilyQueryResultStatusPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoProfileListInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_profiles, (self.profile_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_profiles = writer
            .write_arr_ptr(self.p_profiles, (self.profile_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoFormatPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoProfileInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::VideoSessionMemoryRequirementsKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::BindVideoSessionMemoryInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::VideoPictureResourceInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_picture_resource);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_picture_resource = writer.write_ptr(self.p_picture_resource);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_decode_queue::VideoDecodeUsageInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.dst_picture_resource);
            measure.measure_ptr(self.p_setup_reference_slot);
            measure
                .measure_arr_ptr(
                    self.p_reference_slots,
                    (self.reference_slot_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.dst_picture_resource.copy(&mut (*copy).dst_picture_resource, writer);
        (*copy).p_setup_reference_slot = writer.write_ptr(self.p_setup_reference_slot);
        (*copy)
            .p_reference_slots = writer
            .write_arr_ptr(self.p_reference_slots, (self.reference_slot_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::h264std::StdVideoH264SequenceParameterSetVui {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_hrd_parameters);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_hrd_parameters = writer.write_ptr(self.p_hrd_parameters);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264ProfileInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::h264std::StdVideoH264SequenceParameterSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_offset_for_ref_frame);
            measure.measure_ptr(self.p_scaling_lists);
            measure.measure_ptr(self.p_sequence_parameter_set_vui);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_offset_for_ref_frame = writer.write_ptr(self.p_offset_for_ref_frame);
        (*copy).p_scaling_lists = writer.write_ptr(self.p_scaling_lists);
        (*copy)
            .p_sequence_parameter_set_vui = writer
            .write_ptr(self.p_sequence_parameter_set_vui);
    }
}
unsafe impl DeepCopy for crate::extensions::h264std::StdVideoH264PictureParameterSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_scaling_lists);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_scaling_lists = writer.write_ptr(self.p_scaling_lists);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
            measure.measure_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_std_spss = writer
            .write_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
        (*copy)
            .p_std_ppss = writer
            .write_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_parameters_add_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_parameters_add_info = writer.write_ptr(self.p_parameters_add_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_picture_info);
            measure.measure_arr_ptr(self.p_slice_offsets, (self.slice_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_picture_info = writer.write_ptr(self.p_std_picture_info);
        (*copy)
            .p_slice_offsets = writer
            .write_arr_ptr(self.p_slice_offsets, (self.slice_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_reference_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_reference_info = writer.write_ptr(self.p_std_reference_info);
    }
}
unsafe impl DeepCopy for crate::extensions::h265std::StdVideoH265VideoParameterSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_dec_pic_buf_mgr);
            measure.measure_ptr(self.p_hrd_parameters);
            measure.measure_ptr(self.p_profile_tier_level);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_dec_pic_buf_mgr = writer.write_ptr(self.p_dec_pic_buf_mgr);
        (*copy).p_hrd_parameters = writer.write_ptr(self.p_hrd_parameters);
        (*copy).p_profile_tier_level = writer.write_ptr(self.p_profile_tier_level);
    }
}
unsafe impl DeepCopy for crate::extensions::h265std::StdVideoH265SequenceParameterSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_profile_tier_level);
            measure.measure_ptr(self.p_dec_pic_buf_mgr);
            measure.measure_ptr(self.p_scaling_lists);
            measure.measure_ptr(self.p_short_term_ref_pic_set);
            measure.measure_ptr(self.p_long_term_ref_pics_sps);
            measure.measure_ptr(self.p_sequence_parameter_set_vui);
            measure.measure_ptr(self.p_predictor_palette_entries);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_profile_tier_level = writer.write_ptr(self.p_profile_tier_level);
        (*copy).p_dec_pic_buf_mgr = writer.write_ptr(self.p_dec_pic_buf_mgr);
        (*copy).p_scaling_lists = writer.write_ptr(self.p_scaling_lists);
        (*copy)
            .p_short_term_ref_pic_set = writer.write_ptr(self.p_short_term_ref_pic_set);
        (*copy)
            .p_long_term_ref_pics_sps = writer.write_ptr(self.p_long_term_ref_pics_sps);
        (*copy)
            .p_sequence_parameter_set_vui = writer
            .write_ptr(self.p_sequence_parameter_set_vui);
        (*copy)
            .p_predictor_palette_entries = writer
            .write_ptr(self.p_predictor_palette_entries);
    }
}
unsafe impl DeepCopy for crate::extensions::h265std::StdVideoH265PictureParameterSet {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_scaling_lists);
            measure.measure_ptr(self.p_predictor_palette_entries);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_scaling_lists = writer.write_ptr(self.p_scaling_lists);
        (*copy)
            .p_predictor_palette_entries = writer
            .write_ptr(self.p_predictor_palette_entries);
    }
}
unsafe impl DeepCopy for crate::extensions::h265std::StdVideoH265HrdParameters {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_sub_layer_hrd_parameters_nal);
            measure.measure_ptr(self.p_sub_layer_hrd_parameters_vcl);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_sub_layer_hrd_parameters_nal = writer
            .write_ptr(self.p_sub_layer_hrd_parameters_nal);
        (*copy)
            .p_sub_layer_hrd_parameters_vcl = writer
            .write_ptr(self.p_sub_layer_hrd_parameters_vcl);
    }
}
unsafe impl DeepCopy
for crate::extensions::h265std::StdVideoH265SequenceParameterSetVui {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_hrd_parameters);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_hrd_parameters = writer.write_ptr(self.p_hrd_parameters);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265ProfileInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_std_vpss, (self.std_vpscount) as usize);
            measure.measure_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
            measure.measure_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_std_vpss = writer
            .write_arr_ptr(self.p_std_vpss, (self.std_vpscount) as usize);
        (*copy)
            .p_std_spss = writer
            .write_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
        (*copy)
            .p_std_ppss = writer
            .write_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_parameters_add_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_parameters_add_info = writer.write_ptr(self.p_parameters_add_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265PictureInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_picture_info);
            measure.measure_arr_ptr(self.p_slice_offsets, (self.slice_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_picture_info = writer.write_ptr(self.p_std_picture_info);
        (*copy)
            .p_slice_offsets = writer
            .write_arr_ptr(self.p_slice_offsets, (self.slice_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_reference_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_reference_info = writer.write_ptr(self.p_std_reference_info);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_video_profile);
            measure.measure_ptr(self.p_std_header_version);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_video_profile = writer.write_ptr(self.p_video_profile);
        (*copy).p_std_header_version = writer.write_ptr(self.p_std_header_version);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_reference_slots,
                    (self.reference_slot_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_reference_slots = writer
            .write_arr_ptr(self.p_reference_slots, (self.reference_slot_count) as usize);
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoEndCodingInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_queue::VideoCodingControlInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_encode_queue::VideoEncodeUsageInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.src_picture_resource);
            measure.measure_ptr(self.p_setup_reference_slot);
            measure
                .measure_arr_ptr(
                    self.p_reference_slots,
                    (self.reference_slot_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.src_picture_resource.copy(&mut (*copy).src_picture_resource, writer);
        (*copy).p_setup_reference_slot = writer.write_ptr(self.p_setup_reference_slot);
        (*copy)
            .p_reference_slots = writer
            .write_arr_ptr(self.p_reference_slots, (self.reference_slot_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_layer_configs, (self.layer_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_layer_configs = writer
            .write_arr_ptr(self.p_layer_configs, (self.layer_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::h264std_encode::StdVideoEncodeH264SliceHeader {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_weight_table);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_weight_table = writer.write_ptr(self.p_weight_table);
    }
}
unsafe impl DeepCopy
for crate::extensions::h264std_encode::StdVideoEncodeH264RefMemMgmtCtrlOperations {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_ref_list_0_mod_operations);
            measure.measure_ptr(self.p_ref_list_1_mod_operations);
            measure.measure_ptr(self.p_ref_pic_marking_operations);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_ref_list_0_mod_operations = writer
            .write_ptr(self.p_ref_list_0_mod_operations);
        (*copy)
            .p_ref_list_1_mod_operations = writer
            .write_ptr(self.p_ref_list_1_mod_operations);
        (*copy)
            .p_ref_pic_marking_operations = writer
            .write_ptr(self.p_ref_pic_marking_operations);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
            measure.measure_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_std_spss = writer
            .write_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
        (*copy)
            .p_std_ppss = writer
            .write_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_parameters_add_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_parameters_add_info = writer.write_ptr(self.p_parameters_add_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_reference_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_reference_info = writer.write_ptr(self.p_std_reference_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264VclFrameInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_reference_final_lists);
            measure
                .measure_arr_ptr(
                    self.p_nalu_slice_entries,
                    (self.nalu_slice_entry_count) as usize,
                );
            measure.measure_ptr(self.p_current_picture_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_reference_final_lists = writer.write_ptr(self.p_reference_final_lists);
        (*copy)
            .p_nalu_slice_entries = writer
            .write_arr_ptr(
                self.p_nalu_slice_entries,
                (self.nalu_slice_entry_count) as usize,
            );
        (*copy).p_current_picture_info = writer.write_ptr(self.p_current_picture_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264ReferenceListsInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_reference_list_0_entries,
                    (self.reference_list_0_entry_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_reference_list_1_entries,
                    (self.reference_list_1_entry_count) as usize,
                );
            measure.measure_ptr(self.p_mem_mgmt_ctrl_operations);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_reference_list_0_entries = writer
            .write_arr_ptr(
                self.p_reference_list_0_entries,
                (self.reference_list_0_entry_count) as usize,
            );
        (*copy)
            .p_reference_list_1_entries = writer
            .write_arr_ptr(
                self.p_reference_list_1_entries,
                (self.reference_list_1_entry_count) as usize,
            );
        (*copy)
            .p_mem_mgmt_ctrl_operations = writer
            .write_ptr(self.p_mem_mgmt_ctrl_operations);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264EmitPictureParametersInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.pps_id_entries,
                    (self.pps_id_entry_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .pps_id_entries = writer
            .write_arr_ptr(self.pps_id_entries, (self.pps_id_entry_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264ProfileInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264NaluSliceInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_reference_final_lists);
            measure.measure_ptr(self.p_slice_header_std);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_reference_final_lists = writer.write_ptr(self.p_reference_final_lists);
        (*copy).p_slice_header_std = writer.write_ptr(self.p_slice_header_std);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlLayerInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265CapabilitiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::h265std_encode::StdVideoEncodeH265SliceSegmentHeader {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_short_term_ref_pic_set);
            measure.measure_ptr(self.p_long_term_ref_pics);
            measure.measure_ptr(self.p_weight_table);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_short_term_ref_pic_set = writer.write_ptr(self.p_short_term_ref_pic_set);
        (*copy).p_long_term_ref_pics = writer.write_ptr(self.p_long_term_ref_pics);
        (*copy).p_weight_table = writer.write_ptr(self.p_weight_table);
    }
}
unsafe impl DeepCopy
for crate::extensions::h265std_encode::StdVideoEncodeH265ReferenceModifications {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_ptr(self.p_reference_list_0_modifications);
            measure.measure_ptr(self.p_reference_list_1_modifications);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy)
            .p_reference_list_0_modifications = writer
            .write_ptr(self.p_reference_list_0_modifications);
        (*copy)
            .p_reference_list_1_modifications = writer
            .write_ptr(self.p_reference_list_1_modifications);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_std_vpss, (self.std_vpscount) as usize);
            measure.measure_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
            measure.measure_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_std_vpss = writer
            .write_arr_ptr(self.p_std_vpss, (self.std_vpscount) as usize);
        (*copy)
            .p_std_spss = writer
            .write_arr_ptr(self.p_std_spss, (self.std_spscount) as usize);
        (*copy)
            .p_std_ppss = writer
            .write_arr_ptr(self.p_std_ppss, (self.std_ppscount) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_parameters_add_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_parameters_add_info = writer.write_ptr(self.p_parameters_add_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265VclFrameInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_reference_final_lists);
            measure
                .measure_arr_ptr(
                    self.p_nalu_slice_segment_entries,
                    (self.nalu_slice_segment_entry_count) as usize,
                );
            measure.measure_ptr(self.p_current_picture_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_reference_final_lists = writer.write_ptr(self.p_reference_final_lists);
        (*copy)
            .p_nalu_slice_segment_entries = writer
            .write_arr_ptr(
                self.p_nalu_slice_segment_entries,
                (self.nalu_slice_segment_entry_count) as usize,
            );
        (*copy).p_current_picture_info = writer.write_ptr(self.p_current_picture_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265EmitPictureParametersInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.pps_id_entries,
                    (self.pps_id_entry_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .pps_id_entries = writer
            .write_arr_ptr(self.pps_id_entries, (self.pps_id_entry_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265NaluSliceSegmentInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_reference_final_lists);
            measure.measure_ptr(self.p_slice_segment_header_std);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_reference_final_lists = writer.write_ptr(self.p_reference_final_lists);
        (*copy)
            .p_slice_segment_header_std = writer
            .write_ptr(self.p_slice_segment_header_std);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlLayerInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265ProfileInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_std_reference_info);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_std_reference_info = writer.write_ptr(self.p_std_reference_info);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_reference_list_0_entries,
                    (self.reference_list_0_entry_count) as usize,
                );
            measure
                .measure_arr_ptr(
                    self.p_reference_list_1_entries,
                    (self.reference_list_1_entry_count) as usize,
                );
            measure.measure_ptr(self.p_reference_modifications);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_reference_list_0_entries = writer
            .write_arr_ptr(
                self.p_reference_list_0_entries,
                (self.reference_list_0_entry_count) as usize,
            );
        (*copy)
            .p_reference_list_1_entries = writer
            .write_arr_ptr(
                self.p_reference_list_1_entries,
                (self.reference_list_1_entry_count) as usize,
            );
        (*copy)
            .p_reference_modifications = writer
            .write_ptr(self.p_reference_modifications);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_inherited_viewport_scissor::CommandBufferInheritanceViewportScissorInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_viewport_depths);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_viewport_depths = writer.write_ptr(self.p_viewport_depths);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_provoking_vertex::PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_data = writer
            .write_arr_ptr(self.p_data.cast::<u8>(), (self.data_size) as usize)
            .cast::<c_void>();
    }
}
unsafe impl DeepCopy for crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_cstr(self.p_name);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_name = writer.write_cstr(self.p_name);
    }
}
unsafe impl DeepCopy for crate::extensions::nvx_binary_import::CuLaunchInfoNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            let len = (self.param_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_void>(len);
            for i in 0..len {
                let ptr = *self.p_params.add(i);
            }
            let len = (self.extra_count) as usize;
            measure.alloc_arr::<*const std::os::raw::c_void>(len);
            for i in 0..len {
                let ptr = *self.p_extras.add(i);
            }
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        let len = (self.param_count) as usize;
        (*copy).p_params = writer.alloc_arr::<*const std::os::raw::c_void>(len);
        for i in 0..len {
            let ptr = *self.p_params.add(i);
            let copy = (*copy).p_params.add(i).cast_mut();
        }
        let len = (self.extra_count) as usize;
        (*copy).p_extras = writer.alloc_arr::<*const std::os::raw::c_void>(len);
        for i in 0..len {
            let ptr = *self.p_extras.add(i);
            let copy = (*copy).p_extras.add(i).cast_mut();
        }
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceShaderIntegerDotProductFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceShaderIntegerDotProductProperties {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.vertex_data);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.vertex_data.copy(&mut (*copy).vertex_data, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::ImportMemoryBufferCollectionFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferCollectionImageCreateInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferCollectionBufferCreateInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.sysmem_color_space_index);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        self.sysmem_color_space_index
            .copy(&mut (*copy).sysmem_color_space_index, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.create_info);
            measure.measure_ptr(&self.buffer_collection_constraints);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.create_info.copy(&mut (*copy).create_info, writer);
        self.buffer_collection_constraints
            .copy(&mut (*copy).buffer_collection_constraints, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.image_create_info);
            measure
                .measure_arr_ptr(self.p_color_spaces, (self.color_space_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.image_create_info.copy(&mut (*copy).image_create_info, writer);
        (*copy)
            .p_color_spaces = writer
            .write_arr_ptr(self.p_color_spaces, (self.color_space_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_format_constraints,
                    (self.format_constraints_count) as usize,
                );
            measure.measure_ptr(&self.buffer_collection_constraints);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_format_constraints = writer
            .write_arr_ptr(
                self.p_format_constraints,
                (self.format_constraints_count) as usize,
            );
        self.buffer_collection_constraints
            .copy(&mut (*copy).buffer_collection_constraints, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_rgba10x6_formats::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::FormatProperties3 {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_drm_format_modifier_properties,
                    (self.drm_format_modifier_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        (*copy)
            .p_drm_format_modifier_properties = writer
            .write_arr_ptr(
                self.p_drm_format_modifier_properties,
                (self.drm_format_modifier_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatProperties2ANDROID {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::PipelineRenderingCreateInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachment_formats,
                    (self.color_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachment_formats = writer
            .write_arr_ptr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy for crate::vk13::RenderingInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachments,
                    (self.color_attachment_count) as usize,
                );
            measure.measure_ptr(self.p_depth_attachment);
            measure.measure_ptr(self.p_stencil_attachment);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachments = writer
            .write_arr_ptr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
            );
        (*copy).p_depth_attachment = writer.write_ptr(self.p_depth_attachment);
        (*copy).p_stencil_attachment = writer.write_ptr(self.p_stencil_attachment);
    }
}
unsafe impl DeepCopy for crate::vk13::RenderingAttachmentInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::vk13::PhysicalDeviceDynamicRenderingFeatures {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::vk13::CommandBufferInheritanceRenderingInfo {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachment_formats,
                    (self.color_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachment_formats = writer
            .write_arr_ptr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_color_attachment_samples,
                    (self.color_attachment_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_color_attachment_samples = writer
            .write_arr_ptr(
                self.p_color_attachment_samples,
                (self.color_attachment_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_view_min_lod::ImageViewMinLodCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_rasterization_order_attachment_access::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::valve_descriptor_set_host_mapping::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetBindingReferenceVALVE {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetLayoutHostMappingInfoVALVE {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_module_identifier::PipelineShaderStageModuleIdentifierCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_identifier, (self.identifier_size) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_identifier = writer
            .write_arr_ptr(self.p_identifier, (self.identifier_size) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control::ImageCompressionControlEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_fixed_rate_flags,
                    (self.compression_control_plane_count) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_fixed_rate_flags = writer
            .write_arr_ptr(
                self.p_fixed_rate_flags,
                (self.compression_control_plane_count) as usize,
            );
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control::PhysicalDeviceImageCompressionControlFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control::ImageCompressionPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control_swapchain::PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control::ImageSubresource2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_image_compression_control::SubresourceLayout2EXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_subpass_merge_feedback::RenderPassCreationControlEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_subpass_merge_feedback::RenderPassCreationFeedbackCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_render_pass_feedback);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_render_pass_feedback = writer.write_ptr(self.p_render_pass_feedback);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_subpass_merge_feedback::RenderPassSubpassFeedbackCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_subpass_feedback);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy).p_subpass_feedback = writer.write_ptr(self.p_subpass_feedback);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_subpass_merge_feedback::PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_usage_counts,
                    (self.usage_counts_count) as usize,
                );
            let len = (self.usage_counts_count) as usize;
            measure
                .alloc_arr::<
                    *const crate::extensions::ext_opacity_micromap::MicromapUsageEXT,
                >(len);
            for i in 0..len {
                let ptr = *self.pp_usage_counts.add(i);
                measure.measure_arr_ptr(ptr, (1) as usize);
            }
            measure.measure_ptr(&self.data);
            measure.measure_ptr(&self.scratch_data);
            measure.measure_ptr(&self.triangle_array);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_usage_counts = writer
            .write_arr_ptr(self.p_usage_counts, (self.usage_counts_count) as usize);
        let len = (self.usage_counts_count) as usize;
        (*copy)
            .pp_usage_counts = writer
            .alloc_arr::<
                *const crate::extensions::ext_opacity_micromap::MicromapUsageEXT,
            >(len);
        for i in 0..len {
            let ptr = *self.pp_usage_counts.add(i);
            let copy = (*copy).pp_usage_counts.add(i).cast_mut();
            *copy = writer.write_arr_ptr(ptr, (1) as usize);
        }
        self.data.copy(&mut (*copy).data, writer);
        self.scratch_data.copy(&mut (*copy).scratch_data, writer);
        self.triangle_array.copy(&mut (*copy).triangle_array, writer);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_opacity_micromap::MicromapCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::MicromapVersionInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure
                .measure_arr_ptr(
                    self.p_version_data,
                    (2 * crate::vk10::UUID_SIZE) as usize,
                );
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        (*copy)
            .p_version_data = writer
            .write_arr_ptr(self.p_version_data, (2 * crate::vk10::UUID_SIZE) as usize);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.dst);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.dst.copy(&mut (*copy).dst, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.src);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
        self.src.copy(&mut (*copy).src, writer);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::MicromapBuildSizesInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_opacity_micromap::AccelerationStructureTrianglesOpacityMicromapEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(&self.index_buffer);
            measure
                .measure_arr_ptr(
                    self.p_usage_counts,
                    (self.usage_counts_count) as usize,
                );
            let len = (self.usage_counts_count) as usize;
            measure
                .alloc_arr::<
                    *const crate::extensions::ext_opacity_micromap::MicromapUsageEXT,
                >(len);
            for i in 0..len {
                let ptr = *self.pp_usage_counts.add(i);
                measure.measure_arr_ptr(ptr, (1) as usize);
            }
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        self.index_buffer.copy(&mut (*copy).index_buffer, writer);
        (*copy)
            .p_usage_counts = writer
            .write_arr_ptr(self.p_usage_counts, (self.usage_counts_count) as usize);
        let len = (self.usage_counts_count) as usize;
        (*copy)
            .pp_usage_counts = writer
            .alloc_arr::<
                *const crate::extensions::ext_opacity_micromap::MicromapUsageEXT,
            >(len);
        for i in 0..len {
            let ptr = *self.pp_usage_counts.add(i);
            let copy = (*copy).pp_usage_counts.add(i).cast_mut();
            *copy = writer.write_arr_ptr(ptr, (1) as usize);
        }
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_properties::PipelinePropertiesIdentifierEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_properties::PhysicalDevicePipelinePropertiesFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::amd_shader_early_and_late_fragment_tests::PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalObjectCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalObjectsInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_metal_objects::ExportMetalDeviceInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalCommandQueueInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_metal_objects::ExportMetalBufferInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy for crate::extensions::ext_metal_objects::ImportMetalBufferInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalTextureInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ImportMetalTextureInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalIOSurfaceInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ImportMetalIOSurfaceInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ExportMetalSharedEventInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_metal_objects::ImportMetalSharedEventInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_non_seamless_cube_map::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_robustness::PipelineRobustnessCreateInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessPropertiesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_image_processing::ImageViewSampleWeightCreateInfoQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_image_processing::PhysicalDeviceImageProcessingFeaturesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_image_processing::PhysicalDeviceImageProcessingPropertiesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::qcom_tile_properties::PhysicalDeviceTilePropertiesFeaturesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::qcom_tile_properties::TilePropertiesQCOM {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::sec_amigo_profiling::PhysicalDeviceAmigoProfilingFeaturesSEC {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::sec_amigo_profiling::AmigoProfilingSubmitInfoSEC {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_attachment_feedback_loop_layout::PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_depth_clamp_zero_one::PhysicalDeviceDepthClampZeroOneFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_address_binding_report::PhysicalDeviceAddressBindingReportFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_address_binding_report::DeviceAddressBindingCallbackDataEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowFeaturesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::OpticalFlowImageFormatInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::OpticalFlowImageFormatPropertiesNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next) as *const _;
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::OpticalFlowSessionCreateInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy
for crate::extensions::nv_optical_flow::OpticalFlowSessionCreatePrivateDataInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::nv_optical_flow::OpticalFlowExecuteInfoNV {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_arr_ptr(self.p_regions, (self.region_count) as usize);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        (*copy)
            .p_regions = writer
            .write_arr_ptr(self.p_regions, (self.region_count) as usize);
    }
}
unsafe impl DeepCopy
for crate::extensions::ext_device_fault::PhysicalDeviceFaultFeaturesEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_device_fault::DeviceFaultCountsEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
    }
}
unsafe impl DeepCopy for crate::extensions::ext_device_fault::DeviceFaultInfoEXT {
    fn measure(&self, measure: &mut CopyMeasure) {
        unsafe {
            measure.measure_pnext(self.p_next);
            measure.measure_ptr(self.p_address_infos);
            measure.measure_ptr(self.p_vendor_infos);
        }
    }
    unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
        (*copy).p_next = writer.write_pnext(self.p_next);
        (*copy).p_address_infos = writer.write_ptr(self.p_address_infos);
        (*copy).p_vendor_infos = writer.write_ptr(self.p_vendor_infos);
    }
}
value_type_copy_impl! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize, crate
    ::vk10::QueryPoolCreateFlags, crate ::vk10::PipelineDynamicStateCreateFlags, crate
    ::vk10::PipelineMultisampleStateCreateFlags, crate
    ::vk10::PipelineRasterizationStateCreateFlags, crate
    ::vk10::PipelineViewportStateCreateFlags, crate
    ::vk10::PipelineTessellationStateCreateFlags, crate
    ::vk10::PipelineInputAssemblyStateCreateFlags, crate
    ::vk10::PipelineVertexInputStateCreateFlags, crate ::vk10::BufferViewCreateFlags,
    crate ::vk10::DeviceCreateFlags, crate ::vk10::SemaphoreCreateFlags, crate
    ::vk10::ShaderModuleCreateFlags, crate ::vk10::MemoryMapFlags, crate
    ::vk10::DescriptorPoolResetFlags, crate ::vk13::PrivateDataSlotCreateFlags, crate
    ::vk11::DescriptorUpdateTemplateCreateFlags, crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoFlagsNV,
    crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceFlagsNV,
    crate ::extensions::khr_display::DisplayModeCreateFlagsKHR, crate
    ::extensions::khr_display::DisplaySurfaceCreateFlagsKHR, crate
    ::extensions::khr_android_surface::AndroidSurfaceCreateFlagsKHR, crate
    ::extensions::nn_vi_surface::ViSurfaceCreateFlagsNN, crate
    ::extensions::khr_wayland_surface::WaylandSurfaceCreateFlagsKHR, crate
    ::extensions::khr_win32_surface::Win32SurfaceCreateFlagsKHR, crate
    ::extensions::khr_xlib_surface::XlibSurfaceCreateFlagsKHR, crate
    ::extensions::khr_xcb_surface::XcbSurfaceCreateFlagsKHR, crate
    ::extensions::ext_directfb_surface::DirectFBSurfaceCreateFlagsEXT, crate
    ::extensions::mvk_ios_surface::IOSSurfaceCreateFlagsMVK, crate
    ::extensions::mvk_macos_surface::MacOSSurfaceCreateFlagsMVK, crate
    ::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT, crate
    ::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateFlagsFUCHSIA, crate
    ::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    crate ::extensions::ext_headless_surface::HeadlessSurfaceCreateFlagsEXT, crate
    ::extensions::qnx_screen_surface::ScreenSurfaceCreateFlagsQNX, crate
    ::vk11::CommandPoolTrimFlags, crate
    ::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV, crate
    ::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateFlagsEXT,
    crate
    ::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV,
    crate
    ::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateFlagsNV,
    crate
    ::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateFlagsNV,
    crate ::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT, crate
    ::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT, crate
    ::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateFlagsEXT,
    crate
    ::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateFlagsEXT,
    crate
    ::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateFlagsEXT,
    crate ::extensions::fuchsia_buffer_collection::ImageFormatConstraintsFlagsFUCHSIA,
    crate ::extensions::khr_video_queue::VideoSessionParametersCreateFlagsKHR, crate
    ::extensions::khr_video_queue::VideoBeginCodingFlagsKHR, crate
    ::extensions::khr_video_queue::VideoEndCodingFlagsKHR, crate
    ::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeRateControlFlagsKHR, crate
    ::vk10::Instance, crate ::vk10::PhysicalDevice, crate ::vk10::Device, crate
    ::vk10::Queue, crate ::vk10::CommandBuffer, crate ::vk10::DeviceMemory, crate
    ::vk10::CommandPool, crate ::vk10::Buffer, crate ::vk10::BufferView, crate
    ::vk10::Image, crate ::vk10::ImageView, crate ::vk10::ShaderModule, crate
    ::vk10::Pipeline, crate ::vk10::PipelineLayout, crate ::vk10::Sampler, crate
    ::vk10::DescriptorSet, crate ::vk10::DescriptorSetLayout, crate
    ::vk10::DescriptorPool, crate ::vk10::Fence, crate ::vk10::Semaphore, crate
    ::vk10::Event, crate ::vk10::QueryPool, crate ::vk10::Framebuffer, crate
    ::vk10::RenderPass, crate ::vk10::PipelineCache, crate
    ::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV, crate
    ::vk11::DescriptorUpdateTemplate, crate ::vk11::SamplerYcbcrConversion, crate
    ::extensions::ext_validation_cache::ValidationCacheEXT, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureKHR, crate
    ::extensions::nv_ray_tracing::AccelerationStructureNV, crate
    ::extensions::intel_performance_query::PerformanceConfigurationINTEL, crate
    ::extensions::fuchsia_buffer_collection::BufferCollectionFUCHSIA, crate
    ::extensions::khr_deferred_host_operations::DeferredOperationKHR, crate
    ::vk13::PrivateDataSlot, crate ::extensions::nvx_binary_import::CuModuleNVX, crate
    ::extensions::nvx_binary_import::CuFunctionNVX, crate
    ::extensions::nv_optical_flow::OpticalFlowSessionNV, crate
    ::extensions::ext_opacity_micromap::MicromapEXT, crate
    ::extensions::khr_display::DisplayKHR, crate
    ::extensions::khr_display::DisplayModeKHR, crate
    ::extensions::khr_surface::SurfaceKHR, crate
    ::extensions::khr_swapchain::SwapchainKHR, crate
    ::extensions::ext_debug_report::DebugReportCallbackEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessengerEXT, crate
    ::extensions::khr_video_queue::VideoSessionKHR, crate
    ::extensions::khr_video_queue::VideoSessionParametersKHR, crate ::vk10::Offset2D,
    crate ::vk10::Offset3D, crate ::vk10::Extent2D, crate ::vk10::Extent3D, crate
    ::vk10::Viewport, crate ::vk10::Rect2D, crate ::vk10::ClearRect, crate
    ::vk10::ComponentMapping, crate ::vk10::PhysicalDeviceProperties, crate
    ::vk10::ExtensionProperties, crate ::vk10::LayerProperties, crate
    ::vk10::QueueFamilyProperties, crate ::vk10::PhysicalDeviceMemoryProperties, crate
    ::vk10::MemoryRequirements, crate ::vk10::SparseImageFormatProperties, crate
    ::vk10::SparseImageMemoryRequirements, crate ::vk10::MemoryType, crate
    ::vk10::MemoryHeap, crate ::vk10::FormatProperties, crate
    ::vk10::ImageFormatProperties, crate ::vk10::DescriptorBufferInfo, crate
    ::vk10::DescriptorImageInfo, crate ::vk10::ImageSubresource, crate
    ::vk10::ImageSubresourceLayers, crate ::vk10::ImageSubresourceRange, crate
    ::vk10::SubresourceLayout, crate ::vk10::BufferCopy, crate ::vk10::SparseMemoryBind,
    crate ::vk10::SparseImageMemoryBind, crate ::vk10::ImageCopy, crate
    ::vk10::ImageBlit, crate ::vk10::BufferImageCopy, crate ::vk10::ImageResolve, crate
    ::vk10::DescriptorPoolSize, crate ::vk10::SpecializationMapEntry, crate
    ::vk10::VertexInputBindingDescription, crate ::vk10::VertexInputAttributeDescription,
    crate ::vk10::PipelineColorBlendAttachmentState, crate ::vk10::StencilOpState, crate
    ::vk10::PipelineCacheHeaderVersionOne, crate ::vk10::PushConstantRange, crate
    ::vk10::ClearColorValue, crate ::vk10::ClearDepthStencilValue, crate
    ::vk10::ClearValue, crate ::vk10::ClearAttachment, crate
    ::vk10::AttachmentDescription, crate ::vk10::AttachmentReference, crate
    ::vk10::SubpassDependency, crate ::vk10::PhysicalDeviceFeatures, crate
    ::vk10::PhysicalDeviceSparseProperties, crate ::vk10::PhysicalDeviceLimits, crate
    ::vk10::DrawIndirectCommand, crate ::vk10::DrawIndexedIndirectCommand, crate
    ::vk10::DispatchIndirectCommand, crate
    ::extensions::ext_multi_draw::MultiDrawInfoEXT, crate
    ::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT, crate
    ::extensions::khr_display::DisplayPlanePropertiesKHR, crate
    ::extensions::khr_display::DisplayModeParametersKHR, crate
    ::extensions::khr_display::DisplayModePropertiesKHR, crate
    ::extensions::khr_display::DisplayPlaneCapabilitiesKHR, crate
    ::extensions::khr_surface::SurfaceCapabilitiesKHR, crate
    ::extensions::khr_surface::SurfaceFormatKHR, crate
    ::extensions::nv_external_memory_capabilities::ExternalImageFormatPropertiesNV, crate
    ::extensions::nv_device_generated_commands::BindShaderGroupIndirectCommandNV, crate
    ::extensions::nv_device_generated_commands::BindIndexBufferIndirectCommandNV, crate
    ::extensions::nv_device_generated_commands::BindVertexBufferIndirectCommandNV, crate
    ::extensions::nv_device_generated_commands::SetStateFlagsIndirectCommandNV, crate
    ::extensions::nv_device_generated_commands::IndirectCommandsStreamNV, crate
    ::vk12::ConformanceVersion, crate
    ::extensions::khr_incremental_present::RectLayerKHR, crate
    ::vk11::ExternalMemoryProperties, crate ::vk11::DescriptorUpdateTemplateEntry, crate
    ::extensions::ext_hdr_metadata::XYColorEXT, crate
    ::extensions::google_display_timing::RefreshCycleDurationGOOGLE, crate
    ::extensions::google_display_timing::PastPresentationTimingGOOGLE, crate
    ::extensions::google_display_timing::PresentTimeGOOGLE, crate
    ::extensions::nv_clip_space_w_scaling::ViewportWScalingNV, crate
    ::extensions::nv_viewport_swizzle::ViewportSwizzleNV, crate
    ::vk11::InputAttachmentAspectReference, crate
    ::extensions::ext_sample_locations::SampleLocationEXT, crate
    ::extensions::amd_shader_info::ShaderResourceUsageAMD, crate
    ::extensions::amd_shader_info::ShaderStatisticsInfoAMD, crate
    ::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT,
    crate ::extensions::nv_shading_rate_image::CoarseSampleLocationNV, crate
    ::extensions::nv_mesh_shader::DrawMeshTasksIndirectCommandNV, crate
    ::extensions::ext_mesh_shader::DrawMeshTasksIndirectCommandEXT, crate
    ::extensions::khr_ray_tracing_pipeline::StridedDeviceAddressRegionKHR, crate
    ::extensions::khr_ray_tracing_pipeline::TraceRaysIndirectCommandKHR, crate
    ::extensions::khr_ray_tracing_maintenance1::TraceRaysIndirectCommand2KHR, crate
    ::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT, crate
    ::vk13::PipelineCreationFeedback, crate
    ::extensions::khr_performance_query::PerformanceCounterResultKHR, crate
    ::extensions::intel_performance_query::PerformanceValueDataINTEL, crate
    ::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
    crate ::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR, crate
    ::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
    crate ::extensions::khr_acceleration_structure::AabbPositionsKHR, crate
    ::extensions::khr_acceleration_structure::TransformMatrixKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR, crate
    ::extensions::ext_extended_dynamic_state3::ColorBlendEquationEXT, crate
    ::extensions::ext_extended_dynamic_state3::ColorBlendAdvancedEXT, crate
    ::extensions::h264std::StdVideoH264ProfileIdc, crate
    ::extensions::h264std::StdVideoH264LevelIdc, crate
    ::extensions::h264std::StdVideoH264ChromaFormatIdc, crate
    ::extensions::h264std::StdVideoH264PocType, crate
    ::extensions::h264std::StdVideoH264SpsFlags, crate
    ::extensions::h264std::StdVideoH264ScalingLists, crate
    ::extensions::h264std::StdVideoH264AspectRatioIdc, crate
    ::extensions::h264std::StdVideoH264HrdParameters, crate
    ::extensions::h264std::StdVideoH264SpsVuiFlags, crate
    ::extensions::h264std::StdVideoH264WeightedBipredIdc, crate
    ::extensions::h264std::StdVideoH264PpsFlags, crate
    ::extensions::h264std::StdVideoH264SliceType, crate
    ::extensions::h264std::StdVideoH264CabacInitIdc, crate
    ::extensions::h264std::StdVideoH264DisableDeblockingFilterIdc, crate
    ::extensions::h264std::StdVideoH264PictureType, crate
    ::extensions::h264std::StdVideoH264ModificationOfPicNumsIdc, crate
    ::extensions::h264std::StdVideoH264MemMgmtControlOp, crate
    ::extensions::h264std_decode::StdVideoDecodeH264PictureInfo, crate
    ::extensions::h264std_decode::StdVideoDecodeH264ReferenceInfo, crate
    ::extensions::h264std_decode::StdVideoDecodeH264PictureInfoFlags, crate
    ::extensions::h264std_decode::StdVideoDecodeH264ReferenceInfoFlags, crate
    ::extensions::h265std::StdVideoH265ProfileIdc, crate
    ::extensions::h265std::StdVideoH265DecPicBufMgr, crate
    ::extensions::h265std::StdVideoH265VpsFlags, crate
    ::extensions::h265std::StdVideoH265LevelIdc, crate
    ::extensions::h265std::StdVideoH265SpsFlags, crate
    ::extensions::h265std::StdVideoH265ScalingLists, crate
    ::extensions::h265std::StdVideoH265PredictorPaletteEntries, crate
    ::extensions::h265std::StdVideoH265PpsFlags, crate
    ::extensions::h265std::StdVideoH265SubLayerHrdParameters, crate
    ::extensions::h265std::StdVideoH265HrdFlags, crate
    ::extensions::h265std::StdVideoH265SpsVuiFlags, crate
    ::extensions::h265std::StdVideoH265SliceType, crate
    ::extensions::h265std::StdVideoH265PictureType, crate
    ::extensions::h265std_decode::StdVideoDecodeH265PictureInfo, crate
    ::extensions::h265std_decode::StdVideoDecodeH265ReferenceInfo, crate
    ::extensions::h265std_decode::StdVideoDecodeH265PictureInfoFlags, crate
    ::extensions::h265std_decode::StdVideoDecodeH265ReferenceInfoFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264PictureInfo, crate
    ::extensions::h264std_encode::StdVideoEncodeH264ReferenceInfo, crate
    ::extensions::h264std_encode::StdVideoEncodeH264SliceHeaderFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264PictureInfoFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264ReferenceInfoFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264RefMgmtFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264RefListModEntry, crate
    ::extensions::h264std_encode::StdVideoEncodeH264RefPicMarkingEntry, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264QpEXT, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264FrameSizeEXT, crate
    ::extensions::h265std_encode::StdVideoEncodeH265PictureInfoFlags, crate
    ::extensions::h265std_encode::StdVideoEncodeH265PictureInfo, crate
    ::extensions::h265std_encode::StdVideoEncodeH265ReferenceInfo, crate
    ::extensions::h265std_encode::StdVideoEncodeH265SliceSegmentHeaderFlags, crate
    ::extensions::h265std_encode::StdVideoEncodeH265ReferenceInfoFlags, crate
    ::extensions::h265std_encode::StdVideoEncodeH265ReferenceModificationFlags, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265QpEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265FrameSizeEXT, crate
    ::extensions::nv_ray_tracing_motion_blur::SRTDataNV, crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureSRTMotionInstanceNV,
    crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMatrixMotionInstanceNV,
    crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV,
    crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceNV,
    crate ::extensions::ext_image_drm_format_modifier::DrmFormatModifierProperties2EXT,
    crate ::extensions::ext_subpass_merge_feedback::RenderPassCreationFeedbackInfoEXT,
    crate ::extensions::ext_subpass_merge_feedback::RenderPassSubpassFeedbackInfoEXT,
    crate ::extensions::ext_opacity_micromap::MicromapUsageEXT, crate
    ::extensions::ext_opacity_micromap::MicromapTriangleEXT, crate
    ::extensions::ext_device_fault::DeviceFaultAddressInfoEXT, crate
    ::extensions::ext_device_fault::DeviceFaultVendorInfoEXT, crate
    ::extensions::ext_device_fault::DeviceFaultVendorBinaryHeaderVersionOneEXT, crate
    ::vk10::ImageLayout, crate ::vk10::AttachmentLoadOp, crate ::vk10::AttachmentStoreOp,
    crate ::vk10::ImageType, crate ::vk10::ImageTiling, crate ::vk10::ImageViewType,
    crate ::vk10::CommandBufferLevel, crate ::vk10::ComponentSwizzle, crate
    ::vk10::DescriptorType, crate ::vk10::QueryType, crate ::vk10::BorderColor, crate
    ::vk10::PipelineBindPoint, crate ::vk10::PipelineCacheHeaderVersion, crate
    ::vk10::PipelineCacheCreateFlags, crate ::vk10::PrimitiveTopology, crate
    ::vk10::SharingMode, crate ::vk10::IndexType, crate ::vk10::Filter, crate
    ::vk10::SamplerMipmapMode, crate ::vk10::SamplerAddressMode, crate ::vk10::CompareOp,
    crate ::vk10::PolygonMode, crate ::vk10::FrontFace, crate ::vk10::BlendFactor, crate
    ::vk10::BlendOp, crate ::vk10::StencilOp, crate ::vk10::LogicOp, crate
    ::vk10::InternalAllocationType, crate ::vk10::SystemAllocationScope, crate
    ::vk10::PhysicalDeviceType, crate ::vk10::VertexInputRate, crate ::vk10::Format,
    crate ::vk10::StructureType, crate ::vk10::SubpassContents, crate ::vk10::Result,
    crate ::vk10::DynamicState, crate ::vk11::DescriptorUpdateTemplateType, crate
    ::vk10::ObjectType, crate ::vk10::QueueFlags, crate ::vk10::CullModeFlags, crate
    ::vk10::RenderPassCreateFlags, crate ::vk10::DeviceQueueCreateFlags, crate
    ::vk10::MemoryPropertyFlags, crate ::vk10::MemoryHeapFlags, crate
    ::vk10::AccessFlags, crate ::vk10::BufferUsageFlags, crate ::vk10::BufferCreateFlags,
    crate ::vk10::ShaderStageFlags, crate ::vk10::ImageUsageFlags, crate
    ::vk10::ImageCreateFlags, crate ::vk10::ImageViewCreateFlags, crate
    ::vk10::SamplerCreateFlags, crate ::vk10::PipelineCreateFlags, crate
    ::vk10::PipelineShaderStageCreateFlags, crate ::vk10::ColorComponentFlags, crate
    ::vk10::FenceCreateFlags, crate ::vk10::FormatFeatureFlags, crate
    ::vk10::QueryControlFlags, crate ::vk10::QueryResultFlags, crate
    ::vk10::CommandBufferUsageFlags, crate ::vk10::QueryPipelineStatisticFlags, crate
    ::vk10::ImageAspectFlags, crate ::vk10::SparseImageFormatFlags, crate
    ::vk10::SparseMemoryBindFlags, crate ::vk10::PipelineStageFlags, crate
    ::vk10::CommandPoolCreateFlags, crate ::vk10::CommandPoolResetFlags, crate
    ::vk10::CommandBufferResetFlags, crate ::vk10::SampleCountFlags, crate
    ::vk10::AttachmentDescriptionFlags, crate ::vk10::StencilFaceFlags, crate
    ::vk10::DescriptorPoolCreateFlags, crate ::vk10::DependencyFlags, crate
    ::vk12::SemaphoreType, crate ::vk12::SemaphoreWaitFlags, crate
    ::extensions::khr_surface::PresentModeKHR, crate
    ::extensions::khr_surface::ColorSpaceKHR, crate
    ::extensions::khr_display::DisplayPlaneAlphaFlagsKHR, crate
    ::extensions::khr_surface::CompositeAlphaFlagsKHR, crate
    ::extensions::khr_surface::SurfaceTransformFlagsKHR, crate
    ::extensions::ext_calibrated_timestamps::TimeDomainEXT, crate
    ::extensions::ext_debug_report::DebugReportFlagsEXT, crate
    ::extensions::ext_debug_report::DebugReportObjectTypeEXT, crate
    ::extensions::ext_device_memory_report::DeviceMemoryReportEventTypeEXT, crate
    ::extensions::amd_rasterization_order::RasterizationOrderAMD, crate
    ::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV, crate
    ::extensions::nv_external_memory_capabilities::ExternalMemoryFeatureFlagsNV, crate
    ::extensions::ext_validation_flags::ValidationCheckEXT, crate
    ::extensions::ext_validation_features::ValidationFeatureEnableEXT, crate
    ::extensions::ext_validation_features::ValidationFeatureDisableEXT, crate
    ::vk11::SubgroupFeatureFlags, crate
    ::extensions::nv_device_generated_commands::IndirectCommandsLayoutUsageFlagsNV, crate
    ::extensions::nv_device_generated_commands::IndirectStateFlagsNV, crate
    ::extensions::nv_device_generated_commands::IndirectCommandsTokenTypeNV, crate
    ::vk10::DescriptorSetLayoutCreateFlags, crate ::vk11::ExternalMemoryHandleTypeFlags,
    crate ::vk11::ExternalMemoryFeatureFlags, crate
    ::vk11::ExternalSemaphoreHandleTypeFlags, crate
    ::vk11::ExternalSemaphoreFeatureFlags, crate ::vk11::SemaphoreImportFlags, crate
    ::vk11::ExternalFenceHandleTypeFlags, crate ::vk11::ExternalFenceFeatureFlags, crate
    ::vk11::FenceImportFlags, crate
    ::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT, crate
    ::extensions::ext_display_control::DisplayPowerStateEXT, crate
    ::extensions::ext_display_control::DeviceEventTypeEXT, crate
    ::extensions::ext_display_control::DisplayEventTypeEXT, crate
    ::vk11::PeerMemoryFeatureFlags, crate ::vk11::MemoryAllocateFlags, crate
    ::extensions::khr_swapchain::DeviceGroupPresentModeFlagsKHR, crate
    ::extensions::khr_swapchain::SwapchainCreateFlagsKHR, crate
    ::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV, crate
    ::extensions::ext_discard_rectangles::DiscardRectangleModeEXT, crate
    ::vk10::SubpassDescriptionFlags, crate ::vk11::PointClippingBehavior, crate
    ::vk12::SamplerReductionMode, crate ::vk11::TessellationDomainOrigin, crate
    ::vk11::SamplerYcbcrModelConversion, crate ::vk11::SamplerYcbcrRange, crate
    ::vk11::ChromaLocation, crate
    ::extensions::ext_blend_operation_advanced::BlendOverlapEXT, crate
    ::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV, crate
    ::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV, crate
    ::extensions::ext_validation_cache::ValidationCacheHeaderVersionEXT, crate
    ::extensions::amd_shader_info::ShaderInfoTypeAMD, crate
    ::extensions::khr_global_priority::QueueGlobalPriorityKHR, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT, crate
    ::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT, crate
    ::extensions::ext_conservative_rasterization::ConservativeRasterizationModeEXT, crate
    ::vk12::DescriptorBindingFlags, crate ::vk10::VendorId, crate ::vk12::DriverId, crate
    ::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT, crate
    ::vk12::ResolveModeFlags, crate
    ::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV, crate
    ::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV, crate
    ::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR, crate
    ::extensions::khr_acceleration_structure::GeometryFlagsKHR, crate
    ::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagsKHR, crate
    ::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR, crate
    ::extensions::khr_acceleration_structure::BuildAccelerationStructureModeKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR, crate
    ::extensions::khr_acceleration_structure::GeometryTypeKHR, crate
    ::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureBuildTypeKHR, crate
    ::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR, crate
    ::extensions::khr_acceleration_structure::AccelerationStructureCompatibilityKHR,
    crate ::extensions::khr_ray_tracing_pipeline::ShaderGroupShaderKHR, crate
    ::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD,
    crate ::vk10::FramebufferCreateFlags, crate
    ::extensions::nv_cooperative_matrix::ScopeNV, crate
    ::extensions::nv_cooperative_matrix::ComponentTypeNV, crate
    ::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV, crate
    ::vk13::PipelineCreationFeedbackFlags, crate
    ::extensions::ext_full_screen_exclusive::FullScreenExclusiveEXT, crate
    ::extensions::khr_performance_query::PerformanceCounterScopeKHR, crate
    ::extensions::khr_performance_query::PerformanceCounterUnitKHR, crate
    ::extensions::khr_performance_query::PerformanceCounterStorageKHR, crate
    ::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR, crate
    ::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR, crate
    ::extensions::amd_shader_core_properties2::ShaderCorePropertiesFlagsAMD, crate
    ::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL, crate
    ::extensions::intel_performance_query::QueryPoolSamplingModeINTEL, crate
    ::extensions::intel_performance_query::PerformanceOverrideTypeINTEL, crate
    ::extensions::intel_performance_query::PerformanceParameterTypeINTEL, crate
    ::extensions::intel_performance_query::PerformanceValueTypeINTEL, crate
    ::vk12::ShaderFloatControlsIndependence, crate
    ::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticFormatKHR,
    crate ::extensions::ext_line_rasterization::LineRasterizationModeEXT, crate
    ::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD, crate
    ::vk13::ToolPurposeFlags, crate
    ::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR, crate
    ::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV, crate
    ::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateTypeNV, crate
    ::extensions::ext_subpass_merge_feedback::SubpassMergeStatusEXT, crate
    ::vk13::AccessFlags2, crate ::vk13::PipelineStageFlags2, crate ::vk13::SubmitFlags,
    crate ::vk10::EventCreateFlags, crate ::vk10::PipelineLayoutCreateFlags, crate
    ::extensions::ext_provoking_vertex::ProvokingVertexModeEXT, crate
    ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceTypeNV,
    crate ::vk10::PipelineColorBlendStateCreateFlags, crate
    ::vk10::PipelineDepthStencilStateCreateFlags, crate
    ::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryFlagsEXT, crate
    ::extensions::ext_device_address_binding_report::DeviceAddressBindingFlagsEXT, crate
    ::extensions::ext_device_address_binding_report::DeviceAddressBindingTypeEXT, crate
    ::extensions::khr_video_queue::VideoCodecOperationFlagsKHR, crate
    ::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR, crate
    ::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR, crate
    ::extensions::khr_video_queue::VideoCapabilityFlagsKHR, crate
    ::extensions::khr_video_queue::VideoSessionCreateFlagsKHR, crate
    ::extensions::ext_video_decode_h264::VideoDecodeH264PictureLayoutFlagsEXT, crate
    ::extensions::khr_video_queue::VideoCodingControlFlagsKHR, crate
    ::extensions::khr_video_queue::QueryResultStatusKHR, crate
    ::extensions::khr_video_decode_queue::VideoDecodeUsageFlagsKHR, crate
    ::extensions::khr_video_decode_queue::VideoDecodeCapabilityFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeUsageFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeContentFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeTuningModeKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeCapabilityFlagsKHR, crate
    ::extensions::khr_video_encode_queue::VideoEncodeRateControlModeFlagsKHR, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264CapabilityFlagsEXT, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264InputModeFlagsEXT, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264OutputModeFlagsEXT, crate
    ::extensions::ext_video_encode_h264::VideoEncodeH264RateControlStructureEXT, crate
    ::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFlagsFUCHSIA, crate
    ::vk13::FormatFeatureFlags2, crate ::vk13::RenderingFlags, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265CapabilityFlagsEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265InputModeFlagsEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265OutputModeFlagsEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265RateControlStructureEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265CtbSizeFlagsEXT, crate
    ::extensions::ext_video_encode_h265::VideoEncodeH265TransformBlockSizeFlagsEXT, crate
    ::extensions::ext_metal_objects::ExportMetalObjectTypeFlagsEXT, crate
    ::vk10::InstanceCreateFlags, crate
    ::extensions::ext_image_compression_control::ImageCompressionFlagsEXT, crate
    ::extensions::ext_image_compression_control::ImageCompressionFixedRateFlagsEXT, crate
    ::extensions::ext_pipeline_robustness::PipelineRobustnessBufferBehaviorEXT, crate
    ::extensions::ext_pipeline_robustness::PipelineRobustnessImageBehaviorEXT, crate
    ::extensions::nv_optical_flow::OpticalFlowGridSizeFlagsNV, crate
    ::extensions::nv_optical_flow::OpticalFlowUsageFlagsNV, crate
    ::extensions::nv_optical_flow::OpticalFlowPerformanceLevelNV, crate
    ::extensions::nv_optical_flow::OpticalFlowSessionBindingPointNV, crate
    ::extensions::nv_optical_flow::OpticalFlowSessionCreateFlagsNV, crate
    ::extensions::nv_optical_flow::OpticalFlowExecuteFlagsNV, crate
    ::extensions::ext_opacity_micromap::MicromapTypeEXT, crate
    ::extensions::ext_opacity_micromap::BuildMicromapFlagsEXT, crate
    ::extensions::ext_opacity_micromap::MicromapCreateFlagsEXT, crate
    ::extensions::ext_opacity_micromap::CopyMicromapModeEXT, crate
    ::extensions::ext_opacity_micromap::BuildMicromapModeEXT, crate
    ::extensions::ext_opacity_micromap::OpacityMicromapFormatEXT, crate
    ::extensions::ext_opacity_micromap::OpacityMicromapSpecialIndexEXT, crate
    ::extensions::ext_device_fault::DeviceFaultAddressTypeEXT, crate
    ::extensions::ext_device_fault::DeviceFaultVendorBinaryHeaderVersionEXT, crate
    ::extensions::h264std_encode::StdVideoEncodeH264WeightTableFlags, crate
    ::extensions::h264std_encode::StdVideoEncodeH264WeightTable, crate
    ::extensions::h265std::StdVideoH265ProfileTierLevelFlags, crate
    ::extensions::h265std::StdVideoH265ProfileTierLevel, crate
    ::extensions::h265std::StdVideoH265ShortTermRefPicSetFlags, crate
    ::extensions::h265std::StdVideoH265ShortTermRefPicSet, crate
    ::extensions::h265std::StdVideoH265LongTermRefPicsSps, crate
    ::extensions::h265std_encode::StdVideoEncodeH265WeightTableFlags, crate
    ::extensions::h265std_encode::StdVideoEncodeH265WeightTable, crate
    ::extensions::h265std_encode::StdVideoEncodeH265SliceSegmentLongTermRefPics, crate
    ::extensions::h264std::StdVideoH264NonVclNaluType, crate
    ::extensions::h264std_decode::StdVideoDecodeH264FieldOrderCount, crate
    ::extensions::h265std::StdVideoH265ChromaFormatIdc, crate
    ::extensions::h265std::StdVideoH265AspectRatioIdc
}
