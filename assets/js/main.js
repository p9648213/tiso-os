import { MessageBox } from "./message_box.js"

htmx.config.defaultSettleDelay = 0;
htmx.config.getCacheBusterParam = true;
htmx.config.selfRequestsOnly = true;
htmx.config.historyCacheSize = 0;
htmx.config.refreshOnHistoryMiss = true;

window.addEventListener("htmx:beforeRequest", function () {
  document.body.style.cursor = "wait";
});

window.addEventListener("htmx:afterRequest", function (event) {
  document.body.style.cursor = "auto";

  if (event?.detail?.failed && event?.detail?.xhr?.responseText) {
    MessageBox.error("Error", event?.detail?.xhr?.responseText);
  }
});

window.addEventListener("htmx:configRequest", function (event) {
  if (event.detail.verb !== "get") {
    event.detail.headers["X-Csrf-Protection"] = "1";
  }
});

window.addEventListener("keydown", function (event) {
  if (event.key === "Escape") {
    if (window.canvasRunning.includes("Snake")) {
      window.stopSnake();
    }
    if (window.canvasRunning.includes("Flappy Bird")) {
      window.stopFlappyBird();
    }
  }
});
