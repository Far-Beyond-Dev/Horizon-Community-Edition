package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"log"
	"unsafe"

	"github.com/tidwall/buntdb"
)

//export CreateDB
func CreateDB() uintptr {
	// Open the data.db file. It will be created if it doesn't exist.
	db, err := buntdb.Open("data.db")
	if err != nil {
		log.Fatal(err)
	}
	return uintptr(unsafe.Pointer(db))
	// return uintptr(uintptr(unsafe.Pointer(db)))
}

//export CloseDB
func CloseDB(db uintptr) {
	// Close the database when done.
	(*buntdb.DB)(unsafe.Pointer(db)).Close()
}

//export CreateSpatialIndex
func CreateSpatialIndex(db uintptr, indexName *C.char, indexKey *C.char) {
	// db.CreateSpatialIndex("fleet", "fleet:*:pos", buntdb.IndexRect)
	(*buntdb.DB)(unsafe.Pointer(db)).CreateSpatialIndex(C.GoString(indexName), C.GoString(indexKey), buntdb.IndexRect)
}

//export CreateGalaxy
func CreateGalaxy(db uintptr, key *C.char, value *C.char) {
	/*
		func do add Galaxy Data, made it by:
		Transform data: {Location: {x: 0, y: 0, z: 0},
		Rotation: {x: 0, y: 0, z: 0},
		Scale: {x: 0, y: 0, z: 0},
		Point Data: Array of relative location vectors paired with a brightness value and a color value(RGB):
		[{x: 0, y: 0, z: 0, brightness: 0, color: {r: 0, g: 0, b: 0}}]}
	*/
	(*buntdb.DB)(unsafe.Pointer(db)).Update(func(tx *buntdb.Tx) error {
		tx.Set(C.GoString(key), C.GoString(value), nil)
		return nil
	})
}

//export GetKNearestGalaxys
func GetKNearestGalaxys(db uintptr, key *C.char) *C.char {
	var result string
	(*buntdb.DB)(unsafe.Pointer(db)).View(func(tx *buntdb.Tx) error {
		tx.Nearby("galaxy", C.GoString(key), func(key, val string, dist float64) bool {
			result += key + ":" + val + ","
			return true
		})
		return nil
	})
	return C.CString(result)
}

func main() {
	// Prevent main from exiting immediately.
	select {}
}
