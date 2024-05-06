const net = require('net');

// Define constants
const GAME_SERVER_PORT = 3000; // Port number for the game servers
const MASTER_SERVER_PORT = 4000; // Port number for the master server
const MAX_PLAYERS = 1000; // Maximum number of players

// Create an array to store connected clients
let clients = [];

// Create a TCP server for the game servers
const gameServer = net.createServer(socket => {
    console.log('Client connected: ' + socket.remoteAddress);

    // Add the new client to the array
    clients.push(socket);

    // Handle data received from clients
    socket.on('data', data => {
        // Broadcast the data to all clients except the sender
        clients.forEach(client => {
            if (client !== socket && !client.destroyed) {
                client.write(data);
            }
        });
    });

    // Handle client disconnection
    socket.on('end', () => {
        console.log('Client disconnected: ' + socket.remoteAddress);

        // Remove the disconnected client from the array
        clients = clients.filter(client => client !== socket);
    });
});

// Listen for connections on the game server port
gameServer.listen(GAME_SERVER_PORT, () => {
    console.log('Game server listening on port ' + GAME_SERVER_PORT);
});

// Create a TCP client for the master server
const masterClient = net.createConnection({ port: MASTER_SERVER_PORT }, () => {
    console.log('Connected to master server');
});

// Handle data received from the master server
masterClient.on('data', data => {
    // Process data received from the master server (e.g., syncing game states)
});

// Handle connection errors with the master server
masterClient.on('error', err => {
    console.error('Error connecting to master server:', err);
});

// Handle disconnection from the master server
masterClient.on('end', () => {
    console.log('Disconnected from master server');
});
