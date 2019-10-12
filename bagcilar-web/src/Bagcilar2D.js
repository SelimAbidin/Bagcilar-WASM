// import {Object_2d} from 'bagcilar-wasm'
// Object_2d.new().bagir()
// window.Object_2d = Object_2d

import { Object2D, Container, Scene } from "bagcilar-wasm";

window.Object2D = Object2D;
window.Container = Container;
window.Scene = Scene;

let scene;
function createScene(params) {
  scene = Scene.new("canvas", 20);

  for (let i = 0; i < 10; i++) {
    let obj = Object2D.new();
    scene.add(obj);
  }

  window.scene = scene;
  requestAnimationFrame(render);
}

function render() {
  requestAnimationFrame(render);
  scene.render();
}

createScene();

document.addEventListener("click", () => {
  Array.from({ length: 10 }, () => {
    return Object2D.new();
  }).forEach(e => {
    scene.add(e);
  });
});
// wasm.greet();
