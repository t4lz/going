// #![feature(once_cell)]

// include!("go_detour_ffi.rs");


// use std::ffi::c_void;
use ctor::ctor;
// use frida_gum::{interceptor::Interceptor, Gum};
// use std::sync::LazyLock;
// use tokio::runtime::Runtime;

// lazy_static! {
//     static ref GUM: Gum = unsafe { Gum::obtain() };
// }

// unsafe extern "C" fn detour() {
//     println!("Rust detour!");
// }


#[ctor]
fn on_load() {
    println!("Rust lib loaded.");
    // let mut interceptor = Interceptor::obtain(&GUM);
    // interceptor.begin_transaction();
    // let go_detour_addr = unsafe { Initialize(Some(detour)) };
    // println!("Go detour addr: {:?}", go_detour_addr);
}