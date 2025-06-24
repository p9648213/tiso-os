export function setupGridDimensions() {
  const main = document.querySelector("main");

  htmx.ajax("POST", "/create/grid", {
    target: "main",
    values: {
      height: main.clientHeight,
      width: main.clientWidth,
    },
  });
}
