// gameServer.js
const socketio = require('socket.io');

class GameServer {
    constructor(port) {
        this.port = port;
        this.clients = [];
        this.io = null;
    }

    start() {
        this.io = socketio(this.port);

        this.io.on('connection', socket => {
            console.log('Client connected: ' + socket.id);

            this.clients.push(socket);

            socket.on('disconnect', () => {
                console.log('Client disconnected: ' + socket.id);
                this.removeClient(socket);
            });

            socket.on('data', data => {
                console.log(data);
                this.broadcast(data, socket);
            });
        });

        console.log('Game server listening on port ' + this.port);
    }

    broadcast(data, sender) {
        this.clients.forEach(client => {
            if (client !== sender) {
                client.emit('data', data);
            }
        });
    }

    removeClient(socket) {
        this.clients = this.clients.filter(client => client !== socket);
    }

    stop() {
        this.io.close();
    }
}

module.exports = GameServer;
