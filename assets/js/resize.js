export function setupResize() {
  let resizeTimeout;

  window.addEventListener("resize", () => {
    clearTimeout(resizeTimeout);

    const main = document.querySelector("main");
    if (!main) return;

    main.classList.add("invisible");

    resizeTimeout = setTimeout(() => {
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
    }, 300);
  });
}
