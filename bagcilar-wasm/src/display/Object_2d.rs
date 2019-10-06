#[path = "../utils.rs"]
mod utils;

use wasm_bindgen::prelude::*;

// this.stage = undefined;
// this.context = undefined;
// this.xPos = 0;
// this.yPos = 0;
// this.position = new Vector3(0,0,1);
// this.rotationMatrix = new Matrix3();
// this.scaleMatrix = new Matrix3();
// this.positionMatrix = new Matrix3();
// this.worldMatrix = new Matrix3();
// use wasm_bindgen::prelude::*;
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct Object_2d {
    rotation: f32,
    scale_dirty: bool,
    rotation_dirty: bool,
    position_dirty: bool,
    needs_calculation: bool,
    scale_y:f32,
    scale_x:f32,
}

#[wasm_bindgen]
impl Object_2d {

     pub fn new() -> Object_2d {
        Object_2d {
            rotation: 0.0,
            scale_dirty: true,
            rotation_dirty: true,
            position_dirty: true,
            needs_calculation: true,
            scale_y: 1.0,
            scale_x: 1.0
         }
    }

    pub fn make_scream(&self) {
        utils::page_alert("deneme");
    }

    pub fn setRotation(&self, rotation:f32) {
        self.rotation = rotation;
        self.rotation_dirty = true;
    }

    pub fn getRotation(&self) {
        return self.rotation;
    }


    pub fn updateRotation (&self){
        // this.rotationMatrix.setRotationZ(this.rotation);
        self.isRotationDirty = false;
    }

    pub fn updateScale (&self){
        // this.scaleMatrix.setScale(this.scaleX, this.scaleY);
        self.isScaleDirty = false;
    }

    pub fn updatePosition (&self){
        // self.positionMatrix.translate(this.position.x, this.position.y);
        self.isPositionDirty = false;
    }

    pub fn setScale (&self, scale:f32) {
        self.scaleX = scale;
        self.scaley = scale;
        self.isScaleDirty = true;
    }

    pub fn setScaleX (&self, x:f32) {
        self.scaleX = x;
        self.isScaleDirty = true;
    }

    pub fn setScaleY (&self, y:f32) {
        self.scaleY = y;
        self.isScaleDirty = true;
    }

    pub fn getScaleY (&self){
        return self.scaleY;
    }

    pub fn getScaleX (&self){
        return self.scaleX;
    }
    
    pub fn updateWorldMatrix (&self){
        // self.worldMatrix.makeIdentity();
        self.updateScale();
        self.updateRotation();
        self.updatePosition();
        // self.worldMatrix.multiplyMatrix(self.positionMatrix);
        // self.worldMatrix.multiplyMatrix(self.rotationMatrix);
        // self.worldMatrix.multiplyMatrix(self.scaleMatrix); 
    }

    pub fn update (&self) {
        self.updateWorldMatrix();
    }

}