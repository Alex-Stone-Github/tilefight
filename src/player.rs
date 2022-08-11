use crate::vec::Vec2;
use crate::settings::{GRAVITY, WIDTH, HEIGHT};
use crate::camera::Camera;
use crate::displayer::Displayer;
use crate::tile::Tile;

pub struct Player {
    position: Vec2<f64>,
    size: Vec2<f64>,
    velocity: Vec2<f64>,
    old_position: Vec2<f64>
}

impl Player {
    pub fn new(position: Vec2<f64>, size: Vec2<f64>) -> Self {
        Self {
            position,
            size,
            velocity: Vec2{x:0.0, y:0.0},
            old_position: Vec2{x:0.0, y:0.0}
        }
    }
    pub fn update(&mut self, collidables: &Vec<Tile>) {
        self.velocity.y += GRAVITY;

        self.position.y += self.velocity.y;
        if self.is_colliding(collidables) {
            self.position.y = self.old_position.y;
            self.velocity.y = 0.0;
        }
        self.position.x += self.velocity.x;
        if self.is_colliding(collidables) {
            self.position.x = self.old_position.x;
            self.position.y = 0.0;
        }

        self.old_position.x = self.position.x;
        self.old_position.y = self.position.y;
    }
    pub fn update_camera(&self, camera: &mut Camera) {
        let position = Vec2{x: self.position.x, y: self.position.y};
        camera.set_position(&position);
    }
    pub fn show(&self, camera: &Camera, displayer: &mut Displayer) {
        let position = camera.convert_to_relative_cordinates(self.position);
        displayer.fill_rect(position, self.size, sdl2::pixels::Color::RGB(0, 255, 0)); 
    }
    fn is_colliding(&self, collidables: &Vec<Tile>) -> bool { // stolen from java
        for collidable in collidables.iter() {
            if self.position.x + self.size.x > collidable.position.x &&
                collidable.position.x + collidable.size.x > self.position.x {
                if self.position.y + self.size.y > collidable.position.y &&
                    collidable.position.y + collidable.size.y > self.position.y {
                    return true;
                }
            }
        }
        false
    }
    pub fn jump(&mut self, amount: f64) {
        self.velocity.y = -amount;
    }
    pub fn strafe(&mut self, amount: f64) {
        self.velocity.x = amount;
    }
}

