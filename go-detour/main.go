package main

// #include "bridge.h"
import "C"
import "fmt"

func main() {}

// global pointer to a void function with no args.
var global_func_ptr C.callback

func detour() {
	fmt.Println("go detour")
	// Call whatever function global_func_ref points to.
	C.bridge(global_func_ptr)
}

//export Initialize
func Initialize(c_detour C.callback) func() {
	global_func_ptr = c_detour // Make the go detour call the c-ABI detour.
	return detour              // Return the adress of the Go-abi detour.
}
