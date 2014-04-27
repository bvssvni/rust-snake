use character;
use action;
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;
use settings;
use spring::Spring;
use bar::Bar;
use player::Player;
use snake;
use snake::Snake;


pub enum ObjectData {
    PlayerData(Player),
    SnakeData(Snake),
    BarData(Bar),
}

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub speed_h: [f64, ..2],
    pub speed_v: [f64, ..2],
    pub radius: f64,
    pub test_color: [f32, ..4],
    pub data: ObjectData,
}

impl Object {
    pub fn player(
        pos: [f64, ..2], 
        test_color: [f32, ..4], 
        blood: f64,
        air: f64,
        speed_h: [f64, ..2],
        speed_v: [f64, ..2]) -> Object {
        Object {
            pos: pos,
            vel: [0.0, 0.0],
            speed_h: speed_h,
            speed_v: speed_v,
            radius: settings::PLAYER_RADIUS,
            test_color: test_color,
            data: PlayerData(Player { 
                blood: blood,
                air: air,
                tween_factor: settings::PLAYER_INITIAL_TWEEN_FACTOR,
            }),
        }
    }

    pub fn snake(
        pos: [f64, ..2], 
        settings: settings::SnakeSettings
    ) -> Object {
        
        Object {
            pos: pos,
            vel: [0.0, 0.0],
            speed_h: [settings.speed_left, settings.speed_right],
            speed_v: [settings.speed_up, settings.speed_down],
            radius: settings.radius,
            test_color: settings.test_color,
            data: SnakeData(Snake { 
                sensor_distance: settings.sensor_distance,
                state: settings.initial_state,
                bite_damage: settings.bite_damage,
                attack_distance: settings.attack_distance,
                tail: [0.0, ..16],
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
            speed_h: [0.0, 0.0],
            speed_v: [0.0, 0.0],
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

    pub fn air(&self) -> Option<f64> {
        match self.data {
            PlayerData(player) => Some(player.air),
            _ => None,
        }
    }

    pub fn air_mut<'a>(&'a mut self) -> Option<&'a mut f64> {
        match self.data {
            PlayerData(ref mut player) => Some(&mut player.air),
            _ => None,
        }
    }

    pub fn render(&self, cam: &graphics::Context, c: &graphics::Context, gl: &mut Gl) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = self.radius;      
 
        match self.data {
            SnakeData(_) => {
                cam.square_centered(x, y, rad).color(self.test_color).fill(gl);
            },
            PlayerData(player) => {
                cam.square_centered(x, y, rad).color(self.test_color).fill(gl);
                character::draw_character(player.tween_factor, 
                    &cam.trans_local(x, y).zoom_local(0.002).color(settings::PLAYER_COLOR), gl);
            },
            BarData(bar) => {
                bar.render(&c.trans_local(x, y), gl);
            },
        };
    }

    fn move_snake(&mut self, state: snake::SnakeState, player_dx: f64, player_dy: f64) {
        match state {
            snake::ChasingPlayer => {
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
            SnakeData(ref mut shark) => {
                action = shark.update(dt, player_pos, self.pos);
            },
            PlayerData(ref mut player) => {
                player.tween_factor += dt * settings::PLAYER_TWEEN_SPEED;
            },
            _ => {},
        }

        // Move object.
        match self.data {
            SnakeData(Snake { state: state, ..}) => { 
                self.move_snake(state, player_dx, player_dy);
            },
            _ => {},
        }

        action
    }

    pub fn move_right(&mut self) {
        let speed_right = self.speed_h[1];
        self.vel = [speed_right, self.vel[1]];
    }

    pub fn move_left(&mut self) {
        let speed_left = self.speed_h[0];
        self.vel = [-speed_left, self.vel[1]];
    }

    pub fn move_up(&mut self) {
        let speed_up = self.speed_v[0];
        self.vel = [self.vel[0], speed_up];
    }

    pub fn move_down(&mut self) {
        let speed_down = self.speed_v[1];
        self.vel = [self.vel[0], -speed_down];
    }
}

