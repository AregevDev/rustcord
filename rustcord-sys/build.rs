use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("discord-rpc")
        .define("BUILD_EXAMPLES", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        dst.join("lib64").display()
    );

    let include_path = "discord-rpc/include".to_string();

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

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "linux" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        "macos" => println!("cargo:rustc-link-lib=dylib=c++"),
        "windows" => {},
        _ => panic!("Unsupported platform!"),
    }
}
