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
  const main = document.querySelector("main");
  const menu = document.getElementById("taskbar-menu");
  const menuFiles = document.getElementById("taskbar-menu-files");

  for (const file of menuFiles.querySelectorAll(".file")) {
    let fileType = file.getAttribute("data-file-type");

    file.addEventListener("click", () => {
      if (
        (fileType === "flappybird" || fileType === "snake") &&
        window.canvasRunning.length > 0
      ) {
        return menu.classList.add("hidden");
      }

      if (fileType === "web_builder") {
        let fileId = file.getAttribute("id").split("-")[1];
        
        htmx
          .ajax(
            "GET",
            `/read/file/${fileType}/${fileId}/${main.clientHeight}/${main.clientWidth}`,
            {
              target: "body",
              swap: "beforeend",
            }
          )
          .then(() => {
            menu.classList.add("hidden");
          });
      } else {
        htmx
          .ajax(
            "GET",
            `/read/file/${fileType}/${main.clientHeight}/${main.clientWidth}`,
            {
              target: "body",
              swap: "beforeend",
            }
          )
          .then(() => {
            menu.classList.add("hidden");
          });
      }
    });
  }
}
