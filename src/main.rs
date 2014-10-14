#![feature(globs)]

extern crate piston;
extern crate rand;
extern crate native;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate sdl2;

use opengl_graphics::Gl;
use gfx_graphics::G2D;
use piston::graphics::{
    AddColor,
    Context,
    Draw,
};
use gfx::{Device, DeviceHelper};
use sdl2_game_window::WindowSDL2;
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Update,
    Input,
};
use piston::input;
use piston::{Window};

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

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let (w, h) = window.get_size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let mut renderer = device.create_renderer();

    let mut app = SnakeApp::new();
    app.load();

    let mut event_iterator = EventIterator::new(&mut window,
        &EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        });
    let ref mut gl = Gl::new(opengl);
    let mut g2d = G2D::new(&mut device);
    let mut fps_counter = piston::FPSCounter::new();
    loop {
        let e = match event_iterator.next() {
                None => { break; }
                Some(e) => e
            };

        match e {
            Render(args) => {
                match backend {
                    Gfx => {
                        g2d.draw(&mut renderer, &frame, |c: Context, g| {
                            c.color(settings::WATER_COLOR).draw(g);
                            app.render(&c, g);
                        });
                        device.submit(renderer.as_buffer());
                        renderer.reset();
                    }
                    OpenGL => {
                        gl.viewport(0, 0, args.width as i32, args.height as i32);
                        gl.clear_program();
                        let c = Context::abs(
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
