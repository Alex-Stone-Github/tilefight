use crate::vec::Vec2;
use crate::settings::{GRAVITY, WIDTH, HEIGHT, FRICTION_COEFFICIENCY};
use crate::camera::Camera;
use crate::displayer::Displayer;
use crate::tile::Tile;
use crate::util::tile_2d_collision;

pub struct Player {
    position: Vec2<f64>,
    size: Vec2<f64>,
    velocity: Vec2<f64>,
    old_position: Vec2<f64>,
    grounded: bool,
}

impl Player {
    pub fn new(position: Vec2<f64>, size: Vec2<f64>) -> Self {
        Self {
            position,
            size,
            velocity: Vec2{x:0.0, y:0.0},
            old_position: Vec2{x:0.0, y:0.0},
            grounded: false
        }
    }
    pub fn update(&mut self, collidables: &Vec<Tile>) {
        self.velocity.y += GRAVITY;
        self.velocity.x *= FRICTION_COEFFICIENCY;
        self.velocity.y *= FRICTION_COEFFICIENCY;

        self.grounded = false;
        self.position.y += self.velocity.y;
        if self.is_colliding(collidables) {
            self.position.y = self.old_position.y;
            self.velocity.y = 0.0;
            self.grounded = true;
        }
        self.position.x += self.velocity.x;
        if self.is_colliding(collidables) {
            self.position.x = self.old_position.x;
            self.position.y = 0.0;
            self.grounded = true;
        }

        self.old_position.x = self.position.x;
        self.old_position.y = self.position.y;
    }
    pub fn update_camera(&self, camera: &mut Camera) {
        let position = Vec2{x: self.position.x - WIDTH as f64 / 2.0 + self.size.x / 2.0, y: self.position.y - HEIGHT as f64 / 2.0 + self.size.y / 2.0};
        camera.set_position(&position);
    }
    pub fn show(&self, camera: &Camera, displayer: &mut Displayer) {
        let position = camera.convert_to_relative_cordinates(self.position);
        displayer.draw_texture(position, self.size, 0); 
    }
    fn is_colliding(&self, collidables: &Vec<Tile>) -> bool { // stolen from java
        for collidable in collidables.iter() {
            if tile_2d_collision(self.position, self.size, collidable.position, collidable.size) {
                return true;
            }
        }
        false
    }
    pub fn jump(&mut self, amount: f64) {
        if self.grounded {
            self.velocity.y = -amount;
        }
    }
    pub fn strafe(&mut self, amount: f64) {
        self.velocity.x = amount;
    }
}

