use std::{env::args_os, fs::read_to_string};

use generator::{codegen::write_bindings, context::Context};
use generator_lib::{configuration::GenConfig, Registry};

struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let severity = record.level();
        eprint!("{severity} ");
        // if let Some(module) = record.module_path() {
        //     eprint!("{module}::");
        // }
        if let Some(file) = record.file() {
            eprint!("{file}");
        }
        if let Some(line) = record.line() {
            eprint!(":{line}");
        }
        eprint!(" ");
        eprintln!("{}", record.args());
    }

    fn flush(&self) {
        // stderr is not buffered
    }
}

fn main() {
    log::set_logger(&SimpleLogger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let args = args_os().skip(1).take(5).collect::<Vec<_>>();

    let vk_xml = &args[0];
    let video_xml = &args[1];
    let out = &args[2];

    let mut reg = Registry::new();
    let conf = GenConfig::full(&reg);

    let vk_xml = read_to_string(vk_xml)
        .unwrap_or_else(|_| panic!("Failed to read {}", vk_xml.to_string_lossy()));
    let video_xml = read_to_string(video_xml)
        .unwrap_or_else(|_| panic!("Failed to read {}", video_xml.to_string_lossy()));

    reg.append_registry_xml(&vk_xml, &conf);
    reg.append_registry_xml(&video_xml, &conf);
    reg.finalize();

    let ctx = Context::new(reg, conf);

    write_bindings(ctx, out.as_ref());
}
