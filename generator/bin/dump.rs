use std::{
    fs::File,
    io::{BufWriter, Write},
};

use generator_lib::{process_registry, Interner};

fn main() {
    let file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/dump.ron"
    ))
    .unwrap();

    let interner = Interner::new();
    let registry = process_registry(
        &std::fs::read_to_string("/home/eg/Downloads/vk.xml").unwrap(),
        &interner,
    );

    write!(BufWriter::new(file), "{:#?}", registry).unwrap();
}
