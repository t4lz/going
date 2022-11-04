use bindgen::builder;

fn main() {
    let go_detour_bindings = builder().
        header("../go-detour/libgo-detour.h")
        .blocklist_function("rust_detour.*")
        .generate()
        .unwrap();
    go_detour_bindings.write_to_file("src/go_detour_ffi.rs").unwrap();
    println!("cargo:rustc-link-search=native=../go-detour");
    println!("cargo:rustc-link-lib=dylib=go-detour");
    println!("cargo:rustc-link-arg=-Wl,-rpath,../go-detour");
}
