use std::path::Path;

use generator_lib::process_registry;

fn main() {
    let (reg, errors) = vk_parse::parse_file(&Path::new("/home/eg/Downloads/vk.xml")).unwrap();
    dbg!(errors);
    process_registry(reg).unwrap();
}
