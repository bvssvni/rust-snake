#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;

use piston::*;

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
    let mut game_window = GameWindow::new(
        GameWindowSettings::new(
            "Sea Snake Escape".to_owned(),
            [512, 512],
            false,
            true,
            settings::WATER_COLOR,
        )
    );

    let mut asset_store = AssetStore::empty();

    let mut app = SnakeApp::new();
    app.run(&mut game_window, &mut asset_store);
}

