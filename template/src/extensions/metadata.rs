use std::ffi::CStr;

pub struct ExtensionMetadata {
    pub name: &'static CStr,
    /// true - instance, false - device
    pub instance: bool,
    pub core_version: u32,
    pub requires_extensions: &'static [&'static CStr],
}

pub fn get_metadata(name: &CStr) -> Option<&'static ExtensionMetadata> {
    let index = EXTENSION_METADATA
        .binary_search_by_key(&name, |m| &m.name)
        .ok()?;
    Some(&EXTENSION_METADATA[index])
}
