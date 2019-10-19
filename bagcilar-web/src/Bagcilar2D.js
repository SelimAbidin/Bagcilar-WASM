// import {Object_2d} from 'bagcilar-wasm'
// Object_2d.new().bagir()
// window.Object_2d = Object_2d

import { Object2D, Scene } from "bagcilar-wasm";

window.Object2D = Object2D;
window.Scene = Scene;
let _counter = 0;
let scene;
let objectList = [];
function createScene(params) {
  scene = Scene.new("canvas", 300);

  let cnv = document.getElementById("canvas");
  cnv.width = 500;
  cnv.height = 500;
  let files = addObjects();

  files.forEach(e =>
    objectList.push({
      mesh: e,
      x: Math.random() * Math.PI,
      dist: Math.random() * 40,
      speed: 0.0001 + Math.random() / 5
    })
  );

  window.scene = scene;
  requestAnimationFrame(render);
}

function render() {
  requestAnimationFrame(render);
  objectList.forEach(e => {
    let { mesh, speed } = e;
    e.x += speed;
    let xPost = Math.sin(e.x);
    // mesh.set_pos(xPost, 0);
  });

  scene.render();
}

createScene();

function addObjects(count = 1) {
  return Array.from({ length: count }, (v, i) => {
    let obj = Object2D.new();
    obj.set_pos(i + 5, 0);
    obj.set_id(i + ++_counter);
    return obj;
  }).map(e => {
    scene.add(e);

    return e;
  });
}

document.addEventListener("click", () => {
  let files = addObjects();
  files.forEach(e => objectList.push(e));
});
// wasm.greet();
