#![feature(globs)]
#![feature(if_let)]

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

use current::{ Current, Get, Set, Usage, UseCurrent };
use std::cell::RefCell;
use opengl_graphics::Gl;
use gfx_graphics::G2D;
use gfx::{Device, DeviceHelper};
use sdl2_window::Sdl2Window as Window;
use event::{ Events, WindowSettings };
use event::window::{ Title };
use fps_counter::FPSCounter;

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

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let gl = Gl::new(opengl);
    let g2d = G2D::new(&mut device);
    let renderer = device.create_renderer();
    let event::window::Size([w, h]) = window.get();
    let frame = gfx::Frame::new(w as u16, h as u16);
    let fps_counter = FPSCounter::new();

    let window = RefCell::new(window);
    let backend = RefCell::new(backend);
    let device = RefCell::new(device);
    let gl = RefCell::new(gl);
    let g2d = RefCell::new(g2d);
    let renderer = RefCell::new(renderer);
    let frame = RefCell::new(frame);
    let fps_counter = RefCell::new(fps_counter);

    let window_guard = window.set_current();
    let backend_guard = backend.set_current();
    let device_guard = device.set_current();
    let gl_guard = gl.set_current();
    let g2d_guard = g2d.set_current();
    let renderer_guard = renderer.set_current();
    let frame_guard = frame.set_current();
    let fps_counter = fps_counter.set_current();

    snakeapp::app();

    drop(window_guard);
    drop(backend_guard);
    drop(device_guard);
    drop(gl_guard);
    drop(g2d_guard);
    drop(renderer_guard);
    drop(frame_guard);
    drop(fps_counter);
}

fn current_window() -> Usage<'static, Window> { UseCurrent }
fn current_gfx_device() -> Usage<'static, gfx::GlDevice> { UseCurrent }
fn current_graphics_back_end() -> Usage<'static, GraphicsBackEnd> { UseCurrent }
fn current_gl() -> Usage<'static, Gl> { UseCurrent }
fn current_g2d() -> Usage<'static, G2D> { UseCurrent }
fn current_renderer()
    -> Usage<'static, gfx::Renderer<gfx::GlCommandBuffer>> { UseCurrent }
fn current_frame() -> Usage<'static, gfx::Frame> { UseCurrent }
fn current_fps_counter() -> Usage<'static, FPSCounter> { UseCurrent }

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

fn events() -> event::Events<current::Usage<'static, Window>> {
    Events::new(current_window())
}

/// This gets called from `snakeapp` to render with proper back-end.
fn render(args: &event::RenderArgs) {
    match *current_graphics_back_end() {
        Gfx => {
            current_g2d().draw(&mut *current_renderer(),
                               &*current_frame(), |c, g| {
                use graphics::*;
                c.color(settings::WATER_COLOR).draw(g);
                snakeapp::render(&c, g);
            });
            current_gfx_device().submit(current_renderer().as_buffer());
            current_renderer().reset();
        }
        OpenGL => {
            use graphics::*;
            let gl = &mut *current_gl();
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            gl.clear_program();
            let c = Context::abs(
                args.width as f64,
                args.height as f64
            );
            c.color(settings::WATER_COLOR).draw(gl);
            snakeapp::render(&c, gl);
        }
    };
}

fn fps_tick() {
    current_window().set_mut(Title(
        current_fps_counter().tick().to_string()));
}
