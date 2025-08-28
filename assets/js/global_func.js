export function setupGlobalFunctions() {
  window.loadSnakeModule = function () {
    const script = document.createElement("script");
    script.src = "/assets/snake/snake.js";
    script.onload = () => {
      SnakeModule({ canvas: document.getElementById("canvas") }).then(
        (instance) => {
          window.snakeInstance = instance;
          console.log("Snake wasm is running");
        }
      );
    };
    document.body.appendChild(script);
  };

  window.loadFlappyBirdModule = async function () {
    const init = (await import("/assets/flappybird/flappybird.js")).default;
    await init();
  };
}
