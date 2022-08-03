use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
};

use dependencies::get_sections;
use generator::Context;
use generator_lib::{
    configuration::GenConfig, interner::Intern, process_registry_xml, Registry, Symbol,
};

mod dependencies;

fn main() {
    let mut reg_file = BufWriter::new(
        File::create(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../generated/dump.ron"
        ))
        .unwrap(),
    );

    let vk = true;
    let video = true;
    let workarounds = true;

    let mut reg = Registry::new();

    if vk {
        process_registry_xml(
            &mut reg,
            &std::fs::read_to_string("./vk.xml").unwrap(),
            None,
        );
    }

    if video {
        process_registry_xml(
            &mut reg,
            &std::fs::read_to_string("./video.xml").unwrap(),
            None,
        );
    }

    if !workarounds {
        write!(reg_file, "{:#?}", &reg).unwrap();
    }

    let (feature, extensions) = get_sections(Option::<std::iter::Once<_>>::None, &reg);

    let conf = GenConfig {
        extensions,
        feature,
        profile: None,
        apis: HashSet::from(["vulkan".intern(&reg)]),
        protect: HashSet::new(),
        pass_all: true,
    };

    let ctx = Context::new(conf, reg);

    if workarounds {
        write!(reg_file, "{:#?}", &ctx.reg).unwrap();
    }

    let mut own_file = BufWriter::new(
        File::create(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../generated/ownership.txt"
        ))
        .unwrap(),
    );

    for &Symbol(name, _) in &ctx.reg.symbols {
        let section = if let Some(section) = ctx.symbol_get_section(name) {
            section.name.resolve()
        } else {
            "INVALID"
        };

        writeln!(own_file, "{}: {}", name.resolve(), section).unwrap();
    }
}
