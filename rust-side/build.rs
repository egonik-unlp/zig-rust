fn main() {
    println!("cargo:rustc-link-search=../zig-side/zig-out/lib/");
    println!("cargo:rustc-link-lib=static=lazigsideta")
}

// Si yo quisisera usar una libreria dinamica, deberia hacer todo esto

// {
//  println!("cargo:rustc-link-search=native=../zig-side/zig-out/lib");
//     println!("cargo:rustc-link-lib=dylib=zig_side");
//
//     // Opcional: agregar rpath para evitar LD_LIBRARY_PATH
//     println!("cargo:rustc-link-arg=-Wl,-rpath=../zig-side/zig-out/lib");}
