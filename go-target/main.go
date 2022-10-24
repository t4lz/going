package main

import "C"
import "fmt"

//go:noinline
func HookMe() {
	fmt.Println("go-target: HookMe")
}

//go:noinline
func HookMeWithArgs(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 int64) int64 {
	fmt.Println("go-target: HookMeWithArgs")
	return arg1 + arg2 + arg3 + arg4 + arg5 + arg6 + arg7 + arg8
}

func main() {
	fmt.Println("go-target: main")
	fmt.Println("address of HookMe:", HookMe)
	HookMe()
	fmt.Println("address of HookMeWithArgs:", HookMeWithArgs)
	res := HookMeWithArgs(1, 2, 3, 4, 5, 6, 7, 8)
	fmt.Println("res: ", res)
}
