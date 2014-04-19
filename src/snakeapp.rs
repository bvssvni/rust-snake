use game::{Game, Settings};
use glfwwrapper::GlfwWrapper;
use snakeshader::SnakeShader;

use opengles::gl2;

fn load_vertices() -> Vec<f32> {
    vec!(
         0.0,  0.5,
        -0.5, -0.5,
         0.5, -0.5,
    )
}

fn load_colors() -> Vec<f32> {
    vec!(
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,
    )
}

pub struct SnakeApp {
    shader: Option<SnakeShader>,
    vertices: Vec<f32>,
    colors: Vec<f32>,
    settings: Settings,
    glfw_wrapper: GlfwWrapper,
}

impl Game for SnakeApp {
    fn get_glfw_wrapper<'a>(&'a self) -> &'a GlfwWrapper { &self.glfw_wrapper }
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    fn render(&self) {
        self.shader.unwrap().render(self.vertices.as_slice(), self.colors.as_slice());
    }
    fn update(&mut self) {
    }
    fn load(&mut self) {
        self.shader = Some(SnakeShader::new());
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp { 
        let exit_on_esc = true;
        let background_color = [1.0, 1.0, 1.0, 1.0];
        SnakeApp {
            shader: None,
            glfw_wrapper: GlfwWrapper::window("Snake", 512, 512),
            // glfw_wrapper: GlfwWrapper::fullscreen("Snake"),
            vertices: load_vertices(),
            colors: load_colors(),
            settings: Settings::new(exit_on_esc, background_color),
        }
    }
}

