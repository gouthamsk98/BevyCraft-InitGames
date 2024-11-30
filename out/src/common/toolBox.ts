import "./toolBox.css";
import {
  faAngleDoubleDown,
  faAngleDoubleUp,
} from "@fortawesome/free-solid-svg-icons";
import { library, dom } from "@fortawesome/fontawesome-svg-core";

library.add(faAngleDoubleDown, faAngleDoubleUp);
dom.watch();
import renderImg from "../assets/ImgImports";
const tools = ["Default", "Default", "Select", "rotate", "axis", "scale"];
const groups = ["rotate", "axis", "scale"];

export const ToolBox = (selectedScreen: string): HTMLDivElement | null => {
  const activeTool: string[] = [];
  let isDown = false;

  const handleToolSelection = (tool: string) => {
    if (activeTool.includes(tool)) {
      // @ts-ignore
      window.get_tool_type = function () {
        return "None"; // or any other cursor type logic
      };
      const index = activeTool.indexOf(tool);
      activeTool.splice(index, 1);
    } else {
      if (!groups.includes(tool)) {
        // @ts-ignore
        window.get_tool_type = function () {
          return tool; // or any other cursor type logic
        };
        activeTool.splice(0, activeTool.length, tool);
      } else {
        if (activeTool.some((element) => groups.includes(element))) {
          activeTool.push(tool);
        } else {
          activeTool.splice(0, activeTool.length, tool);
        }
      }
    }
    render();
  };

  const render = () => {
    let toolbar = document.querySelector<HTMLDivElement>(".toolbar");
    if (!toolbar) {
      toolbar = document.createElement("div");
      toolbar.className = `toolbar ${isDown ? "toolbarDown" : ""}`;
      const app = document.querySelector<HTMLDivElement>("#app")!;
      app.appendChild(toolbar);
    } else {
      toolbar.className = `toolbar ${isDown ? "toolbarDown" : ""}`;
    }

    toolbar.innerHTML = "";

    const toolsHeader = document.createElement("div");
    toolsHeader.className = "toolsHeader";
    const p = document.createElement("p");
    p.textContent = "ToolBar";
    toolsHeader.appendChild(p);

    const arrowIcon = document.createElement("div");
    arrowIcon.style.cursor = "pointer";
    arrowIcon.innerHTML = isDown
      ? `<i class="fas fa-angle-double-up arrowDown"></i>`
      : `<i class="fas fa-angle-double-down arrowDown"></i>`;
    arrowIcon.onclick = () => {
      isDown = !isDown;
      render();
    };
    toolsHeader.appendChild(arrowIcon);
    toolbar.appendChild(toolsHeader);

    const toolsDiv = document.createElement("div");
    toolsDiv.className = `tools ${isDown ? "hidden" : ""}`;
    tools.map((tool) => {
      const toolDiv = document.createElement("div");
      toolDiv.className = `tool ${activeTool.includes(tool) ? "selected" : ""}`;
      toolDiv.onclick = () => handleToolSelection(tool);
      const img = document.createElement("img");
      img.src = renderImg(tool) || "";
      img.alt = tool;
      toolDiv.appendChild(img);
      toolsDiv.appendChild(toolDiv);
    });
    toolbar.appendChild(toolsDiv);
  };

  if (selectedScreen === "elements" || selectedScreen === "layout") {
    render();
    return document.querySelector(".toolbar");
  }

  return null;
};
