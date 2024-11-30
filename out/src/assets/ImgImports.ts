import grab from "./toolbox/grab.png";
import rotate from "./toolbox/rotate.png";
import axis from "./toolbox/axis.png";
import scale from "./toolbox/scale.png";
import dragBox from "./toolbox/delete.png";
import drag from "./toolbox/drag.png";

import cylinder from "./2D_assets/flaticon/cylinder.png";
import cone from "./2D_assets/flaticon/cone.png";
import box from "./2D_assets/flaticon/cube.png";
import sphere from "./2D_assets/flaticon/sphere.png";
import male from "./2D_assets/flaticon/male.png";
import female from "./2D_assets/flaticon/female.png";
import buggy from "./2D_assets/flaticon/buggy.png";

import capsule from "./2D_assets/flaticon/capsule.png";
import fire from "./2D_assets/flaticon/fire.png";
import plane from "./2D_assets/flaticon/plane.png";

function renderImg(imgName: string) {
  switch (imgName) {
    case "grab":
      return grab;
    case "axis":
      return axis;
    case "rotate":
      return rotate;
    case "scale":
      return scale;
    case "Select":
      return drag;
    case "Default":
      return dragBox;
    case "Cylinder":
      return cylinder;
    case "cone":
      return cone;
    case "Cube":
      return box;
    case "Capsule3D":
      return capsule;
    case "Sphere":
      return sphere;
    case "maleDummy":
      return male;
    case "femaleDummy":
      return female;
    case "fireBarrel":
      return fire;
    case "buggy":
      return buggy;
    case "airplane":
      return plane;
    case "point":
      return box;
    case "hem":
      return box;
    case "diffuse":
      return box;
  }
}

export default renderImg;
