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