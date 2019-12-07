extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

pub struct Camera {
    pub eye: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub roll: f32,
}

impl Camera {
    pub fn new(eye: Vector3<f32>, forward: Vector3<f32>, roll: f32) -> Camera {
        Camera {
            eye: eye,
            forward: forward,
            roll: roll,
        }
    }
}
