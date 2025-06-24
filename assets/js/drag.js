export function setupDesktopDrag() {
  let main = document.querySelector("main");

  let draggedItem = null;

  main.addEventListener("dragstart", (event) => {
    draggedItem = event.target;
  });

  main.addEventListener("dragover", (event) => {
    event.preventDefault();
  });

  main.addEventListener("drop", async (event) => {
    let destopId = document.getElementById("desktop_id").value;

    if (draggedItem && event.target && draggedItem.id && event.target.id) {
      const dragElement = document.getElementById(draggedItem.id);
      const dropElement = document.getElementById(event.target.id);

      if (dragElement.innerHTML && !dropElement.innerHTML) {
        const dropPosition = dropElement.id;
        const dragChild = dragElement.childNodes[0];
        const dragItemIdSplit = dragChild.id.split("-");

        const itemType = dragItemIdSplit[0];
        const itemId = dragItemIdSplit[1];

        const response = await fetch(
          `/action/update-${itemType}-position/${itemId}/${destopId}/${dropPosition}`,
          {
            method: "POST",
            headers: {
              "X-Csrf-Protection": "1",
            },
          }
        );

        if (response.ok) {
          dropElement.appendChild(dragChild);
        }
      }
    }
  });
}
