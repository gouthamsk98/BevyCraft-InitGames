import "./Loader.css";

export const OverlayLoader = (): HTMLDivElement => {
  const overlay = document.createElement("div");
  overlay.className = "overlay";

  const loader = document.createElement("div");
  loader.className = "loader";

  const loaderBar = document.createElement("div");
  loaderBar.className = "loader-bar";

  const loaderText = document.createElement("div");
  loaderText.className = "loader-text";
  loaderText.textContent = "0%";

  loader.appendChild(loaderBar);
  loader.appendChild(loaderText);
  overlay.appendChild(loader);

  return overlay;
};

export const updateLoader = (percentage: number): void => {
  const loaderBar = document.querySelector(".loader-bar") as HTMLDivElement;
  const loaderText = document.querySelector(".loader-text") as HTMLDivElement;
  if (loaderBar && loaderText) {
    loaderBar.style.width = `${percentage}%`;
    loaderText.textContent = `${percentage}%`;
  }
};
