use glm::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    ray_pos: Vec3,
    length: f32
}

impl Ray {
    pub fn new(origin: &[f32; 3], direction: &[f32; 3]) -> Self {
        Ray {
            origin: Vec3::new(origin[0], origin[1], origin[2]),
            direction: Vec3::new(direction[0], direction[1], direction[2]),
            ray_pos: Vec3::new(origin[0], origin[1], origin[2]),
            length: 0.0
        }
    }

    pub fn step(&mut self, precision: f32) {
        self.ray_pos += self.direction * precision;
        self.length += precision
    }

    pub fn get_ray_pos(&self) -> Vec3 {
        self.ray_pos.clone()
    }

    pub fn get_length(&self) -> f32 {
        self.length
    }
}