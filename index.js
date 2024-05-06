// index.js
const GameServer = require('./gameServer');

const GAME_SERVER_PORT = 3000;
const MASTER_SERVER_PORT = 4000;

const gameServer = new GameServer(GAME_SERVER_PORT);

gameServer.start();

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('Closing servers...');
    gameServer.stop(() => {
        console.log('Game server stopped.');
    });
});
