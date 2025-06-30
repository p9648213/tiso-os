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

export function setupGridResize() {
  let resizeTimeout;

  window.addEventListener("resize", () => {
    clearTimeout(resizeTimeout);

    const main = document.querySelector("main");
    if (!main) return;

    main.classList.add("invisible");

    resizeTimeout = setTimeout(() => {
      htmx
        .ajax("POST", `/create/grid`, {
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

export function setupItemSingleSelect() {
  const main = document.querySelector("main");

  main.addEventListener("click", (event) => {
    const checkDesktopItem = event.target.closest(
      '[id^="file-"], [id^="folder-"]'
    );

    if (checkDesktopItem) {
      if (
        window.selectedItem &&
        window.selectedItem.id == checkDesktopItem.id
      ) {
        return;
      }

      if (window.selectedItem) {
        window.selectedItem.childNodes[0].classList.remove("bg-blue-900");
      }

      if (window.editMode === false) {
        window.selectedItem = checkDesktopItem;
        checkDesktopItem.childNodes[0].classList.add("bg-blue-900");
      }
    } else {
      if (window.selectedItem) {
        window.selectedItem.childNodes[0].classList.remove("bg-blue-900");
        window.selectedItem = null;
      }
    }
  });
}
