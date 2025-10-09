let flappyCleanUpEvent = [];

export function setupFlappyWindowGrab() {
  const flappyHeader = document.getElementById(`flappy-toolbar`);
  const flappyWindow = document.getElementById(`flappy-canvas-container`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  flappyHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = flappyWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    flappyWindow.style.left = `${event.clientX - offsetX}px`;
    flappyWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  flappyCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
  });
  flappyCleanUpEvent.push({
    event: "mouseup",
    handler: handleMouseUp,
  });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

export function setupFlappyToolBar() {
  const flappyToolBar = document.getElementById("flappy-toolbar");

  const close = flappyToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    if (window.canvasRunning.includes("Flappy Bird")) {
      const event = new KeyboardEvent("keydown", {
        key: "Escape",
        code: "Escape",
        keyCode: 27,
        which: 27,
        bubbles: true,
      });
      window.dispatchEvent(event);
      window.stopFlappyBird();
    }
    flappyCleanUpEvent.forEach((event) => {
      document.removeEventListener(event.event, event.handler);
    });
    flappyCleanUpEvent = [];
  });
}
