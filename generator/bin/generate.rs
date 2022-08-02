use std::{collections::HashSet, env::args_os, fs::read_to_string};

use generator::{write_bindings, Context, SectionKind};
use generator_lib::{
    configuration::GenConfig,
    interner::{Intern, UniqueStr},
    process_registry_xml, Registry,
};

fn main() {
    let args = args_os().skip(1).take(5).collect::<Vec<_>>();

    let vk_xml = &args[0];
    let video_xml = &args[1];
    let glue = &args[2];
    let out = &args[3];
    let sections = &args[4];

    let mut reg = Registry::new();

    let vk_xml = read_to_string(vk_xml)
        .unwrap_or_else(|_| panic!("Failed to read {}", vk_xml.to_string_lossy()));
    process_registry_xml(&mut reg, &vk_xml, None);

    let video_xml = read_to_string(video_xml)
        .unwrap_or_else(|_| panic!("Failed to read {}", video_xml.to_string_lossy()));
    process_registry_xml(&mut reg, &video_xml, None);

    let mut ctx = Context::new(reg);

    let (feature, extensions) = if sections == "@all" {
        let feature = ctx
            .reg
            .features
            .iter()
            .max_by_key(|f| f.name.resolve())
            .unwrap()
            .name;
        let extensions = ctx
            .extensions
            .iter()
            .filter(|a| a.supported.get(0) != Some(&ctx.strings.disabled))
            .map(|a| a.name)
            .collect::<HashSet<_>>();
        (feature, extensions)
    } else {
        let selected = sections.to_str().unwrap().split(',').map(|s| s.trim());

        let mut feature: Option<UniqueStr> = None;
        let mut extensions = HashSet::new();

        for section_name in selected {
            let section = ctx.find_section(section_name.intern(&ctx));

            match section.kind {
                // features MUST come before extensions
                SectionKind::Feature(_) => {
                    if let Some(current) = feature {
                        if section.name.resolve() > current.resolve() {
                            feature = Some(section.name);
                        }
                    } else {
                        feature = Some(section.name);
                    }
                }
                SectionKind::Extension(_) => {
                    foreach_dependency(
                        section.name,
                        &mut extensions,
                        &mut |s| assert!(feature.unwrap().resolve() >= s.resolve()),
                        &mut |_| {},
                        &ctx,
                    );
                }
            }
        }

        (feature.unwrap(), extensions)
    };

    let conf = GenConfig {
        extensions,
        feature,
        profile: None,
        apis: HashSet::from(["vulkan".intern(&ctx)]),
        protect: HashSet::new(),
    };

    write_bindings(&mut ctx, glue, out, &conf).unwrap();
}

pub fn foreach_dependency<F1: FnMut(UniqueStr), F2: FnMut(UniqueStr)>(
    name: UniqueStr,
    extensions: &mut HashSet<UniqueStr>,
    on_feature: &mut F1,
    on_extension: &mut F2,
    ctx: &Context,
) {
    let section = ctx.find_section(name);
    match section.kind {
        SectionKind::Feature(_) => unreachable!(),
        SectionKind::Extension(idx) => {
            if extensions.insert(name) == false {
                return;
            }

            on_extension(name);

            let extension = &ctx.reg.extensions[idx as usize];

            if let Some(core) = extension.requires_core {
                on_feature(core);
            }

            for &name in &extension.requires {
                foreach_dependency(name, extensions, on_feature, on_extension, ctx);
            }
        }
    }
}
