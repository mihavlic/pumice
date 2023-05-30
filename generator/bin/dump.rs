use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};

use generator::context::{Context, SectionFunctions};
use generator_lib::{configuration::GenConfig, Registry, Symbol};

fn main() {
    let reg_path = PathBuf::from_str(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/dump.ron"
    ))
    .unwrap();

    std::fs::create_dir_all(reg_path.parent().unwrap()).unwrap();

    let mut reg_file = BufWriter::new(File::create(reg_path).unwrap());

    let vk = true;
    let video = true;
    let workarounds = true;

    let mut reg = Registry::new();
    let conf = GenConfig::full(&reg);

    if vk {
        reg.append_registry_xml(&std::fs::read_to_string("./vk.xml").unwrap(), &conf);
    }

    if video {
        reg.append_registry_xml(&std::fs::read_to_string("./video.xml").unwrap(), &conf);
    }

    reg.finalize();

    if !workarounds {
        write!(reg_file, "{:#?}", &reg).unwrap();
    }

    let ctx = Context::new(reg, conf);

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
        let section = ctx.get_symbol_section_body(name).map(|s| s.name());
        let section = section.as_ref().map(|s| s.resolve()).unwrap_or("INVALID");

        writeln!(own_file, "{}: {}", name.resolve(), section).unwrap();
    }
}
