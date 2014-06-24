#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;

use graphics::*;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSDL2,
    GameWindowSettings,
    KeyPress,
    KeyRelease,
    Render,
    Update,
};

mod snakeapp;
mod object;
mod settings;
mod text;
mod bar;
mod player;
mod snake;
mod action;
mod game_state;
mod character;
mod air_bottle;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    use snakeapp::SnakeApp;
    let mut game_window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = SnakeApp::new();
    app.load();

    // app.run(&mut game_window, &mut asset_store);
    let mut game_iterator = GameIterator::new(&mut game_window,
        &GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        });
    let ref mut gl = Gl::new();
    loop { match game_iterator.next() { None => { break }, Some(e) => {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = graphics::Context::abs(args.width as f64, args.height as f64);
                c.color(settings::WATER_COLOR).clear(gl);
                app.render(&c, gl); 
            },
            Update(args) => {
                app.update(args.dt);
            },
            KeyPress(args) => {
                app.key_press(args.key);
            },
            KeyRelease(args) => {
                app.key_release(args.key);
            },
            _ => {},
        }
    } } }
}

