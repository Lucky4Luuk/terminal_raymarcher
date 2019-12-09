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
    pub position: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            position: [0.0; 3],
        }
    }

    pub fn step(&mut self, distance: f32) {
        self.position = vmath::vec3_add(self.position, [self.direction[0] * distance, self.direction[1] * distance, self.direction[2] * distance]);
    }
}
