

// CODEGEN START

#[macro_export]
macro_rules! pnext_visit {
    ($pnext:expr, $stype:ident, $object:ident, $op:expr) => {
        let $stype = * $pnext .cast:: < $crate ::vk10::StructureType > (); match $stype {
        $crate ::vk10::StructureType::APPLICATION_INFO => { let $object = $pnext .cast::
        < crate ::vk10::ApplicationInfo > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_QUEUE_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk10::DeviceQueueCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::DEVICE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::DeviceCreateInfo > (); $op; $pnext = (* $object).p_next;
        } $crate ::vk10::StructureType::INSTANCE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::InstanceCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::MEMORY_ALLOCATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::MemoryAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::MAPPED_MEMORY_RANGE => { let $object =
        $pnext .cast:: < crate ::vk10::MappedMemoryRange > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::WRITE_DESCRIPTOR_SET => { let $object =
        $pnext .cast:: < crate ::vk10::WriteDescriptorSet > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COPY_DESCRIPTOR_SET => { let $object =
        $pnext .cast:: < crate ::vk10::CopyDescriptorSet > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::BufferCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_VIEW_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::BufferViewCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_BARRIER => { let $object
        = $pnext .cast:: < crate ::vk10::MemoryBarrier > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_MEMORY_BARRIER => { let $object =
        $pnext .cast:: < crate ::vk10::BufferMemoryBarrier > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_MEMORY_BARRIER => { let
        $object = $pnext .cast:: < crate ::vk10::ImageMemoryBarrier > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::IMAGE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::ImageCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_VIEW_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::ImageViewCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::BIND_SPARSE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::BindSparseInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SHADER_MODULE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::ShaderModuleCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::DescriptorSetLayoutCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::DESCRIPTOR_POOL_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk10::DescriptorPoolCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::DescriptorSetAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::PipelineShaderStageCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMPUTE_PIPELINE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::ComputePipelineCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO
        => { let $object = $pnext .cast:: < crate
        ::vk10::PipelineVertexInputStateCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineInputAssemblyStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineTessellationStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineViewportStateCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineRasterizationStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineMultisampleStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineColorBlendStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::PipelineDynamicStateCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::PipelineDepthStencilStateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GRAPHICS_PIPELINE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::GraphicsPipelineCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PIPELINE_CACHE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::PipelineCacheCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_LAYOUT_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::PipelineLayoutCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SAMPLER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk10::SamplerCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COMMAND_POOL_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::CommandPoolCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_ALLOCATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::CommandBufferAllocateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::CommandBufferInheritanceInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_BEGIN_INFO => { let $object = $pnext
        .cast:: < crate ::vk10::CommandBufferBeginInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::RENDER_PASS_BEGIN_INFO => { let $object
        = $pnext .cast:: < crate ::vk10::RenderPassBeginInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::RENDER_PASS_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::RenderPassCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::EVENT_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::EventCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::FENCE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::FenceCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk10::SemaphoreCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::QUERY_POOL_CREATE_INFO => {
        let $object = $pnext .cast:: < crate ::vk10::QueryPoolCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FRAMEBUFFER_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk10::FramebufferCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::SUBMIT_INFO => { let $object = $pnext .cast:: <
        crate ::vk10::SubmitInfo > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FEATURES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceFeatures2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_PROPERTIES_2 => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceProperties2 > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::FORMAT_PROPERTIES_2
        => { let $object = $pnext .cast:: < crate ::vk11::FormatProperties2 > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_FORMAT_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::ImageFormatProperties2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceImageFormatInfo2 > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::QueueFamilyProperties2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceMemoryProperties2 >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => { let $object = $pnext
        .cast:: < crate ::vk11::SparseImageFormatProperties2 > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceSparseImageFormatInfo2 >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceVariablePointersFeatures >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalImageFormatInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::ExternalImageFormatProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalBufferInfo > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::EXTERNAL_BUFFER_PROPERTIES
        => { let $object = $pnext .cast:: < crate ::vk11::ExternalBufferProperties > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ID_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceIDProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::ExternalMemoryImageCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::ExternalMemoryBufferCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::EXPORT_MEMORY_ALLOCATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::ExportMemoryAllocateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalSemaphoreInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_SEMAPHORE_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::ExternalSemaphoreProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::EXPORT_SEMAPHORE_CREATE_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::ExportSemaphoreCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceExternalFenceInfo > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::EXTERNAL_FENCE_PROPERTIES
        => { let $object = $pnext .cast:: < crate ::vk11::ExternalFenceProperties > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_FENCE_CREATE_INFO => { let $object = $pnext .cast::
        < crate ::vk11::ExportFenceCreateInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceMultiviewFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk11::PhysicalDeviceMultiviewProperties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::RenderPassMultiviewCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk11::PhysicalDeviceGroupProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_ALLOCATE_FLAGS_INFO => {
        let $object = $pnext .cast:: < crate ::vk11::MemoryAllocateFlagsInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_BUFFER_MEMORY_INFO => { let $object = $pnext .cast::
        < crate ::vk11::BindBufferMemoryInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::BindBufferMemoryDeviceGroupInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_MEMORY_INFO => { let $object = $pnext .cast:: <
        crate ::vk11::BindImageMemoryInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::BindImageMemoryDeviceGroupInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupRenderPassBeginInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupCommandBufferBeginInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_SUBMIT_INFO => { let $object = $pnext .cast::
        < crate ::vk11::DeviceGroupSubmitInfo > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::DEVICE_GROUP_BIND_SPARSE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DeviceGroupBindSparseInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::DeviceGroupDeviceCreateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::DescriptorUpdateTemplateCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => { let
        $object = $pnext .cast:: < crate
        ::vk11::RenderPassInputAttachmentAspectCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES
        => { let $object = $pnext .cast:: < crate
        ::vk11::PhysicalDevice16BitStorageFeatures > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
        let $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceSubgroupProperties >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => { let $object =
        $pnext .cast:: < crate ::vk11::BufferMemoryRequirementsInfo2 > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => { let $object = $pnext
        .cast:: < crate ::vk11::ImageMemoryRequirementsInfo2 > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => { let $object =
        $pnext .cast:: < crate ::vk11::ImageSparseMemoryRequirementsInfo2 > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_REQUIREMENTS_2 => { let $object = $pnext .cast:: <
        crate ::vk11::MemoryRequirements2 > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => { let $object
        = $pnext .cast:: < crate ::vk11::SparseImageMemoryRequirements2 > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDevicePointClippingProperties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_DEDICATED_REQUIREMENTS => { let $object = $pnext
        .cast:: < crate ::vk11::MemoryDedicatedRequirements > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk11::MemoryDedicatedAllocateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_VIEW_USAGE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk11::ImageViewUsageCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
        let $object = $pnext .cast:: < crate
        ::vk11::PipelineTessellationDomainOriginStateCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::SamplerYcbcrConversionInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk11::SamplerYcbcrConversionCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_PLANE_MEMORY_INFO => { let $object = $pnext
        .cast:: < crate ::vk11::BindImagePlaneMemoryInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO =>
        { let $object = $pnext .cast:: < crate ::vk11::ImagePlaneMemoryRequirementsInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk11::PhysicalDeviceSamplerYcbcrConversionFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk11::SamplerYcbcrConversionImageFormatProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PROTECTED_SUBMIT_INFO => { let
        $object = $pnext .cast:: < crate ::vk11::ProtectedSubmitInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceProtectedMemoryFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => { let
        $object = $pnext .cast:: < crate ::vk11::PhysicalDeviceProtectedMemoryProperties
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_QUEUE_INFO_2 => { let $object = $pnext .cast:: <
        crate ::vk11::DeviceQueueInfo2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk11::PhysicalDeviceMaintenance3Properties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT => { let $object = $pnext
        .cast:: < crate ::vk11::DescriptorSetLayoutSupport > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk11::PhysicalDeviceShaderDrawParametersFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDeviceDriverProperties > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES =>
        { let $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceShaderSubgroupExtendedTypesFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceSamplerFilterMinmaxProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk12::SamplerReductionModeCreateInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk12::ImageFormatListCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceShaderFloat16Int8Features
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk12::PhysicalDeviceFloatControlsProperties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk12::PhysicalDeviceHostQueryResetFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceDescriptorIndexingFeatures
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceDescriptorIndexingProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => { let
        $object = $pnext .cast:: < crate
        ::vk12::DescriptorSetLayoutBindingFlagsCreateInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO =>
        { let $object = $pnext .cast:: < crate
        ::vk12::DescriptorSetVariableDescriptorCountAllocateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT =>
        { let $object = $pnext .cast:: < crate
        ::vk12::DescriptorSetVariableDescriptorCountLayoutSupport > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::ATTACHMENT_DESCRIPTION_2 => {
        let $object = $pnext .cast:: < crate ::vk12::AttachmentDescription2 > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_REFERENCE_2 => { let $object = $pnext .cast:: <
        crate ::vk12::AttachmentReference2 > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::SUBPASS_DESCRIPTION_2 => { let $object = $pnext
        .cast:: < crate ::vk12::SubpassDescription2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SUBPASS_DEPENDENCY_2 => { let $object =
        $pnext .cast:: < crate ::vk12::SubpassDependency2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::RENDER_PASS_CREATE_INFO_2 => { let
        $object = $pnext .cast:: < crate ::vk12::RenderPassCreateInfo2 > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::SUBPASS_BEGIN_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::SubpassBeginInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SUBPASS_END_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::SubpassEndInfo > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceTimelineSemaphoreFeatures
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceTimelineSemaphoreProperties > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SEMAPHORE_TYPE_CREATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::SemaphoreTypeCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO => { let $object = $pnext
        .cast:: < crate ::vk12::TimelineSemaphoreSubmitInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_WAIT_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::SemaphoreWaitInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_SIGNAL_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::SemaphoreSignalInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDevice8BitStorageFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceVulkanMemoryModelFeatures
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceShaderAtomicInt64Features
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceDepthStencilResolveProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => { let $object
        = $pnext .cast:: < crate ::vk12::SubpassDescriptionDepthStencilResolve > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_STENCIL_USAGE_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk12::ImageStencilUsageCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk12::PhysicalDeviceScalarBlockLayoutFeatures
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES =>
        { let $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceUniformBufferStandardLayoutFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceBufferDeviceAddressFeatures > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::BUFFER_DEVICE_ADDRESS_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::BufferDeviceAddressInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => { let $object
        = $pnext .cast:: < crate ::vk12::BufferOpaqueCaptureAddressCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceImagelessFramebufferFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk12::FramebufferAttachmentsCreateInfo > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => { let $object =
        $pnext .cast:: < crate ::vk12::FramebufferAttachmentImageInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO => { let $object =
        $pnext .cast:: < crate ::vk12::RenderPassAttachmentBeginInfo > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES =>
        { let $object = $pnext .cast:: < crate
        ::vk12::PhysicalDeviceSeparateDepthStencilLayoutsFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT => { let $object =
        $pnext .cast:: < crate ::vk12::AttachmentReferenceStencilLayout > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => { let $object =
        $pnext .cast:: < crate ::vk12::AttachmentDescriptionStencilLayout > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => { let
        $object = $pnext .cast:: < crate ::vk12::MemoryOpaqueCaptureAddressAllocateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => { let $object
        = $pnext .cast:: < crate ::vk12::DeviceMemoryOpaqueCaptureAddressInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDeviceVulkan11Features > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDeviceVulkan11Properties > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDeviceVulkan12Features > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk12::PhysicalDeviceVulkan12Properties > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO => { let $object = $pnext
        .cast:: < crate ::vk13::DevicePrivateDataCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PRIVATE_DATA_SLOT_CREATE_INFO =>
        { let $object = $pnext .cast:: < crate ::vk13::PrivateDataSlotCreateInfo > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk13::PhysicalDevicePrivateDataFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS => { let $object =
        $pnext .cast:: < crate ::vk13::DeviceBufferMemoryRequirements > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS => { let $object = $pnext
        .cast:: < crate ::vk13::DeviceImageMemoryRequirements > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk13::PhysicalDeviceInlineUniformBlockFeatures
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceInlineUniformBlockProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK => { let $object
        = $pnext .cast:: < crate ::vk13::WriteDescriptorSetInlineUniformBlock > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO => { let
        $object = $pnext .cast:: < crate
        ::vk13::DescriptorPoolInlineUniformBlockCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk13::PhysicalDeviceMaintenance4Features > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES => { let $object
        = $pnext .cast:: < crate ::vk13::PhysicalDeviceMaintenance4Properties > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES => {
        let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceTextureCompressionASTCHDRFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO => { let $object =
        $pnext .cast:: < crate ::vk13::PipelineCreationFeedbackCreateInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES
        => { let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceShaderDemoteToHelperInvocationFeatures > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceTexelBufferAlignmentProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES => { let
        $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceSubgroupSizeControlFeatures > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES => { let
        $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceSubgroupSizeControlProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO
        => { let $object = $pnext .cast:: < crate
        ::vk13::PipelineShaderStageRequiredSubgroupSizeCreateInfo > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES
        => { let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDevicePipelineCreationCacheControlFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES => { let $object =
        $pnext .cast:: < crate ::vk13::PhysicalDeviceVulkan13Features > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES => { let $object =
        $pnext .cast:: < crate ::vk13::PhysicalDeviceVulkan13Properties > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES => { let $object = $pnext
        .cast:: < crate ::vk13::PhysicalDeviceToolProperties > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES
        => { let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES => { let $object
        = $pnext .cast:: < crate ::vk13::PhysicalDeviceImageRobustnessFeatures > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::BUFFER_COPY_2 => {
        let $object = $pnext .cast:: < crate ::vk13::BufferCopy2 > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_COPY_2 => { let $object =
        $pnext .cast:: < crate ::vk13::ImageCopy2 > (); $op; $pnext = (* $object).p_next;
        } $crate ::vk10::StructureType::IMAGE_BLIT_2 => { let $object = $pnext .cast:: <
        crate ::vk13::ImageBlit2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_IMAGE_COPY_2 => { let $object = $pnext .cast:: <
        crate ::vk13::BufferImageCopy2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_RESOLVE_2 => { let $object = $pnext .cast:: < crate
        ::vk13::ImageResolve2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_BUFFER_INFO_2 => { let $object = $pnext .cast:: <
        crate ::vk13::CopyBufferInfo2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_IMAGE_INFO_2 => { let $object = $pnext .cast:: <
        crate ::vk13::CopyImageInfo2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BLIT_IMAGE_INFO_2 => { let $object = $pnext .cast:: <
        crate ::vk13::BlitImageInfo2 > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_BUFFER_TO_IMAGE_INFO_2 => { let $object = $pnext
        .cast:: < crate ::vk13::CopyBufferToImageInfo2 > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::COPY_IMAGE_TO_BUFFER_INFO_2 => { let
        $object = $pnext .cast:: < crate ::vk13::CopyImageToBufferInfo2 > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::RESOLVE_IMAGE_INFO_2 => {
        let $object = $pnext .cast:: < crate ::vk13::ResolveImageInfo2 > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES => {
        let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceShaderTerminateInvocationFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::MEMORY_BARRIER_2 => { let
        $object = $pnext .cast:: < crate ::vk13::MemoryBarrier2 > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_MEMORY_BARRIER_2 => { let
        $object = $pnext .cast:: < crate ::vk13::ImageMemoryBarrier2 > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::BUFFER_MEMORY_BARRIER_2 => {
        let $object = $pnext .cast:: < crate ::vk13::BufferMemoryBarrier2 > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::DEPENDENCY_INFO => {
        let $object = $pnext .cast:: < crate ::vk13::DependencyInfo > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::SEMAPHORE_SUBMIT_INFO => {
        let $object = $pnext .cast:: < crate ::vk13::SemaphoreSubmitInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_SUBMIT_INFO => { let $object = $pnext
        .cast:: < crate ::vk13::CommandBufferSubmitInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SUBMIT_INFO_2 => { let $object = $pnext
        .cast:: < crate ::vk13::SubmitInfo2 > (); $op; $pnext = (* $object).p_next; }
        $crate ::vk10::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk13::PhysicalDeviceSynchronization2Features >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES => {
        let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceShaderIntegerDotProductFeatures > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES => {
        let $object = $pnext .cast:: < crate
        ::vk13::PhysicalDeviceShaderIntegerDotProductProperties > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::FORMAT_PROPERTIES_3 => { let
        $object = $pnext .cast:: < crate ::vk13::FormatProperties3 > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::PIPELINE_RENDERING_CREATE_INFO
        => { let $object = $pnext .cast:: < crate ::vk13::PipelineRenderingCreateInfo >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDERING_INFO => { let $object = $pnext .cast:: < crate
        ::vk13::RenderingInfo > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDERING_ATTACHMENT_INFO => { let $object = $pnext
        .cast:: < crate ::vk13::RenderingAttachmentInfo > (); $op; $pnext = (* $object)
        .p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => { let
        $object = $pnext .cast:: < crate ::vk13::PhysicalDeviceDynamicRenderingFeatures >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => { let $object
        = $pnext .cast:: < crate ::vk13::CommandBufferInheritanceRenderingInfo > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SWAPCHAIN_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_swapchain::SwapchainCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::PRESENT_INFO_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::PresentInfoKHR > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupPresentCapabilitiesKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_swapchain::ImageSwapchainCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_swapchain::BindImageMemorySwapchainInfoKHR > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::AcquireNextImageInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEVICE_GROUP_PRESENT_INFO_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupPresentInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_swapchain::DeviceGroupSwapchainCreateInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_MODE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_display::DisplayModeCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_display::DisplaySurfaceCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_PRESENT_INFO_KHR => { let $object = $pnext .cast::
        < crate ::extensions::khr_display_swapchain::DisplayPresentInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::XLIB_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::XCB_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WIN32_SURFACE_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => {
        let $object = $pnext .cast:: < crate
        ::extensions::amd_rasterization_order::PipelineRasterizationStateRasterizationOrderAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_MARKER_MARKER_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_video_queue::QueueFamilyVideoPropertiesKHR > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_video_queue::QueueFamilyQueryResultStatusPropertiesKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_PROFILE_LIST_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoProfileListInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_video_queue::PhysicalDeviceVideoFormatInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::VIDEO_FORMAT_PROPERTIES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoFormatPropertiesKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::VIDEO_PROFILE_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoProfileInfoKHR > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::VIDEO_CAPABILITIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoCapabilitiesKHR > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoSessionMemoryRequirementsKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_VIDEO_SESSION_MEMORY_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_video_queue::BindVideoSessionMemoryInfoKHR > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_PICTURE_RESOURCE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoPictureResourceInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_REFERENCE_SLOT_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoReferenceSlotInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_SESSION_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoSessionCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_BEGIN_CODING_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoBeginCodingInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_END_CODING_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoEndCodingInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_CODING_CONTROL_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_queue::VideoCodingControlInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_CAPABILITIES_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_decode_queue::VideoDecodeCapabilitiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_USAGE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_decode_queue::VideoDecodeUsageInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_INFO_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_video_decode_queue::VideoDecodeInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_dedicated_allocation::DedicatedAllocationImageCreateInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_dedicated_allocation::DedicatedAllocationBufferCreateInfoNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_dedicated_allocation::DedicatedAllocationMemoryAllocateInfoNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CU_MODULE_CREATE_INFO_NVX => { let $object = $pnext
        .cast:: < crate ::extensions::nvx_binary_import::CuModuleCreateInfoNVX > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CU_FUNCTION_CREATE_INFO_NVX => { let $object = $pnext
        .cast:: < crate ::extensions::nvx_binary_import::CuFunctionCreateInfoNVX > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CU_LAUNCH_INFO_NVX => { let $object = $pnext .cast:: <
        crate ::extensions::nvx_binary_import::CuLaunchInfoNVX > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_VIEW_HANDLE_INFO_NVX => {
        let $object = $pnext .cast:: < crate
        ::extensions::nvx_image_view_handle::ImageViewHandleInfoNVX > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => { let $object =
        $pnext .cast:: < crate
        ::extensions::nvx_image_view_handle::ImageViewAddressPropertiesNVX > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_CAPABILITIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264CapabilitiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersAddInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264SessionParametersCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264DpbSlotInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264VclFrameInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_REFERENCE_LISTS_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264ReferenceListsInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264EmitPictureParametersInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_PROFILE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264ProfileInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264NaluSliceInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264RateControlInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h264::VideoEncodeH264RateControlLayerInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_CAPABILITIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265CapabilitiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersAddInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265SessionParametersCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265VclFrameInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265EmitPictureParametersInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265NaluSliceSegmentInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265RateControlInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265RateControlLayerInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_PROFILE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265ProfileInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265DpbSlotInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_H265_REFERENCE_LISTS_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_encode_h265::VideoEncodeH265ReferenceListsInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_PROFILE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264ProfileInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_CAPABILITIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264CapabilitiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_PICTURE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h264::VideoDecodeH264DpbSlotInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => { let $object
        = $pnext .cast:: < crate
        ::extensions::amd_texture_gather_bias_lod::TextureLODGatherFormatPropertiesAMD >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::RenderingFragmentShadingRateAttachmentInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::RenderingFragmentDensityMapAttachmentInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD => { let $object = $pnext
        .cast:: < crate ::extensions::khr_dynamic_rendering::AttachmentSampleCountInfoAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_dynamic_rendering::MultiviewPerViewAttributesInfoNVX > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => { let $object
        = $pnext .cast:: < crate
        ::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateInfoGGP
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_corner_sampled_image::PhysicalDeviceCornerSampledImageFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_external_memory::ExternalMemoryImageCreateInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV => { let $object = $pnext
        .cast:: < crate ::extensions::nv_external_memory::ExportMemoryAllocateInfoNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_external_memory_win32::ImportMemoryWin32HandleInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_external_memory_win32::ExportMemoryWin32HandleInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VALIDATION_FLAGS_EXT => { let $object = $pnext .cast:: <
        crate ::extensions::ext_validation_flags::ValidationFlagsEXT > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::VI_SURFACE_CREATE_INFO_NN =>
        { let $object = $pnext .cast:: < crate
        ::extensions::nn_vi_surface::ViSurfaceCreateInfoNN > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::IMAGE_VIEW_ASTC_DECODE_MODE_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_astc_decode_mode::ImageViewASTCDecodeModeEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_astc_decode_mode::PhysicalDeviceASTCDecodeFeaturesEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_ROBUSTNESS_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_pipeline_robustness::PipelineRobustnessCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_pipeline_robustness::PhysicalDevicePipelineRobustnessPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_memory_win32::ImportMemoryWin32HandleInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_memory_win32::ExportMemoryWin32HandleInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_memory_win32::MemoryWin32HandlePropertiesKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_external_memory_win32::MemoryGetWin32HandleInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_FD_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_external_memory_fd::ImportMemoryFdInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_FD_PROPERTIES_KHR => { let $object = $pnext .cast::
        < crate ::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_GET_FD_INFO_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_win32_keyed_mutex::Win32KeyedMutexAcquireReleaseInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_semaphore_win32::ImportSemaphoreWin32HandleInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_semaphore_win32::ExportSemaphoreWin32HandleInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::D3D12_FENCE_SUBMIT_INFO_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_external_semaphore_win32::D3D12FenceSubmitInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_semaphore_win32::SemaphoreGetWin32HandleInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_external_semaphore_fd::ImportSemaphoreFdInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SEMAPHORE_GET_FD_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_external_semaphore_fd::SemaphoreGetFdInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_push_descriptor::PhysicalDevicePushDescriptorPropertiesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_conditional_rendering::CommandBufferInheritanceConditionalRenderingInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_conditional_rendering::PhysicalDeviceConditionalRenderingFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PRESENT_REGIONS_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_incremental_present::PresentRegionsKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_clip_space_w_scaling::PipelineViewportWScalingStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_CAPABILITIES_2_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_display_surface_counter::SurfaceCapabilities2EXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_POWER_INFO_EXT => { let $object = $pnext .cast:: <
        crate ::extensions::ext_display_control::DisplayPowerInfoEXT > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::DEVICE_EVENT_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_display_control::DeviceEventInfoEXT > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DISPLAY_EVENT_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_display_control::DisplayEventInfoEXT > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_display_control::SwapchainCounterCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PRESENT_TIMES_INFO_GOOGLE => { let $object = $pnext
        .cast:: < crate ::extensions::google_display_timing::PresentTimesInfoGOOGLE > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX
        => { let $object = $pnext .cast:: < crate
        ::extensions::nvx_multiview_per_view_attributes::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_discard_rectangles::PhysicalDeviceDiscardRectanglePropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_depth_clip_enable::PhysicalDeviceDepthClipEnableFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::HDR_METADATA_EXT => { let $object = $pnext .cast:: < crate
        ::extensions::ext_hdr_metadata::HdrMetadataEXT > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_fence_win32::ImportFenceWin32HandleInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_external_fence_win32::ExportFenceWin32HandleInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_external_fence_win32::FenceGetWin32HandleInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_FENCE_FD_INFO_KHR => { let $object = $pnext .cast::
        < crate ::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FENCE_GET_FD_INFO_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_external_fence_fd::FenceGetFdInfoKHR > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_COUNTER_KHR => { let $object = $pnext .cast::
        < crate ::extensions::khr_performance_query::PerformanceCounterKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_performance_query::PerformanceCounterDescriptionKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_performance_query::AcquireProfilingLockInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_CAPABILITIES_2_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::SURFACE_FORMAT_2_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::DISPLAY_PROPERTIES_2_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_get_display_properties2::DisplayProperties2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_get_display_properties2::DisplayPlaneProperties2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_MODE_PROPERTIES_2_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_get_display_properties2::DisplayModeProperties2KHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_PLANE_INFO_2_KHR => { let $object = $pnext .cast::
        < crate ::extensions::khr_get_display_properties2::DisplayPlaneInfo2KHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_get_display_properties2::DisplayPlaneCapabilities2KHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IOS_SURFACE_CREATE_INFO_MVK => { let $object = $pnext
        .cast:: < crate ::extensions::mvk_ios_surface::IOSSurfaceCreateInfoMVK > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MACOS_SURFACE_CREATE_INFO_MVK => { let $object = $pnext
        .cast:: < crate ::extensions::mvk_macos_surface::MacOSSurfaceCreateInfoMVK > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_LABEL_EXT => { let $object = $pnext .cast:: <
        crate ::extensions::ext_debug_utils::DebugUtilsLabelEXT > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => { let
        $object = $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::ImportAndroidHardwareBufferInfoANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => { let $object =
        $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferUsageANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => { let
        $object = $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferPropertiesANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => { let
        $object = $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::MemoryGetAndroidHardwareBufferInfoANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => { let
        $object = $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatPropertiesANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXTERNAL_FORMAT_ANDROID => { let $object = $pnext .cast::
        < crate
        ::extensions::android_external_memory_android_hardware_buffer::ExternalFormatANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID => {
        let $object = $pnext .cast:: < crate
        ::extensions::android_external_memory_android_hardware_buffer::AndroidHardwareBufferFormatProperties2ANDROID
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SAMPLE_LOCATIONS_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_sample_locations::SampleLocationsInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_sample_locations::RenderPassSampleLocationsBeginInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_sample_locations::PipelineSampleLocationsStateCreateInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_sample_locations::PhysicalDeviceSampleLocationsPropertiesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MULTISAMPLE_PROPERTIES_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_sample_locations::MultisamplePropertiesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_blend_operation_advanced::PhysicalDeviceBlendOperationAdvancedPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_blend_operation_advanced::PipelineColorBlendAdvancedStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::WriteDescriptorSetAccelerationStructureKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelineFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_pipeline::PhysicalDeviceRayTracingPipelinePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_ray_query::PhysicalDeviceRayQueryFeaturesKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsPropertiesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_shader_sm_builtins::PhysicalDeviceShaderSMBuiltinsFeaturesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierListCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierExplicitCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VALIDATION_CACHE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_validation_cache::ShaderModuleValidationCacheCreateInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_shading_rate_image::PipelineViewportShadingRateImageStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImageFeaturesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_shading_rate_image::PhysicalDeviceShadingRateImagePropertiesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_shading_rate_image::PipelineViewportCoarseSampleOrderStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV > (); $op; $pnext =
        (* $object).p_next; } $crate ::vk10::StructureType::GEOMETRY_TRIANGLES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::GeometryTrianglesNV > (); $op; $pnext = (* $object)
        .p_next; } $crate ::vk10::StructureType::GEOMETRY_AABB_NV => { let $object =
        $pnext .cast:: < crate ::extensions::nv_ray_tracing::GeometryAABBNV > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::GEOMETRY_NV => { let
        $object = $pnext .cast:: < crate ::extensions::nv_ray_tracing::GeometryNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_INFO_NV => { let $object = $pnext
        .cast:: < crate ::extensions::nv_ray_tracing::AccelerationStructureInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::AccelerationStructureCreateInfoNV > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::WriteDescriptorSetAccelerationStructureNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing::PhysicalDeviceRayTracingPropertiesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_representative_fragment_test::PhysicalDeviceRepresentativeFragmentTestFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_representative_fragment_test::PipelineRepresentativeFragmentTestStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_filter_cubic::PhysicalDeviceImageViewImageFormatInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_filter_cubic::FilterCubicImageViewImageFormatPropertiesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_external_memory_host::ImportMemoryHostPointerInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_external_memory_host::MemoryHostPointerPropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_external_memory_host::PhysicalDeviceExternalMemoryHostPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_shader_clock::PhysicalDeviceShaderClockFeaturesKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => { let $object
        = $pnext .cast:: < crate
        ::extensions::amd_pipeline_compiler_control::PipelineCompilerControlCreateInfoAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CALIBRATED_TIMESTAMP_INFO_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_shader_core_properties::PhysicalDeviceShaderCorePropertiesAMD >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_PROFILE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265ProfileInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_CAPABILITIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265CapabilitiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersAddInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265SessionParametersCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_PICTURE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265PictureInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_video_decode_h265::VideoDecodeH265DpbSlotInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_memory_overallocation_behavior::DeviceMemoryOverallocationCreateInfoAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_vertex_attribute_divisor::PipelineVertexInputDivisorStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_vertex_attribute_divisor::PhysicalDeviceVertexAttributeDivisorFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PRESENT_FRAME_TOKEN_GGP => { let $object = $pnext .cast::
        < crate ::extensions::ggp_frame_token::PresentFrameTokenGGP > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV =>
        { let $object = $pnext .cast:: < crate
        ::extensions::nv_compute_shader_derivatives::PhysicalDeviceComputeShaderDerivativesFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderFeaturesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_mesh_shader::PhysicalDeviceMeshShaderPropertiesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_shader_image_footprint::PhysicalDeviceShaderImageFootprintFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_scissor_exclusive::PhysicalDeviceExclusiveScissorFeaturesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_scissor_exclusive::PipelineViewportExclusiveScissorStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_device_diagnostic_checkpoints::QueueFamilyCheckpointPropertiesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::CHECKPOINT_DATA_NV => { let $object = $pnext .cast:: <
        crate ::extensions::nv_device_diagnostic_checkpoints::CheckpointDataNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL
        => { let $object = $pnext .cast:: < crate
        ::extensions::intel_shader_integer_functions2::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL => { let $object =
        $pnext .cast:: < crate
        ::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => { let
        $object = $pnext .cast:: < crate
        ::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_MARKER_INFO_INTEL => { let $object = $pnext
        .cast:: < crate ::extensions::intel_performance_query::PerformanceMarkerInfoINTEL
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL => { let $object =
        $pnext .cast:: < crate
        ::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL => { let $object = $pnext
        .cast:: < crate
        ::extensions::intel_performance_query::PerformanceOverrideInfoINTEL > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => { let
        $object = $pnext .cast:: < crate
        ::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_pci_bus_info::PhysicalDevicePCIBusInfoPropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_imagepipe_surface::ImagePipeSurfaceCreateInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::METAL_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_density_map::PhysicalDeviceFragmentDensityMapPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_density_map::RenderPassFragmentDensityMapCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_fragment_shading_rate::PipelineFragmentShadingRateStateCreateInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => {
        let $object = $pnext .cast:: < crate
        ::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_shader_core_properties2::PhysicalDeviceShaderCoreProperties2AMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => { let
        $object = $pnext .cast:: < crate
        ::extensions::amd_device_coherent_memory::PhysicalDeviceCoherentMemoryFeaturesAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_shader_image_atomic_int64::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_memory_budget::PhysicalDeviceMemoryBudgetPropertiesEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_memory_priority::PhysicalDeviceMemoryPriorityFeaturesEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_memory_priority::MemoryPriorityAllocateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_dedicated_allocation_image_aliasing::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_buffer_device_address::BufferDeviceAddressCreateInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VALIDATION_FEATURES_EXT => { let $object = $pnext .cast::
        < crate ::extensions::ext_validation_features::ValidationFeaturesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_present_wait::PhysicalDevicePresentWaitFeaturesKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV => { let $object = $pnext
        .cast:: < crate
        ::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_coverage_reduction_mode::PhysicalDeviceCoverageReductionModeFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_coverage_reduction_mode::FramebufferMixedSamplesCombinationNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_shader_interlock::PhysicalDeviceFragmentShaderInterlockFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_ycbcr_image_arrays::PhysicalDeviceYcbcrImageArraysFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_provoking_vertex::PipelineRasterizationProvokingVertexStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_headless_surface::HeadlessSurfaceCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_line_rasterization::PhysicalDeviceLineRasterizationPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_line_rasterization::PipelineRasterizationLineStateCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_shader_atomic_float::PhysicalDeviceShaderAtomicFloatFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_index_type_uint8::PhysicalDeviceIndexTypeUint8FeaturesEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_extended_dynamic_state::PhysicalDeviceExtendedDynamicStateFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_pipeline_executable_properties::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_INFO_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_pipeline_executable_properties::PipelineInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_pipeline_executable_properties::PipelineExecutablePropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_EXECUTABLE_INFO_KHR => { let $object = $pnext
        .cast:: < crate
        ::extensions::khr_pipeline_executable_properties::PipelineExecutableInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => { let
        $object = $pnext .cast:: < crate
        ::extensions::khr_pipeline_executable_properties::PipelineExecutableInternalRepresentationKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_shader_atomic_float2::PhysicalDeviceShaderAtomicFloat2FeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV =>
        { let $object = $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::GraphicsPipelineShaderGroupsCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::IndirectCommandsLayoutCreateInfoNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GENERATED_COMMANDS_INFO_NV => { let $object = $pnext
        .cast:: < crate
        ::extensions::nv_device_generated_commands::GeneratedCommandsInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_device_generated_commands::GeneratedCommandsMemoryRequirementsInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV =>
        { let $object = $pnext .cast:: < crate
        ::extensions::nv_inherited_viewport_scissor::PhysicalDeviceInheritedViewportScissorFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_inherited_viewport_scissor::CommandBufferInheritanceViewportScissorInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_texel_buffer_alignment::PhysicalDeviceTexelBufferAlignmentFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => { let $object =
        $pnext .cast:: < crate
        ::extensions::qcom_render_pass_transform::RenderPassTransformBeginInfoQCOM > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM
        => { let $object = $pnext .cast:: < crate
        ::extensions::qcom_render_pass_transform::CommandBufferInheritanceRenderPassTransformInfoQCOM
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_device_memory_report::PhysicalDeviceDeviceMemoryReportFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_device_memory_report::DeviceDeviceMemoryReportCreateInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_device_memory_report::DeviceMemoryReportCallbackDataEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_robustness2::PhysicalDeviceRobustness2FeaturesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_robustness2::PhysicalDeviceRobustness2PropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_custom_border_color::SamplerCustomBorderColorCreateInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_custom_border_color::PhysicalDeviceCustomBorderColorFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_present_barrier::PhysicalDevicePresentBarrierFeaturesNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_present_barrier::SurfaceCapabilitiesPresentBarrierNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_present_barrier::SwapchainPresentBarrierCreateInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_present_id::PhysicalDevicePresentIdFeaturesKHR > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::PRESENT_ID_KHR => {
        let $object = $pnext .cast:: < crate ::extensions::khr_present_id::PresentIdKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_USAGE_INFO_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_encode_queue::VideoEncodeUsageInfoKHR >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_INFO_KHR => { let $object = $pnext .cast:: <
        crate ::extensions::khr_video_encode_queue::VideoEncodeInfoKHR > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_video_encode_queue::VideoEncodeRateControlInfoKHR > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR => { let $object
        = $pnext .cast:: < crate
        ::extensions::khr_video_encode_queue::VideoEncodeRateControlLayerInfoKHR > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VIDEO_ENCODE_CAPABILITIES_KHR => { let $object = $pnext
        .cast:: < crate ::extensions::khr_video_encode_queue::VideoEncodeCapabilitiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_device_diagnostics_config::PhysicalDeviceDiagnosticsConfigFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigCreateInfoNV >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_OBJECT_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_metal_objects::ExportMetalObjectCreateInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_OBJECTS_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ExportMetalObjectsInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_DEVICE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ExportMetalDeviceInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_metal_objects::ExportMetalCommandQueueInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_BUFFER_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ExportMetalBufferInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_METAL_BUFFER_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ImportMetalBufferInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_TEXTURE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ExportMetalTextureInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_METAL_TEXTURE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ImportMetalTextureInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_IO_SURFACE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ExportMetalIOSurfaceInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_metal_objects::ImportMetalIOSurfaceInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::EXPORT_METAL_SHARED_EVENT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_metal_objects::ExportMetalSharedEventInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_METAL_SHARED_EVENT_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_metal_objects::ImportMetalSharedEventInfoEXT > (); $op; $pnext
        = (* $object).p_next; } $crate
        ::vk10::StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::khr_synchronization2::QueueFamilyCheckpointProperties2NV > (); $op;
        $pnext = (* $object).p_next; } $crate ::vk10::StructureType::CHECKPOINT_DATA_2_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_synchronization2::CheckpointData2NV > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_graphics_pipeline_library::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_graphics_pipeline_library::GraphicsPipelineLibraryCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD
        => { let $object = $pnext .cast:: < crate
        ::extensions::amd_shader_early_and_late_fragment_tests::PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_fragment_shader_barycentric::PhysicalDeviceFragmentShaderBarycentricPropertiesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_shader_subgroup_uniform_control_flow::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV =>
        { let $object = $pnext .cast:: < crate
        ::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_fragment_shading_rate_enums::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_fragment_shading_rate_enums::PipelineFragmentShadingRateEnumStateCreateInfoNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV
        => { let $object = $pnext .cast:: < crate
        ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderFeaturesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_mesh_shader::PhysicalDeviceMeshShaderPropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_ycbcr_2plane_444_formats::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2FeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_fragment_density_map2::PhysicalDeviceFragmentDensityMap2PropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM => { let $object = $pnext
        .cast:: < crate
        ::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOM > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR
        => { let $object = $pnext .cast:: < crate
        ::extensions::khr_workgroup_memory_explicit_layout::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_COMPRESSION_CONTROL_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_image_compression_control::ImageCompressionControlEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_image_compression_control::PhysicalDeviceImageCompressionControlFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_COMPRESSION_PROPERTIES_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_image_compression_control::ImageCompressionPropertiesEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_SUBRESOURCE_2_EXT => { let $object = $pnext .cast::
        < crate ::extensions::ext_image_compression_control::ImageSubresource2EXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBRESOURCE_LAYOUT_2_EXT => { let $object = $pnext .cast::
        < crate ::extensions::ext_image_compression_control::SubresourceLayout2EXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_attachment_feedback_loop_layout::PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_4444_formats::PhysicalDevice4444FormatsFeaturesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_device_fault::PhysicalDeviceFaultFeaturesEXT > (); $op; $pnext
        = (* $object).p_next; } $crate ::vk10::StructureType::DEVICE_FAULT_COUNTS_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_device_fault::DeviceFaultCountsEXT > (); $op; $pnext = (*
        $object).p_next; } $crate ::vk10::StructureType::DEVICE_FAULT_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_device_fault::DeviceFaultInfoEXT > (); $op; $pnext = (*
        $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_rgba10x6_formats::PhysicalDeviceRGBA10X6FormatsFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_directfb_surface::DirectFBSurfaceCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_vertex_input_dynamic_state::PhysicalDeviceVertexInputDynamicStateFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_physical_device_drm::PhysicalDeviceDrmPropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_device_address_binding_report::PhysicalDeviceAddressBindingReportFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_device_address_binding_report::DeviceAddressBindingCallbackDataEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_depth_clip_control::PhysicalDeviceDepthClipControlFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_depth_clip_control::PipelineViewportDepthClipControlCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_primitive_topology_list_restart::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => { let $object
        = $pnext .cast:: < crate
        ::extensions::fuchsia_external_memory::ImportMemoryZirconHandleInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_external_memory::MemoryZirconHandlePropertiesFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_external_memory::MemoryGetZirconHandleInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => { let
        $object = $pnext .cast:: < crate
        ::extensions::fuchsia_external_semaphore::ImportSemaphoreZirconHandleInfoFUCHSIA
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => { let $object
        = $pnext .cast:: < crate
        ::extensions::fuchsia_external_semaphore::SemaphoreGetZirconHandleInfoFUCHSIA >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::ImportMemoryBufferCollectionFUCHSIA >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA => { let
        $object = $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferCollectionImageCreateInfoFUCHSIA >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => { let
        $object = $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferCollectionBufferCreateInfoFUCHSIA
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferCollectionCreateInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferCollectionPropertiesFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA => { let $object = $pnext
        .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferConstraintsInfoFUCHSIA > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SYSMEM_COLOR_SPACE_FUCHSIA => { let $object = $pnext
        .cast:: < crate ::extensions::fuchsia_buffer_collection::SysmemColorSpaceFUCHSIA
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA => { let $object =
        $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::ImageFormatConstraintsInfoFUCHSIA > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA => { let $object = $pnext
        .cast:: < crate
        ::extensions::fuchsia_buffer_collection::ImageConstraintsInfoFUCHSIA > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA => { let
        $object = $pnext .cast:: < crate
        ::extensions::fuchsia_buffer_collection::BufferCollectionConstraintsInfoFUCHSIA >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI => { let
        $object = $pnext .cast:: < crate
        ::extensions::huawei_subpass_shading::SubpassShadingPipelineCreateInfoHUAWEI >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI => { let
        $object = $pnext .cast:: < crate
        ::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingPropertiesHUAWEI
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI => { let
        $object = $pnext .cast:: < crate
        ::extensions::huawei_subpass_shading::PhysicalDeviceSubpassShadingFeaturesHUAWEI
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI => { let
        $object = $pnext .cast:: < crate
        ::extensions::huawei_invocation_mask::PhysicalDeviceInvocationMaskFeaturesHUAWEI
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_external_memory_rdma::PhysicalDeviceExternalMemoryRDMAFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_external_memory_rdma::MemoryGetRemoteAddressInfoNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_pipeline_properties::PipelinePropertiesIdentifierEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_pipeline_properties::PhysicalDevicePipelinePropertiesFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_multisampled_render_to_single_sampled::PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_multisampled_render_to_single_sampled::SubpassResolvePerformanceQueryEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_multisampled_render_to_single_sampled::MultisampledRenderToSingleSampledInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_extended_dynamic_state2::PhysicalDeviceExtendedDynamicState2FeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SCREEN_SURFACE_CREATE_INFO_QNX => { let $object = $pnext
        .cast:: < crate ::extensions::qnx_screen_surface::ScreenSurfaceCreateInfoQNX >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_color_write_enable::PhysicalDeviceColorWriteEnableFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_color_write_enable::PipelineColorWriteCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_primitives_generated_query::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR =>
        { let $object = $pnext .cast:: < crate
        ::extensions::khr_ray_tracing_maintenance1::PhysicalDeviceRayTracingMaintenance1FeaturesKHR
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_image_view_min_lod::PhysicalDeviceImageViewMinLodFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_image_view_min_lod::ImageViewMinLodCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT => { let $object
        = $pnext .cast:: < crate
        ::extensions::ext_multi_draw::PhysicalDeviceMultiDrawPropertiesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_multi_draw::PhysicalDeviceMultiDrawFeaturesEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_image_2d_view_of_3d::PhysicalDeviceImage2DViewOf3DFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MICROMAP_BUILD_INFO_EXT => { let $object = $pnext .cast::
        < crate ::extensions::ext_opacity_micromap::MicromapBuildInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MICROMAP_CREATE_INFO_EXT => { let $object = $pnext .cast::
        < crate ::extensions::ext_opacity_micromap::MicromapCreateInfoEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MICROMAP_VERSION_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_opacity_micromap::MicromapVersionInfoEXT > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_MICROMAP_INFO_EXT => { let $object = $pnext .cast:: <
        crate ::extensions::ext_opacity_micromap::CopyMicromapInfoEXT > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_MICROMAP_TO_MEMORY_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_opacity_micromap::CopyMicromapToMemoryInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::COPY_MEMORY_TO_MICROMAP_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_opacity_micromap::CopyMemoryToMicromapInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MICROMAP_BUILD_SIZES_INFO_EXT => { let $object = $pnext
        .cast:: < crate ::extensions::ext_opacity_micromap::MicromapBuildSizesInfoEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_opacity_micromap::PhysicalDeviceOpacityMicromapPropertiesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_opacity_micromap::AccelerationStructureTrianglesOpacityMicromapEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_border_color_swizzle::SamplerBorderColorComponentMappingCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_border_color_swizzle::PhysicalDeviceBorderColorSwizzleFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_pageable_device_local_memory::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE
        => { let $object = $pnext .cast:: < crate
        ::extensions::valve_descriptor_set_host_mapping::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE => { let $object =
        $pnext .cast:: < crate
        ::extensions::valve_descriptor_set_host_mapping::DescriptorSetBindingReferenceVALVE
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE => { let
        $object = $pnext .cast:: < crate
        ::extensions::valve_descriptor_set_host_mapping::DescriptorSetLayoutHostMappingInfoVALVE
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_depth_clamp_zero_one::PhysicalDeviceDepthClampZeroOneFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_non_seamless_cube_map::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM
        => { let $object = $pnext .cast:: < crate
        ::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM
        => { let $object = $pnext .cast:: < crate
        ::extensions::qcom_fragment_density_map_offset::PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM => { let
        $object = $pnext .cast:: < crate
        ::extensions::qcom_fragment_density_map_offset::SubpassFragmentDensityMapOffsetEndInfoQCOM
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV => {
        let $object = $pnext .cast:: < crate
        ::extensions::nv_linear_color_attachment::PhysicalDeviceLinearColorAttachmentFeaturesNV
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_image_compression_control_swapchain::PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM => { let $object
        = $pnext .cast:: < crate
        ::extensions::qcom_image_processing::ImageViewSampleWeightCreateInfoQCOM > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM => { let
        $object = $pnext .cast:: < crate
        ::extensions::qcom_image_processing::PhysicalDeviceImageProcessingFeaturesQCOM >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM => { let
        $object = $pnext .cast:: < crate
        ::extensions::qcom_image_processing::PhysicalDeviceImageProcessingPropertiesQCOM
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3FeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_extended_dynamic_state3::PhysicalDeviceExtendedDynamicState3PropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_CREATION_CONTROL_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_subpass_merge_feedback::RenderPassCreationControlEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_subpass_merge_feedback::RenderPassCreationFeedbackCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_subpass_merge_feedback::RenderPassSubpassFeedbackCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_subpass_merge_feedback::PhysicalDeviceSubpassMergeFeedbackFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_shader_module_identifier::PhysicalDeviceShaderModuleIdentifierPropertiesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_shader_module_identifier::PipelineShaderStageModuleIdentifierCreateInfoEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::SHADER_MODULE_IDENTIFIER_EXT => { let $object = $pnext
        .cast:: < crate
        ::extensions::ext_shader_module_identifier::ShaderModuleIdentifierEXT > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT
        => { let $object = $pnext .cast:: < crate
        ::extensions::ext_rasterization_order_attachment_access::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV => { let $object
        = $pnext .cast:: < crate
        ::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowFeaturesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_optical_flow::PhysicalDeviceOpticalFlowPropertiesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_optical_flow::OpticalFlowImageFormatInfoNV > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_optical_flow::OpticalFlowImageFormatPropertiesNV > (); $op;
        $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::OPTICAL_FLOW_SESSION_CREATE_INFO_NV => { let $object =
        $pnext .cast:: < crate
        ::extensions::nv_optical_flow::OpticalFlowSessionCreateInfoNV > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV => { let
        $object = $pnext .cast:: < crate
        ::extensions::nv_optical_flow::OpticalFlowSessionCreatePrivateDataInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::OPTICAL_FLOW_EXECUTE_INFO_NV => { let $object = $pnext
        .cast:: < crate ::extensions::nv_optical_flow::OpticalFlowExecuteInfoNV > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT => { let
        $object = $pnext .cast:: < crate
        ::extensions::ext_legacy_dithering::PhysicalDeviceLegacyDitheringFeaturesEXT >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT =>
        { let $object = $pnext .cast:: < crate
        ::extensions::ext_pipeline_protected_access::PhysicalDevicePipelineProtectedAccessFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM => { let
        $object = $pnext .cast:: < crate
        ::extensions::qcom_tile_properties::PhysicalDeviceTilePropertiesFeaturesQCOM >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::TILE_PROPERTIES_QCOM => { let $object = $pnext .cast:: <
        crate ::extensions::qcom_tile_properties::TilePropertiesQCOM > (); $op; $pnext =
        (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC => { let
        $object = $pnext .cast:: < crate
        ::extensions::sec_amigo_profiling::PhysicalDeviceAmigoProfilingFeaturesSEC > ();
        $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::AMIGO_PROFILING_SUBMIT_INFO_SEC => { let $object = $pnext
        .cast:: < crate ::extensions::sec_amigo_profiling::AmigoProfilingSubmitInfoSEC >
        (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT => {
        let $object = $pnext .cast:: < crate
        ::extensions::ext_mutable_descriptor_type::PhysicalDeviceMutableDescriptorTypeFeaturesEXT
        > (); $op; $pnext = (* $object).p_next; } $crate
        ::vk10::StructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT => { let $object =
        $pnext .cast:: < crate
        ::extensions::ext_mutable_descriptor_type::MutableDescriptorTypeCreateInfoEXT >
        (); $op; $pnext = (* $object).p_next; } _ =>
        panic!("Unknown StructureType value ({:?})", $stype) }
    };
}
