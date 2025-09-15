export function setupGlobalEvents() {
  window.addEventListener("changebackground", function (event) {
    document.body.setAttribute(
      "style",
      "background: " + event.detail.background
    );
  });
}
