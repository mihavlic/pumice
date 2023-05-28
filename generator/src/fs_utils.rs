use std::path::Path;

pub fn for_all_files(path: impl AsRef<Path>, mut fun: impl FnMut(&Path)) {
    for_all_files_inner(path.as_ref(), &mut fun)
}

fn for_all_files_inner(path: &Path, mut fun: &mut dyn FnMut(&Path)) {
    assert!(path.is_dir());

    let dir = std::fs::read_dir(path).unwrap();
    for path in dir.map(Result::unwrap) {
        let meta = path.metadata().unwrap();
        let path = path.path();
        if meta.is_file() {
            fun(&path);
        } else if meta.is_dir() {
            for_all_files(&path, &mut fun);
        }
    }
}
