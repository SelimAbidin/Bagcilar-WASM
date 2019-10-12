// import {Object_2d} from 'bagcilar-wasm'
// Object_2d.new().bagir()
// window.Object_2d = Object_2d

import { Object2D, Container, Scene } from "bagcilar-wasm";

window.Object2D = Object2D;
window.Container = Container;
window.Scene = Scene;

function createScene(params) {
  let scene = Scene.new("canvas", 20);
  scene.speak();
}

createScene();

// wasm.greet();
