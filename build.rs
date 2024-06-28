use std::env;
use winresource::WindowsResource;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        if let Err(err) = WindowsResource::new().compile() {
            println!("cargo:warning=failed to compile windows resource: {err}");
        }
    }
}
