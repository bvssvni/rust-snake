#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{
    Gl,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Update,
    Input,
};
use piston::input;

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

fn main() {
    use snakeapp::SnakeApp;
    
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut game_window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let mut app = SnakeApp::new();
    app.load();

    let mut event_iterator = EventIterator::new(&mut game_window,
        &EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        });
    let ref mut gl = Gl::new(opengl);
    for e in event_iterator {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = graphics::Context::abs(args.width as f64, args.height as f64);
                c.color(settings::WATER_COLOR).draw(gl);
                app.render(&c, gl); 
            },
            Update(args) => {
                app.update(args.dt);
            },
            Input(input::Press(input::Keyboard(key))) => {
                app.key_press(key);
            },
            Input(input::Release(input::Keyboard(key))) => {
                app.key_release(key);
            },
            _ => {},
        }
    }
}

