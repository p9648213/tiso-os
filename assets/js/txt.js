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

export function setupTxtEditor(txtId) {
  const txtEditor = document.getElementById(`txt-editor-${txtId}`);
  const txtButtons = document.getElementById(`txt-buttons-${txtId}`);

  const boldButton = txtButtons.querySelector(".bold");
  const italicButton = txtButtons.querySelector(".italic");
  const underlineButton = txtButtons.querySelector(".underline");

  boldButton.addEventListener("mousedown", () => {
    applyFormat("STRONG");
    focusAfterChange(txtEditor);
  });

  italicButton.addEventListener("mousedown", () => {
    applyFormat("EM");
    focusAfterChange(txtEditor);
  });

  underlineButton.addEventListener("mousedown", () => {
    applyFormat("U");
    focusAfterChange(txtEditor);
  });
}

function applyFormat(formatTag) {
  const selection = window.getSelection();
  if (!selection.rangeCount) return;

  const range = selection.getRangeAt(0);
  const selectedText = range.toString();

  if (selectedText.length === 0) return;

  let parentElement = range.commonAncestorContainer.parentElement;

  let recursiveParentEl = parentElement;
  let styledEl = null;

  while (!recursiveParentEl.id.includes("txt-editor-")) {
    if (recursiveParentEl.tagName === formatTag) {
      styledEl = recursiveParentEl;
      break;
    }
    recursiveParentEl = recursiveParentEl.parentElement;
  }

  if (styledEl) {
    if (styledEl.children.length > 0) {
      styledEl.parentElement.appendChild(styledEl.children[0]);
    } else {
      styledEl.parentElement.appendChild(document.createTextNode(selectedText));
    }
    styledEl.parentElement.removeChild(styledEl);
  } else {
    const formatElement = document.createElement(formatTag);
    range.surroundContents(formatElement);
  }

  selection.removeAllRanges();
}

function focusAfterChange(txtEditor) {
  txtEditor.focus();
  const range = document.createRange();
  range.selectNodeContents(txtEditor);
  range.collapse(false);
  const selection = window.getSelection();
  selection.removeAllRanges();
  selection.addRange(range);
}
