package main

import (
	"fmt"
	"log"
	"net/http"
	"time"

	socketio "github.com/googollee/go-socket.io"
	"github.com/tidwall/buntdb"
)

func main() {
	fmt.Println("Start server...")

	// Initialize a new Socket.IO server
	server := socketio.NewServer(nil)

	// Handle connection event
	server.OnConnect("/", func(s socketio.Conn) error {
		s.SetContext("")
		fmt.Println("Connected:", s.ID())
		return nil
	})

	// Handle "update" event
	server.OnEvent("/", "update", func(s socketio.Conn, data string) {
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
		s.Emit("updateResult", fmt.Sprintf("Update transaction took: %s", elapsed))
	})

	// Handle disconnection event
	server.OnDisconnect("/", func(s socketio.Conn, reason string) {
		fmt.Println("Disconnected:", s.ID(), reason)
	})

	// Serve the Socket.IO server
	go func() {
		if err := server.Serve(); err != nil {
			log.Fatalf("SocketIO listen error: %s\n", err)
		}
	}()
	defer server.Close()

	http.Handle("/socket.io/", server)

	// Serve a simple HTML page on root path
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Go server is up and running!")
	})

	// Listen on port 3003
	log.Println("Serving at localhost:3001...")
	log.Fatal(http.ListenAndServe(":3001", nil))
}
