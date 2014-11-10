#![feature(globs)]

extern crate current;
extern crate fps_counter;
extern crate input;
extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate rand;
extern crate native;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate sdl2;

use shader_version::opengl;
use current::{ Current, Get, Set, Usage, UseCurrent };
use std::cell::RefCell;
use opengl_graphics::Gl;
use gfx_graphics::G2D;
use gfx::{Device, DeviceHelper};
use sdl2_window::Sdl2Window as Window;
use event::{
    Events, WindowSettings,
    Render, Update, Input,
};
use event::window::{ Title };
use fps_counter::FPSCounter;
use snakeapp::SnakeApp;

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

    let backend = Gfx;
    println!("Running with graphics backend {}", backend);
    println!("Use 'S' to swap back-end");

    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Window::new(
        opengl,
        WindowSettings {
            title: "Sea Snake Escape".to_string(),
            size: [512, 512],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    let window = RefCell::new(window);
    let backend = RefCell::new(backend);
    let device = RefCell::new(device);
    let opengl = RefCell::new(opengl);

    let window_guard = window.set_current();
    let backend_guard = backend.set_current();
    let device_guard = device.set_current();
    let opengl_guard = opengl.set_current();
    start();
    drop(opengl_guard);
    drop(device_guard);
    drop(backend_guard);
    drop(window_guard);
}

fn current_window() -> Usage<'static, Window> { UseCurrent }

fn current_gfx_device() -> Usage<'static, gfx::GlDevice> { UseCurrent }

fn current_opengl() -> Usage<'static, opengl::OpenGL> { UseCurrent }

fn current_graphics_back_end() -> Usage<'static, GraphicsBackEnd> { UseCurrent }

fn swap_backend<E: event::GenericEvent>(e: &E) {
    use event::{ PressEvent };
    e.press(|button| {
        if button == input::Keyboard(input::keyboard::S) {
            *current_graphics_back_end() = match *current_graphics_back_end() {
                    Gfx => { println!("Swapped to OpenGL"); OpenGL }
                    OpenGL => { println!("Swapped to Gfx"); Gfx }
                };
        }
    });
}

fn set_title(text: String) {
    current_window().set_mut(Title(text));
}

fn events() -> event::Events<current::Usage<'static, Window>> {
    Events::new(current_window())
}

fn start() {
    use event::window::Size;
    let Size([w, h]) = current_window().get();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let ref mut gl = Gl::new(*current_opengl());
    let mut g2d = G2D::new(&mut *current_gfx_device());
    let mut renderer = current_gfx_device().create_renderer();

    let mut fps_counter = FPSCounter::new();
    let mut app = SnakeApp::new();
    app.load();
    for e in events() {
        swap_backend(&e);
        match e {
            Render(args) => {
                match *current_graphics_back_end() {
                    Gfx => {
                        g2d.draw(&mut renderer, &frame, |c, g| {
                            use graphics::*;
                            c.color(settings::WATER_COLOR).draw(g);
                            app.render(&c, g);
                        });
                        current_gfx_device().submit(renderer.as_buffer());
                        renderer.reset();
                    }
                    OpenGL => {
                        use graphics::*;
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

                set_title(fps_counter.tick().to_string());
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
