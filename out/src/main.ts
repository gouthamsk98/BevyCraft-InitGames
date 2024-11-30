import "./style.css";
import { createNavBar } from "./common/navbar";
import { OverlayLoader, updateLoader } from "./common/Loader";
import { ToolBox } from "./common/toolbox";
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    
  </div>
`;
const selectedScreen = "elements"; // Example selected screen
document
  .querySelector<HTMLDivElement>("#app")!
  .appendChild(createNavBar("elements", (screen) => console.log(screen)));
const loader = OverlayLoader();
document.querySelector<HTMLDivElement>("#app")!.appendChild(loader);
let percentage = 0;
const interval = setInterval(() => {
  if (percentage < 100) {
    percentage += 10;
    updateLoader(percentage);
  } else {
    clearInterval(interval);
    document.querySelector<HTMLDivElement>("#app")!.removeChild(loader);
  }
}, 500);
ToolBox(selectedScreen);
