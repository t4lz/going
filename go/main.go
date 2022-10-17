package main

// #include <stdio.h>
// #include <stdint.h>
// void placeholder_c_func() {
// }
// void (*const const_c_func)() = &placeholder_c_func;
// void bridge_c_func() {
//   (*const_c_func)();
// }
// uintptr_t c_syscall6(uintptr_t trap, uintptr_t nargs, uintptr_t a1, uintptr_t a2, uintptr_t a3, uintptr_t a4, uintptr_t a5, uintptr_t a6) {
//   (*const_c_func)();
//   return 42;
// }
import "C"

func CallC() {
	C.bridge_c_func()
}

func syscall6(trap, nargs, a1, a2, a3, a4, a5, a6 uintptr) (r1, r2, err uintptr) {
	res := uintptr(C.c_syscall6(C.uintptr_t(trap), C.uintptr_t(nargs), C.uintptr_t(a1), C.uintptr_t(a2), C.uintptr_t(a3), C.uintptr_t(a4), C.uintptr_t(a5), C.uintptr_t(a6)))
	return res, res, 0
}

func main() {
	syscall6(1, 2, 3, 4, 5, 6, 7, 8)
}
