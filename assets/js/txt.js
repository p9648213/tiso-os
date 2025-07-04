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
  const txtButtons = document.getElementById(`txt-buttons-${txtId}`);

  const boldButton = txtButtons.querySelector(".bold");
  const italicButton = txtButtons.querySelector(".italic");
  const underlineButton = txtButtons.querySelector(".underline");

  boldButton.addEventListener("click", () => {
    applyFormat("strong");
  });

  italicButton.addEventListener("click", () => {
    applyFormat("em");
  });

  underlineButton.addEventListener("click", () => {
    applyFormat("u");
  });
}

function applyFormat(formatTag) {
  const selection = window.getSelection();
  if (!selection.rangeCount) return;

  const range = selection.getRangeAt(0);
  const selectedText = range.toString();

  if (selectedText.length === 0) return;

  const parentElement = range.commonAncestorContainer.parentElement;
  if (parentElement.tagName === formatTag) {
    const textNode = document.createTextNode(selectedText);
    parentElement.parentNode.replaceChild(textNode, parentElement);
    selection.removeAllRanges();
    range.selectNode(textNode);
    selection.addRange(range);
  } else {
    const formatElement = document.createElement(formatTag);
    try {
      range.surroundContents(formatElement);
    } catch (e) {
      console.warn("surroundContents failed, falling back to execCommand.", e);
      document.execCommand(formatTag.toLowerCase(), false, null);
    }
  }
}
