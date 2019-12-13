extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

#[derive(Copy, Clone)]
pub struct Camera {
    pub eye: Vector3<f32>,
    pub yaw: f32,
    pub roll: f32,
}

impl Camera {
    pub fn new(eye: Vector3<f32>, yaw: f32, roll: f32) -> Camera {
        Camera {
            eye: eye,
            yaw: yaw,
            roll: roll,
        }
    }
}
