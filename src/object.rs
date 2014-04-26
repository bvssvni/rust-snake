
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;
use settings;
use spring::Spring;

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub test_color: [f32, ..4],
    pub springs: Vec<Spring>,
}

impl Object {
    pub fn new(pos: [f64, ..2], test_color: [f32, ..4]) -> Object {
        Object {
            pos: pos,
            vel: [0.0, 0.0],
            test_color: test_color,
            springs: Vec::new(),
        }
    }

    pub fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = settings::RADIUS;
        c.square_centered(x, y, rad).color(self.test_color).fill(gl);
    }

    pub fn update(&mut self, dt: f64) {
        self.pos = [self.pos[0] + self.vel[0] * dt, self.pos[1] + self.vel[1] * dt];
    }

    pub fn move_right(&mut self) {
        let speed_right = settings::PLAYER_SPEED_RIGHT;
        self.vel = [speed_right, self.vel[1]];
    }

    pub fn move_left(&mut self) {
        let speed_left = settings::PLAYER_SPEED_LEFT;
        self.vel = [-speed_left, self.vel[1]];
    }

    pub fn move_up(&mut self) {
        let speed_up = settings::PLAYER_SPEED_UP;
        self.vel = [self.vel[0], speed_up];
    }

    pub fn move_down(&mut self) {
        let speed_down = settings::PLAYER_SPEED_DOWN;
        self.vel = [self.vel[0], -speed_down];
    }
}

