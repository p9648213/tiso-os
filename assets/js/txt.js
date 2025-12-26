let txtCleanUpEvent = [];
let quillLoaded = false;
let quillLoadingPromise = null;

export function setupTxtWindowGrab(txtId) {
  const txtHeader = document.getElementById(`txt-header-${txtId}`);
  const txtWindow = document.getElementById(`txt-window-${txtId}`);

  let isDragging = false;
  let offsetX = 0;
  let offsetY = 0;

  txtHeader.addEventListener("mousedown", (event) => {
    isDragging = true;
    const rect = txtWindow.getBoundingClientRect();
    offsetX = event.clientX - rect.left;
    offsetY = event.clientY - rect.top;

    event.preventDefault();
  });

  function handleMouseMove(event) {
    if (!isDragging) return;

    txtWindow.style.left = `${event.clientX - offsetX}px`;
    txtWindow.style.top = `${event.clientY - offsetY}px`;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  txtCleanUpEvent.push({
    event: "mousemove",
    handler: handleMouseMove,
    id: txtId,
  });

  txtCleanUpEvent.push({ event: "mouseup", handler: handleMouseUp, id: txtId });

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

export async function setupTxtEditor(txtId, text) {
  if (!quillLoaded) {
    if (!quillLoadingPromise) {
      quillLoadingPromise = new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.src = "/assets/js/lib/quill.js";
        script.onload = () => {
          quillLoaded = true;
          console.log("✅ Quill loaded");
          resolve();
        };
        script.onerror = () => reject(new Error("Failed to load Quill"));
        document.head.appendChild(script);
      });
    }
    await quillLoadingPromise;
  }

  const selector = `#txt-editor-${txtId}`;
  const txtEditor = document.querySelector(selector);

  if (!txtEditor) {
    console.error(`❌ Element ${selector} not found`);
    return;
  }

  const quill = new Quill(selector, {
    theme: "snow",
  });

  quill.focus();

  quill.clipboard.dangerouslyPasteHTML(text);

  let controller = null;

  const saveText = debounce(() => {
    if (controller) {
      controller.abort();
    }

    controller = new AbortController();

    htmx.ajax("POST", `/update/file/txt/${txtId}/text`, {
      swap: "none",
      signal: controller.signal,
      values: {
        text: quill.getSemanticHTML(),
      },
    });
  }, 500);

  quill.on("text-change", (_delta, _oldDelta, source) => {
    if (source == "user") {
      saveText();
    }
  });
}

export function setupTxtToolBar(txt_id) {
  const txtToolBar = document.getElementById(`txt-header-${txt_id}`);

  const close = txtToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`txt-window-${txt_id}`).remove();
    document.getElementById(`taskbar-txt-window-${txt_id}`).remove();
    txtCleanUpEvent.forEach((event) => {
      if (event.id === txt_id) {
        document.removeEventListener(event.event, event.handler);
      }
    });
    txtCleanUpEvent = txtCleanUpEvent.filter((event) => event.id !== txt_id);
  });
}

function debounce(fn, delay = 300) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), delay);
  };
}
