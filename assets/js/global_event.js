export function setupGlobalEvents() {
  window.addEventListener("openFile", function (event) {
    const img = event.detail.image;
    const windowId = event.detail.window_id;

    const duplicateWindow = document.getElementById(windowId);

    if (duplicateWindow) {
      duplicateWindow.remove();
    }

    const taskbarItemId = `taskbar-${windowId}`;

    const duplicateTaskbarItem = document.getElementById(taskbarItemId);

    if (duplicateTaskbarItem) {
      duplicateTaskbarItem.remove();
    }

    const taskbarMinimize = document.getElementById("taskbar-minimize");

    const imageContainer = document.createElement("div");
    imageContainer.id = taskbarItemId;

    imageContainer.classList.add("p-1.5", "bg-zinc-700", "rounded-sm", "flex");
    imageContainer.draggable = false;

    const imageEl = document.createElement("img");

    imageEl.src = img;
    imageEl.classList.add("w-6", "h-6");
    imageEl.draggable = false;

    imageContainer.appendChild(imageEl);
    taskbarMinimize.appendChild(imageContainer);
  });
}
