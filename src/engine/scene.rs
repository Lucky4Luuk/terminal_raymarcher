extern crate vecmath as vmath;
use vmath::{
    Vector3,
};

use super::{
    distance_field,
    camera::Camera,
};

use crate::rendering::{
    raymarching::Ray,
    lighting::lambert,
};

use crossterm::style::Color;

fn clamp(x: f32, a: f32, b: f32) -> f32 {
    if x < a { return a };
    if x > b { return b };
    return x;
}

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

    pub fn get_distance(&self, position: Vector3<f32>) -> (f32, i32) {
        let mut closest_distance = 4096.0;
        let mut idx = -1;

        for i in 0.. self.distance_fields.len() {
            let dist = self.distance_fields[i].get_distance(position);
            // println!("gamer time: {}", dist);
            //Need to write this dumb line, so rust doesn't shit itself.
            //Apparently std::cmp::min requires the Ord trait, which isn't implemented for any floats.
            // closest_distance = if dist < closest_distance {dist} else {closest_distance};
            if dist < closest_distance {
                closest_distance = dist;
                idx = i as i32;
            }
        }

        return (closest_distance, idx);
    }

    pub fn get_normal(&self, position: Vector3<f32>) -> Vector3<f32> {
        let e = [0.00028865, -0.00028865];
        let p1 = [e[0], e[1], e[1]];
        let p2 = [e[1], e[1], e[0]];
        let p3 = [e[1], e[0], e[1]];
        let p4 = [e[0], e[0], e[0]];

        let d1 = self.get_distance(vmath::vec3_add(position, p1)).0;
        let d2 = self.get_distance(vmath::vec3_add(position, p2)).0;
        let d3 = self.get_distance(vmath::vec3_add(position, p3)).0;
        let d4 = self.get_distance(vmath::vec3_add(position, p4)).0;

        return vmath::vec3_normalized(
            [
                p1[0] * d1 + p2[0] * d2 + p3[0] * d3 + p4[0] * d4,
                p1[1] * d1 + p2[1] * d2 + p3[1] * d3 + p4[1] * d4,
                p1[2] * d1 + p2[2] * d2 + p3[2] * d3 + p4[2] * d4,
            ]
        );
    }

    pub fn march(&self, mut ray: Ray) -> (char, Color) {
        let (mut dist, mut idx) = self.get_distance(ray.position);

        while dist > 0.05 {
            if dist == 4096.0 {
                return (' ', Color::Red);
            }
            ray.step(dist);
            let (_dist, _idx) = self.get_distance(ray.position);
            dist = _dist;
            idx = _idx;
        }

        // return ('x', Color::Red);

        if idx >= 0 {
            let sdf = &self.distance_fields[idx as usize];

            let normal = self.get_normal(ray.position);
            // println!("normal: {:?}", normal);

            let color = lambert( Color::Rgb{r: sdf.colour[0], g: sdf.colour[1], b: sdf.colour[2]}, normal, vmath::vec3_normalized([0.25, -0.5, 0.5]));

            let mut intensity = clamp(vmath::vec3_dot(normal, vmath::vec3_neg(vmath::vec3_normalized([0.25, -0.5, 0.5]))), 0.0, 1.0);
            intensity *= clamp(vmath::vec3_dot(normal, vmath::vec3_neg(vmath::vec3_normalized(ray.direction))), 0.0, 1.0);
            let mut value = 'X';
            let gradient = [':', ';', '=', '1', '%', 'X', '#'];
            let gradient_idx = (intensity * gradient.len() as f32) as usize;
            value = gradient[gradient_idx];
            // if intensity < 0.25 {
            //     value = '=';
            // } else if intensity < 0.5 {
            //     value = '+';
            // } else if intensity < 0.75 {
            //     value = '&';
            // }

            return (value, color);
        }
        // return ('x', Color::Rgb{r: clamp(-normal[0] * 255.0, 0.0, 255.0) as u8, g: clamp(-normal[1] * 255.0, 0.0, 255.0) as u8, b: clamp(-normal[2] * 255.0, 0.0, 255.0) as u8});

        return (' ', Color::Red);
    }
}
