use crate::vec::Vec2;

pub struct Displayer<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_index: std::collections::HashMap<u32, sdl2::render::Texture<'a>>,
}

impl<'a> Displayer<'a> {
    pub fn new(
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        texture_index: std::collections::HashMap<u32, sdl2::render::Texture<'a>>) -> Self {

        Self {
            canvas,
            texture_index,
        }
    }
    pub fn set_background(&mut self, color: sdl2::pixels::Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
    pub fn fill_rect(
        &mut self, position: Vec2<f64>, size: Vec2<f64>,
        color: sdl2::pixels::Color) {
        let rect = sdl2::rect::Rect::new(
            position.x as i32, position.y as i32,
            size.x as u32, size.y as u32);
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(rect);
    }
    pub fn draw_texture(
        &mut self, position: Vec2<f64>, size: Vec2<f64>,
        uiid: u32) {
        let rect = sdl2::rect::Rect::new(
            position.x as i32, position.y as i32,
            size.x as u32, size.y as u32);
        let texture = self.texture_index.get(&uiid).expect("bad texture name");
        let _ = self.canvas.copy(texture, None, Some(rect));
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
}
