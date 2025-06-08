export function setupGridDimensions() {
  const main = document.querySelector("main");

  htmx.ajax("POST", "/action/create-grid", {
    target: "main",
    values: {
      height: main.clientHeight,
      width: main.clientWidth,
    },
  });
}
