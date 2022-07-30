use std::{env::args_os, fs::read_to_string};

use generator::write_bindings;

fn main() {
    let args = args_os().skip(1).take(4).collect::<Vec<_>>();

    let vk_xml = &args[0];
    let video_xml = &args[1];
    let glue = &args[2];
    let out = &args[3];

    let selected_sections = vec!["@all", "VK_KHR_swapchain", "VK_KHR_video_queue"];

    write_bindings(
        &read_to_string(vk_xml)
            .unwrap_or_else(|_| panic!("Failed to read {}", vk_xml.to_string_lossy())),
        &read_to_string(video_xml)
            .unwrap_or_else(|_| panic!("Failed to read {}", video_xml.to_string_lossy())),
        &glue,
        &out,
        &selected_sections,
    )
    .unwrap();
}
