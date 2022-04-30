use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use generator_lib::process_registry;

#[rustfmt::skip]
fn main() {
    let (reg, errors) = vk_parse::parse_file(&Path::new("/home/eg/Downloads/vk.xml")).unwrap();
    if !errors.is_empty() {
        eprintln!("{:#?}", errors);
    }

    let file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/dump.ron"
    ))
    .unwrap();

    let registry = process_registry(reg);
    
    write!(BufWriter::new(file), "{:#?}", registry).unwrap();
}
