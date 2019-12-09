extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

use crossterm::style::Color;

fn clamp(x: f32, a: f32, b: f32) -> f32 {
    if x < a { return a };
    if x > b { return b };
    return x;
}

pub fn lambert(albedo: Color, normal: Vector3<f32>, light_dir: Vector3<f32>) -> Color {
    let mut colour = albedo;

    match colour {
        Color::Rgb{r, g, b} => {
            let intensity = clamp(vmath::vec3_dot(normal, vmath::vec3_neg(light_dir)), 0.0, 1.0);
            colour = Color::Rgb{r: (r as f32 * intensity) as u8, g: (g as f32 * intensity) as u8, b: (b as f32 * intensity) as u8};
        },
        _ => {
            return albedo;
        }
    }

    return colour;
}
