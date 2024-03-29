impl crate::vk10::AccessFlags {
    pub const READ_FLAGS: Self = Self(
        Self::COLOR_ATTACHMENT_READ.0 | Self::DEPTH_STENCIL_ATTACHMENT_READ.0
            | Self::HOST_READ.0 | Self::INDEX_READ.0 | Self::INDIRECT_COMMAND_READ.0
            | Self::INPUT_ATTACHMENT_READ.0 | Self::MEMORY_READ.0 | Self::SHADER_READ.0
            | Self::TRANSFER_READ.0 | Self::UNIFORM_READ.0
            | Self::VERTEX_ATTRIBUTE_READ.0,
    );
    pub const WRITE_FLAGS: Self = Self(
        Self::COLOR_ATTACHMENT_WRITE.0 | Self::DEPTH_STENCIL_ATTACHMENT_WRITE.0
            | Self::HOST_WRITE.0 | Self::MEMORY_WRITE.0 | Self::SHADER_WRITE.0
            | Self::TRANSFER_WRITE.0,
    );
    /// Whether the AccessFlags contains flags containing "READ"
    pub const fn contains_read(&self) -> bool {
        self.intersects(Self::READ_FLAGS)
    }
    /// Whether the AccessFlags contains flags containing "WRITE"
    pub const fn contains_write(&self) -> bool {
        self.intersects(Self::WRITE_FLAGS)
    }
}
impl crate::vk13::AccessFlags2 {
    pub const READ_FLAGS: Self = Self(
        Self::COLOR_ATTACHMENT_READ.0 | Self::COLOR_ATTACHMENT_READ_KHR.0
            | Self::DEPTH_STENCIL_ATTACHMENT_READ.0
            | Self::DEPTH_STENCIL_ATTACHMENT_READ_KHR.0 | Self::HOST_READ.0
            | Self::HOST_READ_KHR.0 | Self::INDEX_READ.0 | Self::INDEX_READ_KHR.0
            | Self::INDIRECT_COMMAND_READ.0 | Self::INDIRECT_COMMAND_READ_KHR.0
            | Self::INPUT_ATTACHMENT_READ.0 | Self::INPUT_ATTACHMENT_READ_KHR.0
            | Self::MEMORY_READ.0 | Self::MEMORY_READ_KHR.0 | Self::SHADER_READ.0
            | Self::SHADER_READ_KHR.0 | Self::SHADER_SAMPLED_READ.0
            | Self::SHADER_SAMPLED_READ_KHR.0 | Self::SHADER_STORAGE_READ.0
            | Self::SHADER_STORAGE_READ_KHR.0 | Self::TRANSFER_READ.0
            | Self::TRANSFER_READ_KHR.0 | Self::UNIFORM_READ.0 | Self::UNIFORM_READ_KHR.0
            | Self::VERTEX_ATTRIBUTE_READ.0 | Self::VERTEX_ATTRIBUTE_READ_KHR.0,
    );
    pub const WRITE_FLAGS: Self = Self(
        Self::COLOR_ATTACHMENT_WRITE.0 | Self::COLOR_ATTACHMENT_WRITE_KHR.0
            | Self::DEPTH_STENCIL_ATTACHMENT_WRITE.0
            | Self::DEPTH_STENCIL_ATTACHMENT_WRITE_KHR.0 | Self::HOST_WRITE.0
            | Self::HOST_WRITE_KHR.0 | Self::MEMORY_WRITE.0 | Self::MEMORY_WRITE_KHR.0
            | Self::SHADER_STORAGE_WRITE.0 | Self::SHADER_STORAGE_WRITE_KHR.0
            | Self::SHADER_WRITE.0 | Self::SHADER_WRITE_KHR.0 | Self::TRANSFER_WRITE.0
            | Self::TRANSFER_WRITE_KHR.0,
    );
    /// Whether the AccessFlags contains flags containing "READ"
    pub const fn contains_read(&self) -> bool {
        self.intersects(Self::READ_FLAGS)
    }
    /// Whether the AccessFlags contains flags containing "WRITE"
    pub const fn contains_write(&self) -> bool {
        self.intersects(Self::WRITE_FLAGS)
    }
}
