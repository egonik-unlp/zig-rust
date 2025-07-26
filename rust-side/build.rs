use std::io::{self, Write};

fn main() {
    let command = std::process::Command::new("zig")
        .args(["build"])
        .current_dir("../zig-side/")
        .status()
        .expect("Failed to run zig build");
    if !command.success() {
        panic!("Cannot compile zig side");
    }
    println!("cargo:rerun-if-changed=../zig-side/build.zig");
    println!("cargo:rerun-if-changed=../zig-side/src/root.zig");
    println!("cargo:rerun-if-changed=root.zig");
    println!("cargo:rustc-link-search=../zig-side/zig-out/lib/");
    println!("cargo:rustc-link-lib=static=piclibrs")
}

// Si yo quisisera usar una libreria dinamica, deberia hacer todo esto

// {
//  println!("cargo:rustc-link-search=native=../zig-side/zig-out/lib");
//     println!("cargo:rustc-link-lib=dylib=zig_side");
//
//     // Opcional: agregar rpath para evitar LD_LIBRARY_PATH
//     println!("cargo:rustc-link-arg=-Wl,-rpath=../zig-side/zig-out/lib");}
