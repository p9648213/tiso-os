let explorerCleanUpEvent = [];

export function setupExplorerWindow(folderId) {
  const main = document.querySelector("main");
  const explorerWindow = document.getElementById(`explorer-window-${folderId}`);
  const explorerToolbar = document.getElementById(
    `explorer-toolbar-${folderId}`
  );
  const explorerSidebar = document.getElementById(
    `explorer-sidebar-${folderId}`
  );

  explorerToolbar.addEventListener("mouseup", (event) => {
    if (event.target.id === "explorer-toolbar-close") {
      explorerWindow.remove();
    }
  });

  explorerSidebar.addEventListener("mouseup", (event) => {
    const id = event.target.id;

    if (id.startsWith("sidebar-")) {
      const folderType = id.split("-")[1];

      htmx.ajax(
        "GET",
        `/read/folder/explorer/${folderType}/0/${main.clientHeight}/${main.clientWidth}`,
        {
          target: `#explorer-window-${folderId}`,
          swap: "outerHTML",
        }
      );
    }
  });
}

export function setupExplorerWindowGrab(folderId) {
  const explorerHeader = document.getElementById(
    `explorer-toolbar-${folderId}`
  );
  const explorerWindow = document.getElementById(`explorer-window-${folderId}`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  explorerHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = explorerWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    explorerWindow.style.left = `${event.clientX - offsetX}px`;
    explorerWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    console.log("here");

    isDragging = false;
  }

  explorerCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
    id: folderId,
  });
  explorerCleanUpEvent.push({
    event: "mouseup",
    handler: handleMouseUp,
    id: folderId,
  });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

export function setupExplorerToolBar(folderId) {
  const explorerToolbar = document.getElementById(
    `explorer-toolbar-${folderId}`
  );

  const close = explorerToolbar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`explorer-window-${folderId}`).remove();
    document.getElementById(`taskbar-explorer-window-${folderId}`).remove();
    explorerCleanUpEvent.forEach((event) => {
      if (event.id === folderId) {
        document.removeEventListener(event.event, event.handler);
      }
    });
    explorerCleanUpEvent = explorerCleanUpEvent.filter(
      (event) => event.id !== folderId
    );
  });
}
