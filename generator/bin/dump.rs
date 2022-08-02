use std::{
    fs::File,
    io::{BufWriter, Write},
};

use generator::{Context, INVALID_SECTION};
use generator_lib::{interner::Intern, process_registry_xml, Registry};

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

    let ctx = Context::new(reg);

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

    for (i, section) in ctx.symbol_ownership.iter().enumerate() {
        let name = ctx.reg.symbols[i].0;
        let section = match *section {
            INVALID_SECTION => "INVALID".intern(&ctx),
            other => ctx.sections[other as usize].name,
        };

        writeln!(own_file, "{}: {}", name.resolve(), section.resolve()).unwrap();
    }
}
