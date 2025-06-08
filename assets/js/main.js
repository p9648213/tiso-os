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
    // handler error
  }
});

window.addEventListener("htmx:configRequest", function (event) {
  if (event.detail.verb !== "get") {
    event.detail.headers["X-Csrf-Protection"] = "1";
  }
});
