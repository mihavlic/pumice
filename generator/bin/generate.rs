use std::{collections::HashSet, env::args_os, fs::read_to_string};

use dependencies::get_sections;

use generator::{codegen::write_bindings, context::Context};
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

    reg.finalize();

    let mut _tmp = None;
    let selected =
        if sections == "@all" {
            None
        } else {
            _tmp = Some(sections
            .to_str()
            .expect("Expected a comma separated list of ascii identifiers for section selection.")
            .replace(
                "@surface",
                "\
VK_KHR_surface,
VK_KHR_xlib_surface,
VK_KHR_xcb_surface,
VK_KHR_wayland_surface,
VK_KHR_mir_surface,
VK_KHR_android_surface,
VK_KHR_win32_surface,
VK_GGP_stream_descriptor_surface,
VK_NN_vi_surface,
VK_MVK_ios_surface,
VK_MVK_macos_surface,
VK_FUCHSIA_imagepipe_surface,
VK_EXT_metal_surface,
VK_EXT_headless_surface,
VK_EXT_directfb_surface,
VK_QNX_screen_surface,",
            ));

            Some(
                _tmp.as_ref()
                    .unwrap()
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty()),
            )
        };
    let (feature, extensions) = get_sections(selected, &reg);

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
