package main

import "fmt"

func hook_me() {
	fmt.Println("go-target: hook_me")
}

func main() {
	fmt.Println("go-target: main")
	hook_me()
}
