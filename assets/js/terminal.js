let terminalCleanUpEvent = [];

/** @type {WebSocket | null} */
let terminalSocket = null;

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

export function setupTerminalTextArea() {
  const terminalTextArea = document.getElementById(`terminal-text-area`);
  const terminalDisplay = document.getElementById(`terminal-display`);
  const terminalInput = document.getElementById(`terminal-input`);

  terminalTextArea.focus();

  terminalTextArea.addEventListener("input", function () {
    terminalTextArea.style.height = "auto";
    terminalTextArea.style.height = terminalTextArea.scrollHeight + "px";
  });

  terminalTextArea.addEventListener("keydown", function (e) {
    if (e.key === "Enter") {
      let value = terminalTextArea.value;
      e.preventDefault();
      if (
        terminalTextArea.value == "clear" ||
        terminalTextArea.value == "cls"
      ) {
        terminalTextArea.value = "";
        let terminalInputClone = terminalInput.cloneNode(true);
        terminalDisplay.innerHTML = "";
        terminalDisplay.appendChild(terminalInputClone);
        setupTerminalTextArea();
      }

      terminalSocket.send(value);
    }
  });
}

export function setupTerminalWebSocket() {
  terminalSocket = new WebSocket(`ws://${window.location.host}/ws/terminal`);

  terminalSocket.addEventListener("open", function () {
    console.log("WebSocket connection opened");
  });

  terminalSocket.addEventListener("message", function (event) {
    const message = JSON.parse(event.data);

    if (message.output == "") return;

    const terminalDisplay = document.getElementById(`terminal-display`);
    const terminalInput = document.getElementById(`terminal-input`);
    const terminalTextArea = document.getElementById(`terminal-text-area`);

    let prevInput = document.createElement("div");
    prevInput.classList.add("flex", "gap-2");
    prevInput.appendChild(terminalInput.children[0].cloneNode(true));

    let prevInputText = document.createElement("div");
    prevInputText.textContent = terminalTextArea.value;
    prevInput.appendChild(prevInputText);

    terminalDisplay.appendChild(prevInput);

    let terminalOutput = document.createElement("div");
    terminalOutput.innerHTML = message.output;
    terminalDisplay.appendChild(terminalOutput);

    if (message.script) {
      window.injectScript(message.script);
    }

    terminalTextArea.value = "";
    terminalDisplay.appendChild(terminalInput.cloneNode(true));

    terminalInput.remove();

    setupTerminalTextArea();
  });

  terminalSocket.addEventListener("close", function () {
    console.log("WebSocket connection closed");
  });

  terminalSocket.addEventListener("error", function () {
    console.log("WebSocket connection error");
  });
}
