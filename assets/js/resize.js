let timeout;

window.addEventListener("resize", () => {
  clearTimeout(timeout);

  const main = document.querySelector("main");
  main.classList.add("invisible");

  htmx
    .ajax("POST", `/action/create-grid`, {
      target: "main",
      values: {
        height: main.clientHeight,
        width: main.clientWidth,
      },
    })
    .then(() => {
      main.classList.remove("invisible");
    });
});
