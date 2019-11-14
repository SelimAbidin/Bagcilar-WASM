// import {Object_2d} from 'bagcilar-wasm'
// Object_2d.new().bagir()
// window.Object_2d = Object_2d

import { Object2D, Scene } from "bagcilar-wasm";

let scene = Scene.new("canvas", 300);
let list = [];
for (let i = 0; i < 10; i++) {
  let vect = Object2D.new();
  let id = scene.add(vect);
  list.push({
    id,
    sin: Math.random() * Math.PI,
    vy: Math.random(),
    speed: Math.random() * 3,
    vx: Math.random()
  });
}

function update() {
  list.forEach(e => {
    e.sin += e.vx;
    scene.set_xy_by_id(
      Math.sin(e.sin) * e.speed,
      Math.cos(e.sin) * e.speed,
      e.id
    );
  });
  scene.render();

  requestAnimationFrame(update);
}
update();

document.addEventListener("click", () => {
  for (let i = 0; i < 100; i++) {
    let vect = Object2D.new();
    let id = scene.add(vect);
    list.push({
      id,
      sin: Math.random() * Math.PI,
      vy: Math.random(),
      speed: Math.random() * 10,
      vx: Math.random()
    });
  }

  console.log(list.length);
});

// setInterval(() => {
//   for (let i = 0; i < list.length; i++) {
//     const element = list[i];
//     // element.y = Math.round(Math.random() * 999);
//     console.log(element);
//   }
// }, 1000);

// window.Object2D = Object2D;
// window.Scene = Scene;
// let _counter = 0;
// let scene;
// let objectList = [];
// function createScene(params) {
//   scene = Scene.new("canvas", 300);

//   let cnv = document.getElementById("canvas");
//   cnv.width = 500;
//   cnv.height = 500;
//   let files = addObjects();

//   files.forEach(e =>
//     objectList.push({
//       mesh: e,
//       x: Math.random() * Math.PI,
//       dist: Math.random() * 40,
//       speed: 0.0001 + Math.random() / 5
//     })
//   );

//   window.scene = scene;
//   requestAnimationFrame(render);
// }

// function render() {
//   requestAnimationFrame(render);
//   objectList.forEach(e => {
//     let { mesh, speed } = e;

//     console.log(mesh);

//     // e.x += speed;
//     // let xPost = Math.sin(e.x);
//     // scene.get_by_id(e.id);
//     // // console.log(mesh);
//     // mesh.set_pos(xPost, 0);
//   });

//   scene.render();
// }

// createScene();

// function addObjects(count = 1) {
//   return Array.from({ length: count }, (v, i) => {
//     let obj = Object2D.new();
//     obj.set_pos(i + 5, 0);
//     obj.set_id(i + ++_counter);
//     return obj;
//   }).map(e => {
//     scene.add(e);
//     return e;
//   });
// }

// document.addEventListener("click", () => {
//   let files = addObjects();
//   files.forEach(e => objectList.push(e));
// });
// // wasm.greet();
