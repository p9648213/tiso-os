const CONTEXT_MENU_SCREEN = ["Create document", "Create folder"];
const CONTEXT_MENU_ITEM = ["Rename", "Delete"];

export function setupDesktopContextMenu() {
  document.addEventListener("mouseup", (event) => {
    let contextMenuEl = document.getElementById("context_menu");

    if (contextMenuEl && !event.target.contains(contextMenuEl)) {
      switch (event.button) {
        case 0:
          document.body.removeChild(contextMenuEl);
          if (window.selectedItem) {
            window.selectedItem.childNodes[0].classList.remove("bg-blue-900");
            window.selectedItem = null;
          }
          break;
        default:
          break;
      }
    }
  });

  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();

    if (window.selectedItem) {
      window.selectedItem.childNodes[0].classList.remove("bg-blue-900");
      window.selectedItem = null;
    }

    if (document.body.style.cursor == "wait") {
      return;
    }

    let contextMenuEl = document.getElementById("context_menu");
    let desktopId = document.getElementById("desktop_id").value;

    if (contextMenuEl) {
      document.body.removeChild(contextMenuEl);
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
        let itemsType = "txt";

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
          });
        } else if (itemsType === "folder") {
          menuForm.addEventListener("mouseup", () => {
            let targetId = checkEmptySpace();
            if (targetId) {
              htmx.ajax("POST", `/create/folder/${desktopId}/${targetId}`, {
                target: `#${targetId}`,
              });
            }
          });
        }

        contextMenu.appendChild(menuForm);
      }
    }

    document.body.appendChild(contextMenu);
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
