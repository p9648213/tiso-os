import { MessageBox } from "./message_box.js";

export function setupGlobalFunctions() {
  window.loadSnakeModule = function () {
    const script = document.createElement("script");
    script.src = "/assets/snake/snake.js";
    script.onload = () => {
      SnakeModule({ canvas: document.getElementById("canvas") }).then(
        (instance) => {
          window.snakeState.wasmModule = instance;
          window.canvasRunning.push("Snake");
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
      window.snakeState.wasmModule.pauseMainLoop();
      window.snakeState.canvasContainer.remove();
      window.canvasRunning = window.canvasRunning.filter(
        (item) => item !== "Snake"
      );
      window.snakeState.wasmModule = null;
      window.snakeState.canvasContainer = null;

      if (window.gc) {
        window.gc();
      }

      document.getElementById("taskbar-snake-canvas-container").remove();

      console.log("Snake stopped");
    }
  };

  window.loadFlappyBirdModule = async function () {
    try {
      const canvasContainer = document.getElementById(
        "flappy-canvas-container"
      );
      if (!canvasContainer) {
        MessageBox.error("Error", "Canvas container not found");
        return;
      }

      const existingCanvas = canvasContainer.querySelector("canvas");
      if (existingCanvas) {
        existingCanvas.remove();
      }

      const canvas = document.createElement("canvas");
      canvas.id = `canvas`;
      canvasContainer.appendChild(canvas);

      const timestamp = Date.now();
      const wasm = await import(
        `/assets/flappybird/flappybird.js?t=${timestamp}`
      );

      await wasm.default();

      window.canvasRunning.push("Flappy Bird");
      window.flappyBirdState.wasmModule = wasm;
      window.flappyBirdState.canvasContainer = canvasContainer;

      console.log("Flappy Bird loaded successfully");
    } catch (error) {
      MessageBox.error("Error", "Failed to load Flappy Bird: " + error);
      window.canvasRunning = window.canvasRunning.filter(
        (item) => item !== "Flappy Bird"
      );
    }
  };

  window.stopFlappyBird = function () {
    if (window.flappyBirdState.canvasContainer) {
      window.flappyBirdState.canvasContainer.remove();
      window.canvasRunning = window.canvasRunning.filter(
        (item) => item !== "Flappy Bird"
      );
      window.flappyBirdState.wasmModule = null;
      window.flappyBirdState.canvasContainer = null;

      if (window.gc) {
        window.gc();
      }

      document.getElementById("taskbar-flappy-canvas-container").remove();

      console.log("Flappy Bird stopped");
    }
  };

  window.reloadTailwind = function () {
    const link = document.querySelector('link[href*="tailwind.css"]');

    if (link) {
      const baseUrl = link.href.split("?")[0];
      link.href = `${baseUrl}?v=${new Date().getTime()}`;
      console.log("Tailwind CSS reloaded");
    } else {
      console.error("Tailwind CSS link not found");
    }
  };

  window.injectScript = function (scriptString) {
    const container = document.createElement("div");
    container.innerHTML = scriptString.trim();

    const scriptEl = container.querySelector("script");
    if (!scriptEl) return;

    const newScript = document.createElement("script");
    newScript.type = "module";

    newScript.textContent = scriptEl.textContent;

    [...scriptEl.attributes].forEach((attr) =>
      newScript.setAttribute(attr.name, attr.value)
    );

    document.body.appendChild(newScript);
  };
}
