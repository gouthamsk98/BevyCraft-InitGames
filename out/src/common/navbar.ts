import "./navbar.css";

interface NavElementProps {
  key: number;
  onClick: (index: number) => void;
  isSelected: boolean;
  children: string;
}

const createNavElement = (props: NavElementProps): HTMLDivElement => {
  const element = document.createElement("div");
  element.className = `navElement ${
    props.isSelected ? "selected" : "unselected"
  }`;
  element.textContent = props.children;
  element.onclick = () => props.onClick(props.key);
  return element;
};

export const createNavBar = (
  selectedScreen: string,
  setSelectedScreen: (screen: string) => void
): HTMLDivElement => {
  const buttonLabels = ["Elements", "Layout", "Logic", "Test"];
  const navHeader = document.createElement("div");
  navHeader.className = "navHeader";
  let selectedElement: HTMLDivElement | null = null;

  buttonLabels.forEach((label, index) => {
    const navElement = createNavElement({
      key: index,
      onClick: (i) => {
        if (selectedElement) {
          selectedElement.classList.remove("selected");
          selectedElement.classList.add("unselected");
        }
        setSelectedScreen(label.toLowerCase());
        navElement.classList.remove("unselected");
        navElement.classList.add("selected");
        selectedElement = navElement;
      },
      isSelected: index === 0,
      children: label,
    });
    if (index === 0) {
      selectedElement = navElement;
    }
    navHeader.appendChild(navElement);
  });
  return navHeader;
};
