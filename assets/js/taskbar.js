export function setupTaskbarMenuToggle() {
  const menu = document.getElementById("taskbar-menu");
  const menuIcon = document.getElementById("taskbar-menu-icon");

  menuIcon.addEventListener("click", () => {
    menu.classList.toggle("hidden");
  });

  document.addEventListener("click", (event) => {
    if (!menu.contains(event.target) && !menuIcon.contains(event.target)) {
      menu.classList.add("hidden");
    }
  });
}

export function setupTaskbarMenuFiles() {
  const menu = document.getElementById("taskbar-menu");
  const menuFiles = document.getElementById("taskbar-menu-files");

  for (const file of menuFiles.querySelectorAll("div")) {
    let fileType = file.getAttribute("data-file-type");

    file.addEventListener("click", () => {
      htmx
        .ajax("GET", `/read/file/${fileType}`, {
          target: "body",
          swap: "beforeend",
        })
        .then(() => {
          menu.classList.add("hidden");
        });
    });
  }
}
