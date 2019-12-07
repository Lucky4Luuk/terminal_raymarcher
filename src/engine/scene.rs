use super::{
    distance_field,
    camera::Camera,
};

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
}
