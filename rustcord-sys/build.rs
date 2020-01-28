use std::env;
use std::path::PathBuf;

const LIB_VERSION: &str = "3.4.0";

fn main() {
    let config = cmake::Config::new("discord-rpc".to_string()).build();
    println!("cargo:rustc-link-search={}", config.join("lib").display());
    println!("cargo:rustc-link-search={}", config.join("lib64").display());

    let include_path = format!("discord-rpc-{}/include", LIB_VERSION);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=static=discord-rpc");
}
