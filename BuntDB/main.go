package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"time"

	"github.com/googollee/go-socket.io"
	"github.com/tidwall/buntdb"
)

func main() {
	fmt.Println("Start server...")

	// Serve a simple HTML page on root path
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Go server is up and running!")
	})

	// Listen on port 8000
	ln, err := net.Listen("tcp", ":3001")
	if err != nil {
		log.Fatal(err)
	}
	defer ln.Close()

	// Initialize a new server
	server := socketio.NewServer(nil)
	if err != nil {
		log.Fatal(err)
	} else {
		fmt.Println("Server listening on port 3001")
	}

	// Handle '/socket.io/' endpoint
	http.Handle("/socket.io/", server)
	go http.Serve(ln, nil)

	// Handle events
	server.OnEvent("/", "update", func(s socketio.Conn, txData string) {
		fmt.Println("Received update request:", txData)

		// Open the database (use in-memory for this example)
		db, err := buntdb.Open(":memory:")
		if err != nil {
			log.Fatal(err)
		}
		defer db.Close()

		start := time.Now()
		err = db.Update(func(tx *buntdb.Tx) error {
			// Perform transaction here based on txData
			return nil
		})
		if err != nil {
			log.Println("Error in update transaction:", err)
			return
		}
		elapsed := time.Since(start)

		// Emit performance measurement
		s.Emit("updateResult", fmt.Sprintf("Update transaction took: %s", elapsed))
	})

	// Run forever
	select {}
}
