let resumeCleanUpEvent = [];

export function setupResumeToolBar() {
  const displaySettingToolBar = document.getElementById("resume-header");

  const close = displaySettingToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById("resume-window").remove();
    document.getElementById("taskbar-resume-window").remove();
    resumeCleanUpEvent.forEach((event) => {
      document.removeEventListener(event.event, event.handler);
    });
    resumeCleanUpEvent = [];
  });
}

export function setupResumeWindowGrab() {
  const resumeHeader = document.getElementById(`resume-header`);
  const resumeWindow = document.getElementById(`resume-window`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  resumeHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = resumeWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    resumeWindow.style.left = `${event.clientX - offsetX}px`;
    resumeWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  resumeCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
  });

  resumeCleanUpEvent.push({
    event: "mouseup",
    handler: handleMouseUp,
  });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}
