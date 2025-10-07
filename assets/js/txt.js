let quillLoaded = false;
let quillLoadingPromise = null;

export function setupTxtToolBar(txt_id) {
  const txtToolBar = document.getElementById(`txt-header-${txt_id}`);

  const close = txtToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`txt-window-${txt_id}`).remove();
    document.getElementById(`taskbar-txt-window-${txt_id}`).remove();
  });
}

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

  document.addEventListener("mousemove", (event) => {
    if (!isDragging) return;

    txtWindow.style.left = `${event.clientX - offsetX}px`;
    txtWindow.style.top = `${event.clientY - offsetY}px`;
  });

  document.addEventListener("mouseup", () => {
    isDragging = false;
  });
}

export async function setupTxtEditor(txtId) {
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

  new Quill(selector, {
    theme: "snow",
  });
}
