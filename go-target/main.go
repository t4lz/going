package main

import "C"
import "fmt"

//go:noinline
func HookMe() {
	fmt.Println("go-target: HookeMe")
}

func main() {
	fmt.Println("go-target: main")
	fmt.Println("address of HookMe:", HookMe)
	HookMe()
}
