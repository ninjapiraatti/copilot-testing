import init, { GameController } from '../pkg/breakout_wasm.js';

async function start() {
    // Initialize the WASM module
    await init();

    const canvas = document.getElementById('gameCanvas');
    const gameController = GameController.new('gameCanvas');
    
    // Start the game loop
    gameController.start();
    
    // Set up keyboard controls
    window.addEventListener('keydown', (event) => {
        gameController.handle_key_down(event);
    });
    
    window.addEventListener('keyup', (event) => {
        gameController.handle_key_up(event);
    });
    
    // Game loop
    function gameLoop(timestamp) {
        gameController.update(timestamp);
        gameController.render();
        requestAnimationFrame(gameLoop);
    }
    
    requestAnimationFrame(gameLoop);
}

start().catch(console.error);
