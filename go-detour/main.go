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

func detourRoutine(trap, arg1, arg2, arg3 uintptr, uintptrChannel chan uintptr, errnoChannel chan syscall.Errno) {
	res := C.syscall_bridge(global_func_ptr, C.uintptr_t(trap), C.uintptr_t(arg1), C.uintptr_t(arg2), C.uintptr_t(arg3))
	uintptrChannel <- uintptr(res.res1)
	uintptrChannel <- uintptr(res.res2)
	errnoChannel <- syscall.Errno(res.errno)
}

//go:noinline
func syscallDetour(trap, arg1, arg2, arg3 uintptr) (uintptr, uintptr, syscall.Errno) {
	uintptrChannel := make(chan uintptr)
	errnoChannel := make(chan syscall.Errno)
	go detourRoutine(trap, arg1, arg2, arg3, uintptrChannel, errnoChannel)
	return <-uintptrChannel, <-uintptrChannel, <-errnoChannel
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
