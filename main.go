package main

// #include <stdio.h>
//
// void placeholder_c_func() {
//   printf("Cgo: placeholder_c_func\n");
// }
//
// void (*const const_c_func)() = &placeholder_c_func;
//
// static inline void c_caller(void (*const f)()) {
//   printf("Cgo: c_caller\n");
//   f();
// }
import "C"
import (
	"fmt"
)

func CallC() {
	fmt.Println("Go: CallC")
	C.c_caller(C.const_c_func)
}

func main() {
	fmt.Println("Go: main")
	CallC()
}
