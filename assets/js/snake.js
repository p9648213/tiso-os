export function setupSnakeToolBar() {
  const snakeToolBar = document.getElementById("snake-toolbar");

  const close = snakeToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    if (window.canvasRunning.includes("Snake")) {
      const event = new KeyboardEvent("keydown", {
        key: "Escape",
        code: "Escape",
        keyCode: 27,
        which: 27,
        bubbles: true,
      });
      window.dispatchEvent(event);
      window.stopSnake();
    }
  });
}
