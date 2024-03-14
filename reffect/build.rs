use std::{env, fs, path::Path};

const DEPS_PATH: &str = "deps";

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    // target path of parent workspace
    let target = Path::new(&manifest).join("..").join("target").join(profile);

    let deps = Path::new(&manifest).join(DEPS_PATH);
    for entry in deps.read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            fs::copy(entry.path(), target.join(entry.file_name())).unwrap();
        }
    }
}
