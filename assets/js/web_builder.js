import { MessageBox } from "./message_box.js";

let webBuilderCleanUpEvent = [];

let currentSelectElement = null;
let currentSettingElement = null;

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

    currentSelectElement = null;
    currentSettingElement = null;
    sectionType = "Header";
    templateNumber = 1;

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

export function setupWebBuilderWebTreeElement() {
  const webTree = document.getElementById(`builder-webtree`);
  const webReview = document.getElementById(`builder-review`);

  webTree.addEventListener("click", function (event) {
    const dataId = event.target.getAttribute("data-id");

    if (dataId) {
      if (currentSettingElement) {
        currentSettingElement.style.fontWeight = "inherit";
      }
      event.target.style.fontWeight = "bold";
      currentSettingElement = event.target;

      const reviewElement = webReview.querySelector(`[data-id="${dataId}"]`);

      if (reviewElement) {
        if (currentSelectElement) {
          currentSelectElement.classList.remove("outline-highlight");
        }
        reviewElement.classList.add("outline-highlight");
        currentSelectElement = reviewElement;
        setupWebBuilderEdit()
      }
    }
  });
}

export function setupWebBuilderWebReviewElement() {
  const webTree = document.getElementById(`builder-webtree`);
  const webReview = document.getElementById(`builder-review`);

  webReview.addEventListener("click", function (event) {
    const dataId = event.target.getAttribute("data-id");

    if (dataId) {
      if (currentSelectElement) {
        currentSelectElement.classList.remove("outline-highlight");
      }
      event.target.classList.add("outline-highlight");
      currentSelectElement = event.target;

      const reviewElement = webTree.querySelector(`[data-id="${dataId}"]`);

      if (reviewElement) {
        if (currentSettingElement) {
          currentSettingElement.style.fontWeight = "inherit";
        }
        reviewElement.style.fontWeight = "bold";
        currentSettingElement = reviewElement;
        setupWebBuilderEdit()
      }
    }
  });
}

export function setupWebBuilderTreeActions(builderId) {
  const viewWebsiteBtn = document.getElementById("view-website-btn");
  const downloadWebsiteBtn = document.getElementById("download-website-btn");

  downloadWebsiteBtn.addEventListener("click", async function () {
    try {
      const response = await fetch(
        `/create/web_builder/${builderId}/download`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "X-Csrf-Protection": "1",
          },
        }
      );

      if (!response.ok) {
        return MessageBox.error(
          "Error",
          "Failed to download website: " + (await response.text())
        );
      }

      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `website-${builderId}.zip`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch (error) {
      MessageBox.error("Error", "Failed to download website: " + error);
    }
  });

  viewWebsiteBtn.addEventListener("click", function () {
    window.open(
      `${window.location.origin}/read/web_builder/view_website/${builderId}`
    );
  });
}

export function setupWebBuilderEdit() {
  if (currentSelectElement) {
    console.log(currentSelectElement);
  }
}

export function setupWebBuilderKeyboardEvent(builderId) {
  function handleKeyDown(event) {
    if (event.key === "Delete" && currentSelectElement) {
      let nodeId = currentSelectElement.getAttribute("data-id");

      MessageBox.warning(
        "Confirm Delete",
        `Are you sure you want to delete this node: ${nodeId} ?`
      ).then(async (result) => {
        if (result) {
          const response = await fetch(
            `/delete/web_builder/${builderId}/node/delete/${nodeId}`,
            {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
                "X-Csrf-Protection": "1",
              },
            }
          );

          if (!response.ok) {
            return MessageBox.error(
              "Error",
              "Failed to delete node: " + (await response.text())
            );
          }

          currentSelectElement.remove();
          currentSettingElement.remove();
          currentSelectElement = null;
          currentSettingElement = null;
        }
      });
    }
  }

  webBuilderCleanUpEvent.push({
    event: "keydown",
    handler: handleKeyDown,
    id: builderId,
  });

  document.addEventListener("keydown", handleKeyDown);
}
