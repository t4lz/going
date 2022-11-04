package main

/*
#include "../hooks/hooks.h"
*/
import "C"
import "reflect"

func main() {}

//go:noinline
func goDetour() {
	// Call whatever function global_func_ref points to.
	C.rust_detour()
}

//go:noinline
func goDetourWithArgs(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 int64) int64 {
	// Call whatever function global_func_ref points to.
	return int64(C.rust_detour_with_args(C.int64_t(arg1), C.int64_t(arg2), C.int64_t(arg3), C.int64_t(arg4), C.int64_t(arg5), C.int64_t(arg6), C.int64_t(arg7), C.int64_t(arg8)))
}

//go:noinline
//export LoadMePls
func LoadMePls() (uintptr, uintptr) {
	// Otherwise the function is not present because of dead code elimination.
	return reflect.ValueOf(goDetour).Pointer(), reflect.ValueOf(goDetourWithArgs).Pointer()
}
