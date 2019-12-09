extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

use crate::engine::{
    scene::Scene,
};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub distance: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            distance: 0.0,
        }
    }

    pub fn step(&mut self, scene: &Scene) -> f32 {
        
        return -1.0;
    }
}
