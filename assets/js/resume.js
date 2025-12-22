export function setupResumeToolBar() {
  const displaySettingToolBar = document.getElementById(
    "resume-toolbar"
  );

  const close = displaySettingToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById("resume-window").remove();
    document.getElementById("taskbar-resume-window").remove();
  });
}