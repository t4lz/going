package main

// #include <stdio.h>
// void placeholder_c_func() {
//   printf("Cgo: placeholder_c_func\n");
// }
// void (*const const_c_func)() = &placeholder_c_func;
// void bridge_c_func() {
//   printf("Cgo: bridge_c_func\n");
//   (*const_c_func)();
// }
import "C"
import (
	"fmt"
)

func CallC() {
	fmt.Println("Go: CallC")
	C.bridge_c_func()
}

func main() {
	fmt.Println("Go: main")
	CallC()
}
