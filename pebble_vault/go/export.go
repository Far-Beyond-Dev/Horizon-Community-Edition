package main

/*
#include <stdint.h>

// Declare the Add function here to make it visible to other languages
int32_t Add(int32_t a, int32_t b);
*/
import "C"

//export Add
func Add(a, b C.int) C.int {
	return a + b
}

func main() {}
