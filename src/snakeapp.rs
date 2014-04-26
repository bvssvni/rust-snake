
use Settings = piston::game::Settings;
use Game = piston::game::Game;
use piston::gl::Gl;
use graphics;
use graphics::*;

pub static RADIUS: f64 = 0.5;

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub pos: [f64, ..2],
}

impl Object {
    pub fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = RADIUS;
        c.square_centered(x, y, rad).rgba(1.0, 0.0, 0.0, 1.0).fill(gl);
    }
}

pub struct SnakeApp {
    settings: Settings,
    player_index: Option<uint>,
    // Contains the game objects.
    objects: Vec<Object>,
}

impl Game for SnakeApp {
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        for obj in self.objects.iter() {
            obj.render(c, gl);
        }
    }
    fn update(&mut self) {
    }
    fn load(&mut self) {
        self.objects.push(Object { pos: [0.0, 0.0] });
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp { 
        let exit_on_esc = true;
        let background_color = [1.0, 1.0, 1.0, 1.0];
        SnakeApp {
            settings: Settings::new(exit_on_esc, background_color),
            objects: Vec::new(),
            player_index: None,
        }
    }
}

