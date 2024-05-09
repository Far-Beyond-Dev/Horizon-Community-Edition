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
	// Create a new Socket.IO server
	server, err := socketio.NewServer(nil)
	if err != nil {
		log.Fatal(err)
	}

	// Handle connection events
	server.OnConnect("/", func(s socketio.Conn) error {
		s.SetContext("")
		fmt.Println("New connection:", s.ID())
		return nil
	})

	// Handle disconnection events
	server.OnDisconnect("/", func(s socketio.Conn, reason string) {
		fmt.Println("Connection closed:", s.ID(), reason)
	})

	// Handle transactions
	server.OnEvent("/", "update", func(s socketio.Conn, txData string) {
		fmt.Println("Received update request:", txData)

		// Open the data.db file. It will be created if it doesn't exist.
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

	// Serve the Socket.IO server at /socket.io endpoint
	http.Handle("/socket.io/", server)
	http.Handle("/", http.FileServer(http.Dir("./public")))

	// Start the HTTP server
	fmt.Println("Server started at :3001")
	log.Fatal(http.ListenAndServe(":3001", nil))
}
