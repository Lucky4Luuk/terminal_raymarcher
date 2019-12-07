extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

pub struct SDF_Sphere {
    pub position: Vector3<f32>,
    pub scale: f32,
}

impl SDF_Sphere {
    pub fn new(position: Vector3<f32>, scale: f32) -> SDF_Sphere {
        SDF_Sphere {
            position: position,
            scale: scale,
        }
    }

    pub fn get_distance(&self, ray_position: Vector3<f32>) -> f32 {
        let local_pos = vmath::vec3_sub(ray_position, self.position);
        return vmath::vec3_len(local_pos) - self.scale;
    }
}
