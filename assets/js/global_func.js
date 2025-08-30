export function setupGlobalFunctions() {
  window.loadSnakeModule = function () {
    if (window.snakeState.isRunning) {
      console.log("Snake already running");
      return;
    }
    const script = document.createElement("script");
    script.src = "/assets/snake/snake.js";
    script.onload = () => {
      SnakeModule({ canvas: document.getElementById("canvas") }).then(
        (instance) => {
          window.snakeState.wasmModule = instance;
          window.snakeState.isRunning = true;
          window.snakeState.canvasContainer = document.getElementById(
            "snake-canvas-container"
          );
          console.log("Snake wasm is running");
        }
      );
    };
    document.body.appendChild(script);
  };

  window.stopSnake = function () {
    if (window.snakeState.canvasContainer) {
      window.snakeState.canvasContainer.remove();
      window.snakeState.isRunning = false;
      window.snakeState.wasmModule = null;
      window.snakeState.canvasContainer = null;

      // Force garbage collection if available
      if (window.gc) {
        window.gc();
      }

      console.log("Snake stopped");
    }
  };

  window.loadFlappyBirdModule = async function () {
    // Prevent multiple instances
    if (window.flappyBirdState.isRunning) {
      console.log("Flappy bird already running");
      return;
    }

    try {
      // Create a fresh canvas each time
      const canvasContainer = document.getElementById(
        "flappy-canvas-container"
      );
      if (!canvasContainer) {
        console.error("Canvas container not found");
        return;
      }

      // Clear any existing canvas
      const existingCanvas = canvasContainer.querySelector("canvas");
      if (existingCanvas) {
        existingCanvas.remove();
      }

      // Create new canvas
      const canvas = document.createElement("canvas");
      canvas.id = `canvas`;
      canvasContainer.appendChild(canvas);

      // Import and initialize WASM module
      // Note: Using dynamic import with timestamp to bypass cache
      const timestamp = Date.now();
      const wasm = await import(
        `/assets/flappybird/flappybird.js?t=${timestamp}`
      );

      await wasm.default();

      // Update state
      window.flappyBirdState.isRunning = true;
      window.flappyBirdState.wasmModule = wasm;
      window.flappyBirdState.canvasContainer = canvasContainer;

      console.log("Flappy Bird loaded successfully");
    } catch (error) {
      console.error("Failed to load Flappy Bird:", error);
      window.flappyBirdState.isRunning = false;
    }
  };

  window.stopFlappyBird = function () {
    if (window.flappyBirdState.canvasContainer) {
      window.flappyBirdState.canvasContainer.remove();
      window.flappyBirdState.isRunning = false;
      window.flappyBirdState.wasmModule = null;
      window.flappyBirdState.canvasContainer = null;

      // Force garbage collection if available
      if (window.gc) {
        window.gc();
      }

      console.log("Flappy Bird stopped");
    }
  };
}
