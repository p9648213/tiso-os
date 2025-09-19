export function setupGlobalEvents() {
  window.addEventListener("openFile", function (event) {
    const taskbarMinimize = document.getElementById("taskbar-minimize");

    const img = event.detail.image;
    const windowId = event.detail.window_id;

    const imageContainer = document.createElement("div");
    imageContainer.id = `taskbar-${windowId}`;

    imageContainer.classList.add(
      "p-1.5",
      "bg-zinc-700",
      "rounded-sm",
      "flex",
      "cursor-pointer"
    );
    imageContainer.draggable = false;

    const imageEl = document.createElement("img");

    imageEl.src = img;
    imageEl.classList.add("w-6", "h-6");
    imageEl.draggable = false;

    imageContainer.appendChild(imageEl);
    taskbarMinimize.appendChild(imageContainer);
  });
}
