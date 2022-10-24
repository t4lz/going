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

unsafe extern "C" fn detour() {
    println!("Rust detour!");
}

unsafe extern "C" fn detour_with_args(arg1: i64, arg2: i64) -> i64 {
    println!("Rust detour!");
    return 10 * (arg1 + arg2);
}

#[ctor]
fn on_load() {
    println!("Rust lib loaded.");
    let mut interceptor = Interceptor::obtain(&GUM);
    let hook_target = Module::find_symbol_by_name("", "main.HookMe").unwrap();
    let go_detour_addrs = unsafe { Initialize(Some(detour), Some(detour_with_args)) };
    let go_detour_addr= go_detour_addrs.r0;
    let go_detour_with_args_addr= go_detour_addrs.r1;
    println!("Go detour addr: {:#x}", go_detour_addr);
    interceptor
        .replace(
            hook_target,
            NativePointer(go_detour_addr as *mut c_void),
            NativePointer(0 as *mut c_void),
        )
        .unwrap();

    let hook_target = Module::find_symbol_by_name("", "main.HookMeWithArgs").unwrap();
    println!("Go detour addr: {:#x}", go_detour_with_args_addr);
    interceptor
        .replace(
            hook_target,
            NativePointer(go_detour_with_args_addr as *mut c_void),
            NativePointer(0 as *mut c_void),
        )
        .unwrap();
}
