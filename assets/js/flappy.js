export function setupFlappyToolBar() {
  const flappyToolBar = document.getElementById("flappy-toolbar");

  const close = flappyToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    window.stopFlappyBird();
  });
}
