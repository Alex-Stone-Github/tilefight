use crate::vec::Vec2;
use crate::camera::Camera;
use crate::displayer::Displayer;

#[derive(Clone)]
pub struct Tile {
    pub position: Vec2<f64>,
    pub size: Vec2<f64>,
}
impl Tile {
    pub fn show(&self, camera: &Camera, displayer: &mut Displayer) {
        let position = camera.convert_to_relative_cordinates(self.position);
        displayer.fill_rect(position, self.size, sdl2::pixels::Color::RGB(0, 255, 255)); 
    }
}
