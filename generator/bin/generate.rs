use std::{env::args_os, fs::read_to_string};

use generator::write_bindings;

fn main() {
    let args = args_os().skip(1).take(3).collect::<Vec<_>>();

    let vk_xml = &args[0];
    let video_xml = &args[1];
    let out = &args[2];

    let selected_sections = vec!["VK_VERSION_1_0", "VK_KHR_swapchain", "VK_KHR_video_queue"];

    write_bindings(
        &read_to_string(vk_xml).unwrap(),
        &read_to_string(video_xml).unwrap(),
        &out,
        &selected_sections,
    )
    .unwrap();
}
