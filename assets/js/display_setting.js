export function setupSelectBackgroundType() {
  const backgroundTypeSelect = document.getElementById(
    "display-setting-background-type"
  );

  backgroundTypeSelect.addEventListener("change", (event) => {
    const backgroundColor = document.getElementById(
      "display-setting-background-color"
    );
    const backgroundPicture = document.getElementById(
      "display-setting-background-picture"
    );

    if (event.target.value === "SolidColor") {
      backgroundColor.classList.remove("hidden");
      backgroundColor.classList.add("flex");
      backgroundPicture.classList.add("hidden");
      backgroundPicture.classList.remove("flex");
    } else if (event.target.value === "Picture") {
      backgroundColor.classList.add("hidden");
      backgroundColor.classList.remove("flex");
      backgroundPicture.classList.remove("hidden");
      backgroundPicture.classList.add("flex");
    }

    htmx.ajax(
      "POST",
      `/update/setting/display/background_type/${event.target.value}`,
      {
        swap: "outerHTML",
        target: "#background-container",
      }
    );
  });
}

export function setupBackgroundColorList() {
  const backgroundColorList = document.getElementById("background-color-list");

  backgroundColorList.addEventListener("click", (event) => {
    if (event.target.dataset?.color) {
      backgroundColorList.childNodes.forEach((node) => {
        node.style.outline = "none";
      });
      event.target.style.outline = "3px solid #155dfc";

      htmx
        .ajax(
          "POST",
          `/update/setting/display/background_color/${encodeURIComponent(
            event.target.dataset.color
          )}`,
          {
            swap: "none",
          }
        )
        .then(() => {
          const backgroundContainer = document.getElementById(
            "background-container"
          );
          backgroundContainer.childNodes[0].style.background =
            event.target.dataset.color;
        });
    }
  });
}

export function setupSelectBackgroundPicture() {
  const backgroundPictureInput = document.getElementById("background-picture");
  const backgroundPictureName = document.getElementById(
    "background-picture-name"
  );

  backgroundPictureInput.addEventListener("change", () => {
    if (backgroundPictureInput.files.length > 0) {
      backgroundPictureName.textContent = backgroundPictureInput.files[0].name;
    } else {
      backgroundPictureName.textContent = "No file selected";
    }
  });
}

export function setupDisplaySettingToolBar() {
  const displaySettingToolBar = document.getElementById(
    "display-setting-header"
  );

  const close = displaySettingToolBar.querySelector(".close");

  close.addEventListener("click", function () {
    document.getElementById("display-setting-window").remove();
    document.getElementById("taskbar-display-setting-window").remove();
  });
}
