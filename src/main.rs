extern crate start_piston;
extern crate piston;
extern crate gfx;
extern crate shader_version;
extern crate graphics;
extern crate interpolation;
extern crate current;
extern crate num;

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
    use piston::window::{ WindowSettings, Size };

    let opengl = OpenGL::_3_2;
    start_piston::start(
        opengl,
        WindowSettings::new(
            "Sea Snake Escape".to_string(),
            Size { width: 512, height: 512 })
            .fullscreen(false)
            .exit_on_esc(true)
            .samples(4),
        || snakeapp::app()
    );
}
