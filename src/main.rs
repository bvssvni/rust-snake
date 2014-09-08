#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx;

use opengl_graphics::Gl;
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

#[deriving(Show)]
pub enum GraphicsBackEnd {
    Gfx,
    OpenGL,
}

fn main() {
    use snakeapp::SnakeApp;
    
    let mut backend = Gfx;
    println!("Running with graphics backend {}", backend);
    println!("Use 'S' to swap back-end");

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

    let mut app = SnakeApp::new();
    app.load();

    let mut event_iterator = EventIterator::new(&mut window,
        &EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        });
    let ref mut gl = Gl::new(opengl);
    let mut gfx2d = Gfx2d::new(&mut device);
    let mut fps_counter = piston::FPSCounter::new();
    for e in event_iterator {
        match e {
            Render(args) => {
                match backend {
                    Gfx => {
                        {
                            let ref mut gl = RenderContext::new(&mut renderer, &frame, &mut gfx2d);
                            let c = graphics::Context::abs(
                                args.width as f64,
                                args.height as f64
                            );
                            c.color(settings::WATER_COLOR).draw(gl);
                            app.render(&c, gl); 
                        }
                        device.submit(renderer.as_buffer());
                        renderer.reset();
                    }
                    OpenGL => {
                        gl.viewport(0, 0, args.width as i32, args.height as i32);
                        let c = graphics::Context::abs(
                            args.width as f64, 
                            args.height as f64
                        );
                        c.color(settings::WATER_COLOR).draw(gl);
                        app.render(&c, gl); 
                    }
                };

                event_iterator.window.window.set_title(fps_counter.tick().to_string().as_slice());
            },
            Update(args) => {
                app.update(args.dt);
            },
            Input(input::Press(input::Keyboard(input::keyboard::S))) => {
                backend = match backend {
                        Gfx => { println!("Swapped to OpenGL"); OpenGL }
                        OpenGL => { println!("Swapped to Gfx"); Gfx }
                    };
            }
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

