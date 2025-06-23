export function setupDesktopDrag() {
  let main = document.querySelector("main");

  let draggedItem = null;

  main.addEventListener("dragstart", (event) => {
    draggedItem = event.target;
  });

  main.addEventListener("dragover", (event) => {
    event.preventDefault();
  });

  main.addEventListener("drop", (event) => {
    if (draggedItem && event.target) {
      const dragElement = document.getElementById(draggedItem.id);
      const dropElement = document.getElementById(event.target.id);
      dropElement.appendChild(dragElement.childNodes[0]);
    }
  });
}
