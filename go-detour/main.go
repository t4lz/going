package main

// #include "bridge.h"
import "C"
import (
	"fmt"
	"reflect"
)

func main() {}

// global pointer to a void function with no args.
var global_func_ptr C.callback

// global pointer to a void function with 2 args returning int64.
var global_func_ptr_args C.callback_with_args

//go:noinline
func detour() {
	// Call whatever function global_func_ref points to.
	C.bridge(global_func_ptr)
}

//go:noinline
func detour_with_args(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 int64) int64 {
	// Call whatever function global_func_ref points to.
	return int64(C.bridge_with_args(global_func_ptr_args, C.int64_t(arg1), C.int64_t(arg2), C.int64_t(arg3), C.int64_t(arg4), C.int64_t(arg5), C.int64_t(arg6), C.int64_t(arg7), C.int64_t(arg8)))
}

//export Initialize
func Initialize(c_detour C.callback, c_detour_with_args C.callback_with_args) (uintptr, uintptr) {
	fmt.Println("Initialize (Go)")
	global_func_ptr = c_detour                                         // Make the go detour call the c-ABI detour.
	global_func_ptr_args = c_detour_with_args                          // Make the go detour call the c-ABI detour.
	fmt.Println("Call detour directly from Initialize for debugging:") // TODO: DELETE
	detour()                                                           // TODO: DELETE
	detourFunc := reflect.ValueOf(detour)
	detourPtr := detourFunc.Pointer()
	fmt.Println("Go detour address:\t", detour)
	fmt.Printf("Go detour address:\t 0x%x\n", detourPtr)

	detourArgsFunc := reflect.ValueOf(detour_with_args)
	detourArgsPtr := detourArgsFunc.Pointer()
	fmt.Println("Go detour address:\t", detour_with_args)
	fmt.Printf("Go detour address:\t 0x%x\n", detourArgsPtr)

	fmt.Printf("C (Rust) detour: %p\n", c_detour)
	return detourPtr, detourArgsPtr
}
