#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate glfw;
extern crate opengles;

use piston::*;

mod snakeapp;
mod object;
mod settings;
mod spring;
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

    let game_window = GameWindow::window("Sea Snake Escape", 512, 512,
        GameWindowSettings {
            exit_on_esc: true,
            background_color: settings::WATER_COLOR,
        }
    );
    let mut app = SnakeApp::new();    
    app.run(&game_window);
}

