package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"fmt"
	"unsafe"
)

//export Greet
func Greet(name *C.char) *C.char {
	return C.CString(fmt.Sprintf("Hello from Go, %s!", C.GoString(name)))
}

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func main() {}
