extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

pub enum SDF_Type {
    SDF_Sphere,
    SDF_Box,
    SDF_Plane,
}

pub struct SDF {
    pub position: Vector3<f32>,
    pub size: Vector3<f32>,

    pub sdf_type: SDF_Type,
}

impl SDF {
    pub fn new_sphere(position: Vector3<f32>, radius: f32) -> SDF {
        SDF {
            position: position,
            size: [radius, radius, radius],
            sdf_type: SDF_Type::SDF_Sphere,
        }
    }

    pub fn get_distance(&self, ray_position: Vector3<f32>) -> f32 {
        match self.sdf_type {
            SDF_Type::SDF_Sphere => {
                let local_pos = vmath::vec3_sub(ray_position, self.position);
                return vmath::vec3_len(local_pos) - self.size[0];
            },
            _ => {
                return -1.0;
            }
        }
    }
}
