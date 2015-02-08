#![feature(core)]

extern crate start_piston;
extern crate piston;
extern crate gfx;
extern crate shader_version;
extern crate graphics;
extern crate interpolation;
extern crate current;

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
mod colors;

fn main() {
    use shader_version::opengl::OpenGL;

    let opengl = OpenGL::_3_2;
    start_piston::start(
        opengl,
        piston::window::WindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        },
        || snakeapp::app()
    );
}
