#[derive(Clone)]
#[repr(C)]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicStateFeaturesEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html)
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: crate::vk10::StructureType,
    pub p_next: *mut std::os::raw::c_void,
    pub extended_dynamic_state: crate::vk10::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state: Default::default(),
        }
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetCullModeEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html)
pub unsafe fn cmd_set_cull_mode_ext(
    command_buffer: crate::vk10::CommandBuffer,
    cull_mode: crate::vk10::CullModeFlags,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_cull_mode
        .unwrap())(command_buffer, cull_mode)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetCullModeEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html)
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        cull_mode: Option<crate::vk10::CullModeFlags>,
    ) {
        let cmd_set_cull_mode_ext = (*self.table).cmd_set_cull_mode_ext.unwrap();
        cmd_set_cull_mode_ext(
            command_buffer,
            match cull_mode {
                Some(v) => v,
                None => Default::default(),
            },
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetFrontFaceEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html)
pub unsafe fn cmd_set_front_face_ext(
    command_buffer: crate::vk10::CommandBuffer,
    front_face: crate::vk10::FrontFace,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_front_face
        .unwrap())(command_buffer, front_face)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html)
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        front_face: crate::vk10::FrontFace,
    ) {
        let cmd_set_front_face_ext = (*self.table).cmd_set_front_face_ext.unwrap();
        cmd_set_front_face_ext(command_buffer, front_face as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html)
pub unsafe fn cmd_set_primitive_topology_ext(
    command_buffer: crate::vk10::CommandBuffer,
    primitive_topology: crate::vk10::PrimitiveTopology,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_primitive_topology
        .unwrap())(command_buffer, primitive_topology)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html)
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        primitive_topology: crate::vk10::PrimitiveTopology,
    ) {
        let cmd_set_primitive_topology_ext = (*self.table)
            .cmd_set_primitive_topology_ext
            .unwrap();
        cmd_set_primitive_topology_ext(command_buffer, primitive_topology as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html)
pub unsafe fn cmd_set_viewport_with_count_ext(
    command_buffer: crate::vk10::CommandBuffer,
    viewport_count: u32,
    p_viewports: *const crate::vk10::Viewport,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_viewport_with_count
        .unwrap())(command_buffer, viewport_count, p_viewports)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html)
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        viewports: &[crate::vk10::Viewport],
    ) {
        let cmd_set_viewport_with_count_ext = (*self.table)
            .cmd_set_viewport_with_count_ext
            .unwrap();
        let viewport_count = viewports.len();
        cmd_set_viewport_with_count_ext(
            command_buffer,
            viewport_count as _,
            viewports.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html)
pub unsafe fn cmd_set_scissor_with_count_ext(
    command_buffer: crate::vk10::CommandBuffer,
    scissor_count: u32,
    p_scissors: *const crate::vk10::Rect2D,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_scissor_with_count
        .unwrap())(command_buffer, scissor_count, p_scissors)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html)
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        scissors: &[crate::vk10::Rect2D],
    ) {
        let cmd_set_scissor_with_count_ext = (*self.table)
            .cmd_set_scissor_with_count_ext
            .unwrap();
        let scissor_count = scissors.len();
        cmd_set_scissor_with_count_ext(
            command_buffer,
            scissor_count as _,
            scissors.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
pub unsafe fn cmd_bind_vertex_buffers_2_ext(
    command_buffer: crate::vk10::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk10::Buffer,
    p_offsets: *const crate::vk10::DeviceSize,
    p_sizes: *const crate::vk10::DeviceSize,
    p_strides: *const crate::vk10::DeviceSize,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_bind_vertex_buffers_2
        .unwrap())(
        command_buffer,
        first_binding,
        binding_count,
        p_buffers,
        p_offsets,
        p_sizes,
        p_strides,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
    pub unsafe fn cmd_bind_vertex_buffers_2_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk10::Buffer],
        offsets: &[crate::vk10::DeviceSize],
        sizes: &[crate::vk10::DeviceSize],
        strides: &[crate::vk10::DeviceSize],
    ) {
        let cmd_bind_vertex_buffers_2_ext = (*self.table)
            .cmd_bind_vertex_buffers_2_ext
            .unwrap();
        let binding_count = buffers
            .len()
            .min(offsets.len())
            .min(sizes.len())
            .min(strides.len());
        cmd_bind_vertex_buffers_2_ext(
            command_buffer,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr(),
            offsets.as_ptr(),
            sizes.as_ptr(),
            strides.as_ptr(),
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html)
pub unsafe fn cmd_set_depth_test_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_test_enable
        .unwrap())(command_buffer, depth_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html)
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_test_enable: bool,
    ) {
        let cmd_set_depth_test_enable_ext = (*self.table)
            .cmd_set_depth_test_enable_ext
            .unwrap();
        cmd_set_depth_test_enable_ext(command_buffer, depth_test_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html)
pub unsafe fn cmd_set_depth_write_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_write_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_write_enable
        .unwrap())(command_buffer, depth_write_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html)
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_write_enable: bool,
    ) {
        let cmd_set_depth_write_enable_ext = (*self.table)
            .cmd_set_depth_write_enable_ext
            .unwrap();
        cmd_set_depth_write_enable_ext(command_buffer, depth_write_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html)
pub unsafe fn cmd_set_depth_compare_op_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_compare_op: crate::vk10::CompareOp,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_compare_op
        .unwrap())(command_buffer, depth_compare_op)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html)
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_compare_op: crate::vk10::CompareOp,
    ) {
        let cmd_set_depth_compare_op_ext = (*self.table)
            .cmd_set_depth_compare_op_ext
            .unwrap();
        cmd_set_depth_compare_op_ext(command_buffer, depth_compare_op as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html)
pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    depth_bounds_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_depth_bounds_test_enable
        .unwrap())(command_buffer, depth_bounds_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html)
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        let cmd_set_depth_bounds_test_enable_ext = (*self.table)
            .cmd_set_depth_bounds_test_enable_ext
            .unwrap();
        cmd_set_depth_bounds_test_enable_ext(
            command_buffer,
            depth_bounds_test_enable as _,
        );
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html)
pub unsafe fn cmd_set_stencil_test_enable_ext(
    command_buffer: crate::vk10::CommandBuffer,
    stencil_test_enable: crate::vk10::Bool32,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_test_enable
        .unwrap())(command_buffer, stencil_test_enable)
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html)
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        stencil_test_enable: bool,
    ) {
        let cmd_set_stencil_test_enable_ext = (*self.table)
            .cmd_set_stencil_test_enable_ext
            .unwrap();
        cmd_set_stencil_test_enable_ext(command_buffer, stencil_test_enable as _);
    }
}
#[track_caller]
#[cfg(all(feature = "global", feature = "raw"))]
#[doc(alias = "vkCmdSetStencilOpEXT")]
/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html)
pub unsafe fn cmd_set_stencil_op_ext(
    command_buffer: crate::vk10::CommandBuffer,
    face_mask: crate::vk10::StencilFaceFlags,
    fail_op: crate::vk10::StencilOp,
    pass_op: crate::vk10::StencilOp,
    depth_fail_op: crate::vk10::StencilOp,
    compare_op: crate::vk10::CompareOp,
) {
    (crate::loader::tables::GLOBAL_DEVICE_TABLE
        .cmd_set_stencil_op
        .unwrap())(
        command_buffer,
        face_mask,
        fail_op,
        pass_op,
        depth_fail_op,
        compare_op,
    )
}
#[cfg(feature = "wrappers")]
impl crate::DeviceWrapper {
    #[track_caller]
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    /// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html)
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: crate::vk10::CommandBuffer,
        face_mask: crate::vk10::StencilFaceFlags,
        fail_op: crate::vk10::StencilOp,
        pass_op: crate::vk10::StencilOp,
        depth_fail_op: crate::vk10::StencilOp,
        compare_op: crate::vk10::CompareOp,
    ) {
        let cmd_set_stencil_op_ext = (*self.table).cmd_set_stencil_op_ext.unwrap();
        cmd_set_stencil_op_ext(
            command_buffer,
            face_mask as _,
            fail_op as _,
            pass_op as _,
            depth_fail_op as _,
            compare_op as _,
        );
    }
}
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
pub const EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: &std::ffi::CStr = crate::cstr!(
    "VK_EXT_extended_dynamic_state"
);
