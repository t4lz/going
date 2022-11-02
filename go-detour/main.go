package main

// #include "bridge.h"
import "C"
import (
	"fmt"
	"reflect"
	"syscall"
)

func main() {}

// global pointer to a C-ABI function with the same signature as RawSyscall.
var global_func_ptr C.syscall_callback

//go:noinline
func syscallDetour(trap, arg1, arg2, arg3 uintptr) (uintptr, uintptr, syscall.Errno) {
	res := C.syscall_bridge(global_func_ptr, C.uintptr_t(trap), C.uintptr_t(arg1), C.uintptr_t(arg2), C.uintptr_t(arg3))
	return uintptr(res.res1), uintptr(res.res2), syscall.Errno(res.errno)
}

//export Initialize
func Initialize(rustDetour C.syscall_callback) uintptr {
	fmt.Println("Initialize (Go)")
	global_func_ptr = rustDetour
	detourFunc := reflect.ValueOf(syscallDetour)
	detourPtr := detourFunc.Pointer()
	fmt.Printf("Go detour address:\t 0x%x\n", detourPtr)

	fmt.Printf("C (Rust) detour: %p\n", rustDetour)
	return detourPtr
}
