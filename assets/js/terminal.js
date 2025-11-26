let terminalCleanUpEvent = [];

export function setupTerminalWindowGrab() {
  const terminalHeader = document.getElementById(`terminal-header`);
  const terminalWindow = document.getElementById(`terminal-window`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  terminalHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = terminalWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    terminalWindow.style.left = `${event.clientX - offsetX}px`;
    terminalWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  terminalCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
  });
  terminalCleanUpEvent.push({
    event: "mouseup",
    handler: handleMouseUp,
  });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

export function setupTerminalToolBar() {
  const terminalToolBar = document.getElementById("terminal-header");

  const close = terminalToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`terminal-window`).remove();
    document.getElementById(`taskbar-terminal-window`).remove();
    terminalCleanUpEvent.forEach((event) => {
      document.removeEventListener(event.event, event.handler);
    });
    terminalCleanUpEvent = [];
  });
}

export function setupTerminalInput() {
  const terminalInput = document.getElementById(`terminal-input`);

  terminalInput.addEventListener("input", function () {
    terminalInput.style.height = "auto";
    terminalInput.style.height = terminalInput.scrollHeight + "px";
  });

  terminalInput.addEventListener("keydown", function (e) {
    if (e.key === "Enter") {
      e.preventDefault();
    }
  });
}