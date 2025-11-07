let webBuilderCleanUpEvent = [];

let sectionType = "Header";
let templateNumber = 1;

window.addEventListener("changeSectionType", function (event) {
  sectionType = event.detail.sectionType;
});

window.addEventListener("changeTemplateNumber", function (event) {
  templateNumber = event.detail.templateNumber;
});

export function setupWebBuilderToolBar(builderId) {
  const webBuilderToolBar = document.getElementById(
    `web-builder-header-${builderId}`
  );

  const close = webBuilderToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById(`web-builder-window-${builderId}`).remove();
    document.getElementById(`taskbar-web-builder-window-${builderId}`).remove();
    webBuilderCleanUpEvent.forEach((event) => {
      if (event.id === builderId) {
        document.removeEventListener(event.event, event.handler);
      }
    });
    webBuilderCleanUpEvent = webBuilderCleanUpEvent.filter(
      (event) => event.id !== builderId
    );
  });
}

export function setupAddSectionDialog(builderId) {
  const sectionDialog = document.getElementById("builder-section");
  const openSectionDialogBtn = document.getElementById("open-section-btn");
  const closeSectionDialogBtn = document.getElementById("close-section-btn");
  const builderAddSectionBtn = document.getElementById(
    "builder-add-section-btn"
  );
  const viewWebsiteBtn = document.getElementById("view-website-btn");

  viewWebsiteBtn.addEventListener("click", function () {
    window.open(
      `${window.location.origin}/read/web_builder/view_website/${builderId}`
    );
  });

  openSectionDialogBtn.addEventListener("click", function () {
    sectionDialog.showModal();
  });

  closeSectionDialogBtn.addEventListener("click", function () {
    sectionDialog.close();
  });

  builderAddSectionBtn.addEventListener("click", function () {
    htmx
      .ajax(
        "POST",
        `/create/web_builder/${builderId}/section/add/${sectionType}/${templateNumber}`,
        {
          target: `#builder-review`,
          swap: "outerHTML",
        }
      )
      .then(() => {
        sectionDialog.close();
      });
  });
}
