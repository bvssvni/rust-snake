
use action;
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;
use settings;
use spring::Spring;
use bar::Bar;
use player::Player;
use shark;
use shark::Shark;


pub enum ObjectData {
    PlayerData(Player),
    SharkData(Shark),
    BarData(Bar),
}

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub radius: f64,
    pub test_color: [f32, ..4],
    pub data: ObjectData,
}

impl Object {
    pub fn player(pos: [f64, ..2], test_color: [f32, ..4], blood: f64) -> Object {
        Object {
            pos: pos,
            vel: [0.0, 0.0],
            radius: settings::PLAYER_RADIUS,
            test_color: test_color,
            data: PlayerData(Player { blood: blood }),
        }
    }

    pub fn shark(
        pos: [f64, ..2], 
        settings: settings::SharkSettings
    ) -> Object {
        
        Object {
            pos: pos,
            vel: [0.0, 0.0],
            radius: settings.radius,
            test_color: settings.test_color,
            data: SharkData(Shark { 
                sensor_distance: settings.sensor_distance,
                state: settings.initial_state,
                bite_damage: settings.bite_damage,
                attack_distance: settings.attack_distance,
                wait_seconds_before_initial_attack: settings.wait_seconds_before_initial_attack,
                wait_seconds_before_repeat_attack: settings.wait_seconds_before_repeat_attack,
            }),
        }
    }

    pub fn bar(
        pos: [f64, ..2], 
        text: &'static str, 
        text_color: [f32, ..4], 
        background_color: [f32, ..4],
        bar_color: [f32, ..4],
        value: f64
    ) -> Object {
     
       Object {
            pos: pos,
            radius: 0.0,
            vel: [0.0, 0.0],
            test_color: settings::BLACK,
            data: BarData(Bar { 
                text: text, 
                value: value, 
                text_color: text_color, 
                background_color: background_color,
                bar_color: bar_color,
            }),
        }
    }

    pub fn blood(&self) -> Option<f64> {
        match self.data {
            PlayerData(player) => Some(player.blood),
            _ => None,
        }
    }

    pub fn blood_mut<'a>(&'a mut self) -> Option<&'a mut f64> {
        match self.data {
            PlayerData(ref mut player) => Some(&mut player.blood),
            _ => None,
        }
    }

    pub fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = self.radius;      
 
        match self.data {
            SharkData(_) => {
                c.square_centered(x, y, rad).color(self.test_color).fill(gl);
            },
            PlayerData(_) => {
                c.square_centered(x, y, rad).color(self.test_color).fill(gl);
            },
            BarData(bar) => {
                bar.render(&c.trans_local(x, y), gl);
            },
        };
    }

    fn move_shark(&mut self, state: shark::SharkState, player_dx: f64, player_dy: f64) {
        match state {
            shark::ChasingPlayer => {
                if player_dx > 0.0 { self.move_right(); }
                else { self.move_left(); }

                if player_dy > 0.0 { self.move_up(); }
                else { self.move_down(); }
            },
            _ => {},
       }
    }

    pub fn update(&mut self, dt: f64, player_pos: [f64, ..2]) -> action::Action {
        self.pos = [self.pos[0] + self.vel[0] * dt, self.pos[1] + self.vel[1] * dt];
   
        let mut action = action::Passive;
        // Update object state. 
        let (player_dx, player_dy) = (player_pos[0] - self.pos[0], player_pos[1] - self.pos[1]);
        match self.data {
            SharkData(ref mut shark) => {
                action = shark.update(dt, player_pos, self.pos);
            },
            _ => {},
        }

        // Move object.
        match self.data {
            SharkData(Shark { state: state, ..}) => { 
                self.move_shark(state, player_dx, player_dy);
            },
            _ => {},
        }

        action
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

