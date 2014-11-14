#![feature(globs)]
#![feature(if_let)]

extern crate piston;
extern crate gfx;

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
    use piston::shader_version::opengl;

    let opengl = opengl::OpenGL_3_2;
    piston::start(
        opengl,
        piston::WindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        },
        || snakeapp::app()
    );
}
