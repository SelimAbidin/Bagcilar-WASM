// import {Object_2d} from 'bagcilar-wasm'
// Object_2d.new().bagir()
// window.Object_2d = Object_2d

import { Object2D, Container, Scene } from "bagcilar-wasm";

window.Object2D = Object2D;
window.Container = Container;
window.Scene = Scene;
let _counter = 0;
let scene;
function createScene(params) {
  scene = Scene.new("canvas", 20);

  addObjects();

  window.scene = scene;
  requestAnimationFrame(render);
}

function render() {
  requestAnimationFrame(render);
  scene.render();
}

createScene();

function addObjects(count = 1) {
  Array.from({ length: count }, (v, i) => {
    let obj = Object2D.new();
    obj.set_pos_x(i);
    obj.set_id(i + ++_counter);
    return obj;
  }).forEach(e => {
    scene.add(e);
  });
}

document.addEventListener("click", () => {
  addObjects();
});
// wasm.greet();
