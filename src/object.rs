use character;
use action;
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;
use settings;
use spring::Spring;
use bar::Bar;
use player;
use player::Player;
use snake;
use snake::Snake;
use graphics::interpolation::{lerp_4};

pub enum ObjectData {
    PlayerData(Player),
    SnakeData(Snake),
    BarData(Bar),
    BarBackgroundData,
}

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub layer: uint,
    pub pos: [f64, ..2],
    pub vel: [f64, ..2],
    pub acc: [f64, ..2],
    pub acceleration_h: [f64, ..2],
    pub acceleration_v: [f64, ..2],
    pub radius: f64,
    pub test_color: [f32, ..4],
    pub data: ObjectData,
}

impl Object {
    pub fn bar_background() -> Object {
        Object {
            pos: [0.0, 0.0],
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            layer: 1,
            radius: 0.0,
            test_color: [0.0, 0.0, 0.0, 0.0],
            acceleration_h: [0.0, 0.0],
            acceleration_v: [0.0, 0.0],
            data: BarBackgroundData,
        }
    }

    pub fn player(
        pos: [f64, ..2], 
        test_color: [f32, ..4], 
        blood: f64,
        air: f64,
        acceleration_h: [f64, ..2],
        acceleration_v: [f64, ..2]) -> Object {
        Object {
            layer: 0,
            pos: pos,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: acceleration_h,
            acceleration_v: acceleration_v,
            radius: settings::PLAYER_RADIUS,
            test_color: test_color,
            data: PlayerData(Player { 
                blood: blood,
                air: air,
                tween_factor: settings::PLAYER_INITIAL_TWEEN_FACTOR,
                state: settings::PLAYER_INITIAL_STATE,
            }),
        }
    }

    pub fn snake(
        pos: [f64, ..2], 
        settings: settings::SnakeSettings
    ) -> Object {
        
        let mut tail = [0.0, ..256];
        let n = tail.len() / 2;
        for i in range(0, n) {
            tail[i * 2] = pos[0];
            tail[i * 2 + 1] = pos[1];
        }
        Object {
            layer: 0,
            pos: pos,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: [settings.acceleration_left, settings.acceleration_right],
            acceleration_v: [settings.acceleration_up, settings.acceleration_down],
            radius: settings.radius,
            test_color: settings.test_color,
            data: SnakeData(Snake { 
                sensor_distance: settings.sensor_distance,
                state: settings.initial_state,
                bite_damage: settings.bite_damage,
                attack_distance: settings.attack_distance,
                tail: tail,
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
            layer: 2,
            pos: pos,
            radius: 0.0,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: [0.0, 0.0],
            acceleration_v: [0.0, 0.0],
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

    pub fn player_mut<'a>(&'a mut self) -> Option<&'a mut Player> {
        match self.data {
            PlayerData(ref mut player) => Some(player),
            _ => None,
        }
    }

    pub fn render(&self, cam: &graphics::Context, c: &graphics::Context, gl: &mut Gl) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = self.radius;      
 
        match self.data {
            SnakeData(ref snake) => {
                // cam.square_centered(x, y, rad).color(self.test_color).fill(gl);
                cam.circle_centered(x, y, rad).color(self.test_color).fill(gl);
                let n = snake.tail.len() / 2;
                for i in range(0, n) {
                    let x = snake.tail[i * 2];
                    let y = snake.tail[i * 2 + 1];
                    cam.circle_centered(x, y, rad).color(settings::SNAKE_TAIL_COLOR).fill(gl);
                } 
            },
            PlayerData(ref player) => {
                // cam.square_centered(x, y, rad).color(self.test_color).fill(gl);
                match player.state {
                    player::Normal => {
                        character::draw_character(player.tween_factor, 
                        &cam.trans_local(x, y).zoom_local(0.002).color(settings::PLAYER_COLOR), gl);
                    },
                    player::Bitten(sec) => {
                        let t = (1.0 - sec / settings::PLAYER_BITTEN_FADE_OUT_SECONDS);
                        let color = lerp_4(&settings::PLAYER_BITTEN_COLOR, &settings::PLAYER_COLOR, &(t as f32));
                        character::draw_character(player.tween_factor, 
                        &cam.trans_local(x, y).zoom_local(0.002).color(color), gl);
                    },
                }
            },
            BarData(bar) => {
                bar.render(&c.trans_local(x, y), gl);
            },
            BarBackgroundData => {
                // Render round rectangle around bars.
                let bar_bgh = settings::BAR_BACKGROUND_HEIGHT;
                let bar_color = settings::BAR_BACKGROUND_COLOR;
                let bar_color_2 = settings::BAR_BACKGROUND_COLOR_2;
                let margin = settings::BAR_BACKGROUND_MARGIN;
                let margin_2 = settings::BAR_BACKGROUND_MARGIN_2;
                c.rect(-1.0, 1.0 - bar_bgh, 2.0, bar_bgh).margin(margin).round(0.1).color(bar_color).fill(gl);
                c.rect(-1.0, 1.0 - bar_bgh, 2.0, bar_bgh).margin(margin_2).round(0.1).color(bar_color_2).fill(gl);
            }
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
        self.pos = [
            self.pos[0] + 0.5 * self.vel[0] * dt, 
            self.pos[1] + 0.5 * self.vel[1] * dt
        ];
        self.vel = [
            self.vel[0] + self.acc[0] * dt,
            self.vel[1] + self.acc[1] * dt
        ];
        let vel_len = self.vel[0] * self.vel[0] + self.vel[1] * self.vel[1];
        let friction = settings::WATER_FRICTION;
        let drag = 1.0 / (vel_len * friction).exp();
        self.vel = [
            self.vel[0] * drag,
            self.vel[1] * drag
        ];
        self.pos = [
            self.pos[0] + 0.5 * self.vel[0] * dt,
            self.pos[1] + 0.5 * self.vel[1] * dt
        ];
   
        let mut action = action::Passive;
        // Update object state. 
        let (player_dx, player_dy) = (player_pos[0] - self.pos[0], player_pos[1] - self.pos[1]);
        match self.data {
            SnakeData(ref mut shark) => {
                action = shark.update(dt, player_pos, self.pos);
            },
            PlayerData(ref mut player) => {
                player.update(dt);
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
        let acc_right = self.acceleration_h[1];
        self.acc = [acc_right, self.acc[1]];
    }

    pub fn move_left(&mut self) {
        let acc_left = self.acceleration_h[0];
        self.acc = [-acc_left, self.acc[1]];
    }

    pub fn move_up(&mut self) {
        let acc_up = self.acceleration_v[0];
        self.acc = [self.acc[0], acc_up];
    }

    pub fn move_down(&mut self) {
        let acc_down = self.acceleration_v[1];
        self.acc = [self.acc[0], -acc_down];
    }
}

