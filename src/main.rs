#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate glfw;
extern crate opengles;

use Game = piston::game::Game;
use GameWindow = piston::game_window::GameWindow;

mod snakeapp;
mod object;
mod settings;
mod spring;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    use snakeapp::SnakeApp;

    let game_window = GameWindow::window("Snake", 512, 512);
    let mut app = SnakeApp::new();    
    app.run(&game_window);
}

