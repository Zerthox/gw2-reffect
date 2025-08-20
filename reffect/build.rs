use std::env;
use winresource::WindowsResource;

fn main() {
    #[cfg(feature = "profile")]
    println!("cargo:warning=profiling enabled");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    #[allow(clippy::collapsible_if)]
    if target_os == "windows" {
        if let Err(err) = WindowsResource::new().compile() {
            println!("cargo:warning=failed to compile windows resource: {err}");
        }
    }
}
