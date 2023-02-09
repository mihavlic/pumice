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
impl DumbHash for crate::vk10::BaseOutStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_ptr(self.p_next, state);
        }
    }
}
impl DumbHash for crate::vk10::BaseInStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_ptr(self.p_next, state);
        }
    }
}
impl DumbHash for crate::vk10::Viewport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.x.hash(state);
            self.y.hash(state);
            self.width.hash(state);
            self.height.hash(state);
            self.min_depth.hash(state);
            self.max_depth.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PhysicalDeviceProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.api_version.hash(state);
            self.driver_version.hash(state);
            self.vendor_id.hash(state);
            self.device_id.hash(state);
            self.device_type.hash(state);
            self.device_name.hash(state);
            self.pipeline_cache_uuid.hash(state);
            self.limits.hash(state);
            self.sparse_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ApplicationInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_cstr(self.p_application_name, state);
            self.application_version.hash(state);
            hash_cstr(self.p_engine_name, state);
            self.engine_version.hash(state);
            self.api_version.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::AllocationCallbacks {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.p_user_data.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_allocation),
                state,
            );
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_reallocation),
                state,
            );
            std::ptr::hash(std::mem::transmute::<_, *const ()>(self.pfn_free), state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_internal_allocation),
                state,
            );
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_internal_free),
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::DeviceQueueCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
            self.queue_count.hash(state);
            hash_raw_arr(self.p_queue_priorities, (self.queue_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DeviceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_create_info_count.hash(state);
            hash_raw_arr(
                self.p_queue_create_infos,
                (self.queue_create_info_count) as usize,
                state,
            );
            self.enabled_layer_count.hash(state);
            let len = (self.enabled_layer_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                hash_cstr(ptr, state);
            }
            self.enabled_extension_count.hash(state);
            let len = (self.enabled_extension_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                hash_cstr(ptr, state);
            }
            hash_ptr(self.p_enabled_features, state);
        }
    }
}
impl DumbHash for crate::vk10::InstanceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_ptr(self.p_application_info, state);
            self.enabled_layer_count.hash(state);
            let len = (self.enabled_layer_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_layer_names.add(i);
                hash_cstr(ptr, state);
            }
            self.enabled_extension_count.hash(state);
            let len = (self.enabled_extension_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_enabled_extension_names.add(i);
                hash_cstr(ptr, state);
            }
        }
    }
}
impl DumbHash for crate::vk10::MemoryAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.allocation_size.hash(state);
            self.memory_type_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::MappedMemoryRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::WriteDescriptorSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dst_set.hash(state);
            self.dst_binding.hash(state);
            self.dst_array_element.hash(state);
            self.descriptor_count.hash(state);
            self.descriptor_type.hash(state);
            hash_raw_arr(self.p_image_info, (self.descriptor_count) as usize, state);
            hash_raw_arr(self.p_buffer_info, (self.descriptor_count) as usize, state);
            hash_raw_arr(
                self.p_texel_buffer_view,
                (self.descriptor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::CopyDescriptorSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_set.hash(state);
            self.src_binding.hash(state);
            self.src_array_element.hash(state);
            self.dst_set.hash(state);
            self.dst_binding.hash(state);
            self.dst_array_element.hash(state);
            self.descriptor_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::BufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.size.hash(state);
            self.usage.hash(state);
            self.sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::BufferViewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.buffer.hash(state);
            self.format.hash(state);
            self.offset.hash(state);
            self.range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::MemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::BufferMemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageMemoryBarrier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.old_layout.hash(state);
            self.new_layout.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.image.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image_type.hash(state);
            self.format.hash(state);
            self.extent.hash(state);
            self.mip_levels.hash(state);
            self.array_layers.hash(state);
            self.samples.hash(state);
            self.tiling.hash(state);
            self.usage.hash(state);
            self.sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
            self.initial_layout.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ImageViewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image.hash(state);
            self.view_type.hash(state);
            self.format.hash(state);
            self.components.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SparseBufferMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.buffer.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::SparseImageOpaqueMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.image.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::SparseImageMemoryBindInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.image.hash(state);
            self.bind_count.hash(state);
            hash_raw_arr(self.p_binds, (self.bind_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::BindSparseInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.buffer_bind_count.hash(state);
            hash_raw_arr(self.p_buffer_binds, (self.buffer_bind_count) as usize, state);
            self.image_opaque_bind_count.hash(state);
            hash_raw_arr(
                self.p_image_opaque_binds,
                (self.image_opaque_bind_count) as usize,
                state,
            );
            self.image_bind_count.hash(state);
            hash_raw_arr(self.p_image_binds, (self.image_bind_count) as usize, state);
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::ShaderModuleCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.code_size.hash(state);
            hash_raw_arr(self.p_code, (self.code_size / 4) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetLayoutBinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.binding.hash(state);
            self.descriptor_type.hash(state);
            self.descriptor_count.hash(state);
            self.stage_flags.hash(state);
            hash_raw_arr(
                self.p_immutable_samplers,
                (self.descriptor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetLayoutCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.binding_count.hash(state);
            hash_raw_arr(self.p_bindings, (self.binding_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.max_sets.hash(state);
            self.pool_size_count.hash(state);
            hash_raw_arr(self.p_pool_sizes, (self.pool_size_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::DescriptorSetAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_pool.hash(state);
            self.descriptor_set_count.hash(state);
            hash_raw_arr(
                self.p_set_layouts,
                (self.descriptor_set_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::SpecializationInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.map_entry_count.hash(state);
            hash_raw_arr(self.p_map_entries, (self.map_entry_count) as usize, state);
            self.data_size.hash(state);
            hash_raw_arr(self.p_data.cast::<u8>(), (self.data_size) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineShaderStageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage.hash(state);
            self.module.hash(state);
            hash_cstr(self.p_name, state);
            hash_ptr(self.p_specialization_info, state);
        }
    }
}
impl DumbHash for crate::vk10::ComputePipelineCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage.hash(state);
            self.layout.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineVertexInputStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.vertex_binding_description_count.hash(state);
            hash_raw_arr(
                self.p_vertex_binding_descriptions,
                (self.vertex_binding_description_count) as usize,
                state,
            );
            self.vertex_attribute_description_count.hash(state);
            hash_raw_arr(
                self.p_vertex_attribute_descriptions,
                (self.vertex_attribute_description_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineInputAssemblyStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.topology.hash(state);
            self.primitive_restart_enable.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineTessellationStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.patch_control_points.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineViewportStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.viewport_count.hash(state);
            hash_raw_arr(self.p_viewports, (self.viewport_count) as usize, state);
            self.scissor_count.hash(state);
            hash_raw_arr(self.p_scissors, (self.scissor_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineRasterizationStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.depth_clamp_enable.hash(state);
            self.rasterizer_discard_enable.hash(state);
            self.polygon_mode.hash(state);
            self.cull_mode.hash(state);
            self.front_face.hash(state);
            self.depth_bias_enable.hash(state);
            self.depth_bias_constant_factor.hash(state);
            self.depth_bias_clamp.hash(state);
            self.depth_bias_slope_factor.hash(state);
            self.line_width.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineMultisampleStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.rasterization_samples.hash(state);
            self.sample_shading_enable.hash(state);
            self.min_sample_shading.hash(state);
            hash_raw_arr(
                self.p_sample_mask,
                ((self.rasterization_samples.0 + 31) / 32) as usize,
                state,
            );
            self.alpha_to_coverage_enable.hash(state);
            self.alpha_to_one_enable.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineColorBlendStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.logic_op_enable.hash(state);
            self.logic_op.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.blend_constants.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineDynamicStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dynamic_state_count.hash(state);
            hash_raw_arr(
                self.p_dynamic_states,
                (self.dynamic_state_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineDepthStencilStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.depth_test_enable.hash(state);
            self.depth_write_enable.hash(state);
            self.depth_compare_op.hash(state);
            self.depth_bounds_test_enable.hash(state);
            self.stencil_test_enable.hash(state);
            self.front.hash(state);
            self.back.hash(state);
            self.min_depth_bounds.hash(state);
            self.max_depth_bounds.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::GraphicsPipelineCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage_count.hash(state);
            hash_raw_arr(self.p_stages, (self.stage_count) as usize, state);
            hash_ptr(self.p_vertex_input_state, state);
            hash_ptr(self.p_input_assembly_state, state);
            hash_ptr(self.p_tessellation_state, state);
            hash_ptr(self.p_viewport_state, state);
            hash_ptr(self.p_rasterization_state, state);
            hash_ptr(self.p_multisample_state, state);
            hash_ptr(self.p_depth_stencil_state, state);
            hash_ptr(self.p_color_blend_state, state);
            hash_ptr(self.p_dynamic_state, state);
            self.layout.hash(state);
            self.render_pass.hash(state);
            self.subpass.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PipelineCacheCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.initial_data_size.hash(state);
            hash_raw_arr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::PipelineLayoutCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.set_layout_count.hash(state);
            hash_raw_arr(self.p_set_layouts, (self.set_layout_count) as usize, state);
            self.push_constant_range_count.hash(state);
            hash_raw_arr(
                self.p_push_constant_ranges,
                (self.push_constant_range_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::SamplerCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.mag_filter.hash(state);
            self.min_filter.hash(state);
            self.mipmap_mode.hash(state);
            self.address_mode_u.hash(state);
            self.address_mode_v.hash(state);
            self.address_mode_w.hash(state);
            self.mip_lod_bias.hash(state);
            self.anisotropy_enable.hash(state);
            self.max_anisotropy.hash(state);
            self.compare_enable.hash(state);
            self.compare_op.hash(state);
            self.min_lod.hash(state);
            self.max_lod.hash(state);
            self.border_color.hash(state);
            self.unnormalized_coordinates.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.command_pool.hash(state);
            self.level.hash(state);
            self.command_buffer_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferInheritanceInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.render_pass.hash(state);
            self.subpass.hash(state);
            self.framebuffer.hash(state);
            self.occlusion_query_enable.hash(state);
            self.query_flags.hash(state);
            self.pipeline_statistics.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::CommandBufferBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_ptr(self.p_inheritance_info, state);
        }
    }
}
impl DumbHash for crate::vk10::RenderPassBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.render_pass.hash(state);
            self.framebuffer.hash(state);
            self.render_area.hash(state);
            self.clear_value_count.hash(state);
            hash_raw_arr(self.p_clear_values, (self.clear_value_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::ClearColorValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearDepthStencilValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.depth.hash(state);
            self.stencil.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::vk10::ClearAttachment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.aspect_mask.hash(state);
            self.color_attachment.hash(state);
            self.clear_value.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SubpassDescription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.pipeline_bind_point.hash(state);
            self.input_attachment_count.hash(state);
            hash_raw_arr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
                state,
            );
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_stencil_attachment, state);
            self.preserve_attachment_count.hash(state);
            hash_raw_arr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk10::RenderPassCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_subpasses, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_dependencies, (self.dependency_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk10::EventCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::FenceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::PhysicalDeviceLimits {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.max_image_dimension_1_d.hash(state);
            self.max_image_dimension_2_d.hash(state);
            self.max_image_dimension_3_d.hash(state);
            self.max_image_dimension_cube.hash(state);
            self.max_image_array_layers.hash(state);
            self.max_texel_buffer_elements.hash(state);
            self.max_uniform_buffer_range.hash(state);
            self.max_storage_buffer_range.hash(state);
            self.max_push_constants_size.hash(state);
            self.max_memory_allocation_count.hash(state);
            self.max_sampler_allocation_count.hash(state);
            self.buffer_image_granularity.hash(state);
            self.sparse_address_space_size.hash(state);
            self.max_bound_descriptor_sets.hash(state);
            self.max_per_stage_descriptor_samplers.hash(state);
            self.max_per_stage_descriptor_uniform_buffers.hash(state);
            self.max_per_stage_descriptor_storage_buffers.hash(state);
            self.max_per_stage_descriptor_sampled_images.hash(state);
            self.max_per_stage_descriptor_storage_images.hash(state);
            self.max_per_stage_descriptor_input_attachments.hash(state);
            self.max_per_stage_resources.hash(state);
            self.max_descriptor_set_samplers.hash(state);
            self.max_descriptor_set_uniform_buffers.hash(state);
            self.max_descriptor_set_uniform_buffers_dynamic.hash(state);
            self.max_descriptor_set_storage_buffers.hash(state);
            self.max_descriptor_set_storage_buffers_dynamic.hash(state);
            self.max_descriptor_set_sampled_images.hash(state);
            self.max_descriptor_set_storage_images.hash(state);
            self.max_descriptor_set_input_attachments.hash(state);
            self.max_vertex_input_attributes.hash(state);
            self.max_vertex_input_bindings.hash(state);
            self.max_vertex_input_attribute_offset.hash(state);
            self.max_vertex_input_binding_stride.hash(state);
            self.max_vertex_output_components.hash(state);
            self.max_tessellation_generation_level.hash(state);
            self.max_tessellation_patch_size.hash(state);
            self.max_tessellation_control_per_vertex_input_components.hash(state);
            self.max_tessellation_control_per_vertex_output_components.hash(state);
            self.max_tessellation_control_per_patch_output_components.hash(state);
            self.max_tessellation_control_total_output_components.hash(state);
            self.max_tessellation_evaluation_input_components.hash(state);
            self.max_tessellation_evaluation_output_components.hash(state);
            self.max_geometry_shader_invocations.hash(state);
            self.max_geometry_input_components.hash(state);
            self.max_geometry_output_components.hash(state);
            self.max_geometry_output_vertices.hash(state);
            self.max_geometry_total_output_components.hash(state);
            self.max_fragment_input_components.hash(state);
            self.max_fragment_output_attachments.hash(state);
            self.max_fragment_dual_src_attachments.hash(state);
            self.max_fragment_combined_output_resources.hash(state);
            self.max_compute_shared_memory_size.hash(state);
            self.max_compute_work_group_count.hash(state);
            self.max_compute_work_group_invocations.hash(state);
            self.max_compute_work_group_size.hash(state);
            self.sub_pixel_precision_bits.hash(state);
            self.sub_texel_precision_bits.hash(state);
            self.mipmap_precision_bits.hash(state);
            self.max_draw_indexed_index_value.hash(state);
            self.max_draw_indirect_count.hash(state);
            self.max_sampler_lod_bias.hash(state);
            self.max_sampler_anisotropy.hash(state);
            self.max_viewports.hash(state);
            self.max_viewport_dimensions.hash(state);
            self.viewport_bounds_range.hash(state);
            self.viewport_sub_pixel_bits.hash(state);
            self.min_memory_map_alignment.hash(state);
            self.min_texel_buffer_offset_alignment.hash(state);
            self.min_uniform_buffer_offset_alignment.hash(state);
            self.min_storage_buffer_offset_alignment.hash(state);
            self.min_texel_offset.hash(state);
            self.max_texel_offset.hash(state);
            self.min_texel_gather_offset.hash(state);
            self.max_texel_gather_offset.hash(state);
            self.min_interpolation_offset.hash(state);
            self.max_interpolation_offset.hash(state);
            self.sub_pixel_interpolation_offset_bits.hash(state);
            self.max_framebuffer_width.hash(state);
            self.max_framebuffer_height.hash(state);
            self.max_framebuffer_layers.hash(state);
            self.framebuffer_color_sample_counts.hash(state);
            self.framebuffer_depth_sample_counts.hash(state);
            self.framebuffer_stencil_sample_counts.hash(state);
            self.framebuffer_no_attachments_sample_counts.hash(state);
            self.max_color_attachments.hash(state);
            self.sampled_image_color_sample_counts.hash(state);
            self.sampled_image_integer_sample_counts.hash(state);
            self.sampled_image_depth_sample_counts.hash(state);
            self.sampled_image_stencil_sample_counts.hash(state);
            self.storage_image_sample_counts.hash(state);
            self.max_sample_mask_words.hash(state);
            self.timestamp_compute_and_graphics.hash(state);
            self.timestamp_period.hash(state);
            self.max_clip_distances.hash(state);
            self.max_cull_distances.hash(state);
            self.max_combined_clip_and_cull_distances.hash(state);
            self.discrete_queue_priorities.hash(state);
            self.point_size_range.hash(state);
            self.line_width_range.hash(state);
            self.point_size_granularity.hash(state);
            self.line_width_granularity.hash(state);
            self.strict_lines.hash(state);
            self.standard_sample_locations.hash(state);
            self.optimal_buffer_copy_offset_alignment.hash(state);
            self.optimal_buffer_copy_row_pitch_alignment.hash(state);
            self.non_coherent_atom_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SemaphoreCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::QueryPoolCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.query_type.hash(state);
            self.query_count.hash(state);
            self.pipeline_statistics.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::FramebufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.render_pass.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.width.hash(state);
            self.height.hash(state);
            self.layers.hash(state);
        }
    }
}
impl DumbHash for crate::vk10::SubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_wait_dst_stage_mask,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.command_buffer_count.hash(state);
            hash_raw_arr(
                self.p_command_buffers,
                (self.command_buffer_count) as usize,
                state,
            );
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphores,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_display::DisplayPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.display.hash(state);
            hash_cstr(self.display_name, state);
            self.physical_dimensions.hash(state);
            self.physical_resolution.hash(state);
            self.supported_transforms.hash(state);
            self.plane_reorder_possible.hash(state);
            self.persistent_content.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_display::DisplayModeCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.parameters.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_display::DisplaySurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.display_mode.hash(state);
            self.plane_index.hash(state);
            self.plane_stack_index.hash(state);
            self.transform.hash(state);
            self.global_alpha.hash(state);
            self.alpha_mode.hash(state);
            self.image_extent.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_display_swapchain::DisplayPresentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_rect.hash(state);
            self.dst_rect.hash(state);
            self.persistent.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nn_vi_surface::ViSurfaceCreateInfoNN {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.display.hash(state);
            self.surface.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.hinstance.hash(state);
            self.hwnd.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dpy.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.connection.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.dfb.hash(state);
            self.surface.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.image_pipe_handle.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stream_descriptor.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.context.hash(state);
            self.window.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::SwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.surface.hash(state);
            self.min_image_count.hash(state);
            self.image_format.hash(state);
            self.image_color_space.hash(state);
            self.image_extent.hash(state);
            self.image_array_layers.hash(state);
            self.image_usage.hash(state);
            self.image_sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
            self.pre_transform.hash(state);
            self.composite_alpha.hash(state);
            self.present_mode.hash(state);
            self.clipped.hash(state);
            self.old_swapchain.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::PresentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphores,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_swapchains, (self.swapchain_count) as usize, state);
            hash_raw_arr(self.p_image_indices, (self.swapchain_count) as usize, state);
            hash_raw_arr(self.p_results, (self.swapchain_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_callback),
                state,
            );
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_validation_flags::ValidationFlagsEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.disabled_validation_check_count.hash(state);
            hash_raw_arr(
                self.p_disabled_validation_checks,
                (self.disabled_validation_check_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::ext_validation_features::ValidationFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.enabled_validation_feature_count.hash(state);
            hash_raw_arr(
                self.p_enabled_validation_features,
                (self.enabled_validation_feature_count) as usize,
                state,
            );
            self.disabled_validation_feature_count.hash(state);
            hash_raw_arr(
                self.p_disabled_validation_features,
                (self.disabled_validation_feature_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::amd_rasterization_order::PipelineRasterizationStateRasterizationOrderAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.rasterization_order.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object.hash(state);
            hash_cstr(self.p_object_name, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object.hash(state);
            self.tag_name.hash(state);
            self.tag_size.hash(state);
            hash_raw_arr(self.p_tag.cast::<u8>(), (self.tag_size) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_cstr(self.p_marker_name, state);
            self.color.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationImageCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dedicated_allocation.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationBufferCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dedicated_allocation.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_dedicated_allocation::DedicatedAllocationMemoryAllocateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_external_memory::ExternalMemoryImageCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_external_memory::ExportMemoryAllocateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_external_memory_win32::ImportMemoryWin32HandleInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
            self.handle.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_external_memory_win32::ExportMemoryWin32HandleInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.p_attributes.hash(state);
            self.dw_access.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acquire_count.hash(state);
            hash_raw_arr(self.p_acquire_syncs, (self.acquire_count) as usize, state);
            hash_raw_arr(self.p_acquire_keys, (self.acquire_count) as usize, state);
            hash_raw_arr(
                self.p_acquire_timeout_milliseconds,
                (self.acquire_count) as usize,
                state,
            );
            self.release_count.hash(state);
            hash_raw_arr(self.p_release_syncs, (self.release_count) as usize, state);
            hash_raw_arr(self.p_release_keys, (self.release_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_generated_commands.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::DevicePrivateDataCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.private_data_slot_request_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PrivateDataSlotCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDevicePrivateDataFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.private_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_graphics_shader_group_count.hash(state);
            self.max_indirect_sequence_count.hash(state);
            self.max_indirect_commands_token_count.hash(state);
            self.max_indirect_commands_stream_count.hash(state);
            self.max_indirect_commands_token_offset.hash(state);
            self.max_indirect_commands_stream_stride.hash(state);
            self.min_sequences_count_buffer_offset_alignment.hash(state);
            self.min_sequences_index_buffer_offset_alignment.hash(state);
            self.min_indirect_commands_buffer_offset_alignment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_multi_draw_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stage_count.hash(state);
            hash_raw_arr(self.p_stages, (self.stage_count) as usize, state);
            hash_ptr(self.p_vertex_input_state, state);
            hash_ptr(self.p_tessellation_state, state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::GraphicsPipelineShaderGroupsCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.group_count.hash(state);
            hash_raw_arr(self.p_groups, (self.group_count) as usize, state);
            self.pipeline_count.hash(state);
            hash_raw_arr(self.p_pipelines, (self.pipeline_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.token_type.hash(state);
            self.stream.hash(state);
            self.offset.hash(state);
            self.vertex_binding_unit.hash(state);
            self.vertex_dynamic_stride.hash(state);
            self.pushconstant_pipeline_layout.hash(state);
            self.pushconstant_shader_stage_flags.hash(state);
            self.pushconstant_offset.hash(state);
            self.pushconstant_size.hash(state);
            self.indirect_state_flags.hash(state);
            self.index_type_count.hash(state);
            hash_raw_arr(self.p_index_types, (self.index_type_count) as usize, state);
            hash_raw_arr(
                self.p_index_type_values,
                (self.index_type_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.pipeline_bind_point.hash(state);
            self.token_count.hash(state);
            hash_raw_arr(self.p_tokens, (self.token_count) as usize, state);
            self.stream_count.hash(state);
            hash_raw_arr(self.p_stream_strides, (self.stream_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_bind_point.hash(state);
            self.pipeline.hash(state);
            self.indirect_commands_layout.hash(state);
            self.stream_count.hash(state);
            hash_raw_arr(self.p_streams, (self.stream_count) as usize, state);
            self.sequences_count.hash(state);
            self.preprocess_buffer.hash(state);
            self.preprocess_offset.hash(state);
            self.preprocess_size.hash(state);
            self.sequences_count_buffer.hash(state);
            self.sequences_count_offset.hash(state);
            self.sequences_index_buffer.hash(state);
            self.sequences_index_offset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_bind_point.hash(state);
            self.pipeline.hash(state);
            self.indirect_commands_layout.hash(state);
            self.max_sequences_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceFeatures2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::FormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageFormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_format_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceImageFormatInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.kind.hash(state);
            self.tiling.hash(state);
            self.usage.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::QueueFamilyProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.queue_family_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMemoryProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SparseImageFormatProperties2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSparseImageFormatInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.kind.hash(state);
            self.samples.hash(state);
            self.usage.hash(state);
            self.tiling.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_push_descriptors.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceDriverProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.driver_id.hash(state);
            self.driver_name.hash(state);
            self.driver_info.hash(state);
            self.conformance_version.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_incremental_present::PresentRegionsKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_regions, (self.swapchain_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_incremental_present::PresentRegionKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.rectangle_count.hash(state);
            hash_raw_arr(self.p_rectangles, (self.rectangle_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceVariablePointersFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.variable_pointers_storage_buffer.hash(state);
            self.variable_pointers.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalImageFormatInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalImageFormatProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalBufferInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.usage.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalBufferProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_memory_properties.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceIDProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_uuid.hash(state);
            self.driver_uuid.hash(state);
            self.device_luid.hash(state);
            self.device_node_mask.hash(state);
            self.device_luidvalid.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalMemoryImageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalMemoryBufferCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportMemoryAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_memory_win32::ImportMemoryWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
            self.handle.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_memory_win32::ExportMemoryWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.p_attributes.hash(state);
            self.dw_access.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_external_memory::ImportMemoryZirconHandleInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
            self.handle.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_type_bits.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_type_bits.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_memory_win32::MemoryGetWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_memory_fd::ImportMemoryFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
            self.fd.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_type_bits.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acquire_count.hash(state);
            hash_raw_arr(self.p_acquire_syncs, (self.acquire_count) as usize, state);
            hash_raw_arr(self.p_acquire_keys, (self.acquire_count) as usize, state);
            hash_raw_arr(self.p_acquire_timeouts, (self.acquire_count) as usize, state);
            self.release_count.hash(state);
            hash_raw_arr(self.p_release_syncs, (self.release_count) as usize, state);
            hash_raw_arr(self.p_release_keys, (self.release_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalSemaphoreInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalSemaphoreProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.export_from_imported_handle_types.hash(state);
            self.compatible_handle_types.hash(state);
            self.external_semaphore_features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportSemaphoreCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.flags.hash(state);
            self.handle_type.hash(state);
            self.handle.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_semaphore_win32::ExportSemaphoreWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.p_attributes.hash(state);
            self.dw_access.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_semaphore_win32::D3D12FenceSubmitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_values_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_values_count) as usize,
                state,
            );
            self.signal_semaphore_values_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_values_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.flags.hash(state);
            self.handle_type.hash(state);
            self.fd.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.flags.hash(state);
            self.handle_type.hash(state);
            self.zircon_handle.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceExternalFenceInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExternalFenceProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.export_from_imported_handle_types.hash(state);
            self.compatible_handle_types.hash(state);
            self.external_fence_features.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ExportFenceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_types.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_fence_win32::ImportFenceWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fence.hash(state);
            self.flags.hash(state);
            self.handle_type.hash(state);
            self.handle.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_fence_win32::ExportFenceWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.p_attributes.hash(state);
            self.dw_access.hash(state);
            self.name.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fence.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fence.hash(state);
            self.flags.hash(state);
            self.handle_type.hash(state);
            self.fd.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fence.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMultiviewFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.multiview.hash(state);
            self.multiview_geometry_shader.hash(state);
            self.multiview_tessellation_shader.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMultiviewProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_multiview_view_count.hash(state);
            self.max_multiview_instance_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::RenderPassMultiviewCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_view_masks, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_view_offsets, (self.dependency_count) as usize, state);
            self.correlation_mask_count.hash(state);
            hash_raw_arr(
                self.p_correlation_masks,
                (self.correlation_mask_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_image_count.hash(state);
            self.max_image_count.hash(state);
            self.current_extent.hash(state);
            self.min_image_extent.hash(state);
            self.max_image_extent.hash(state);
            self.max_image_array_layers.hash(state);
            self.supported_transforms.hash(state);
            self.current_transform.hash(state);
            self.supported_composite_alpha.hash(state);
            self.supported_usage_flags.hash(state);
            self.supported_surface_counters.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_display_control::DisplayPowerInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.power_state.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_display_control::DeviceEventInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_event.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_display_control::DisplayEventInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.display_event.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.surface_counters.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceGroupProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.physical_device_count.hash(state);
            self.physical_devices.hash(state);
            self.subset_allocation.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryAllocateFlagsInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindBufferMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindBufferMemoryDeviceGroupInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_index_count.hash(state);
            hash_raw_arr(
                self.p_device_indices,
                (self.device_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::BindImageMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindImageMemoryDeviceGroupInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_index_count.hash(state);
            hash_raw_arr(
                self.p_device_indices,
                (self.device_index_count) as usize,
                state,
            );
            self.split_instance_bind_region_count.hash(state);
            hash_raw_arr(
                self.p_split_instance_bind_regions,
                (self.split_instance_bind_region_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupRenderPassBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_mask.hash(state);
            self.device_render_area_count.hash(state);
            hash_raw_arr(
                self.p_device_render_areas,
                (self.device_render_area_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupCommandBufferBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_device_indices,
                (self.wait_semaphore_count) as usize,
                state,
            );
            self.command_buffer_count.hash(state);
            hash_raw_arr(
                self.p_command_buffer_device_masks,
                (self.command_buffer_count) as usize,
                state,
            );
            self.signal_semaphore_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_device_indices,
                (self.signal_semaphore_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupBindSparseInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.resource_device_index.hash(state);
            self.memory_device_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_mask.hash(state);
            self.modes.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
            self.image_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::AcquireNextImageInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain.hash(state);
            self.timeout.hash(state);
            self.semaphore.hash(state);
            self.fence.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupPresentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_device_masks, (self.swapchain_count) as usize, state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceGroupDeviceCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.physical_device_count.hash(state);
            hash_raw_arr(
                self.p_physical_devices,
                (self.physical_device_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.modes.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DescriptorUpdateTemplateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.descriptor_update_entry_count.hash(state);
            hash_raw_arr(
                self.p_descriptor_update_entries,
                (self.descriptor_update_entry_count) as usize,
                state,
            );
            self.template_type.hash(state);
            self.descriptor_set_layout.hash(state);
            self.pipeline_bind_point.hash(state);
            self.pipeline_layout.hash(state);
            self.set.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_hdr_metadata::XYColorEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.x.hash(state);
            self.y.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_id.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_present_id::PresentIdKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_present_ids, (self.swapchain_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_wait.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_hdr_metadata::HdrMetadataEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.display_primary_red.hash(state);
            self.display_primary_green.hash(state);
            self.display_primary_blue.hash(state);
            self.white_point.hash(state);
            self.max_luminance.hash(state);
            self.min_luminance.hash(state);
            self.max_content_light_level.hash(state);
            self.max_frame_average_light_level.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.local_dimming_support.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.local_dimming_enable.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::google_display_timing::PresentTimesInfoGOOGLE {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.swapchain_count.hash(state);
            hash_raw_arr(self.p_times, (self.swapchain_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_view.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_view.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.p_layer.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.xcoeff.hash(state);
            self.ycoeff.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_clip_space_w_scaling::PipelineViewportWScalingStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.viewport_wscaling_enable.hash(state);
            self.viewport_count.hash(state);
            hash_raw_arr(
                self.p_viewport_wscalings,
                (self.viewport_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.viewport_count.hash(state);
            hash_raw_arr(
                self.p_viewport_swizzles,
                (self.viewport_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_discard_rectangles.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.discard_rectangle_mode.hash(state);
            self.discard_rectangle_count.hash(state);
            hash_raw_arr(
                self.p_discard_rectangles,
                (self.discard_rectangle_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.per_view_position_all_components.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::RenderPassInputAttachmentAspectCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.aspect_reference_count.hash(state);
            hash_raw_arr(
                self.p_aspect_references,
                (self.aspect_reference_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.surface.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.surface_capabilities.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.surface_format.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_get_display_properties2::DisplayProperties2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.display_properties.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.display_plane_properties.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_get_display_properties2::DisplayModeProperties2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.display_mode_properties.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mode.hash(state);
            self.plane_index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.capabilities.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shared_present_supported_usage_flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDevice16BitStorageFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_buffer_16_bit_access.hash(state);
            self.uniform_and_storage_buffer_16_bit_access.hash(state);
            self.storage_push_constant_16.hash(state);
            self.storage_input_output_16.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSubgroupProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subgroup_size.hash(state);
            self.supported_stages.hash(state);
            self.supported_operations.hash(state);
            self.quad_operations_in_all_stages.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_subgroup_extended_types.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BufferMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::DeviceBufferMemoryRequirements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_create_info, state);
        }
    }
}
impl DumbHash for crate::vk11::ImageMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageSparseMemoryRequirementsInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::DeviceImageMemoryRequirements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_create_info, state);
            self.plane_aspect.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryRequirements2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_requirements.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SparseImageMemoryRequirements2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_requirements.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDevicePointClippingProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.point_clipping_behavior.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryDedicatedRequirements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.prefers_dedicated_allocation.hash(state);
            self.requires_dedicated_allocation.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::MemoryDedicatedAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImageViewUsageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.usage.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PipelineTessellationDomainOriginStateCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.domain_origin.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.conversion.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.ycbcr_model.hash(state);
            self.ycbcr_range.hash(state);
            self.components.hash(state);
            self.x_chroma_offset.hash(state);
            self.y_chroma_offset.hash(state);
            self.chroma_filter.hash(state);
            self.force_explicit_reconstruction.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::BindImagePlaneMemoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.plane_aspect.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ImagePlaneMemoryRequirementsInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.plane_aspect.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sampler_ycbcr_conversion.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::SamplerYcbcrConversionImageFormatProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.combined_image_sampler_descriptor_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_texture_gather_bias_lod::TextureLODGatherFormatPropertiesAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supports_texture_gather_lodbias_amd.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::ProtectedSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_submit.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProtectedMemoryFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_memory.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceProtectedMemoryProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.protected_no_fault.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DeviceQueueInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.queue_family_index.hash(state);
            self.queue_index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.coverage_to_color_enable.hash(state);
            self.coverage_to_color_location.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceSamplerFilterMinmaxProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.filter_minmax_single_component_formats.hash(state);
            self.filter_minmax_image_component_mapping.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_sample_locations::SampleLocationEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.x.hash(state);
            self.y.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_sample_locations::SampleLocationsInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sample_locations_per_pixel.hash(state);
            self.sample_location_grid_size.hash(state);
            self.sample_locations_count.hash(state);
            hash_raw_arr(
                self.p_sample_locations,
                (self.sample_locations_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::ext_sample_locations::AttachmentSampleLocationsEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.attachment_index.hash(state);
            self.sample_locations_info.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_sample_locations::SubpassSampleLocationsEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.subpass_index.hash(state);
            self.sample_locations_info.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_sample_locations::RenderPassSampleLocationsBeginInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment_initial_sample_locations_count.hash(state);
            hash_raw_arr(
                self.p_attachment_initial_sample_locations,
                (self.attachment_initial_sample_locations_count) as usize,
                state,
            );
            self.post_subpass_sample_locations_count.hash(state);
            hash_raw_arr(
                self.p_post_subpass_sample_locations,
                (self.post_subpass_sample_locations_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_sample_locations::PipelineSampleLocationsStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sample_locations_enable.hash(state);
            self.sample_locations_info.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sample_location_sample_counts.hash(state);
            self.max_sample_location_grid_size.hash(state);
            self.sample_location_coordinate_range.hash(state);
            self.sample_location_sub_pixel_bits.hash(state);
            self.variable_sample_locations.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_sample_locations::MultisamplePropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_sample_location_grid_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::SamplerReductionModeCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.reduction_mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.advanced_blend_coherent_operations.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.multi_draw.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.advanced_blend_max_color_attachments.hash(state);
            self.advanced_blend_independent_blend.hash(state);
            self.advanced_blend_non_premultiplied_src_color.hash(state);
            self.advanced_blend_non_premultiplied_dst_color.hash(state);
            self.advanced_blend_correlated_overlap.hash(state);
            self.advanced_blend_all_operations.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_blend_operation_advanced::PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_premultiplied.hash(state);
            self.dst_premultiplied.hash(state);
            self.blend_overlap.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceInlineUniformBlockFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.inline_uniform_block.hash(state);
            self.descriptor_binding_inline_uniform_block_update_after_bind.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceInlineUniformBlockProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_inline_uniform_block_size.hash(state);
            self.max_per_stage_descriptor_inline_uniform_blocks.hash(state);
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
                .hash(state);
            self.max_descriptor_set_inline_uniform_blocks.hash(state);
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::WriteDescriptorSetInlineUniformBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.data_size.hash(state);
            hash_raw_arr(self.p_data.cast::<u8>(), (self.data_size) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::DescriptorPoolInlineUniformBlockCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_inline_uniform_block_bindings.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.coverage_modulation_mode.hash(state);
            self.coverage_modulation_table_enable.hash(state);
            self.coverage_modulation_table_count.hash(state);
            hash_raw_arr(
                self.p_coverage_modulation_table,
                (self.coverage_modulation_table_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::ImageFormatListCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.view_format_count.hash(state);
            hash_raw_arr(self.p_view_formats, (self.view_format_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.initial_data_size.hash(state);
            hash_raw_arr(
                self.p_initial_data.cast::<u8>(),
                (self.initial_data_size) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_validation_cache::ShaderModuleValidationCacheCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.validation_cache.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceMaintenance3Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_per_set_descriptors.hash(state);
            self.max_memory_allocation_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceMaintenance4Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.maintenance_4.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceMaintenance4Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_buffer_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::DescriptorSetLayoutSupport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supported.hash(state);
        }
    }
}
impl DumbHash for crate::vk11::PhysicalDeviceShaderDrawParametersFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_draw_parameters.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceShaderFloat16Int8Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_float_16.hash(state);
            self.shader_int_8.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceFloatControlsProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.denorm_behavior_independence.hash(state);
            self.rounding_mode_independence.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_16.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_32.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_64.hash(state);
            self.shader_denorm_preserve_float_16.hash(state);
            self.shader_denorm_preserve_float_32.hash(state);
            self.shader_denorm_preserve_float_64.hash(state);
            self.shader_denorm_flush_to_zero_float_16.hash(state);
            self.shader_denorm_flush_to_zero_float_32.hash(state);
            self.shader_denorm_flush_to_zero_float_64.hash(state);
            self.shader_rounding_mode_rtefloat_16.hash(state);
            self.shader_rounding_mode_rtefloat_32.hash(state);
            self.shader_rounding_mode_rtefloat_64.hash(state);
            self.shader_rounding_mode_rtzfloat_16.hash(state);
            self.shader_rounding_mode_rtzfloat_32.hash(state);
            self.shader_rounding_mode_rtzfloat_64.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceHostQueryResetFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.host_query_reset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.global_priority.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.global_priority_query.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.priority_count.hash(state);
            self.priorities.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object_handle.hash(state);
            hash_cstr(self.p_object_name, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.object_type.hash(state);
            self.object_handle.hash(state);
            self.tag_name.hash(state);
            self.tag_size.hash(state);
            hash_raw_arr(self.p_tag.cast::<u8>(), (self.tag_size) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsLabelEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_cstr(self.p_label_name, state);
            self.color.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.message_severity.hash(state);
            self.message_type.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_user_callback),
                state,
            );
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            hash_cstr(self.p_message_id_name, state);
            self.message_id_number.hash(state);
            hash_cstr(self.p_message, state);
            self.queue_label_count.hash(state);
            hash_raw_arr(self.p_queue_labels, (self.queue_label_count) as usize, state);
            self.cmd_buf_label_count.hash(state);
            hash_raw_arr(
                self.p_cmd_buf_labels,
                (self.cmd_buf_label_count) as usize,
                state,
            );
            self.object_count.hash(state);
            hash_raw_arr(self.p_objects, (self.object_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_memory_report.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_device_memory_report::DeviceDeviceMemoryReportCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            std::ptr::hash(
                std::mem::transmute::<_, *const ()>(self.pfn_user_callback),
                state,
            );
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.kind.hash(state);
            self.memory_object_id.hash(state);
            self.size.hash(state);
            self.object_type.hash(state);
            self.object_handle.hash(state);
            self.heap_index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_external_memory_host::ImportMemoryHostPointerInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.handle_type.hash(state);
            self.p_host_pointer.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_type_bits.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_imported_host_pointer_alignment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.primitive_overestimation_size.hash(state);
            self.max_extra_primitive_overestimation_size.hash(state);
            self.extra_primitive_overestimation_size_granularity.hash(state);
            self.primitive_underestimation.hash(state);
            self.conservative_point_and_line_rasterization.hash(state);
            self.degenerate_triangles_rasterized.hash(state);
            self.degenerate_lines_rasterized.hash(state);
            self.fully_covered_fragment_shader_input_variable.hash(state);
            self.conservative_rasterization_post_depth_coverage.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.time_domain.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_engine_count.hash(state);
            self.shader_arrays_per_engine_count.hash(state);
            self.compute_units_per_shader_array.hash(state);
            self.simd_per_compute_unit.hash(state);
            self.wavefronts_per_simd.hash(state);
            self.wavefront_size.hash(state);
            self.sgprs_per_simd.hash(state);
            self.min_sgpr_allocation.hash(state);
            self.max_sgpr_allocation.hash(state);
            self.sgpr_allocation_granularity.hash(state);
            self.vgprs_per_simd.hash(state);
            self.min_vgpr_allocation.hash(state);
            self.max_vgpr_allocation.hash(state);
            self.vgpr_allocation_granularity.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_core_features.hash(state);
            self.active_compute_unit_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.conservative_rasterization_mode.hash(state);
            self.extra_primitive_overestimation_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceDescriptorIndexingFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_input_attachment_array_dynamic_indexing.hash(state);
            self.shader_uniform_texel_buffer_array_dynamic_indexing.hash(state);
            self.shader_storage_texel_buffer_array_dynamic_indexing.hash(state);
            self.shader_uniform_buffer_array_non_uniform_indexing.hash(state);
            self.shader_sampled_image_array_non_uniform_indexing.hash(state);
            self.shader_storage_buffer_array_non_uniform_indexing.hash(state);
            self.shader_storage_image_array_non_uniform_indexing.hash(state);
            self.shader_input_attachment_array_non_uniform_indexing.hash(state);
            self.shader_uniform_texel_buffer_array_non_uniform_indexing.hash(state);
            self.shader_storage_texel_buffer_array_non_uniform_indexing.hash(state);
            self.descriptor_binding_uniform_buffer_update_after_bind.hash(state);
            self.descriptor_binding_sampled_image_update_after_bind.hash(state);
            self.descriptor_binding_storage_image_update_after_bind.hash(state);
            self.descriptor_binding_storage_buffer_update_after_bind.hash(state);
            self.descriptor_binding_uniform_texel_buffer_update_after_bind.hash(state);
            self.descriptor_binding_storage_texel_buffer_update_after_bind.hash(state);
            self.descriptor_binding_update_unused_while_pending.hash(state);
            self.descriptor_binding_partially_bound.hash(state);
            self.descriptor_binding_variable_descriptor_count.hash(state);
            self.runtime_descriptor_array.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceDescriptorIndexingProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_update_after_bind_descriptors_in_all_pools.hash(state);
            self.shader_uniform_buffer_array_non_uniform_indexing_native.hash(state);
            self.shader_sampled_image_array_non_uniform_indexing_native.hash(state);
            self.shader_storage_buffer_array_non_uniform_indexing_native.hash(state);
            self.shader_storage_image_array_non_uniform_indexing_native.hash(state);
            self.shader_input_attachment_array_non_uniform_indexing_native.hash(state);
            self.robust_buffer_access_update_after_bind.hash(state);
            self.quad_divergent_implicit_lod.hash(state);
            self.max_per_stage_descriptor_update_after_bind_samplers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_uniform_buffers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_storage_buffers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_sampled_images.hash(state);
            self.max_per_stage_descriptor_update_after_bind_storage_images.hash(state);
            self.max_per_stage_descriptor_update_after_bind_input_attachments
                .hash(state);
            self.max_per_stage_update_after_bind_resources.hash(state);
            self.max_descriptor_set_update_after_bind_samplers.hash(state);
            self.max_descriptor_set_update_after_bind_uniform_buffers.hash(state);
            self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
                .hash(state);
            self.max_descriptor_set_update_after_bind_storage_buffers.hash(state);
            self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
                .hash(state);
            self.max_descriptor_set_update_after_bind_sampled_images.hash(state);
            self.max_descriptor_set_update_after_bind_storage_images.hash(state);
            self.max_descriptor_set_update_after_bind_input_attachments.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::DescriptorSetLayoutBindingFlagsCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.binding_count.hash(state);
            hash_raw_arr(self.p_binding_flags, (self.binding_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk12::DescriptorSetVariableDescriptorCountAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_set_count.hash(state);
            hash_raw_arr(
                self.p_descriptor_counts,
                (self.descriptor_set_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::DescriptorSetVariableDescriptorCountLayoutSupport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_variable_descriptor_count.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::AttachmentDescription2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.format.hash(state);
            self.samples.hash(state);
            self.load_op.hash(state);
            self.store_op.hash(state);
            self.stencil_load_op.hash(state);
            self.stencil_store_op.hash(state);
            self.initial_layout.hash(state);
            self.final_layout.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::AttachmentReference2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment.hash(state);
            self.layout.hash(state);
            self.aspect_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::SubpassDescription2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.pipeline_bind_point.hash(state);
            self.view_mask.hash(state);
            self.input_attachment_count.hash(state);
            hash_raw_arr(
                self.p_input_attachments,
                (self.input_attachment_count) as usize,
                state,
            );
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_raw_arr(
                self.p_resolve_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_stencil_attachment, state);
            self.preserve_attachment_count.hash(state);
            hash_raw_arr(
                self.p_preserve_attachments,
                (self.preserve_attachment_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::SubpassDependency2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_subpass.hash(state);
            self.dst_subpass.hash(state);
            self.src_stage_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.dependency_flags.hash(state);
            self.view_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::RenderPassCreateInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
            self.subpass_count.hash(state);
            hash_raw_arr(self.p_subpasses, (self.subpass_count) as usize, state);
            self.dependency_count.hash(state);
            hash_raw_arr(self.p_dependencies, (self.dependency_count) as usize, state);
            self.correlated_view_mask_count.hash(state);
            hash_raw_arr(
                self.p_correlated_view_masks,
                (self.correlated_view_mask_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::SubpassBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.contents.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::SubpassEndInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceTimelineSemaphoreFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.timeline_semaphore.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceTimelineSemaphoreProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_timeline_semaphore_value_difference.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::SemaphoreTypeCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore_type.hash(state);
            self.initial_value.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::TimelineSemaphoreSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.wait_semaphore_value_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_values,
                (self.wait_semaphore_value_count) as usize,
                state,
            );
            self.signal_semaphore_value_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_values,
                (self.signal_semaphore_value_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::SemaphoreWaitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.semaphore_count.hash(state);
            hash_raw_arr(self.p_semaphores, (self.semaphore_count) as usize, state);
            hash_raw_arr(self.p_values, (self.semaphore_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk12::SemaphoreSignalInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_attribute_divisor::PipelineVertexInputDivisorStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_binding_divisor_count.hash(state);
            hash_raw_arr(
                self.p_vertex_binding_divisors,
                (self.vertex_binding_divisor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_vertex_attrib_divisor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pci_bus_info::PhysicalDevicePCIBusInfoPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pci_domain.hash(state);
            self.pci_bus.hash(state);
            self.pci_device.hash(state);
            self.pci_function.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::ImportAndroidHardwareBufferInfoANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.android_hardware_buffer_usage.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.allocation_size.hash(state);
            self.memory_type_bits.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatPropertiesANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.external_format.hash(state);
            self.format_features.hash(state);
            self.sampler_ycbcr_conversion_components.hash(state);
            self.suggested_ycbcr_model.hash(state);
            self.suggested_ycbcr_range.hash(state);
            self.suggested_xchroma_offset.hash(state);
            self.suggested_ychroma_offset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_conditional_rendering::CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.conditional_rendering_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_format.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDevice8BitStorageFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_buffer_8_bit_access.hash(state);
            self.uniform_and_storage_buffer_8_bit_access.hash(state);
            self.storage_push_constant_8.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.conditional_rendering.hash(state);
            self.inherited_conditional_rendering.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceVulkanMemoryModelFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vulkan_memory_model.hash(state);
            self.vulkan_memory_model_device_scope.hash(state);
            self.vulkan_memory_model_availability_visibility_chains.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceShaderAtomicInt64Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_buffer_int_64_atomics.hash(state);
            self.shader_shared_int_64_atomics.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_buffer_float_32_atomics.hash(state);
            self.shader_buffer_float_32_atomic_add.hash(state);
            self.shader_buffer_float_64_atomics.hash(state);
            self.shader_buffer_float_64_atomic_add.hash(state);
            self.shader_shared_float_32_atomics.hash(state);
            self.shader_shared_float_32_atomic_add.hash(state);
            self.shader_shared_float_64_atomics.hash(state);
            self.shader_shared_float_64_atomic_add.hash(state);
            self.shader_image_float_32_atomics.hash(state);
            self.shader_image_float_32_atomic_add.hash(state);
            self.sparse_image_float_32_atomics.hash(state);
            self.sparse_image_float_32_atomic_add.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_buffer_float_16_atomics.hash(state);
            self.shader_buffer_float_16_atomic_add.hash(state);
            self.shader_buffer_float_16_atomic_min_max.hash(state);
            self.shader_buffer_float_32_atomic_min_max.hash(state);
            self.shader_buffer_float_64_atomic_min_max.hash(state);
            self.shader_shared_float_16_atomics.hash(state);
            self.shader_shared_float_16_atomic_add.hash(state);
            self.shader_shared_float_16_atomic_min_max.hash(state);
            self.shader_shared_float_32_atomic_min_max.hash(state);
            self.shader_shared_float_64_atomic_min_max.hash(state);
            self.shader_image_float_32_atomic_min_max.hash(state);
            self.sparse_image_float_32_atomic_min_max.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_attribute_instance_rate_divisor.hash(state);
            self.vertex_attribute_instance_rate_zero_divisor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.checkpoint_execution_stage_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stage.hash(state);
            self.p_checkpoint_marker.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceDepthStencilResolveProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supported_depth_resolve_modes.hash(state);
            self.supported_stencil_resolve_modes.hash(state);
            self.independent_resolve_none.hash(state);
            self.independent_resolve.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::SubpassDescriptionDepthStencilResolve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.depth_resolve_mode.hash(state);
            self.stencil_resolve_mode.hash(state);
            hash_ptr(self.p_depth_stencil_resolve_attachment, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_astc_decode_mode::ImageViewASTCDecodeModeEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.decode_mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_astc_decode_mode::PhysicalDeviceASTCDecodeFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.decode_mode_shared_exponent.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.transform_feedback.hash(state);
            self.geometry_streams.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_transform_feedback_streams.hash(state);
            self.max_transform_feedback_buffers.hash(state);
            self.max_transform_feedback_buffer_size.hash(state);
            self.max_transform_feedback_stream_data_size.hash(state);
            self.max_transform_feedback_buffer_data_size.hash(state);
            self.max_transform_feedback_buffer_data_stride.hash(state);
            self.transform_feedback_queries.hash(state);
            self.transform_feedback_streams_lines_triangles.hash(state);
            self.transform_feedback_rasterization_stream_select.hash(state);
            self.transform_feedback_draw.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.rasterization_stream.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.representative_fragment_test.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_representative_fragment_test::PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.representative_fragment_test_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.exclusive_scissor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_scissor_exclusive::PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.exclusive_scissor_count.hash(state);
            hash_raw_arr(
                self.p_exclusive_scissors,
                (self.exclusive_scissor_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.corner_sampled_image.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.compute_derivative_group_quads.hash(state);
            self.compute_derivative_group_linear.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_footprint.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dedicated_allocation_image_aliasing.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.shading_rate_palette_entry_count.hash(state);
            hash_raw_arr(
                self.p_shading_rate_palette_entries,
                (self.shading_rate_palette_entry_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_shading_rate_image::PipelineViewportShadingRateImageStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shading_rate_image_enable.hash(state);
            self.viewport_count.hash(state);
            hash_raw_arr(
                self.p_shading_rate_palettes,
                (self.viewport_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shading_rate_image.hash(state);
            self.shading_rate_coarse_sample_order.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shading_rate_texel_size.hash(state);
            self.shading_rate_palette_size.hash(state);
            self.shading_rate_max_coarse_samples.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.invocation_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.shading_rate.hash(state);
            self.sample_count.hash(state);
            self.sample_location_count.hash(state);
            hash_raw_arr(
                self.p_sample_locations,
                (self.sample_location_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_shading_rate_image::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sample_order_type.hash(state);
            self.custom_sample_order_count.hash(state);
            hash_raw_arr(
                self.p_custom_sample_orders,
                (self.custom_sample_order_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.task_shader.hash(state);
            self.mesh_shader.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_draw_mesh_tasks_count.hash(state);
            self.max_task_work_group_invocations.hash(state);
            self.max_task_work_group_size.hash(state);
            self.max_task_total_memory_size.hash(state);
            self.max_task_output_count.hash(state);
            self.max_mesh_work_group_invocations.hash(state);
            self.max_mesh_work_group_size.hash(state);
            self.max_mesh_total_memory_size.hash(state);
            self.max_mesh_output_vertices.hash(state);
            self.max_mesh_output_primitives.hash(state);
            self.max_mesh_multiview_view_count.hash(state);
            self.mesh_output_per_vertex_granularity.hash(state);
            self.mesh_output_per_primitive_granularity.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.task_shader.hash(state);
            self.mesh_shader.hash(state);
            self.multiview_mesh_shader.hash(state);
            self.primitive_fragment_shading_rate_mesh_shader.hash(state);
            self.mesh_shader_queries.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_task_work_group_total_count.hash(state);
            self.max_task_work_group_count.hash(state);
            self.max_task_work_group_invocations.hash(state);
            self.max_task_work_group_size.hash(state);
            self.max_task_payload_size.hash(state);
            self.max_task_shared_memory_size.hash(state);
            self.max_task_payload_and_shared_memory_size.hash(state);
            self.max_mesh_work_group_total_count.hash(state);
            self.max_mesh_work_group_count.hash(state);
            self.max_mesh_work_group_invocations.hash(state);
            self.max_mesh_work_group_size.hash(state);
            self.max_mesh_shared_memory_size.hash(state);
            self.max_mesh_payload_and_shared_memory_size.hash(state);
            self.max_mesh_output_memory_size.hash(state);
            self.max_mesh_payload_and_output_memory_size.hash(state);
            self.max_mesh_output_components.hash(state);
            self.max_mesh_output_vertices.hash(state);
            self.max_mesh_output_primitives.hash(state);
            self.max_mesh_output_layers.hash(state);
            self.max_mesh_multiview_view_count.hash(state);
            self.mesh_output_per_vertex_granularity.hash(state);
            self.mesh_output_per_primitive_granularity.hash(state);
            self.max_preferred_task_work_group_invocations.hash(state);
            self.max_preferred_mesh_work_group_invocations.hash(state);
            self.prefers_local_invocation_vertex_output.hash(state);
            self.prefers_local_invocation_primitive_output.hash(state);
            self.prefers_compact_vertex_output.hash(state);
            self.prefers_compact_primitive_output.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.general_shader.hash(state);
            self.closest_hit_shader.hash(state);
            self.any_hit_shader.hash(state);
            self.intersection_shader.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.general_shader.hash(state);
            self.closest_hit_shader.hash(state);
            self.any_hit_shader.hash(state);
            self.intersection_shader.hash(state);
            self.p_shader_group_capture_replay_handle.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage_count.hash(state);
            hash_raw_arr(self.p_stages, (self.stage_count) as usize, state);
            self.group_count.hash(state);
            hash_raw_arr(self.p_groups, (self.group_count) as usize, state);
            self.max_recursion_depth.hash(state);
            self.layout.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.stage_count.hash(state);
            hash_raw_arr(self.p_stages, (self.stage_count) as usize, state);
            self.group_count.hash(state);
            hash_raw_arr(self.p_groups, (self.group_count) as usize, state);
            self.max_pipeline_ray_recursion_depth.hash(state);
            hash_ptr(self.p_library_info, state);
            hash_ptr(self.p_library_interface, state);
            hash_ptr(self.p_dynamic_state, state);
            self.layout.hash(state);
            self.base_pipeline_handle.hash(state);
            self.base_pipeline_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::GeometryTrianglesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_data.hash(state);
            self.vertex_offset.hash(state);
            self.vertex_count.hash(state);
            self.vertex_stride.hash(state);
            self.vertex_format.hash(state);
            self.index_data.hash(state);
            self.index_offset.hash(state);
            self.index_count.hash(state);
            self.index_type.hash(state);
            self.transform_data.hash(state);
            self.transform_offset.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::GeometryAABBNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.aabb_data.hash(state);
            self.num_aabbs.hash(state);
            self.stride.hash(state);
            self.offset.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::GeometryDataNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.triangles.hash(state);
            self.aabbs.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::GeometryNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.geometry_type.hash(state);
            self.geometry.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.flags.hash(state);
            self.instance_count.hash(state);
            self.geometry_count.hash(state);
            hash_raw_arr(self.p_geometries, (self.geometry_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.compacted_size.hash(state);
            self.info.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
            self.device_index_count.hash(state);
            hash_raw_arr(
                self.p_device_indices,
                (self.device_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::WriteDescriptorSetAccelerationStructureKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure_count.hash(state);
            hash_raw_arr(
                self.p_acceleration_structures,
                (self.acceleration_structure_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing::WriteDescriptorSetAccelerationStructureNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure_count.hash(state);
            hash_raw_arr(
                self.p_acceleration_structures,
                (self.acceleration_structure_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.acceleration_structure.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure.hash(state);
            self.acceleration_structure_capture_replay.hash(state);
            self.acceleration_structure_indirect_build.hash(state);
            self.acceleration_structure_host_commands.hash(state);
            self.descriptor_binding_acceleration_structure_update_after_bind.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ray_tracing_pipeline.hash(state);
            self.ray_tracing_pipeline_shader_group_handle_capture_replay.hash(state);
            self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
                .hash(state);
            self.ray_tracing_pipeline_trace_rays_indirect.hash(state);
            self.ray_traversal_primitive_culling.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ray_query.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_geometry_count.hash(state);
            self.max_instance_count.hash(state);
            self.max_primitive_count.hash(state);
            self.max_per_stage_descriptor_acceleration_structures.hash(state);
            self.max_per_stage_descriptor_update_after_bind_acceleration_structures
                .hash(state);
            self.max_descriptor_set_acceleration_structures.hash(state);
            self.max_descriptor_set_update_after_bind_acceleration_structures
                .hash(state);
            self.min_acceleration_structure_scratch_offset_alignment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_group_handle_size.hash(state);
            self.max_ray_recursion_depth.hash(state);
            self.max_shader_group_stride.hash(state);
            self.shader_group_base_alignment.hash(state);
            self.shader_group_handle_capture_replay_size.hash(state);
            self.max_ray_dispatch_invocation_count.hash(state);
            self.shader_group_handle_alignment.hash(state);
            self.max_ray_hit_attribute_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_group_handle_size.hash(state);
            self.max_recursion_depth.hash(state);
            self.max_shader_group_stride.hash(state);
            self.shader_group_base_alignment.hash(state);
            self.max_geometry_count.hash(state);
            self.max_instance_count.hash(state);
            self.max_triangle_count.hash(state);
            self.max_descriptor_set_acceleration_structures.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_maintenance1::PhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ray_tracing_maintenance_1.hash(state);
            self.ray_tracing_pipeline_trace_rays_indirect_2.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier_count.hash(state);
            hash_raw_arr(
                self.p_drm_format_modifier_properties,
                (self.drm_format_modifier_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier.hash(state);
            self.sharing_mode.hash(state);
            self.queue_family_index_count.hash(state);
            hash_raw_arr(
                self.p_queue_family_indices,
                (self.queue_family_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierListCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier_count.hash(state);
            hash_raw_arr(
                self.p_drm_format_modifiers,
                (self.drm_format_modifier_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier.hash(state);
            self.drm_format_modifier_plane_count.hash(state);
            hash_raw_arr(
                self.p_plane_layouts,
                (self.drm_format_modifier_plane_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::ImageStencilUsageCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stencil_usage.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_memory_overallocation_behavior::DeviceMemoryOverallocationCreateInfoAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.overallocation_behavior.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_map.hash(state);
            self.fragment_density_map_dynamic.hash(state);
            self.fragment_density_map_non_subsampled_images.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_map_deferred.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_map_offset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_fragment_density_texel_size.hash(state);
            self.max_fragment_density_texel_size.hash(state);
            self.fragment_density_invocations.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subsampled_loads.hash(state);
            self.subsampled_coarse_reconstruction_early_access.hash(state);
            self.max_subsampled_array_layers.hash(state);
            self.max_descriptor_set_subsampled_samplers.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_offset_granularity.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_map_attachment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_fragment_density_map_offset::SubpassFragmentDensityMapOffsetEndInfoQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_density_offset_count.hash(state);
            hash_raw_arr(
                self.p_fragment_density_offsets,
                (self.fragment_density_offset_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceScalarBlockLayoutFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.scalar_block_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supports_protected.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.uniform_buffer_standard_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.depth_clip_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.depth_clip_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.heap_budget.hash(state);
            self.heap_usage.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_priority.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_memory_priority::MemoryPriorityAllocateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.priority.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pageable_device_local_memory::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pageable_device_local_memory.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceBufferDeviceAddressFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer_device_address.hash(state);
            self.buffer_device_address_capture_replay.hash(state);
            self.buffer_device_address_multi_device.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer_device_address.hash(state);
            self.buffer_device_address_capture_replay.hash(state);
            self.buffer_device_address_multi_device.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::BufferDeviceAddressInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::BufferOpaqueCaptureAddressCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.opaque_capture_address.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_buffer_device_address::BufferDeviceAddressCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_address.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.filter_cubic.hash(state);
            self.filter_cubic_minmax.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceImagelessFramebufferFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.imageless_framebuffer.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::FramebufferAttachmentsCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment_image_info_count.hash(state);
            hash_raw_arr(
                self.p_attachment_image_infos,
                (self.attachment_image_info_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk12::FramebufferAttachmentImageInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.usage.hash(state);
            self.width.hash(state);
            self.height.hash(state);
            self.layer_count.hash(state);
            self.view_format_count.hash(state);
            hash_raw_arr(self.p_view_formats, (self.view_format_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk12::RenderPassAttachmentBeginInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment_count.hash(state);
            hash_raw_arr(self.p_attachments, (self.attachment_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceTextureCompressionASTCHDRFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.texture_compression_astc_hdr.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.cooperative_matrix.hash(state);
            self.cooperative_matrix_robust_buffer_access.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.cooperative_matrix_supported_stages.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.msize.hash(state);
            self.nsize.hash(state);
            self.ksize.hash(state);
            self.atype.hash(state);
            self.btype.hash(state);
            self.ctype.hash(state);
            self.dtype.hash(state);
            self.scope.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ycbcr_image_arrays.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view.hash(state);
            self.descriptor_type.hash(state);
            self.sampler.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_address.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ggp_frame_token::PresentFrameTokenGGP {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.frame_token.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PipelineCreationFeedbackCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_pipeline_creation_feedback, state);
            self.pipeline_stage_creation_feedback_count.hash(state);
            hash_raw_arr(
                self.p_pipeline_stage_creation_feedbacks,
                (self.pipeline_stage_creation_feedback_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.full_screen_exclusive.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.hmonitor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.full_screen_exclusive_supported.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_present_barrier::PhysicalDevicePresentBarrierFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_barrier.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_present_barrier::SurfaceCapabilitiesPresentBarrierNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_barrier_supported.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_present_barrier::SwapchainPresentBarrierCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.present_barrier_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.performance_counter_query_pools.hash(state);
            self.performance_counter_multiple_query_pools.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.allow_command_buffer_query_copies.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_performance_query::PerformanceCounterKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.unit.hash(state);
            self.scope.hash(state);
            self.storage.hash(state);
            self.uuid.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_performance_query::PerformanceCounterDescriptionKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.name.hash(state);
            self.category.hash(state);
            self.description.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.queue_family_index.hash(state);
            self.counter_index_count.hash(state);
            hash_raw_arr(
                self.p_counter_indices,
                (self.counter_index_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_performance_query::PerformanceCounterResultKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_performance_query::AcquireProfilingLockInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.timeout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.counter_pass_index.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.coverage_reduction_mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.coverage_reduction_mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.coverage_reduction_mode.hash(state);
            self.rasterization_samples.hash(state);
            self.depth_stencil_samples.hash(state);
            self.color_samples.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_integer_functions_2.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::intel_performance_query::PerformanceValueDataINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash for crate::extensions::intel_performance_query::PerformanceValueINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.kind.hash(state);
            self.data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.p_user_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.performance_counters_sampling.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::PerformanceMarkerInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.marker.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.marker.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::PerformanceOverrideInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.enable.hash(state);
            self.parameter.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_subgroup_clock.hash(state);
            self.shader_device_clock.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.index_type_uint_8.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_smcount.hash(state);
            self.shader_warps_per_sm.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_smbuiltins.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_shader_sample_interlock.hash(state);
            self.fragment_shader_pixel_interlock.hash(state);
            self.fragment_shader_shading_rate_interlock.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.separate_depth_stencil_layouts.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::AttachmentReferenceStencilLayout {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stencil_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_primitive_topology_list_restart::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.primitive_topology_list_restart.hash(state);
            self.primitive_topology_patch_list_restart.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::AttachmentDescriptionStencilLayout {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stencil_initial_layout.hash(state);
            self.stencil_final_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_executable_info.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stages.hash(state);
            self.name.hash(state);
            self.description.hash(state);
            self.subgroup_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline.hash(state);
            self.executable_index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.name.hash(state);
            self.description.hash(state);
            self.format.hash(state);
            self.value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.name.hash(state);
            self.description.hash(state);
            self.is_text.hash(state);
            self.data_size.hash(state);
            hash_raw_arr(self.p_data.cast::<u8>(), (self.data_size) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_demote_to_helper_invocation.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.texel_buffer_alignment.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceTexelBufferAlignmentProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_texel_buffer_offset_alignment_bytes.hash(state);
            self.storage_texel_buffer_offset_single_texel_alignment.hash(state);
            self.uniform_texel_buffer_offset_alignment_bytes.hash(state);
            self.uniform_texel_buffer_offset_single_texel_alignment.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceSubgroupSizeControlFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subgroup_size_control.hash(state);
            self.compute_full_subgroups.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceSubgroupSizeControlProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_subgroup_size.hash(state);
            self.max_subgroup_size.hash(state);
            self.max_compute_workgroup_subgroups.hash(state);
            self.required_subgroup_size_stages.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.required_subgroup_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::huawei_subpass_shading::SubpassShadingPipelineCreateInfoHUAWEI {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.render_pass.hash(state);
            self.subpass.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_subpass_shading_workgroup_size_aspect_ratio.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::MemoryOpaqueCaptureAddressAllocateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.opaque_capture_address.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::DeviceMemoryOpaqueCaptureAddressInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.rectangular_lines.hash(state);
            self.bresenham_lines.hash(state);
            self.smooth_lines.hash(state);
            self.stippled_rectangular_lines.hash(state);
            self.stippled_bresenham_lines.hash(state);
            self.stippled_smooth_lines.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.line_sub_pixel_precision_bits.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_line_rasterization::PipelineRasterizationLineStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.line_rasterization_mode.hash(state);
            self.stippled_line_enable.hash(state);
            self.line_stipple_factor.hash(state);
            self.line_stipple_pattern.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDevicePipelineCreationCacheControlFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_creation_cache_control.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceVulkan11Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_buffer_16_bit_access.hash(state);
            self.uniform_and_storage_buffer_16_bit_access.hash(state);
            self.storage_push_constant_16.hash(state);
            self.storage_input_output_16.hash(state);
            self.multiview.hash(state);
            self.multiview_geometry_shader.hash(state);
            self.multiview_tessellation_shader.hash(state);
            self.variable_pointers_storage_buffer.hash(state);
            self.variable_pointers.hash(state);
            self.protected_memory.hash(state);
            self.sampler_ycbcr_conversion.hash(state);
            self.shader_draw_parameters.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceVulkan11Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_uuid.hash(state);
            self.driver_uuid.hash(state);
            self.device_luid.hash(state);
            self.device_node_mask.hash(state);
            self.device_luidvalid.hash(state);
            self.subgroup_size.hash(state);
            self.subgroup_supported_stages.hash(state);
            self.subgroup_supported_operations.hash(state);
            self.subgroup_quad_operations_in_all_stages.hash(state);
            self.point_clipping_behavior.hash(state);
            self.max_multiview_view_count.hash(state);
            self.max_multiview_instance_index.hash(state);
            self.protected_no_fault.hash(state);
            self.max_per_set_descriptors.hash(state);
            self.max_memory_allocation_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceVulkan12Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sampler_mirror_clamp_to_edge.hash(state);
            self.draw_indirect_count.hash(state);
            self.storage_buffer_8_bit_access.hash(state);
            self.uniform_and_storage_buffer_8_bit_access.hash(state);
            self.storage_push_constant_8.hash(state);
            self.shader_buffer_int_64_atomics.hash(state);
            self.shader_shared_int_64_atomics.hash(state);
            self.shader_float_16.hash(state);
            self.shader_int_8.hash(state);
            self.descriptor_indexing.hash(state);
            self.shader_input_attachment_array_dynamic_indexing.hash(state);
            self.shader_uniform_texel_buffer_array_dynamic_indexing.hash(state);
            self.shader_storage_texel_buffer_array_dynamic_indexing.hash(state);
            self.shader_uniform_buffer_array_non_uniform_indexing.hash(state);
            self.shader_sampled_image_array_non_uniform_indexing.hash(state);
            self.shader_storage_buffer_array_non_uniform_indexing.hash(state);
            self.shader_storage_image_array_non_uniform_indexing.hash(state);
            self.shader_input_attachment_array_non_uniform_indexing.hash(state);
            self.shader_uniform_texel_buffer_array_non_uniform_indexing.hash(state);
            self.shader_storage_texel_buffer_array_non_uniform_indexing.hash(state);
            self.descriptor_binding_uniform_buffer_update_after_bind.hash(state);
            self.descriptor_binding_sampled_image_update_after_bind.hash(state);
            self.descriptor_binding_storage_image_update_after_bind.hash(state);
            self.descriptor_binding_storage_buffer_update_after_bind.hash(state);
            self.descriptor_binding_uniform_texel_buffer_update_after_bind.hash(state);
            self.descriptor_binding_storage_texel_buffer_update_after_bind.hash(state);
            self.descriptor_binding_update_unused_while_pending.hash(state);
            self.descriptor_binding_partially_bound.hash(state);
            self.descriptor_binding_variable_descriptor_count.hash(state);
            self.runtime_descriptor_array.hash(state);
            self.sampler_filter_minmax.hash(state);
            self.scalar_block_layout.hash(state);
            self.imageless_framebuffer.hash(state);
            self.uniform_buffer_standard_layout.hash(state);
            self.shader_subgroup_extended_types.hash(state);
            self.separate_depth_stencil_layouts.hash(state);
            self.host_query_reset.hash(state);
            self.timeline_semaphore.hash(state);
            self.buffer_device_address.hash(state);
            self.buffer_device_address_capture_replay.hash(state);
            self.buffer_device_address_multi_device.hash(state);
            self.vulkan_memory_model.hash(state);
            self.vulkan_memory_model_device_scope.hash(state);
            self.vulkan_memory_model_availability_visibility_chains.hash(state);
            self.shader_output_viewport_index.hash(state);
            self.shader_output_layer.hash(state);
            self.subgroup_broadcast_dynamic_id.hash(state);
        }
    }
}
impl DumbHash for crate::vk12::PhysicalDeviceVulkan12Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.driver_id.hash(state);
            self.driver_name.hash(state);
            self.driver_info.hash(state);
            self.conformance_version.hash(state);
            self.denorm_behavior_independence.hash(state);
            self.rounding_mode_independence.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_16.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_32.hash(state);
            self.shader_signed_zero_inf_nan_preserve_float_64.hash(state);
            self.shader_denorm_preserve_float_16.hash(state);
            self.shader_denorm_preserve_float_32.hash(state);
            self.shader_denorm_preserve_float_64.hash(state);
            self.shader_denorm_flush_to_zero_float_16.hash(state);
            self.shader_denorm_flush_to_zero_float_32.hash(state);
            self.shader_denorm_flush_to_zero_float_64.hash(state);
            self.shader_rounding_mode_rtefloat_16.hash(state);
            self.shader_rounding_mode_rtefloat_32.hash(state);
            self.shader_rounding_mode_rtefloat_64.hash(state);
            self.shader_rounding_mode_rtzfloat_16.hash(state);
            self.shader_rounding_mode_rtzfloat_32.hash(state);
            self.shader_rounding_mode_rtzfloat_64.hash(state);
            self.max_update_after_bind_descriptors_in_all_pools.hash(state);
            self.shader_uniform_buffer_array_non_uniform_indexing_native.hash(state);
            self.shader_sampled_image_array_non_uniform_indexing_native.hash(state);
            self.shader_storage_buffer_array_non_uniform_indexing_native.hash(state);
            self.shader_storage_image_array_non_uniform_indexing_native.hash(state);
            self.shader_input_attachment_array_non_uniform_indexing_native.hash(state);
            self.robust_buffer_access_update_after_bind.hash(state);
            self.quad_divergent_implicit_lod.hash(state);
            self.max_per_stage_descriptor_update_after_bind_samplers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_uniform_buffers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_storage_buffers.hash(state);
            self.max_per_stage_descriptor_update_after_bind_sampled_images.hash(state);
            self.max_per_stage_descriptor_update_after_bind_storage_images.hash(state);
            self.max_per_stage_descriptor_update_after_bind_input_attachments
                .hash(state);
            self.max_per_stage_update_after_bind_resources.hash(state);
            self.max_descriptor_set_update_after_bind_samplers.hash(state);
            self.max_descriptor_set_update_after_bind_uniform_buffers.hash(state);
            self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
                .hash(state);
            self.max_descriptor_set_update_after_bind_storage_buffers.hash(state);
            self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
                .hash(state);
            self.max_descriptor_set_update_after_bind_sampled_images.hash(state);
            self.max_descriptor_set_update_after_bind_storage_images.hash(state);
            self.max_descriptor_set_update_after_bind_input_attachments.hash(state);
            self.supported_depth_resolve_modes.hash(state);
            self.supported_stencil_resolve_modes.hash(state);
            self.independent_resolve_none.hash(state);
            self.independent_resolve.hash(state);
            self.filter_minmax_single_component_formats.hash(state);
            self.filter_minmax_image_component_mapping.hash(state);
            self.max_timeline_semaphore_value_difference.hash(state);
            self.framebuffer_integer_color_sample_counts.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceVulkan13Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.robust_image_access.hash(state);
            self.inline_uniform_block.hash(state);
            self.descriptor_binding_inline_uniform_block_update_after_bind.hash(state);
            self.pipeline_creation_cache_control.hash(state);
            self.private_data.hash(state);
            self.shader_demote_to_helper_invocation.hash(state);
            self.shader_terminate_invocation.hash(state);
            self.subgroup_size_control.hash(state);
            self.compute_full_subgroups.hash(state);
            self.synchronization_2.hash(state);
            self.texture_compression_astc_hdr.hash(state);
            self.shader_zero_initialize_workgroup_memory.hash(state);
            self.dynamic_rendering.hash(state);
            self.shader_integer_dot_product.hash(state);
            self.maintenance_4.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceVulkan13Properties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_subgroup_size.hash(state);
            self.max_subgroup_size.hash(state);
            self.max_compute_workgroup_subgroups.hash(state);
            self.required_subgroup_size_stages.hash(state);
            self.max_inline_uniform_block_size.hash(state);
            self.max_per_stage_descriptor_inline_uniform_blocks.hash(state);
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
                .hash(state);
            self.max_descriptor_set_inline_uniform_blocks.hash(state);
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks.hash(state);
            self.max_inline_uniform_total_size.hash(state);
            self.integer_dot_product_8_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_8_bit_signed_accelerated.hash(state);
            self.integer_dot_product_8_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_unsigned_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_signed_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_16_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_16_bit_signed_accelerated.hash(state);
            self.integer_dot_product_16_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_32_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_32_bit_signed_accelerated.hash(state);
            self.integer_dot_product_32_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_64_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_64_bit_signed_accelerated.hash(state);
            self.integer_dot_product_64_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                .hash(state);
            self.storage_texel_buffer_offset_alignment_bytes.hash(state);
            self.storage_texel_buffer_offset_single_texel_alignment.hash(state);
            self.uniform_texel_buffer_offset_alignment_bytes.hash(state);
            self.uniform_texel_buffer_offset_single_texel_alignment.hash(state);
            self.max_buffer_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlCreateInfoAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.compiler_control_flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_coherent_memory.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceToolProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.name.hash(state);
            self.version.hash(state);
            self.purposes.hash(state);
            self.description.hash(state);
            self.layer.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_custom_border_color::SamplerCustomBorderColorCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.custom_border_color.hash(state);
            self.format.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_custom_border_color_samplers.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.custom_border_colors.hash(state);
            self.custom_border_color_without_format.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_border_color_swizzle::SamplerBorderColorComponentMappingCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.components.hash(state);
            self.srgb.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.border_color_swizzle.hash(state);
            self.border_color_swizzle_from_image.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_format.hash(state);
            self.vertex_data.hash(state);
            self.vertex_stride.hash(state);
            self.max_vertex.hash(state);
            self.index_type.hash(state);
            self.index_data.hash(state);
            self.transform_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.data.hash(state);
            self.stride.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.array_of_pointers.hash(state);
            self.data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.geometry_type.hash(state);
            self.geometry.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.flags.hash(state);
            self.mode.hash(state);
            self.src_acceleration_structure.hash(state);
            self.dst_acceleration_structure.hash(state);
            self.geometry_count.hash(state);
            hash_raw_arr(self.p_geometries, (self.geometry_count) as usize, state);
            let len = (self.geometry_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_geometries.add(i);
                hash_raw_arr(ptr, (1) as usize, state);
            }
            self.scratch_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.create_flags.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
            self.kind.hash(state);
            self.device_address.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_acceleration_structure::AabbPositionsKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.min_x.hash(state);
            self.min_y.hash(state);
            self.min_z.hash(state);
            self.max_x.hash(state);
            self.max_y.hash(state);
            self.max_z.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_acceleration_structure::TransformMatrixKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            for i in 0..3 as usize {
                let ptr = &self.matrix[i];
                ptr.hash(state);
            }
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.transform.hash(state);
            self.instance_custom_index_mask.hash(state);
            self.instance_shader_binding_table_record_offset.hash(state);
            self.flags.hash(state);
            self.acceleration_structure_reference.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_raw_arr(
                self.p_version_data,
                (2 * crate::vk10::UUID_SIZE) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_pipeline_ray_payload_size.hash(state);
            self.max_pipeline_ray_hit_attribute_size.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.library_count.hash(state);
            hash_raw_arr(self.p_libraries, (self.library_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.extended_dynamic_state.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.extended_dynamic_state_2.hash(state);
            self.extended_dynamic_state_2_logic_op.hash(state);
            self.extended_dynamic_state_2_patch_control_points.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.extended_dynamic_state_3_tessellation_domain_origin.hash(state);
            self.extended_dynamic_state_3_depth_clamp_enable.hash(state);
            self.extended_dynamic_state_3_polygon_mode.hash(state);
            self.extended_dynamic_state_3_rasterization_samples.hash(state);
            self.extended_dynamic_state_3_sample_mask.hash(state);
            self.extended_dynamic_state_3_alpha_to_coverage_enable.hash(state);
            self.extended_dynamic_state_3_alpha_to_one_enable.hash(state);
            self.extended_dynamic_state_3_logic_op_enable.hash(state);
            self.extended_dynamic_state_3_color_blend_enable.hash(state);
            self.extended_dynamic_state_3_color_blend_equation.hash(state);
            self.extended_dynamic_state_3_color_write_mask.hash(state);
            self.extended_dynamic_state_3_rasterization_stream.hash(state);
            self.extended_dynamic_state_3_conservative_rasterization_mode.hash(state);
            self.extended_dynamic_state_3_extra_primitive_overestimation_size
                .hash(state);
            self.extended_dynamic_state_3_depth_clip_enable.hash(state);
            self.extended_dynamic_state_3_sample_locations_enable.hash(state);
            self.extended_dynamic_state_3_color_blend_advanced.hash(state);
            self.extended_dynamic_state_3_provoking_vertex_mode.hash(state);
            self.extended_dynamic_state_3_line_rasterization_mode.hash(state);
            self.extended_dynamic_state_3_line_stipple_enable.hash(state);
            self.extended_dynamic_state_3_depth_clip_negative_one_to_one.hash(state);
            self.extended_dynamic_state_3_viewport_wscaling_enable.hash(state);
            self.extended_dynamic_state_3_viewport_swizzle.hash(state);
            self.extended_dynamic_state_3_coverage_to_color_enable.hash(state);
            self.extended_dynamic_state_3_coverage_to_color_location.hash(state);
            self.extended_dynamic_state_3_coverage_modulation_mode.hash(state);
            self.extended_dynamic_state_3_coverage_modulation_table_enable.hash(state);
            self.extended_dynamic_state_3_coverage_modulation_table.hash(state);
            self.extended_dynamic_state_3_coverage_reduction_mode.hash(state);
            self.extended_dynamic_state_3_representative_fragment_test_enable
                .hash(state);
            self.extended_dynamic_state_3_shading_rate_image_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dynamic_primitive_topology_unrestricted.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_render_pass_transform::RenderPassTransformBeginInfoQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.transform.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.transform.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_render_pass_transform::CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.transform.hash(state);
            self.render_area.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.diagnostics_config.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_zero_initialize_workgroup_memory.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_shader_subgroup_uniform_control_flow::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_subgroup_uniform_control_flow.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.robust_buffer_access_2.hash(state);
            self.robust_image_access_2.hash(state);
            self.null_descriptor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.robust_storage_buffer_access_size_alignment.hash(state);
            self.robust_uniform_buffer_access_size_alignment.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceImageRobustnessFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.robust_image_access.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.workgroup_memory_explicit_layout.hash(state);
            self.workgroup_memory_explicit_layout_scalar_block_layout.hash(state);
            self.workgroup_memory_explicit_layout_8_bit_access.hash(state);
            self.workgroup_memory_explicit_layout_16_bit_access.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.constant_alpha_color_blend_factors.hash(state);
            self.events.hash(state);
            self.image_view_format_reinterpretation.hash(state);
            self.image_view_format_swizzle.hash(state);
            self.image_view_2_don_3_dimage.hash(state);
            self.multisample_array_image.hash(state);
            self.mutable_comparison_samplers.hash(state);
            self.point_polygons.hash(state);
            self.sampler_mip_lod_bias.hash(state);
            self.separate_stencil_mask_ref.hash(state);
            self.shader_sample_rate_interpolation_functions.hash(state);
            self.tessellation_isolines.hash(state);
            self.tessellation_point_mode.hash(state);
            self.triangle_fans.hash(state);
            self.vertex_attribute_access_beyond_stride.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_vertex_input_binding_stride_alignment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format_a4_r4_g4_b4.hash(state);
            self.format_a4_b4_g4_r4.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subpass_shading.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::BufferCopy2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_offset.hash(state);
            self.dst_offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::ImageCopy2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_subresource.hash(state);
            self.src_offset.hash(state);
            self.dst_subresource.hash(state);
            self.dst_offset.hash(state);
            self.extent.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::ImageBlit2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_subresource.hash(state);
            self.src_offsets.hash(state);
            self.dst_subresource.hash(state);
            self.dst_offsets.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::BufferImageCopy2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.buffer_offset.hash(state);
            self.buffer_row_length.hash(state);
            self.buffer_image_height.hash(state);
            self.image_subresource.hash(state);
            self.image_offset.hash(state);
            self.image_extent.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::ImageResolve2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_subresource.hash(state);
            self.src_offset.hash(state);
            self.dst_subresource.hash(state);
            self.dst_offset.hash(state);
            self.extent.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::CopyBufferInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_buffer.hash(state);
            self.dst_buffer.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::CopyImageInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_image.hash(state);
            self.src_image_layout.hash(state);
            self.dst_image.hash(state);
            self.dst_image_layout.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::BlitImageInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_image.hash(state);
            self.src_image_layout.hash(state);
            self.dst_image.hash(state);
            self.dst_image_layout.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
            self.filter.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::CopyBufferToImageInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_buffer.hash(state);
            self.dst_image.hash(state);
            self.dst_image_layout.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::CopyImageToBufferInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_image.hash(state);
            self.src_image_layout.hash(state);
            self.dst_buffer.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash for crate::vk13::ResolveImageInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_image.hash(state);
            self.src_image_layout.hash(state);
            self.dst_image.hash(state);
            self.dst_image_layout.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_image_int_64_atomics.hash(state);
            self.sparse_image_int_64_atomics.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_fragment_shading_rate_attachment, state);
            self.shading_rate_attachment_texel_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shading_rate::PipelineFragmentShadingRateStateCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_size.hash(state);
            self.combiner_ops.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_fragment_shading_rate.hash(state);
            self.primitive_fragment_shading_rate.hash(state);
            self.attachment_fragment_shading_rate.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_fragment_shading_rate_attachment_texel_size.hash(state);
            self.max_fragment_shading_rate_attachment_texel_size.hash(state);
            self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio
                .hash(state);
            self.primitive_fragment_shading_rate_with_multiple_viewports.hash(state);
            self.layered_shading_rate_attachments.hash(state);
            self.fragment_shading_rate_non_trivial_combiner_ops.hash(state);
            self.max_fragment_size.hash(state);
            self.max_fragment_size_aspect_ratio.hash(state);
            self.max_fragment_shading_rate_coverage_samples.hash(state);
            self.max_fragment_shading_rate_rasterization_samples.hash(state);
            self.fragment_shading_rate_with_shader_depth_stencil_writes.hash(state);
            self.fragment_shading_rate_with_sample_mask.hash(state);
            self.fragment_shading_rate_with_shader_sample_mask.hash(state);
            self.fragment_shading_rate_with_conservative_rasterization.hash(state);
            self.fragment_shading_rate_with_fragment_shader_interlock.hash(state);
            self.fragment_shading_rate_with_custom_sample_locations.hash(state);
            self.fragment_shading_rate_strict_multiply_combiner.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sample_counts.hash(state);
            self.fragment_size.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceShaderTerminateInvocationFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_terminate_invocation.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_shading_rate_enums.hash(state);
            self.supersample_fragment_shading_rates.hash(state);
            self.no_invocation_fragment_shading_rates.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_fragment_shading_rate_invocation_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_fragment_shading_rate_enums::PipelineFragmentShadingRateEnumStateCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shading_rate_type.hash(state);
            self.shading_rate.hash(state);
            self.combiner_ops.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.acceleration_structure_size.hash(state);
            self.update_scratch_size.hash(state);
            self.build_scratch_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_2d_view_of_3d::PhysicalDeviceImage2DViewOf3DFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_2_dview_of_3_d.hash(state);
            self.sampler_2_dview_of_3_d.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mutable_descriptor_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_mutable_descriptor_type::MutableDescriptorTypeListEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.descriptor_type_count.hash(state);
            hash_raw_arr(
                self.p_descriptor_types,
                (self.descriptor_type_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_mutable_descriptor_type::MutableDescriptorTypeCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mutable_descriptor_type_list_count.hash(state);
            hash_raw_arr(
                self.p_mutable_descriptor_type_lists,
                (self.mutable_descriptor_type_list_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.depth_clip_control.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_depth_clip_control::PipelineViewportDepthClipControlCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.negative_one_to_one.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_input_dynamic_state.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.external_memory_rdma.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.binding.hash(state);
            self.stride.hash(state);
            self.input_rate.hash(state);
            self.divisor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.location.hash(state);
            self.binding.hash(state);
            self.format.hash(state);
            self.offset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.color_write_enable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_color_write_enable::PipelineColorWriteCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_write_enables,
                (self.attachment_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk13::MemoryBarrier2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::ImageMemoryBarrier2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.old_layout.hash(state);
            self.new_layout.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.image.hash(state);
            self.subresource_range.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::BufferMemoryBarrier2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src_stage_mask.hash(state);
            self.src_access_mask.hash(state);
            self.dst_stage_mask.hash(state);
            self.dst_access_mask.hash(state);
            self.src_queue_family_index.hash(state);
            self.dst_queue_family_index.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::DependencyInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dependency_flags.hash(state);
            self.memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_memory_barriers,
                (self.memory_barrier_count) as usize,
                state,
            );
            self.buffer_memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_buffer_memory_barriers,
                (self.buffer_memory_barrier_count) as usize,
                state,
            );
            self.image_memory_barrier_count.hash(state);
            hash_raw_arr(
                self.p_image_memory_barriers,
                (self.image_memory_barrier_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::vk13::SemaphoreSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.value.hash(state);
            self.stage_mask.hash(state);
            self.device_index.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::CommandBufferSubmitInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.command_buffer.hash(state);
            self.device_mask.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::SubmitInfo2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.wait_semaphore_info_count.hash(state);
            hash_raw_arr(
                self.p_wait_semaphore_infos,
                (self.wait_semaphore_info_count) as usize,
                state,
            );
            self.command_buffer_info_count.hash(state);
            hash_raw_arr(
                self.p_command_buffer_infos,
                (self.command_buffer_info_count) as usize,
                state,
            );
            self.signal_semaphore_info_count.hash(state);
            hash_raw_arr(
                self.p_signal_semaphore_infos,
                (self.signal_semaphore_info_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.checkpoint_execution_stage_mask.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_synchronization2::CheckpointData2NV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.stage.hash(state);
            self.p_checkpoint_marker.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceSynchronization2Features {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.synchronization_2.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_primitives_generated_query::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.primitives_generated_query.hash(state);
            self.primitives_generated_query_with_rasterizer_discard.hash(state);
            self.primitives_generated_query_with_non_zero_streams.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_legacy_dithering::PhysicalDeviceLegacyDitheringFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.legacy_dithering.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_multisampled_render_to_single_sampled::PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.multisampled_render_to_single_sampled.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_multisampled_render_to_single_sampled::SubpassResolvePerformanceQueryEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.optimal.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_multisampled_render_to_single_sampled::MultisampledRenderToSingleSampledInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.multisampled_render_to_single_sampled_enable.hash(state);
            self.rasterization_samples.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_protected_access::PhysicalDevicePipelineProtectedAccessFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_protected_access.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::QueueFamilyVideoPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.video_codec_operations.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_video_queue::QueueFamilyQueryResultStatusPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.query_result_status_support.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoProfileListInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.profile_count.hash(state);
            hash_raw_arr(self.p_profiles, (self.profile_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_usage.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoFormatPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.component_mapping.hash(state);
            self.image_create_flags.hash(state);
            self.image_type.hash(state);
            self.image_tiling.hash(state);
            self.image_usage_flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoProfileInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.video_codec_operation.hash(state);
            self.chroma_subsampling.hash(state);
            self.luma_bit_depth.hash(state);
            self.chroma_bit_depth.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.min_bitstream_buffer_offset_alignment.hash(state);
            self.min_bitstream_buffer_size_alignment.hash(state);
            self.picture_access_granularity.hash(state);
            self.min_coded_extent.hash(state);
            self.max_coded_extent.hash(state);
            self.max_dpb_slots.hash(state);
            self.max_active_reference_pictures.hash(state);
            self.std_header_version.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoSessionMemoryRequirementsKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_bind_index.hash(state);
            self.memory_requirements.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::BindVideoSessionMemoryInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_bind_index.hash(state);
            self.memory.hash(state);
            self.memory_offset.hash(state);
            self.memory_size.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoPictureResourceInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.coded_offset.hash(state);
            self.coded_extent.hash(state);
            self.base_array_layer.hash(state);
            self.image_view_binding.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoReferenceSlotInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.slot_index.hash(state);
            hash_ptr(self.p_picture_resource, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_decode_queue::VideoDecodeUsageInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.video_usage_hints.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.src_buffer.hash(state);
            self.src_buffer_offset.hash(state);
            self.src_buffer_range.hash(state);
            self.dst_picture_resource.hash(state);
            hash_ptr(self.p_setup_reference_slot, state);
            self.reference_slot_count.hash(state);
            hash_raw_arr(
                self.p_reference_slots,
                (self.reference_slot_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::h264std::StdVideoH264SequenceParameterSetVui {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.aspect_ratio_idc.hash(state);
            self.sar_width.hash(state);
            self.sar_height.hash(state);
            self.video_format.hash(state);
            self.colour_primaries.hash(state);
            self.transfer_characteristics.hash(state);
            self.matrix_coefficients.hash(state);
            self.num_units_in_tick.hash(state);
            self.time_scale.hash(state);
            self.max_num_reorder_frames.hash(state);
            self.max_dec_frame_buffering.hash(state);
            self.chroma_sample_loc_type_top_field.hash(state);
            self.chroma_sample_loc_type_bottom_field.hash(state);
            self.reserved_1.hash(state);
            hash_ptr(self.p_hrd_parameters, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264ProfileInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_profile_idc.hash(state);
            self.picture_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_level_idc.hash(state);
            self.field_offset_granularity.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::h264std::StdVideoH264SequenceParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.profile_idc.hash(state);
            self.level_idc.hash(state);
            self.chroma_format_idc.hash(state);
            self.seq_parameter_set_id.hash(state);
            self.bit_depth_luma_minus_8.hash(state);
            self.bit_depth_chroma_minus_8.hash(state);
            self.log_2_max_frame_num_minus_4.hash(state);
            self.pic_order_cnt_type.hash(state);
            self.offset_for_non_ref_pic.hash(state);
            self.offset_for_top_to_bottom_field.hash(state);
            self.log_2_max_pic_order_cnt_lsb_minus_4.hash(state);
            self.num_ref_frames_in_pic_order_cnt_cycle.hash(state);
            self.max_num_ref_frames.hash(state);
            self.reserved_1.hash(state);
            self.pic_width_in_mbs_minus_1.hash(state);
            self.pic_height_in_map_units_minus_1.hash(state);
            self.frame_crop_left_offset.hash(state);
            self.frame_crop_right_offset.hash(state);
            self.frame_crop_top_offset.hash(state);
            self.frame_crop_bottom_offset.hash(state);
            self.reserved_2.hash(state);
            hash_ptr(self.p_offset_for_ref_frame, state);
            hash_ptr(self.p_scaling_lists, state);
            hash_ptr(self.p_sequence_parameter_set_vui, state);
        }
    }
}
impl DumbHash for crate::extensions::h264std::StdVideoH264PictureParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.seq_parameter_set_id.hash(state);
            self.pic_parameter_set_id.hash(state);
            self.num_ref_idx_l_0_default_active_minus_1.hash(state);
            self.num_ref_idx_l_1_default_active_minus_1.hash(state);
            self.weighted_bipred_idc.hash(state);
            self.pic_init_qp_minus_26.hash(state);
            self.pic_init_qs_minus_26.hash(state);
            self.chroma_qp_index_offset.hash(state);
            self.second_chroma_qp_index_offset.hash(state);
            hash_ptr(self.p_scaling_lists, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_spscount.hash(state);
            hash_raw_arr(self.p_std_spss, (self.std_spscount) as usize, state);
            self.std_ppscount.hash(state);
            hash_raw_arr(self.p_std_ppss, (self.std_ppscount) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_std_spscount.hash(state);
            self.max_std_ppscount.hash(state);
            hash_ptr(self.p_parameters_add_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_std_picture_info, state);
            self.slice_count.hash(state);
            hash_raw_arr(self.p_slice_offsets, (self.slice_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_std_reference_info, state);
        }
    }
}
impl DumbHash for crate::extensions::h265std::StdVideoH265VideoParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.vps_video_parameter_set_id.hash(state);
            self.vps_max_sub_layers_minus_1.hash(state);
            self.reserved_1.hash(state);
            self.reserved_2.hash(state);
            self.vps_num_units_in_tick.hash(state);
            self.vps_time_scale.hash(state);
            self.vps_num_ticks_poc_diff_one_minus_1.hash(state);
            self.reserved_3.hash(state);
            hash_ptr(self.p_dec_pic_buf_mgr, state);
            hash_ptr(self.p_hrd_parameters, state);
            hash_ptr(self.p_profile_tier_level, state);
        }
    }
}
impl DumbHash for crate::extensions::h265std::StdVideoH265SequenceParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.chroma_format_idc.hash(state);
            self.pic_width_in_luma_samples.hash(state);
            self.pic_height_in_luma_samples.hash(state);
            self.sps_video_parameter_set_id.hash(state);
            self.sps_max_sub_layers_minus_1.hash(state);
            self.sps_seq_parameter_set_id.hash(state);
            self.bit_depth_luma_minus_8.hash(state);
            self.bit_depth_chroma_minus_8.hash(state);
            self.log_2_max_pic_order_cnt_lsb_minus_4.hash(state);
            self.log_2_min_luma_coding_block_size_minus_3.hash(state);
            self.log_2_diff_max_min_luma_coding_block_size.hash(state);
            self.log_2_min_luma_transform_block_size_minus_2.hash(state);
            self.log_2_diff_max_min_luma_transform_block_size.hash(state);
            self.max_transform_hierarchy_depth_inter.hash(state);
            self.max_transform_hierarchy_depth_intra.hash(state);
            self.num_short_term_ref_pic_sets.hash(state);
            self.num_long_term_ref_pics_sps.hash(state);
            self.pcm_sample_bit_depth_luma_minus_1.hash(state);
            self.pcm_sample_bit_depth_chroma_minus_1.hash(state);
            self.log_2_min_pcm_luma_coding_block_size_minus_3.hash(state);
            self.log_2_diff_max_min_pcm_luma_coding_block_size.hash(state);
            self.reserved_1.hash(state);
            self.reserved_2.hash(state);
            self.palette_max_size.hash(state);
            self.delta_palette_max_predictor_size.hash(state);
            self.motion_vector_resolution_control_idc.hash(state);
            self.sps_num_palette_predictor_initializers_minus_1.hash(state);
            self.conf_win_left_offset.hash(state);
            self.conf_win_right_offset.hash(state);
            self.conf_win_top_offset.hash(state);
            self.conf_win_bottom_offset.hash(state);
            hash_ptr(self.p_profile_tier_level, state);
            hash_ptr(self.p_dec_pic_buf_mgr, state);
            hash_ptr(self.p_scaling_lists, state);
            hash_ptr(self.p_short_term_ref_pic_set, state);
            hash_ptr(self.p_long_term_ref_pics_sps, state);
            hash_ptr(self.p_sequence_parameter_set_vui, state);
            hash_ptr(self.p_predictor_palette_entries, state);
        }
    }
}
impl DumbHash for crate::extensions::h265std::StdVideoH265PictureParameterSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.pps_pic_parameter_set_id.hash(state);
            self.pps_seq_parameter_set_id.hash(state);
            self.sps_video_parameter_set_id.hash(state);
            self.num_extra_slice_header_bits.hash(state);
            self.num_ref_idx_l_0_default_active_minus_1.hash(state);
            self.num_ref_idx_l_1_default_active_minus_1.hash(state);
            self.init_qp_minus_26.hash(state);
            self.diff_cu_qp_delta_depth.hash(state);
            self.pps_cb_qp_offset.hash(state);
            self.pps_cr_qp_offset.hash(state);
            self.pps_beta_offset_div_2.hash(state);
            self.pps_tc_offset_div_2.hash(state);
            self.log_2_parallel_merge_level_minus_2.hash(state);
            self.log_2_max_transform_skip_block_size_minus_2.hash(state);
            self.diff_cu_chroma_qp_offset_depth.hash(state);
            self.chroma_qp_offset_list_len_minus_1.hash(state);
            self.cb_qp_offset_list.hash(state);
            self.cr_qp_offset_list.hash(state);
            self.log_2_sao_offset_scale_luma.hash(state);
            self.log_2_sao_offset_scale_chroma.hash(state);
            self.pps_act_y_qp_offset_plus_5.hash(state);
            self.pps_act_cb_qp_offset_plus_5.hash(state);
            self.pps_act_cr_qp_offset_plus_3.hash(state);
            self.pps_num_palette_predictor_initializers.hash(state);
            self.luma_bit_depth_entry_minus_8.hash(state);
            self.chroma_bit_depth_entry_minus_8.hash(state);
            self.num_tile_columns_minus_1.hash(state);
            self.num_tile_rows_minus_1.hash(state);
            self.reserved_1.hash(state);
            self.reserved_2.hash(state);
            self.column_width_minus_1.hash(state);
            self.row_height_minus_1.hash(state);
            self.reserved_3.hash(state);
            hash_ptr(self.p_scaling_lists, state);
            hash_ptr(self.p_predictor_palette_entries, state);
        }
    }
}
impl DumbHash for crate::extensions::h265std::StdVideoH265HrdParameters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.tick_divisor_minus_2.hash(state);
            self.du_cpb_removal_delay_increment_length_minus_1.hash(state);
            self.dpb_output_delay_du_length_minus_1.hash(state);
            self.bit_rate_scale.hash(state);
            self.cpb_size_scale.hash(state);
            self.cpb_size_du_scale.hash(state);
            self.initial_cpb_removal_delay_length_minus_1.hash(state);
            self.au_cpb_removal_delay_length_minus_1.hash(state);
            self.dpb_output_delay_length_minus_1.hash(state);
            self.cpb_cnt_minus_1.hash(state);
            self.elemental_duration_in_tc_minus_1.hash(state);
            self.reserved.hash(state);
            hash_ptr(self.p_sub_layer_hrd_parameters_nal, state);
            hash_ptr(self.p_sub_layer_hrd_parameters_vcl, state);
        }
    }
}
impl DumbHash for crate::extensions::h265std::StdVideoH265SequenceParameterSetVui {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.aspect_ratio_idc.hash(state);
            self.sar_width.hash(state);
            self.sar_height.hash(state);
            self.video_format.hash(state);
            self.colour_primaries.hash(state);
            self.transfer_characteristics.hash(state);
            self.matrix_coeffs.hash(state);
            self.chroma_sample_loc_type_top_field.hash(state);
            self.chroma_sample_loc_type_bottom_field.hash(state);
            self.reserved_1.hash(state);
            self.reserved_2.hash(state);
            self.def_disp_win_left_offset.hash(state);
            self.def_disp_win_right_offset.hash(state);
            self.def_disp_win_top_offset.hash(state);
            self.def_disp_win_bottom_offset.hash(state);
            self.vui_num_units_in_tick.hash(state);
            self.vui_time_scale.hash(state);
            self.vui_num_ticks_poc_diff_one_minus_1.hash(state);
            self.min_spatial_segmentation_idc.hash(state);
            self.reserved_3.hash(state);
            self.max_bytes_per_pic_denom.hash(state);
            self.max_bits_per_min_cu_denom.hash(state);
            self.log_2_max_mv_length_horizontal.hash(state);
            self.log_2_max_mv_length_vertical.hash(state);
            hash_ptr(self.p_hrd_parameters, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265ProfileInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_profile_idc.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_level_idc.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_vpscount.hash(state);
            hash_raw_arr(self.p_std_vpss, (self.std_vpscount) as usize, state);
            self.std_spscount.hash(state);
            hash_raw_arr(self.p_std_spss, (self.std_spscount) as usize, state);
            self.std_ppscount.hash(state);
            hash_raw_arr(self.p_std_ppss, (self.std_ppscount) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_std_vpscount.hash(state);
            self.max_std_spscount.hash(state);
            self.max_std_ppscount.hash(state);
            hash_ptr(self.p_parameters_add_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265PictureInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_std_picture_info, state);
            self.slice_count.hash(state);
            hash_raw_arr(self.p_slice_offsets, (self.slice_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_std_reference_info, state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.queue_family_index.hash(state);
            self.flags.hash(state);
            hash_ptr(self.p_video_profile, state);
            self.picture_format.hash(state);
            self.max_coded_extent.hash(state);
            self.reference_picture_format.hash(state);
            self.max_dpb_slots.hash(state);
            self.max_active_reference_pictures.hash(state);
            hash_ptr(self.p_std_header_version, state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.video_session_parameters_template.hash(state);
            self.video_session.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.update_sequence_count.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoBeginCodingInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.video_session.hash(state);
            self.video_session_parameters.hash(state);
            self.reference_slot_count.hash(state);
            hash_raw_arr(
                self.p_reference_slots,
                (self.reference_slot_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoEndCodingInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_queue::VideoCodingControlInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_encode_queue::VideoEncodeUsageInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.video_usage_hints.hash(state);
            self.video_content_hints.hash(state);
            self.tuning_mode.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_encode_queue::VideoEncodeInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.quality_level.hash(state);
            self.dst_bitstream_buffer.hash(state);
            self.dst_bitstream_buffer_offset.hash(state);
            self.dst_bitstream_buffer_max_range.hash(state);
            self.src_picture_resource.hash(state);
            hash_ptr(self.p_setup_reference_slot, state);
            self.reference_slot_count.hash(state);
            hash_raw_arr(
                self.p_reference_slots,
                (self.reference_slot_count) as usize,
                state,
            );
            self.preceding_externally_encoded_bytes.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.rate_control_mode.hash(state);
            self.layer_count.hash(state);
            hash_raw_arr(self.p_layer_configs, (self.layer_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.average_bitrate.hash(state);
            self.max_bitrate.hash(state);
            self.frame_rate_numerator.hash(state);
            self.frame_rate_denominator.hash(state);
            self.virtual_buffer_size_in_ms.hash(state);
            self.initial_virtual_buffer_size_in_ms.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.rate_control_modes.hash(state);
            self.rate_control_layer_count.hash(state);
            self.quality_level_count.hash(state);
            self.input_image_data_fill_alignment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.input_mode_flags.hash(state);
            self.output_mode_flags.hash(state);
            self.max_ppicture_l_0_reference_count.hash(state);
            self.max_bpicture_l_0_reference_count.hash(state);
            self.max_l_1_reference_count.hash(state);
            self.motion_vectors_over_pic_boundaries_flag.hash(state);
            self.max_bytes_per_pic_denom.hash(state);
            self.max_bits_per_mb_denom.hash(state);
            self.log_2_max_mv_length_horizontal.hash(state);
            self.log_2_max_mv_length_vertical.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::h264std_encode::StdVideoEncodeH264SliceHeader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.first_mb_in_slice.hash(state);
            self.slice_type.hash(state);
            self.idr_pic_id.hash(state);
            self.num_ref_idx_l_0_active_minus_1.hash(state);
            self.num_ref_idx_l_1_active_minus_1.hash(state);
            self.cabac_init_idc.hash(state);
            self.disable_deblocking_filter_idc.hash(state);
            self.slice_alpha_c_0_offset_div_2.hash(state);
            self.slice_beta_offset_div_2.hash(state);
            hash_ptr(self.p_weight_table, state);
        }
    }
}
impl DumbHash
for crate::extensions::h264std_encode::StdVideoEncodeH264RefMemMgmtCtrlOperations {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.ref_list_0_mod_op_count.hash(state);
            hash_ptr(self.p_ref_list_0_mod_operations, state);
            self.ref_list_1_mod_op_count.hash(state);
            hash_ptr(self.p_ref_list_1_mod_operations, state);
            self.ref_pic_marking_op_count.hash(state);
            hash_ptr(self.p_ref_pic_marking_operations, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_spscount.hash(state);
            hash_raw_arr(self.p_std_spss, (self.std_spscount) as usize, state);
            self.std_ppscount.hash(state);
            hash_raw_arr(self.p_std_ppss, (self.std_ppscount) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_std_spscount.hash(state);
            self.max_std_ppscount.hash(state);
            hash_ptr(self.p_parameters_add_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.slot_index.hash(state);
            hash_ptr(self.p_std_reference_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264VclFrameInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_reference_final_lists, state);
            self.nalu_slice_entry_count.hash(state);
            hash_raw_arr(
                self.p_nalu_slice_entries,
                (self.nalu_slice_entry_count) as usize,
                state,
            );
            hash_ptr(self.p_current_picture_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264ReferenceListsInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.reference_list_0_entry_count.hash(state);
            hash_raw_arr(
                self.p_reference_list_0_entries,
                (self.reference_list_0_entry_count) as usize,
                state,
            );
            self.reference_list_1_entry_count.hash(state);
            hash_raw_arr(
                self.p_reference_list_1_entries,
                (self.reference_list_1_entry_count) as usize,
                state,
            );
            hash_ptr(self.p_mem_mgmt_ctrl_operations, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264EmitPictureParametersInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.sps_id.hash(state);
            self.emit_sps_enable.hash(state);
            self.pps_id_entry_count.hash(state);
            hash_raw_arr(self.pps_id_entries, (self.pps_id_entry_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264ProfileInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_profile_idc.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264NaluSliceInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mb_count.hash(state);
            hash_ptr(self.p_reference_final_lists, state);
            hash_ptr(self.p_slice_header_std, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.gop_frame_count.hash(state);
            self.idr_period.hash(state);
            self.consecutive_bframe_count.hash(state);
            self.rate_control_structure.hash(state);
            self.temporal_layer_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h264::VideoEncodeH264RateControlLayerInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.temporal_layer_id.hash(state);
            self.use_initial_rc_qp.hash(state);
            self.initial_rc_qp.hash(state);
            self.use_min_qp.hash(state);
            self.min_qp.hash(state);
            self.use_max_qp.hash(state);
            self.max_qp.hash(state);
            self.use_max_frame_size.hash(state);
            self.max_frame_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265CapabilitiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.input_mode_flags.hash(state);
            self.output_mode_flags.hash(state);
            self.ctb_sizes.hash(state);
            self.transform_block_sizes.hash(state);
            self.max_ppicture_l_0_reference_count.hash(state);
            self.max_bpicture_l_0_reference_count.hash(state);
            self.max_l_1_reference_count.hash(state);
            self.max_sub_layers_count.hash(state);
            self.min_log_2_min_luma_coding_block_size_minus_3.hash(state);
            self.max_log_2_min_luma_coding_block_size_minus_3.hash(state);
            self.min_log_2_min_luma_transform_block_size_minus_2.hash(state);
            self.max_log_2_min_luma_transform_block_size_minus_2.hash(state);
            self.min_max_transform_hierarchy_depth_inter.hash(state);
            self.max_max_transform_hierarchy_depth_inter.hash(state);
            self.min_max_transform_hierarchy_depth_intra.hash(state);
            self.max_max_transform_hierarchy_depth_intra.hash(state);
            self.max_diff_cu_qp_delta_depth.hash(state);
            self.min_max_num_merge_cand.hash(state);
            self.max_max_num_merge_cand.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::h265std_encode::StdVideoEncodeH265SliceSegmentHeader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.slice_type.hash(state);
            self.slice_segment_address.hash(state);
            self.short_term_ref_pic_set_idx.hash(state);
            self.collocated_ref_idx.hash(state);
            self.num_ref_idx_l_0_active_minus_1.hash(state);
            self.num_ref_idx_l_1_active_minus_1.hash(state);
            self.max_num_merge_cand.hash(state);
            self.slice_cb_qp_offset.hash(state);
            self.slice_cr_qp_offset.hash(state);
            self.slice_beta_offset_div_2.hash(state);
            self.slice_tc_offset_div_2.hash(state);
            self.slice_act_y_qp_offset.hash(state);
            self.slice_act_cb_qp_offset.hash(state);
            self.slice_act_cr_qp_offset.hash(state);
            hash_ptr(self.p_short_term_ref_pic_set, state);
            hash_ptr(self.p_long_term_ref_pics, state);
            hash_ptr(self.p_weight_table, state);
        }
    }
}
impl DumbHash
for crate::extensions::h265std_encode::StdVideoEncodeH265ReferenceModifications {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.flags.hash(state);
            self.reference_list_0_modifications_count.hash(state);
            hash_ptr(self.p_reference_list_0_modifications, state);
            self.reference_list_1_modifications_count.hash(state);
            hash_ptr(self.p_reference_list_1_modifications, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_vpscount.hash(state);
            hash_raw_arr(self.p_std_vpss, (self.std_vpscount) as usize, state);
            self.std_spscount.hash(state);
            hash_raw_arr(self.p_std_spss, (self.std_spscount) as usize, state);
            self.std_ppscount.hash(state);
            hash_raw_arr(self.p_std_ppss, (self.std_ppscount) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_std_vpscount.hash(state);
            self.max_std_spscount.hash(state);
            self.max_std_ppscount.hash(state);
            hash_ptr(self.p_parameters_add_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265VclFrameInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_reference_final_lists, state);
            self.nalu_slice_segment_entry_count.hash(state);
            hash_raw_arr(
                self.p_nalu_slice_segment_entries,
                (self.nalu_slice_segment_entry_count) as usize,
                state,
            );
            hash_ptr(self.p_current_picture_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265EmitPictureParametersInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vps_id.hash(state);
            self.sps_id.hash(state);
            self.emit_vps_enable.hash(state);
            self.emit_sps_enable.hash(state);
            self.pps_id_entry_count.hash(state);
            hash_raw_arr(self.pps_id_entries, (self.pps_id_entry_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265NaluSliceSegmentInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ctb_count.hash(state);
            hash_ptr(self.p_reference_final_lists, state);
            hash_ptr(self.p_slice_segment_header_std, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.gop_frame_count.hash(state);
            self.idr_period.hash(state);
            self.consecutive_bframe_count.hash(state);
            self.rate_control_structure.hash(state);
            self.sub_layer_count.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265RateControlLayerInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.temporal_id.hash(state);
            self.use_initial_rc_qp.hash(state);
            self.initial_rc_qp.hash(state);
            self.use_min_qp.hash(state);
            self.min_qp.hash(state);
            self.use_max_qp.hash(state);
            self.max_qp.hash(state);
            self.use_max_frame_size.hash(state);
            self.max_frame_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265ProfileInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.std_profile_idc.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.slot_index.hash(state);
            hash_ptr(self.p_std_reference_info, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.reference_list_0_entry_count.hash(state);
            hash_raw_arr(
                self.p_reference_list_0_entries,
                (self.reference_list_0_entry_count) as usize,
                state,
            );
            self.reference_list_1_entry_count.hash(state);
            hash_raw_arr(
                self.p_reference_list_1_entries,
                (self.reference_list_1_entry_count) as usize,
                state,
            );
            hash_ptr(self.p_reference_modifications, state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.inherited_viewport_scissor_2_d.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_inherited_viewport_scissor::CommandBufferInheritanceViewportScissorInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.viewport_scissor_2_d.hash(state);
            self.viewport_depth_count.hash(state);
            hash_ptr(self.p_viewport_depths, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ycbcr_2plane_444_formats.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.provoking_vertex_last.hash(state);
            self.transform_feedback_preserves_provoking_vertex.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.provoking_vertex_mode_per_pipeline.hash(state);
            self.transform_feedback_preserves_triangle_fan_provoking_vertex.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_provoking_vertex::PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.provoking_vertex_mode.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.data_size.hash(state);
            hash_raw_arr(self.p_data.cast::<u8>(), (self.data_size) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.module.hash(state);
            hash_cstr(self.p_name, state);
        }
    }
}
impl DumbHash for crate::extensions::nvx_binary_import::CuLaunchInfoNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.function.hash(state);
            self.grid_dim_x.hash(state);
            self.grid_dim_y.hash(state);
            self.grid_dim_z.hash(state);
            self.block_dim_x.hash(state);
            self.block_dim_y.hash(state);
            self.block_dim_z.hash(state);
            self.shared_mem_bytes.hash(state);
            self.param_count.hash(state);
            let len = (self.param_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.p_params.add(i);
                ptr.hash(state);
            }
            self.extra_count.hash(state);
            let len = (self.extra_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.p_extras.add(i);
                ptr.hash(state);
            }
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceShaderIntegerDotProductFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_integer_dot_product.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceShaderIntegerDotProductProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.integer_dot_product_8_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_8_bit_signed_accelerated.hash(state);
            self.integer_dot_product_8_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_unsigned_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_signed_accelerated.hash(state);
            self.integer_dot_product_4x_8_bit_packed_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_16_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_16_bit_signed_accelerated.hash(state);
            self.integer_dot_product_16_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_32_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_32_bit_signed_accelerated.hash(state);
            self.integer_dot_product_32_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_64_bit_unsigned_accelerated.hash(state);
            self.integer_dot_product_64_bit_signed_accelerated.hash(state);
            self.integer_dot_product_64_bit_mixed_signedness_accelerated.hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_4x_8_bit_packed_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated
                .hash(state);
            self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.has_primary.hash(state);
            self.has_render.hash(state);
            self.primary_major.hash(state);
            self.primary_minor.hash(state);
            self.render_major.hash(state);
            self.render_minor.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.fragment_shader_barycentric.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.tri_strip_vertex_order_independent_of_provoking_vertex.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.ray_tracing_motion_blur.hash(state);
            self.ray_tracing_motion_blur_pipeline_trace_rays_indirect.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.vertex_data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_instances.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_ray_tracing_motion_blur::SRTDataNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.sx.hash(state);
            self.a.hash(state);
            self.b.hash(state);
            self.pvx.hash(state);
            self.sy.hash(state);
            self.c.hash(state);
            self.pvy.hash(state);
            self.sz.hash(state);
            self.pvz.hash(state);
            self.qx.hash(state);
            self.qy.hash(state);
            self.qz.hash(state);
            self.qw.hash(state);
            self.tx.hash(state);
            self.ty.hash(state);
            self.tz.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureSRTMotionInstanceNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.transform_t_0.hash(state);
            self.transform_t_1.hash(state);
            self.instance_custom_index_mask.hash(state);
            self.instance_shader_binding_table_record_offset.hash(state);
            self.flags.hash(state);
            self.acceleration_structure_reference.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMatrixMotionInstanceNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.transform_t_0.hash(state);
            self.transform_t_1.hash(state);
            self.instance_custom_index_mask.hash(state);
            self.instance_shader_binding_table_record_offset.hash(state);
            self.flags.hash(state);
            self.acceleration_structure_reference.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    std::mem::size_of::<Self>(),
                )
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.kind.hash(state);
            self.flags.hash(state);
            self.data.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.handle_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::ImportMemoryBufferCollectionFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.collection.hash(state);
            self.index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferCollectionImageCreateInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.collection.hash(state);
            self.index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferCollectionBufferCreateInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.collection.hash(state);
            self.index.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.collection_token.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory_type_bits.hash(state);
            self.buffer_count.hash(state);
            self.create_info_index.hash(state);
            self.sysmem_pixel_format.hash(state);
            self.format_features.hash(state);
            self.sysmem_color_space_index.hash(state);
            self.sampler_ycbcr_conversion_components.hash(state);
            self.suggested_ycbcr_model.hash(state);
            self.suggested_ycbcr_range.hash(state);
            self.suggested_xchroma_offset.hash(state);
            self.suggested_ychroma_offset.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.create_info.hash(state);
            self.required_format_features.hash(state);
            self.buffer_collection_constraints.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.color_space.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_create_info.hash(state);
            self.required_format_features.hash(state);
            self.flags.hash(state);
            self.sysmem_pixel_format.hash(state);
            self.color_space_count.hash(state);
            hash_raw_arr(self.p_color_spaces, (self.color_space_count) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format_constraints_count.hash(state);
            hash_raw_arr(
                self.p_format_constraints,
                (self.format_constraints_count) as usize,
                state,
            );
            self.buffer_collection_constraints.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_buffer_count.hash(state);
            self.max_buffer_count.hash(state);
            self.min_buffer_count_for_camping.hash(state);
            self.min_buffer_count_for_dedicated_slack.hash(state);
            self.min_buffer_count_for_shared_slack.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_rgba10x6_formats::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format_rgba_10x_6_without_ycb_cr_sampler.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::FormatProperties3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.linear_tiling_features.hash(state);
            self.optimal_tiling_features.hash(state);
            self.buffer_features.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.drm_format_modifier_count.hash(state);
            hash_raw_arr(
                self.p_drm_format_modifier_properties,
                (self.drm_format_modifier_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatProperties2ANDROID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
            self.external_format.hash(state);
            self.format_features.hash(state);
            self.sampler_ycbcr_conversion_components.hash(state);
            self.suggested_ycbcr_model.hash(state);
            self.suggested_ycbcr_range.hash(state);
            self.suggested_xchroma_offset.hash(state);
            self.suggested_ychroma_offset.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PipelineRenderingCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
                state,
            );
            self.depth_attachment_format.hash(state);
            self.stencil_attachment_format.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::RenderingInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.render_area.hash(state);
            self.layer_count.hash(state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachments,
                (self.color_attachment_count) as usize,
                state,
            );
            hash_ptr(self.p_depth_attachment, state);
            hash_ptr(self.p_stencil_attachment, state);
        }
    }
}
impl DumbHash for crate::vk13::RenderingAttachmentInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view.hash(state);
            self.image_layout.hash(state);
            self.resolve_mode.hash(state);
            self.resolve_image_view.hash(state);
            self.resolve_image_layout.hash(state);
            self.load_op.hash(state);
            self.store_op.hash(state);
            self.clear_value.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view.hash(state);
            self.image_layout.hash(state);
            self.shading_rate_attachment_texel_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_view.hash(state);
            self.image_layout.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::PhysicalDeviceDynamicRenderingFeatures {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.dynamic_rendering.hash(state);
        }
    }
}
impl DumbHash for crate::vk13::CommandBufferInheritanceRenderingInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.view_mask.hash(state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachment_formats,
                (self.color_attachment_count) as usize,
                state,
            );
            self.depth_attachment_format.hash(state);
            self.stencil_attachment_format.hash(state);
            self.rasterization_samples.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.color_attachment_count.hash(state);
            hash_raw_arr(
                self.p_color_attachment_samples,
                (self.color_attachment_count) as usize,
                state,
            );
            self.depth_stencil_attachment_samples.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.per_view_attributes.hash(state);
            self.per_view_attributes_position_xonly.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_lod.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_view_min_lod::ImageViewMinLodCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.min_lod.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_rasterization_order_attachment_access::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.rasterization_order_color_attachment_access.hash(state);
            self.rasterization_order_depth_attachment_access.hash(state);
            self.rasterization_order_stencil_attachment_access.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.linear_color_attachment.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.graphics_pipeline_library.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.graphics_pipeline_library_fast_linking.hash(state);
            self.graphics_pipeline_library_independent_interpolation_decoration
                .hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::valve_descriptor_set_host_mapping::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_set_host_mapping.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetBindingReferenceVALVE {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_set_layout.hash(state);
            self.binding.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::valve_descriptor_set_host_mapping::DescriptorSetLayoutHostMappingInfoVALVE {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.descriptor_offset.hash(state);
            self.descriptor_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_module_identifier.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_module_identifier_algorithm_uuid.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_module_identifier::PipelineShaderStageModuleIdentifierCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.identifier_size.hash(state);
            hash_raw_arr(self.p_identifier, (self.identifier_size) as usize, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.identifier_size.hash(state);
            self.identifier.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control::ImageCompressionControlEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.compression_control_plane_count.hash(state);
            hash_raw_arr(
                self.p_fixed_rate_flags,
                (self.compression_control_plane_count) as usize,
                state,
            );
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control::PhysicalDeviceImageCompressionControlFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_compression_control.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control::ImageCompressionPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_compression_flags.hash(state);
            self.image_compression_fixed_rate_flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control_swapchain::PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_compression_control_swapchain.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control::ImageSubresource2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image_subresource.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_image_compression_control::SubresourceLayout2EXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subresource_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_subpass_merge_feedback::RenderPassCreationControlEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.disallow_merging.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_subpass_merge_feedback::RenderPassCreationFeedbackCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_render_pass_feedback, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_subpass_merge_feedback::RenderPassSubpassFeedbackCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_ptr(self.p_subpass_feedback, state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_subpass_merge_feedback::PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.subpass_merge_feedback.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::MicromapBuildInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.kind.hash(state);
            self.flags.hash(state);
            self.mode.hash(state);
            self.dst_micromap.hash(state);
            self.usage_counts_count.hash(state);
            hash_raw_arr(self.p_usage_counts, (self.usage_counts_count) as usize, state);
            let len = (self.usage_counts_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_usage_counts.add(i);
                hash_raw_arr(ptr, (1) as usize, state);
            }
            self.data.hash(state);
            self.scratch_data.hash(state);
            self.triangle_array.hash(state);
            self.triangle_array_stride.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::MicromapCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.create_flags.hash(state);
            self.buffer.hash(state);
            self.offset.hash(state);
            self.size.hash(state);
            self.kind.hash(state);
            self.device_address.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::MicromapVersionInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            hash_raw_arr(
                self.p_version_data,
                (2 * crate::vk10::UUID_SIZE) as usize,
                state,
            );
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::CopyMicromapInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.src.hash(state);
            self.dst.hash(state);
            self.mode.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_opacity_micromap::MicromapBuildSizesInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.micromap_size.hash(state);
            self.build_scratch_size.hash(state);
            self.discardable.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.micromap.hash(state);
            self.micromap_capture_replay.hash(state);
            self.micromap_host_commands.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_opacity_2_state_subdivision_level.hash(state);
            self.max_opacity_4_state_subdivision_level.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_opacity_micromap::AccelerationStructureTrianglesOpacityMicromapEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.index_type.hash(state);
            self.index_buffer.hash(state);
            self.index_stride.hash(state);
            self.base_triangle.hash(state);
            self.usage_counts_count.hash(state);
            hash_raw_arr(self.p_usage_counts, (self.usage_counts_count) as usize, state);
            let len = (self.usage_counts_count) as usize;
            len.hash(state);
            for i in 0..len {
                let ptr = *self.pp_usage_counts.add(i);
                hash_raw_arr(ptr, (1) as usize, state);
            }
            self.micromap.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_properties::PipelinePropertiesIdentifierEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_identifier.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_properties::PhysicalDevicePipelinePropertiesFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_properties_identifier.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::amd_shader_early_and_late_fragment_tests::PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.shader_early_and_late_fragment_tests.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalObjectCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.export_object_type.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalObjectsInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalDeviceInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mtl_device.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalCommandQueueInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.queue.hash(state);
            self.mtl_command_queue.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalBufferInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.memory.hash(state);
            self.mtl_buffer.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ImportMetalBufferInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mtl_buffer.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalTextureInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.image_view.hash(state);
            self.buffer_view.hash(state);
            self.plane.hash(state);
            self.mtl_texture.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ImportMetalTextureInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.plane.hash(state);
            self.mtl_texture.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalIOSurfaceInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.image.hash(state);
            self.io_surface.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ImportMetalIOSurfaceInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.io_surface.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ExportMetalSharedEventInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.semaphore.hash(state);
            self.event.hash(state);
            self.mtl_shared_event.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_metal_objects::ImportMetalSharedEventInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.mtl_shared_event.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_non_seamless_cube_map::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.non_seamless_cube_map.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.pipeline_robustness.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_robustness::PipelineRobustnessCreateInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.storage_buffers.hash(state);
            self.uniform_buffers.hash(state);
            self.vertex_inputs.hash(state);
            self.images.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessPropertiesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.default_robustness_storage_buffers.hash(state);
            self.default_robustness_uniform_buffers.hash(state);
            self.default_robustness_vertex_inputs.hash(state);
            self.default_robustness_images.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_image_processing::ImageViewSampleWeightCreateInfoQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.filter_center.hash(state);
            self.filter_size.hash(state);
            self.num_phases.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_image_processing::PhysicalDeviceImageProcessingFeaturesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.texture_sample_weighted.hash(state);
            self.texture_box_filter.hash(state);
            self.texture_block_match.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_image_processing::PhysicalDeviceImageProcessingPropertiesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.max_weight_filter_phases.hash(state);
            self.max_weight_filter_dimension.hash(state);
            self.max_block_match_region.hash(state);
            self.max_box_filter_block_size.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::qcom_tile_properties::PhysicalDeviceTilePropertiesFeaturesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.tile_properties.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::qcom_tile_properties::TilePropertiesQCOM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.tile_size.hash(state);
            self.apron_size.hash(state);
            self.origin.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::sec_amigo_profiling::PhysicalDeviceAmigoProfilingFeaturesSEC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.amigo_profiling.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::sec_amigo_profiling::AmigoProfilingSubmitInfoSEC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.first_draw_timestamp.hash(state);
            self.swap_buffer_timestamp.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_attachment_feedback_loop_layout::PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.attachment_feedback_loop_layout.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_depth_clamp_zero_one::PhysicalDeviceDepthClampZeroOneFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.depth_clamp_zero_one.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_device_address_binding_report::PhysicalDeviceAddressBindingReportFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.report_address_binding.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::ext_device_address_binding_report::DeviceAddressBindingCallbackDataEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.base_address.hash(state);
            self.size.hash(state);
            self.binding_type.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowFeaturesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.optical_flow.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.supported_output_grid_sizes.hash(state);
            self.supported_hint_grid_sizes.hash(state);
            self.hint_supported.hash(state);
            self.cost_supported.hash(state);
            self.bidirectional_flow_supported.hash(state);
            self.global_flow_supported.hash(state);
            self.min_width.hash(state);
            self.min_height.hash(state);
            self.max_width.hash(state);
            self.max_height.hash(state);
            self.max_num_regions_of_interest.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_optical_flow::OpticalFlowImageFormatInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.usage.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_optical_flow::OpticalFlowImageFormatPropertiesNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.format.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_optical_flow::OpticalFlowSessionCreateInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.width.hash(state);
            self.height.hash(state);
            self.image_format.hash(state);
            self.flow_vector_format.hash(state);
            self.cost_format.hash(state);
            self.output_grid_size.hash(state);
            self.hint_grid_size.hash(state);
            self.performance_level.hash(state);
            self.flags.hash(state);
        }
    }
}
impl DumbHash
for crate::extensions::nv_optical_flow::OpticalFlowSessionCreatePrivateDataInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.id.hash(state);
            self.size.hash(state);
            self.p_private_data.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::nv_optical_flow::OpticalFlowExecuteInfoNV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.flags.hash(state);
            self.region_count.hash(state);
            hash_raw_arr(self.p_regions, (self.region_count) as usize, state);
        }
    }
}
impl DumbHash for crate::extensions::ext_device_fault::PhysicalDeviceFaultFeaturesEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.device_fault.hash(state);
            self.device_fault_vendor_binary.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_device_fault::DeviceFaultCountsEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.address_info_count.hash(state);
            self.vendor_info_count.hash(state);
            self.vendor_binary_size.hash(state);
        }
    }
}
impl DumbHash for crate::extensions::ext_device_fault::DeviceFaultInfoEXT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.s_type.hash(state);
            hash_pnext(self.p_next, state);
            self.description.hash(state);
            hash_ptr(self.p_address_infos, state);
            hash_ptr(self.p_vendor_infos, state);
            self.p_vendor_binary_data.hash(state);
        }
    }
}
dumb_hash_passthrough_impl! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, std::ffi::CStr, crate
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
    ::vk10::Rect2D, crate ::vk10::ClearRect, crate ::vk10::ComponentMapping, crate
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
    ::vk10::AttachmentDescription, crate ::vk10::AttachmentReference, crate
    ::vk10::SubpassDependency, crate ::vk10::PhysicalDeviceFeatures, crate
    ::vk10::PhysicalDeviceSparseProperties, crate ::vk10::DrawIndirectCommand, crate
    ::vk10::DrawIndexedIndirectCommand, crate ::vk10::DispatchIndirectCommand, crate
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
    ::extensions::google_display_timing::RefreshCycleDurationGOOGLE, crate
    ::extensions::google_display_timing::PastPresentationTimingGOOGLE, crate
    ::extensions::google_display_timing::PresentTimeGOOGLE, crate
    ::extensions::nv_viewport_swizzle::ViewportSwizzleNV, crate
    ::vk11::InputAttachmentAspectReference, crate
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
    ::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR,
    crate ::extensions::ext_extended_dynamic_state3::ColorBlendEquationEXT, crate
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
    ::extensions::ext_image_drm_format_modifier::DrmFormatModifierProperties2EXT, crate
    ::extensions::ext_subpass_merge_feedback::RenderPassCreationFeedbackInfoEXT, crate
    ::extensions::ext_subpass_merge_feedback::RenderPassSubpassFeedbackInfoEXT, crate
    ::extensions::ext_opacity_micromap::MicromapUsageEXT, crate
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
