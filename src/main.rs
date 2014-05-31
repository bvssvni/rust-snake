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
    let mut game_window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            background_color: settings::WATER_COLOR,
        }
    );

    let mut asset_store = AssetStore::empty();

    let mut app = SnakeApp::new();
    app.load();

    // app.run(&mut game_window, &mut asset_store);
    let mut game_iterator = GameIterator::new(&mut game_window);
    loop { match game_iterator.next() { None => { break }, Some(e) => {
        match e {
            Render(args) => {
                app.render(0.0, &args.context, &mut Gl::new(args.gl_data, &mut asset_store)); 
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

