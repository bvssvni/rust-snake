#![feature(globs)]

extern crate gameengine;
extern crate native;
extern crate glfw;
extern crate opengles;

use Game = gameengine::game::Game;

mod snakeshader;
mod snakeapp;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    use snakeapp::SnakeApp;

    let mut app = SnakeApp::new();    
    app.run();
}

