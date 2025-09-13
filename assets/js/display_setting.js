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
        swap: "none",
      }
    );
  });
}
