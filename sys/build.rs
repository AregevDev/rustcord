use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=sys/lib/windows");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=sys/lib/macos");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-search=sys/lib/linux");

    println!("cargo:rustc-link-lib=discord-rpc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}