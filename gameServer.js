// gameServer.js
const net = require('net');

class GameServer {
    constructor(port) {
        this.port = port;
        this.clients = [];
        this.server = null;
    }

    start() {
        this.server = net.createServer(socket => {
            console.log('Client connected: ' + socket.remoteAddress);

            this.clients.push(socket);

            socket.on('data', data => {
                this.broadcast(data, socket);
            });

            socket.on('end', () => {
                console.log('Client disconnected: ' + socket.remoteAddress);
                this.removeClient(socket);
            });
        });

        this.server.on('error', err => {
            console.error('Game server error:', err);
        });

        this.server.listen(this.port, () => {
            console.log('Game server listening on port ' + this.port);
        });
    }

    broadcast(data, sender) {
        this.clients.forEach(client => {
            if (client !== sender && !client.destroyed) {
                client.write(data);
            }
        });
    }

    removeClient(socket) {
        this.clients = this.clients.filter(client => client !== socket);
    }

    stop(callback) {
        this.server.close(callback);
    }
}

module.exports = GameServer;
