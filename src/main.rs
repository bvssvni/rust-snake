#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate sdl2_game_window;
extern crate gfx_graphics;
extern crate gfx;

use gfx_graphics::{
    Gfx2d,
};
use gfx::{Device, DeviceHelper};
use gfx_graphics::RenderContext;
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
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let (mut device, frame) = window.gfx();
    let mut renderer = device.create_renderer();
    let mut gfx2d = Gfx2d::new(&mut device);

    let mut app = SnakeApp::new();
    app.load();

    let mut event_iterator = EventIterator::new(&mut window,
        &EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        });
    // let ref mut gl = Gl::new(opengl);
    let mut gfx2d = Gfx2d::new(&mut device);
    for e in event_iterator {
        match e {
            Render(args) => {
                {
                    let ref mut gl = RenderContext::new(&mut renderer, &frame, &mut gfx2d);
                    let c = graphics::Context::abs(
                            args.width as f64, args.height as f64);
                    c.color(settings::WATER_COLOR).draw(gl);
                    app.render(&c, gl); 
                }
                device.submit(renderer.as_buffer());
                renderer.reset();
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

