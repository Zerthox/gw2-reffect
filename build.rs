use std::{env, fs, path::Path};
use winresource::WindowsResource;

const DEPS_PATH: &str = "deps";

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        if let Err(err) = WindowsResource::new().compile() {
            println!("cargo:warning=failed to compile windows resource: {err}");
        }
    }

    let target_path = Path::new(&manifest).join("target").join(profile);
    let deps = Path::new(&manifest).join(DEPS_PATH);
    for entry in deps.read_dir().unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
        if entry.metadata().unwrap().is_file() {
            fs::copy(entry.path(), target_path.join(entry.file_name())).unwrap();
        }
    } // TODO: avoid unnecessary copy? remove? link?
}
