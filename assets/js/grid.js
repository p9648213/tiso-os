const CONTEXT_MENU_SCREEN = ["Create document", "Create folder", "Refresh"];
const CONTEXT_MENU_ITEM = ["Rename", "Delete"];

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

export function setupGridContextMenu() {
  const main = document.querySelector("main");

  main.addEventListener("mouseup", (event) => {
    let contextMenuEl = document.getElementById("context_menu");

    if (contextMenuEl && !event.target.contains(contextMenuEl)) {
      switch (event.button) {
        case 0:
          document.body.removeChild(contextMenuEl);
          removeSelectedItem();
          break;
        default:
          break;
      }
    }
  });

  main.addEventListener("contextmenu", (event) => {
    event.preventDefault();

    removeSelectedItem();

    if (document.body.style.cursor == "wait") {
      return;
    }

    let oldContextMenu = document.getElementById("context_menu");
    let desktopId = document.getElementById("desktop_id").value;

    if (oldContextMenu) {
      document.body.removeChild(oldContextMenu);
    }

    let contextMenu = document.createElement("div");

    contextMenu.addEventListener("contextmenu", (event) => {
      event.preventDefault();
    });

    contextMenu.setAttribute("id", "context_menu");
    contextMenu.classList.add(
      "w-50",
      "py-1.5",
      "px-1.5",
      "flex",
      "flex-col",
      "rounded-sm",
      "bg-zinc-900",
      "text-white",
      "absolute",
      "border",
      "border-zinc-700"
    );
    contextMenu.style.left = `${event.x}px`;
    contextMenu.style.top = `${event.y}px`;

    const desktopItem = event.target.closest('[id^="file-"], [id^="folder-"]');

    if (desktopItem) {
      window.selectedItem = desktopItem;
      desktopItem.childNodes[0].classList.add("bg-blue-900");

      for (const contextItem of CONTEXT_MENU_ITEM) {
        let menuForm = document.createElement("form");
        let menuItem = document.createElement("div");

        menuItem.textContent = contextItem;
        menuItem.classList.add(
          "hover:bg-zinc-800",
          "px-2",
          "py-1",
          "rounded-sm"
        );

        menuForm.append(menuItem);
        menuForm.style.cursor = "pointer";

        const splitId = desktopItem.id.split("-");
        const itemType = splitId[0];
        const itemId = splitId[1];

        if (contextItem === "Delete") {
          menuForm.addEventListener("mouseup", () => {
            htmx.ajax("POST", `/delete/${itemType}/${itemId}`, {
              target: `#${desktopItem.id}`,
              swap: "outerHTML",
            });
            document.body.removeChild(contextMenu);
          });
        }

        if (contextItem === "Rename") {
          if (itemType === "file") {
            let fileType = desktopItem.getAttribute("data-file-type");

            menuForm.addEventListener("mouseup", () => {
              htmx
                .ajax("GET", `/read/${fileType}/input/${itemId}`, {
                  target: `#${desktopItem.id}`,
                  swap: "outerHTML",
                })
                .then(() => {
                  window.editMode = true;
                });
              document.body.removeChild(contextMenu);
            });
          }
          if (itemType === "folder") {
            menuForm.addEventListener("mouseup", () => {
              htmx
                .ajax("GET", `/read/folder/input/${itemId}`, {
                  target: `#${desktopItem.id}`,
                  swap: "outerHTML",
                })
                .then(() => {
                  window.editMode = true;
                });
              document.body.removeChild(contextMenu);
            });
          }
        }

        contextMenu.appendChild(menuForm);
      }
    } else {
      for (const contextItem of CONTEXT_MENU_SCREEN) {
        let menuForm = document.createElement("form");
        let menuItem = document.createElement("div");

        let id = contextItem.replace(/\s/g, "").toLowerCase();
        let itemsType = null;

        if (contextItem === "Create document") {
          itemsType = "txt";
        }

        if (contextItem === "Create folder") {
          itemsType = "folder";
        }

        menuItem.textContent = contextItem;
        menuItem.classList.add(
          "hover:bg-zinc-800",
          "px-2",
          "py-1",
          "rounded-sm"
        );

        menuForm.append(menuItem);
        menuForm.style.cursor = "pointer";
        menuForm.setAttribute("id", id);

        if (itemsType === "txt") {
          menuForm.addEventListener("mouseup", () => {
            let targetId = checkEmptySpace();
            if (targetId) {
              htmx.ajax("POST", `/create/txt/${desktopId}/${targetId}`, {
                target: `#${targetId}`,
              });
            }
            document.body.removeChild(contextMenu);
          });
        } else if (itemsType === "folder") {
          menuForm.addEventListener("mouseup", () => {
            let targetId = checkEmptySpace();
            if (targetId) {
              htmx.ajax("POST", `/create/folder/${desktopId}/${targetId}`, {
                target: `#${targetId}`,
              });
            }
            document.body.removeChild(contextMenu);
          });
        }

        if (contextItem === "Refresh") {
          menuForm.addEventListener("mouseup", () => {
            window.location.reload();
          });
        }

        contextMenu.appendChild(menuForm);
      }
    }

    document.body.appendChild(contextMenu);
  });
}

