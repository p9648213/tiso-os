export function setupExplorerWindow(folderId) {
  const main = document.querySelector("main");
  const explorerWindow = document.getElementById(`explorer-window-${folderId}`);
  const explorerToolbar = document.getElementById(`explorer-toolbar`);
  const explorerSidebar = document.getElementById(`explorer-sidebar`);

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
