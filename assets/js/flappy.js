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
      window.stopFlappy();
    }
  });
}