export function setupGridItemSingleSelect() {
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
        removeSelectedItem();
      }

      if (window.editMode === false) {
        window.selectedItem = checkDesktopItem;
        checkDesktopItem.childNodes[0].classList.add("bg-blue-900");
      }
    } else {
      removeSelectedItem();
    }
  });
}

export function setupGridItemDrag() {
  let main = document.querySelector("main");

  let draggedItem = null;

  main.addEventListener("dragstart", (event) => {
    if (event.target.innerHTML && !window.editMode) {
      draggedItem = event.target;
    } else {
      draggedItem = null;
      event.preventDefault();
    }
  });

  main.addEventListener("dragover", (event) => {
    event.preventDefault();
  });

  main.addEventListener("drop", async (event) => {
    let destopId = document.getElementById("desktop_id").value;

    if (draggedItem && event.target && draggedItem.id && event.target.id) {
      const dragElement = document.getElementById(draggedItem.id);
      const dropElement = document.getElementById(event.target.id);

      if (dragElement.innerHTML && !dropElement.innerHTML) {
        const dropPosition = dropElement.id;
        const dragChild = dragElement.childNodes[0];
        const dragItemIdSplit = dragChild.id.split("-");

        const itemType = dragItemIdSplit[0];
        const itemId = dragItemIdSplit[1];

        const response = await fetch(
          `/update/${itemType}/position/${itemId}/${destopId}/${dropPosition}`,
          {
            method: "POST",
            headers: {
              "X-Csrf-Protection": "1",
            },
          }
        );

        if (response.ok) {
          dropElement.appendChild(dragChild);
        }
      }
    }
  });
}

export function setupGridItemOpen() {
  const main = document.querySelector("main");

  main.addEventListener("dblclick", () => {
    if (window.selectedItem) {
      const splitId = window.selectedItem.id.split("-");
      const itemType = splitId[0];
      const itemId = splitId[1];

      if (itemType === "file") {
        const fileType = window.selectedItem.getAttribute("data-file-type");

        htmx
          .ajax(
            "GET",
            `/read/${fileType}/${itemId}/${main.clientHeight}/${main.clientWidth}`,
            {
              target: "body",
              swap: "beforeend",
            }
          )
          .then(() => {
            removeSelectedItem();
          });
      }
    }
  });
}

function checkEmptySpace() {
  const totalRows = document.getElementById("screen_rows")?.value;
  const totalCols = document.getElementById("screen_cols")?.value;

  if (totalRows && totalCols) {
    for (let i = 0; i < totalCols; i++) {
      for (let j = 0; j < totalRows; j++) {
        const item = document.getElementById(`item-${j}-${i}`);
        if (item && item.innerHTML == "") {
          return `item-${j}-${i}`;
        }
      }
    }
  } else {
    return null;
  }
}

function removeSelectedItem() {
  if (window.selectedItem) {
    window.selectedItem.childNodes[0].classList.remove("bg-blue-900");
    window.selectedItem = null;
  }
}
