#![feature(globs)]

extern crate native;
extern crate glfw;
extern crate opengles;

use game::Game;

mod snakeshader;
mod glfwwrapper;
mod snakeapp;
mod graphics;
mod game;

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

