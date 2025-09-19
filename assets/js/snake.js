export function setupSnakeToolBar() {
  const snakeToolBar = document.getElementById("snake-toolbar");

  const close = snakeToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    window.stopSnake();
  });
}
