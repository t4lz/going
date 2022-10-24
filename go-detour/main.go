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

func detour() {
	// fmt.Println("go detour")
	// Call whatever function global_func_ref points to.
	C.bridge(global_func_ptr)
}

//export Initialize
func Initialize(c_detour C.callback) uintptr {
	detourFunc := reflect.ValueOf(detour)
	detourPtr := detourFunc.Pointer()
	fmt.Println("Initialize (Go)")
	fmt.Println("Go detour address:\t", detour)
	fmt.Printf("Go detour address:\t 0x%x\n", detourPtr)
	fmt.Printf("C (Rust) detour: %p\n", c_detour)
	global_func_ptr = c_detour // Make the go detour call the c-ABI detour.
	return detourPtr
}
