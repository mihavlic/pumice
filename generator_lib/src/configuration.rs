use std::collections::HashSet;

use crate::interner::{Intern, Interner, UniqueStr};

pub struct GenConfig {
    /// names of extensions, for example
    ///   [VK_KHR_surface, VK_KHR_swapchain, VK_KHR_display, ..]
    pub extensions: HashSet<UniqueStr>,
    /// name of a vulkan version
    ///   [VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3]
    /// It is expected that selecting a given feature also selects all lower versions.
    /// Currently comparison is done directly on strings, fight me.
    pub feature: UniqueStr,
    /// I have no idea what a profile *is*, but here the xml spec is worded like it's always something singular?
    ///   https://registry.khronos.org/vulkan/specs/1.3/registry.html#schema:profile
    /// It is not currently being exercised in any way.
    pub profile: Option<UniqueStr>,
    /// Some other selection mechanism, currently the registry only uses "vulkan" but the spec also talks about "openxr"
    ///   https://registry.khronos.org/vulkan/specs/1.3/registry.html#schema:apiname
    pub apis: HashSet<UniqueStr>,
    /// "preprocessor" tokens that may further adjust the emitted items
    /// it seems to only ever be "VK_ENABLE_BETA_EXTENSIONS"
    pub protect: HashSet<UniqueStr>,
    /// if true passes everything as if enabled
    pub pass_all: bool,
}

impl GenConfig {
    pub fn from_strs(
        extensions: &[&str],
        feature: &str,
        profile: Option<&str>,
        apis: &[&str],
        protect: &[&str],
        int: &Interner,
        pass_all: bool,
    ) -> Self {
        Self {
            extensions: extensions.iter().map(|s| s.intern(int)).collect(),
            feature: feature.intern(int),
            profile: profile.map(|s| s.intern(int)),
            apis: apis.iter().map(|s| s.intern(int)).collect(),
            protect: protect.iter().map(|s| s.intern(int)).collect(),
            pass_all,
        }
    }
    pub fn is_extension_used(&self, name: UniqueStr) -> bool {
        if self.pass_all {
            return true;
        }
        self.extensions.contains(&name)
    }
    pub fn is_feature_used(&self, name: UniqueStr) -> bool {
        if self.pass_all {
            return true;
        }
        name.resolve_original() <= self.feature.resolve_original()
    }
    pub fn is_profile_used(&self, name: UniqueStr) -> bool {
        if self.pass_all {
            return true;
        }
        self.profile.map(|p| p == name).unwrap_or(false)
    }
    pub fn is_api_used(&self, name: UniqueStr) -> bool {
        if self.pass_all {
            return true;
        }
        self.apis.contains(&name)
    }
    pub fn is_protect_used(&self, name: UniqueStr) -> bool {
        if self.pass_all {
            return true;
        }
        self.protect.contains(&name)
    }
}

#[test]
#[rustfmt::skip]
fn test_gen_config() {
    let int = Interner::new();
    let conf = GenConfig::from_strs(
        &["VK_KHR_surface", "VK_KHR_swapchain"],
        "VK_VERSION_1_2",
        None,
        &["vulkan"],
        &["VK_ENABLE_BETA_EXTENSIONS"],
        &int,
        false
    );

    assert_eq!(conf.is_extension_used("VK_KHR_surface".intern(&int)), true);
    assert_eq!(conf.is_extension_used("VK_KHR_imageless_framebuffer".intern(&int)), false);

    assert_eq!(conf.is_feature_used("VK_VERSION_1_0".intern(&int)), true);
    assert_eq!(conf.is_feature_used("VK_VERSION_1_1".intern(&int)), true);
    assert_eq!(conf.is_feature_used("VK_VERSION_1_2".intern(&int)), true);
    assert_eq!(conf.is_feature_used("VK_VERSION_1_3".intern(&int)), false);

    assert_eq!(conf.is_profile_used("huh".intern(&int)), false);

    assert_eq!(conf.is_api_used("vulkan".intern(&int)), true);
    assert_eq!(conf.is_api_used("openxr".intern(&int)), false);
}
