use std::{
    fs::File,
    io::{BufWriter, Write},
};

use generator_lib::process_registry;

#[rustfmt::skip]
fn main() {
    let file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/dump.ron"
    ))
    .unwrap();

    let registry = process_registry(&std::fs::read_to_string("/home/eg/Downloads/vk.xml").unwrap());
    
    write!(BufWriter::new(file), "{:#?}", registry).unwrap();
}
