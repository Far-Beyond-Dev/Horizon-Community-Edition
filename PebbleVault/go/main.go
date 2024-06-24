package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"fmt"
	"unsafe"
	//"log"
	//"github.com/tidwall/buntdb"
)

//export Greet
func Greet(name *C.char) *C.char {
	return C.CString(fmt.Sprintf("Hello from Go, %s!", C.GoString(name)))
}

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func main() {
	// Open the data.db file. It will be created if it doesn't exist.
	//db, err := buntdb.Open("data.db")
	//if err != nil {
	//	log.Fatal(err)
	//}
	//
	//defer db.Close()
}
