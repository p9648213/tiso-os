htmx.config.defaultSettleDelay = 0;
htmx.config.getCacheBusterParam = true;
htmx.config.selfRequestsOnly = true;
htmx.config.historyCacheSize = 0;
htmx.config.refreshOnHistoryMiss = true;

window.addEventListener("htmx:afterRequest", function (event) {
  if (event?.detail?.failed && event?.detail?.xhr?.responseText) {
    // handler error
  }
});

window.addEventListener("htmx:configRequest", function (event) {
  if (event.detail.verb !== "get") {
    event.detail.headers["X-Csrf-Protection"] = "1";
  }
});

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
