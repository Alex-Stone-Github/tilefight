use crate::vec::Vec2;
use crate::tile::Tile;
use crate::settings::GRAVITY;
use crate::camera::Camera;
use crate::displayer::Displayer;

pub struct Bullet {
    position: Vec2<f64>,
    velocity: Vec2<f64>,
    pub collided: bool,
    damage: i32,
}
impl Bullet {
    pub fn new(position: Vec2<f64>, velocity: Vec2<f64>, damage: i32) -> Self {
        Self {
            position,
            velocity,
            collided: false,
            damage,
        }
    }
    pub fn update(&mut self, tiles: &mut Vec<Tile>) {
        // self.velocity.y += GRAVITY;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        // collisions
        for tile in tiles {
            if self.position.x > tile.position.x && self.position.x < tile.position.x + tile.size.x {
                if self.position.y > tile.position.y && self.position.y < tile.position.y + tile.size.y {
                    tile.integrity -= self.damage;
                    self.collided = true;
                }
            }
        }
    }
    pub fn show(&self, camera: &Camera, displayer: &mut Displayer) {
        let position = camera.convert_to_relative_cordinates(self.position);
        displayer.fill_rect(position, Vec2{x:10.0, y:10.0}, sdl2::pixels::Color::RGB(0, 255, 0));
    }
}
