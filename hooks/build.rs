fn main() {
    println!("cargo:rustc-link-search=native=../go-detour");
    println!("cargo:rustc-link-lib=dylib=go-detour");
    println!("cargo:rustc-link-arg=-Wl,-rpath,../go-detour");
}