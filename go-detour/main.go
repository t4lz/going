package main

// #include "bridge.h"
import "C"
import (
	"fmt"
	"strconv"
	"unsafe"
)

func main() {}

// global pointer to a void function with no args.
var global_func_ptr C.callback

func detour() {
	fmt.Println("go detour")
	// Call whatever function global_func_ref points to.
	C.bridge(global_func_ptr)
}

//export Initialize
func Initialize(c_detour C.callback) uintptr {
	detour_func := detour
	detourPtr := *(*uintptr)(unsafe.Pointer(&detour_func))
	fmt.Println("Initialize (Go)")
	fmt.Println("Go detour address (func value):\t\t", detour)
	fmt.Printf("Go detour address (with printf):\t%p\n", detour)
	fmt.Printf("Go detour address (uintptr):\t\t0x%x\n", detourPtr)

	s := fmt.Sprintf("%p", detour)
	res, _ := strconv.ParseUint(s[2:], 16, 64)
	ptr := uintptr(res)
	fmt.Printf("Go detour address (parsed string):\t0x%x\n", ptr)

	fmt.Printf("C (Rust) detour: %p\n", c_detour)
	global_func_ptr = c_detour // Make the go detour call the c-ABI detour.
	return ptr
}
