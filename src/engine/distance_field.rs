extern crate vecmath as vmath;
use crate::engine::rotation::get_rotation_matrix;
use vmath::{
    Vector2, Vector3, Matrix3,
};

fn abs(x: f32) -> f32 {
    if x >= 0.0 {
        return x;
    } else {
        -x
    }
}

fn min(x: f32, a: f32) -> f32 {
    if a < x { return a };
    return x;
}

fn max(x: f32, a: f32) -> f32 {
    if a > x { return a };
    return x;
}

pub enum SDF_Type {
    SDF_Sphere,
    SDF_Box,
    SDF_Torus,
    SDF_Plane,
}

pub struct SDF {
    pub position: Vector3<f32>,
    pub size: Vector3<f32>,
    pub rotation: Option<Matrix3<f32>>,

    pub sdf_type: SDF_Type,

    pub colour: Vector3<u8>,
}

impl SDF {
    pub fn new_sphere(position: Vector3<f32>, radius: f32, colour: Vector3<u8>) -> SDF {
        SDF {
            position: position,
            size: [radius, radius, radius],
            sdf_type: SDF_Type::SDF_Sphere,
            colour: colour,
            rotation: None
        }
    }

    pub fn new_cube(position: Vector3<f32>, size: Vector3<f32>, colour: Vector3<u8>, rotation: Vector3<f32>) -> SDF {
        SDF {
            position: position,
            size: size,
            sdf_type: SDF_Type::SDF_Box,
            colour: colour,
            rotation: Some(get_rotation_matrix(rotation)),
        }
    }

    //NOTE: t is probably the outer radius and inner radius
    pub fn new_torus(position: Vector3<f32>, t: Vector2<f32>, colour: Vector3<u8>, rotation: Vector3<f32>) -> SDF {
        SDF {
            position: position,
            size: [t[0], t[1], 0.0],
            sdf_type: SDF_Type::SDF_Torus,
            colour: colour,
            rotation: Some(get_rotation_matrix([rotation[0] / 180.0 * 3.14, rotation[1] / 180.0 * 3.14, rotation[2] / 180.0 * 3.14])),
        }
    }

    pub fn new_plane(height: f32, colour: Vector3<u8>) -> SDF {
        SDF {
            position: [0.0, height, 0.0],
            size: [0.0, 0.0, 0.0],
            sdf_type: SDF_Type::SDF_Plane,
            colour: colour,
            rotation: None,
        }
    }

    pub fn update_rotation(&mut self, new_rotation: Vector3<f32>) {
        if self.rotation.is_some() {
            self.rotation = Some(get_rotation_matrix([new_rotation[0] / 180.0 * 3.14, new_rotation[1] / 180.0 * 3.14, new_rotation[2] / 180.0 * 3.14]));
        }
    }

    pub fn get_distance(&self, ray_position: Vector3<f32>) -> f32 {
        let mut local_pos = vmath::vec3_sub(ray_position, self.position);
        if self.rotation.is_some() {
            local_pos = vmath::col_mat3_transform(self.rotation.expect("reee"), local_pos);
        }
        match self.sdf_type {
            SDF_Type::SDF_Sphere => {
                return vmath::vec3_len(local_pos) - self.size[0];
            },
            SDF_Type::SDF_Box => {
                let apos = [abs(local_pos[0]), abs(local_pos[1]), abs(local_pos[2])];
                let q = vmath::vec3_sub(apos, self.size);
                let a = vmath::vec3_len([max(q[0],0.0), max(q[1],0.0), max(q[2],0.0)]);
                let b = min(max(q[0], max(q[1], q[2])), 0.0);
                return a + b;
            },
            SDF_Type::SDF_Torus => {
                let q = [vmath::vec2_len([local_pos[0], local_pos[2]]) - self.size[0], local_pos[1]];
                return vmath::vec2_len(q) - self.size[1];
            }
            SDF_Type::SDF_Plane => {
                return local_pos[1];
            }
            _ => {
                return -1.0;
            }
        }
    }
}
