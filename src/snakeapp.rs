
use Settings = piston::game::Settings;
use Game = piston::game::Game;
use GameWindow = piston::game_window::GameWindow;
use snakeshader::SnakeShader;
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
    shader: Option<SnakeShader>,
    vertices: Vec<f32>,
    colors: Vec<f32>,
    settings: Settings,
    game_window: GameWindow,
}

impl Game for SnakeApp {
    fn get_game_window<'a>(&'a self) -> &'a GameWindow { &self.game_window }
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    fn render(&self) {
        // Render triangle.
        // self.shader.unwrap().render(self.vertices.as_slice(), self.colors.as_slice());

        let shader = self.shader.unwrap();
        for _ in range(0, 1 << 19) {
            let x: f32 = random();
            let y: f32 = random();
            let rect = [x - 0.5, y - 0.5, 0.005, 0.005];
            let color = [random(), 0.0, 0.0, 1.0];
            shader.fill_rect(rect, color);
        }
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
            game_window: GameWindow::window("Snake", 512, 512),
            vertices: load_vertices(),
            colors: load_colors(),
            settings: Settings::new(exit_on_esc, background_color),
        }
    }
}

