use std::collections::HashSet;

use generator_lib::{
    interner::{Intern, UniqueStr},
    Registry,
};

pub fn get_sections<'a>(
    section_selection: Option<impl IntoIterator<Item = &'a str>>,
    reg: &Registry,
) -> (UniqueStr, HashSet<UniqueStr>) {
    if let Some(selected) = section_selection {
        let mut feature: Option<UniqueStr> = None;
        let mut extensions = HashSet::new();

        for str in selected {
            let section_name = str.intern(reg);

            if reg.get_feature_index(section_name).is_some() {
                if let Some(current) = feature {
                    if str > current.resolve_original() {
                        feature = Some(section_name);
                    }
                } else {
                    feature = Some(section_name);
                }
            } else if reg.get_extension_index(section_name).is_some() {
                foreach_extension_dependency(
                    section_name,
                    &mut extensions,
                    &mut |e, f| {
                        assert!(
                            feature.unwrap().resolve() >= f.resolve(),
                            "Extension '{}' requires core level '{}' while the configured is '{}'.",
                            e,
                            f,
                            feature.unwrap()
                        )
                    },
                    &mut |_| {},
                    reg,
                );
            } else {
                panic!("No such section '{}'", str);
            }
        }

        (feature.unwrap(), extensions)
    } else {
        let disabled = "disabled".intern(reg);
        let feature = reg
            .features
            .iter()
            .max_by_key(|f| f.name.resolve())
            .unwrap()
            .name;
        let extensions = reg
            .extensions
            .iter()
            .filter(|a| a.supported.get(0) != Some(&disabled))
            .map(|a| a.name)
            .collect::<HashSet<_>>();
        (feature, extensions)
    }
}

pub fn foreach_extension_dependency<F1: FnMut(UniqueStr, UniqueStr), F2: FnMut(UniqueStr)>(
    name: UniqueStr,
    extensions: &mut HashSet<UniqueStr>,
    on_feature: &mut F1,
    on_extension: &mut F2,
    reg: &Registry,
) {
    let extension = reg
        .get_extension(name)
        .unwrap_or_else(|| panic!("No such extension '{}'", name));

    if extensions.insert(name) == false {
        return;
    }

    on_extension(name);

    if let Some(core) = extension.requires_core {
        on_feature(name, core);
    }

    for &name in &extension.requires {
        foreach_extension_dependency(name, extensions, on_feature, on_extension, reg);
    }
}
