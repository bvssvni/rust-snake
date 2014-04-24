
use Settings = piston::game::Settings;
use Game = piston::game::Game;
use GameWindow = piston::game_window::GameWindow;
use piston::gl::Gl;
use graphics;
use graphics::*;
use rand::random;

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
    vertices: Vec<f32>,
    colors: Vec<f32>,
    settings: Settings,
    game_window: GameWindow,
}

impl Game for SnakeApp {
    fn get_game_window<'a>(&'a self) -> &'a GameWindow { &self.game_window }
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        // Render triangle.
        // self.shader.unwrap().render(self.vertices.as_slice(), self.colors.as_slice());

        for _ in range(0, 1 << 10) {
            let x: f64 = random();
            let y: f64 = random();
            c
            .rect(x - 0.5, y - 0.5, 0.005, 0.005)
            .rgba(random(), 0.0, 0.0, 1.0)
            .fill(gl);
        }
    }
    fn update(&mut self) {
    }
    fn load(&mut self) {
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp { 
        let exit_on_esc = true;
        let background_color = [1.0, 1.0, 1.0, 1.0];
        SnakeApp {
            game_window: GameWindow::window("Snake", 512, 512),
            vertices: load_vertices(),
            colors: load_colors(),
            settings: Settings::new(exit_on_esc, background_color),
        }
    }
}

