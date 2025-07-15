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

  for (const file of menuFiles.childNodes) {
    file.addEventListener("click", () => {
      menu.classList.add("hidden");
    });
  }
}
