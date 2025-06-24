const CONTEXT_MENU_SCREEN = ["Create document", "Create folder"];
const CONTEXT_MENU_ITEM = ["Rename", "Delete"];

export function setupDesktopContextMenu() {
  document.addEventListener("mouseup", (event) => {
    let contextMenuEl = document.getElementById("context_menu");

    if (contextMenuEl && !event.target.contains(contextMenuEl)) {
      switch (event.button) {
        case 0:
          document.body.removeChild(contextMenuEl);
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
      "p-2",
      "flex",
      "flex-col",
      "g-3",
      "rounded-md",
      "bg-white",
      "absolute"
    );
    contextMenu.style.left = `${event.x}px`;
    contextMenu.style.top = `${event.y}px`;

    const checkDesktopItem = event.target.closest(
      '[id^="file-"], [id^="folder-"]'
    );

    if (checkDesktopItem) {
      window.selectedItem = checkDesktopItem;
      checkDesktopItem.childNodes[0].classList.add("bg-blue-900");

      for (const itemText of CONTEXT_MENU_ITEM) {
        let menuItems = document.createElement("form");

        menuItems.textContent = itemText;
        menuItems.style.cursor = "pointer";

        contextMenu.appendChild(menuItems);
      }
    } else {
      for (const itemText of CONTEXT_MENU_SCREEN) {
        let menuItems = document.createElement("form");
        let id = itemText.replace(/\s/g, "").toLowerCase();
        let itemsType = "txt";

        if (itemText === "Create folder") {
          itemsType = "folder";
        }

        menuItems.textContent = itemText;
        menuItems.style.cursor = "pointer";
        menuItems.setAttribute("id", id);

        if (itemsType === "txt") {
          menuItems.addEventListener("mouseup", () => {
            let targetId = checkEmptySpace();
            if (targetId) {
              htmx.ajax("POST", `/create/txt/${desktopId}/${targetId}`, {
                target: `#${targetId}`,
              });
            }
          });
        } else if (itemsType === "folder") {
          menuItems.addEventListener("mouseup", () => {
            let targetId = checkEmptySpace();
            if (targetId) {
              htmx.ajax("POST", `/create/folder/${desktopId}/${targetId}`, {
                target: `#${targetId}`,
              });
            }
          });
        }

        contextMenu.appendChild(menuItems);
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
