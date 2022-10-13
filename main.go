package main

// #include <stdio.h>
// void placeholder_c_func() {
//   printf("Hello, Cgo!");
// }
// void (*const const_c_func)() = &placeholder_c_func;
// void bridge_c_func() {
//   (*const_c_func)();
// }
import "C"

func CallC() {
	C.bridge_c_func()
}

func main() {
	CallC()
}
