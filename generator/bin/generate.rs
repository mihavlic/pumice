use std::{collections::HashSet, env::args_os, fs::read_to_string};

use dependencies::get_sections;
use generator::{write_bindings, Context};
use generator_lib::{configuration::GenConfig, interner::Intern, process_registry_xml, Registry};

mod dependencies;

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

    let selected = sections.to_str().unwrap().split(',').map(|s| s.trim());
    let (feature, extensions) = get_sections(Some(selected), &reg);

    let conf = GenConfig {
        extensions,
        feature,
        profile: None,
        apis: HashSet::from(["vulkan".intern(&reg)]),
        protect: HashSet::new(),
    };

    let mut ctx = Context::new(conf, reg);

    write_bindings(&mut ctx, glue, out).unwrap();
}
