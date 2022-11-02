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

unsafe extern "C" fn detour_for_raw_syscall(_trap: usize, _arg1: usize, _arg2: usize, _arg3: usize) -> raw_syscall_result {
    println!("Rust detour!");
    return raw_syscall_result {
        res1: 42,
        res2: 1337,
        errno: 0
    }
}

#[ctor]
fn on_load() {
    println!("Rust lib loaded.");
    let mut interceptor = Interceptor::obtain(&GUM);
    // for module in Module::enumerate_modules() {
    //     for sym in Module::enumerate_symbols(&module.name) {
    //         if sym.name == "syscall.RawSyscall" {
    //             println!("{} - {}: {:#x}", &module.name, sym.name, sym.address);
    //         }
    //     }
    // }

    let hook_target = Module::find_symbol_by_name("", "syscall.RawSyscall").unwrap();
    let _ = unsafe { Initialize(Some(detour_for_raw_syscall)) };
    let go_detour_frida = Module::find_symbol_by_name("libgo-detour.so", "main.syscallDetour").unwrap();
    println!("Go detour addr from Frida: {:?}", go_detour_frida.0);
    interceptor
        .replace(
            hook_target,
            go_detour_frida,
            NativePointer(0 as *mut c_void),
        )
        .unwrap();
}
