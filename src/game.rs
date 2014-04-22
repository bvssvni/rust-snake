//! Game loop.

use glfw;
use glfw::Context;
use gl = opengles::gl2;

use glfwwrapper::GlfwWrapper;

/// Basic settings for window behavior.
pub struct Settings {
    /// If true, exit when pressing Esc.
    pub exit_on_esc: bool,
    /// The color to use as background.
    pub background_color: [f32, ..4],
}

impl Settings {
    /// Gets default settings.
    pub fn default() -> Settings {
        Settings {
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates a new Settings.
    pub fn new(exit_on_esc: bool, background_color: [f32, ..4]) -> Settings {
        Settings {
            exit_on_esc: exit_on_esc,
            background_color: background_color,
        }
    }
}

/// Implement default behavior for a game.
pub trait Game {
    /// Get the GLFW wrapper.
    fn get_glfw_wrapper<'a>(&'a self) -> &'a GlfwWrapper;
    
    /// Read settings.
    fn get_settings<'a>(&'a self) -> &'a Settings;
    
    /// Render graphics.
    fn render(&self); 
    
    /// Update the physical state of the game.
    fn update(&mut self);
    
    /// Perform tasks for loading before showing anything.
    fn load(&mut self);

    /// Clears the background with color from settings.
    #[inline(always)]
    fn clear_background(&self) {
        let rgb  = self.get_settings().background_color;
        gl::clear_color(rgb[0], rgb[1], rgb[2], rgb[3]);
        gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    /// Sets up viewport.
    #[inline(always)]
    fn viewport(&self) {
        let glfw = self.get_glfw_wrapper();
        let (w, h) = glfw.window.get_size();
        gl::viewport(0, 0, w as gl::GLint, h as gl::GLint); 
        self.clear_background();
    }

    /// Whether the window should be closed.
    fn should_close(&self) -> bool {
        self.get_glfw_wrapper().window.should_close()
    }

    /// Swaps the front buffer with the back buffer.
    /// This shows the next frame.
    fn swap_buffers(&self) {
        self.get_glfw_wrapper().window.swap_buffers()
    }

    /// Handles events with default settings..
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

