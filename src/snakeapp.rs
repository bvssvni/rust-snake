
use Settings = gameengine::game::Settings;
use Game = gameengine::game::Game;
use GameWindow = gameengine::game_window::GameWindow;
use snakeshader::SnakeShader;

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
    
        let rect = [0.0, 0.0, 0.5, 0.5];
        let color = [1.0, 0.0, 0.0, 1.0];
        self.shader.unwrap().fill_rect(rect, color);
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

