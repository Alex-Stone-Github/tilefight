use crate::vec::Vec2;

pub struct Camera {
    position: Vec2<f64>,
}

impl Camera {
    pub fn new(position: Vec2<f64>) -> Self {
        Self {
           position
        }
    }
    pub fn convert_to_relative_cordinates(&self, position: Vec2<f64>) -> Vec2<f64> {
        Vec2 {
            x: position.x - self.position.x,
            y: position.y - self.position.y,
        }
    }
    pub fn set_position(&mut self, position: &Vec2<f64>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}
