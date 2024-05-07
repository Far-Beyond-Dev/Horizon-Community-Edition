// gameServer.js
const socketio = require('socket.io');

// Disable perMessageDeflate
const io = socketio({
    perMessageDeflate: false
});

class GameServer {
    constructor(port) {
        this.port = port;
        this.io = io; // Assign io to the instance
        this.MAX_CLIENTS = 1000; // Maximum number of clients to keep in memory
        this.clientSockets = new Map(); // Map to store socket objects by client ID
    }

    start() {
        this.io = socketio(this.port);

        this.io.on('connection', socket => {
            console.log('Client connected: ' + socket.id);

            // Reject connection if maximum clients reached
            if (this.clientSockets.size >= this.MAX_CLIENTS) {
                console.log('Max clients reached. Rejecting connection for: ' + socket.id);
                socket.disconnect(true);
                return;
            }

            // Add socket to map
            this.clientSockets.set(socket.id, socket);

            socket.on('disconnect', () => {
                console.log('Client disconnected: ' + socket.id);
                this.removeClient(socket.id);
            });

            socket.on('data', data => {
                // Process data efficiently to avoid memory leaks
                this.broadcast(data, socket.id);
            });

            socket.on('PrintMe', message => {
                console.log('PrintMe event received with message:', message);
            });
        });

        console.log('Game server listening on port ' + this.port);
    }

    broadcast(data, senderId) {
        // Efficiently handle broadcasting to avoid memory leaks
        for (const [clientId, clientSocket] of this.clientSockets) {
            if (clientId !== senderId) {
                // Use try-catch block to handle any errors in broadcasting
                try {
                    clientSocket.emit('data', data);
                } catch (error) {
                    console.error('Error broadcasting data to client:', error);
                }
            }
        }
    }

    removeClient(socketId) {
        this.clientSockets.delete(socketId);
    }

    stop() {
        // Disconnect all clients
        for (const [clientId, clientSocket] of this.clientSockets) {
            clientSocket.disconnect(true);
        }
        this.io.close();
    }
}

module.exports = GameServer;
