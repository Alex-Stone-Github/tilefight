use sdl2::image::LoadTexture;

mod player;
mod vec;
mod settings;
mod camera;
mod displayer;
mod tile;
mod util;
mod world;

use displayer::Displayer;
use player::Player;
use settings::{FPS, WIDTH, HEIGHT};
use vec::Vec2;
use camera::Camera;
use world::WorldManager;

fn main() {
    // library setup
    let sdl2_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl2_ctx.video().unwrap();
    let window = video_subsystem.window(
        "Tile Fighter",
        WIDTH as u32,
        HEIGHT as u32)
        .build()
        .unwrap();


    // asset loading
    let canvas = window.into_canvas().build().unwrap();
    let texture_loader = canvas.texture_creator();
    let texture_index = std::collections::HashMap::from([
        (0, texture_loader.load_texture("./res/ex.png").unwrap()), // example image
    ]);
    let mut displayer = Displayer::new(canvas, texture_index);


    // game entities
    let mut camera = Camera::new(Vec2{x:0.0,y:0.0});
    let mut world_manager = WorldManager::new();
    let mut current_world = world_manager.next().unwrap();

    // main loop
    let mut event_pump = sdl2_ctx.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            // input
            match event {
                sdl2::event::Event::Quit{..} => {
                    break 'main;
                },
                sdl2::event::Event::KeyDown{ keycode: Some(x), .. } => {
                    match x {
                        sdl2::keyboard::Keycode::W => {
                            current_world.player.jump(20.0);
                        },
                        // sdl2::keyboard::Keycode::A => {
                        //     current_world.player.strafe(-8.0);
                        // },
                        // sdl2::keyboard::Keycode::D => {
                        //     current_world.player.strafe(8.0);
                        // },
                        _ => ()
                    }
                },
                _ => {}
            }
        }
        // other input
        // TODO: add in a dash
        let keys: Vec<_> = event_pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(sdl2::keyboard::Keycode::from_scancode)
            .collect();
        if keys.contains(&sdl2::keyboard::Keycode::A) {
            current_world.player.strafe(-5.0);
        }
        if keys.contains(&sdl2::keyboard::Keycode::D) {
            current_world.player.strafe(5.0);
        }

        // process
        current_world.player.update(&current_world.tiles);
        current_world.player.update_camera(&mut camera);

        // draw
        displayer.set_background(sdl2::pixels::Color::RGB(255, 0, 0));
        current_world.player.show(&camera, &mut displayer);
        for tile in current_world.tiles.iter() {
            tile.show(&camera, &mut displayer);
        }
        displayer.present();

        // wait
        std::thread::sleep(std::time::Duration::from_millis(1000u64 / FPS));
    }
}
