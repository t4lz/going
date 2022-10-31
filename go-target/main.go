package main

import "C"
import (
	"fmt"
	"syscall"
)

func main() {
	fmt.Println("go-target: main")
	r1, r2, err := syscall.RawSyscall(0, 1, 2, 3)
	if r1 != 42 || r2 != 1337 || err != 0 {
		panic("Did not get expected return values from hook.")
	}
}
