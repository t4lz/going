#![feature(once_cell)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("go_detour_ffi.rs");

#[macro_use]
extern crate lazy_static;


use std::ffi::c_void;
use ctor::ctor;
use frida_gum::{interceptor::Interceptor, Gum, Module, NativePointer};

lazy_static! {
    static ref GUM: Gum = unsafe { Gum::obtain() };
}

unsafe extern "C" fn detour() {
    println!("Rust detour!");
}


#[ctor]
fn on_load() {
    println!("Rust lib loaded.");
    let go_detour_addr = unsafe { Initialize(Some(detour)) };
    println!("Go detour addr: {:?}", go_detour_addr);
    let mut interceptor = Interceptor::obtain(&GUM);
    // let hook_target = Module::find_symbol_by_name("main", "HookMe").unwrap();
    let hook_target = Module::find_symbol_by_name("", "main.HookMe").unwrap();
    // let hook_target = Module::find_export_by_name(None, "main.HookMe").unwrap();
    // let hook_target = Module::find_export_by_name(Some("go-target"), "main.HookMe").unwrap();
    interceptor.replace(hook_target, NativePointer(go_detour_addr), NativePointer(0 as *mut c_void)).unwrap();
}