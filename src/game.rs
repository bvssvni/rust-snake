
use glfw;
use glfw::Context;
use gl = opengles::gl2;

use glfwwrapper::GlfwWrapper;

pub struct Settings {
    pub exit_on_esc: bool,
    pub background_color: [f32, ..4],
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn new(exit_on_esc: bool, background_color: [f32, ..4]) -> Settings {
        Settings {
            exit_on_esc: true,
            background_color: background_color,
        }
    }
}

pub trait Game {
    fn get_glfw_wrapper<'a>(&'a self) -> &'a GlfwWrapper;
    fn get_settings<'a>(&'a self) -> &'a Settings;
    fn render(&self); 
    fn update(&mut self);
    fn load(&mut self);

    /// Clears the background with color from settings.
    #[inline(always)]
    fn clear_background(&self) {
        let rgb  = self.get_settings().background_color;
        gl::clear_color(rgb[0], rgb[1], rgb[2], rgb[3]);
        gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    #[inline(always)]
    fn viewport(&self) {
        let glfw = self.get_glfw_wrapper();
        let (w, h) = glfw.window.get_size();
        gl::viewport(0, 0, w as gl::GLint, h as gl::GLint); 
        self.clear_background();
    }

    fn should_close(&self) -> bool {
        self.get_glfw_wrapper().window.should_close()
    }

    fn swap_buffers(&self) {
        self.get_glfw_wrapper().window.swap_buffers()
    }

    fn handle_events(&self) {
        let glfw_wrapper = self.get_glfw_wrapper();
        let glfw = &glfw_wrapper.glfw;
        let settings = self.get_settings();
        glfw.poll_events();
        for (_, event) in 
        glfw::flush_messages(&glfw_wrapper.events) {
            match event {
                // Close with Esc.
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _)
                if settings.exit_on_esc  => {
                    glfw_wrapper.window.set_should_close(true)
                },
                _ => {},
            }
        }
    }

    /// Executes a game loop.
    fn run(&mut self) {
        self.load();
        while !self.should_close() {
            self.viewport();
            self.clear_background();
            self.render();
            self.swap_buffers();
            self.update();
            self.handle_events();
        }
    }
}

