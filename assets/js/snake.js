let snakeCleanUpEvent = [];

export function setupSnakeWindowGrab() {
  const snakeHeader = document.getElementById(`snake-toolbar`);
  const snakeWindow = document.getElementById(`snake-canvas-container`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  snakeHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = snakeWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    snakeWindow.style.left = `${event.clientX - offsetX}px`;
    snakeWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  snakeCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
  });
  snakeCleanUpEvent.push({
    event: "mouseup",
    handler: handleMouseUp,
  });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

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
    snakeCleanUpEvent.forEach((event) => {
      document.removeEventListener(event.event, event.handler);
    });
    snakeCleanUpEvent = [];
  });
}
