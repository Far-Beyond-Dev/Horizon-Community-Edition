package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"time"

	socketio "github.com/zhouhui8915/go-socket.io-client"
	"github.com/tidwall/buntdb"
)

func main() {
	fmt.Println("Start server...")

	// Serve a simple HTML page on root path
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Go server is up and running!")
	})

	// Listen on port 8000
	ln, err := net.Listen("tcp", ":3003")
	if err != nil {
		log.Fatal(err)
	}
	defer ln.Close()

	// Initialize a new server
	server, err := socketio.NewClient([]string{"http://localhost:3001"}, nil)
	if err != nil {
		fmt.Println("Connected to Socket.IO server: ", err)
	} else {
		fmt.Println("Connected to Socket.IO server")
	}

	// Handle '/socket.io/' endpoint
	go server.On("updateResult", func(data string) {
		fmt.Println("Received update result:", data)
	})

	go http.Serve(ln, nil)

	// Emit "DBUp" event
	go func() {
		time.Sleep(2 * time.Second) // Assuming 2 seconds for the server to start
		server.Emit("DBUp", "")
	}()

	// Handle events
	server.On("update", func(data string) {
		fmt.Println("Received update request:", data)

		// Open the database (use in-memory for this example)
		db, err := buntdb.Open(":memory:")
		if err != nil {
			log.Fatal(err)
		}
		defer db.Close()

		start := time.Now()
		err = db.Update(func(tx *buntdb.Tx) error {
			// Perform transaction here based on data
			return nil
		})
		if err != nil {
			log.Println("Error in update transaction:", err)
			return
		}
		elapsed := time.Since(start)

		// Emit performance measurement
		server.Emit("updateResult", fmt.Sprintf("Update transaction took: %s", elapsed))
	})

	// Run forever
	select {}
}
