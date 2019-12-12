extern crate vecmath as vmath;
use vmath::{
    Vector3,
    Matrix3,
};

pub fn get_rotation_matrix(rotation: Vector3<f32>) -> Matrix3<f32> {
    let mat_x = [
                [1.0, 0.0, 0.0],
                [0.0, rotation[0].cos(), -rotation[0].sin()],
                [0.0, rotation[0].sin(), rotation[0].cos()],
    ];

    let mat_y = [
                [rotation[1].cos(), 0.0, rotation[1].sin()],
                [0.0, 1.0, 0.0],
                [-rotation[1].sin(), 0.0, rotation[1].cos()],
    ];

    let mat_z = [
                [rotation[2].cos(), -rotation[2].sin(), 0.0],
                [rotation[2].sin(), rotation[2].cos(), 0.0],
                [0.0, 0.0, 1.0],
    ];

    return vmath::row_mat3_mul(vmath::row_mat3_mul(mat_y, mat_x), mat_z);
}
