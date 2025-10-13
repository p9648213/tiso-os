let webBuilderCleanUpEvent = [];

export function setupWebBuilderToolBar(web_builder_id) {
  const webBuilderToolBar = document.getElementById(
    `web-builder-header-${web_builder_id}`
  );

  const close = webBuilderToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`web-builder-window-${web_builder_id}`).remove();
    document
      .getElementById(`taskbar-web-builder-window-${web_builder_id}`)
      .remove();
    webBuilderCleanUpEvent.forEach((event) => {
      if (event.id === web_builder_id) {
        document.removeEventListener(event.event, event.handler);
      }
    });
    webBuilderCleanUpEvent = webBuilderCleanUpEvent.filter(
      (event) => event.id !== web_builder_id
    );
  });
}
