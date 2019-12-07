extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub distance: f32,
}
