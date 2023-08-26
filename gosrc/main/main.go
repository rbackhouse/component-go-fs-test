package main

import (
	"fmt"
	"os"

	"potpie.org/fs-test/gosrc/testfs"
)

type TestfsImpl struct{}

func (ti TestfsImpl) Openfile() {
	fmt.Printf("Test GO fs called\n")

	f, err := os.Create("test.log")
	//f, err := os.OpenFile("test.log", os.O_APPEND|os.O_WRONLY|os.O_CREATE, 0644)
	if err != nil {
		fmt.Printf("Failed to open file %s\n", err)
		return
	}
	defer f.Close()
	f.WriteString(fmt.Sprintf("%s\n", "Some data!"))
	fmt.Printf("test.log created\n")
}

func init() {
	impl := TestfsImpl{}
	testfs.SetTestfs(impl)
}

func main() {}
