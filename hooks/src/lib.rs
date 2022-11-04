#![feature(once_cell)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("go_detour_ffi.rs");


#[macro_use]
extern crate lazy_static;
extern crate core;

use ctor::ctor;
use frida_gum::{interceptor::Interceptor, Gum, Module, NativePointer};
use std::ffi::c_void;

lazy_static! {
    static ref GUM: Gum = unsafe { Gum::obtain() };
}

#[no_mangle]
pub unsafe extern "C" fn rust_detour() {
    println!("Rust detour!");
}

#[no_mangle]
pub unsafe extern "C" fn rust_detour_with_args(arg1: i64, arg2: i64, arg3: i64, arg4: i64, arg5: i64, arg6: i64, arg7: i64, arg8: i64) -> i64 {
    println!("Rust detour!");
    return 10 * (arg1 + arg2 + arg3 + arg4 + arg5 + arg6 + arg7 + arg8);
}

#[ctor]
fn on_load() {
    println!("Rust lib loaded.");
    let _go_pointers = unsafe { LoadMePls() };
    let mut interceptor = Interceptor::obtain(&GUM);
    // for module in Module::enumerate_modules() {
    //     println!("{}: {:#x}", &module.name, &module.base_address);
    //     for sym in Module::enumerate_symbols(&module.name) {
    //         println!("{} - {}: {:#x}", &module.name, sym.name, sym.address)
    //     }
    // }
    let hook_target = Module::find_symbol_by_name("", "main.HookMe").unwrap();
    let go_detour_frida = Module::find_symbol_by_name("libgo-detour.so", "main.goDetour").unwrap();
    println!("Go detour addr from Frida: {:?}", go_detour_frida.0);
    interceptor
        .replace(
            hook_target,
            go_detour_frida,
            NativePointer(0 as *mut c_void),
        )
        .unwrap();

    let hook_target = Module::find_symbol_by_name("", "main.HookMeWithArgs").unwrap();
    let go_detour_args_frida = Module::find_symbol_by_name("libgo-detour.so", "main.goDetourWithArgs").unwrap();
    println!("Go detour (args) addr from Frida: {:?}", go_detour_args_frida.0);
    interceptor
        .replace(
            hook_target,
            go_detour_args_frida,
            NativePointer(0 as *mut c_void),
        )
        .unwrap();
}
