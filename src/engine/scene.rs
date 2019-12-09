use super::{
    distance_field,
    camera::Camera,
};

use crate::rendering::raymarching::Ray;

pub struct Scene {
    pub distance_fields: Vec<distance_field::SDF>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            distance_fields: Vec::new(),
            camera: Camera::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
        }
    }

    pub fn march(&self, ray: Ray) -> char {
        if ray.direction[0] > 0.0 {
            return 'c';
        } else if ray.direction[0] < 0.0 {
            return 'a';
        } else if ray.direction[0] == 0.0 {
            return 'b';
        }
        return '-';
    }
}
