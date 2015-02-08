// External crates.
use graphics;
use graphics::{ BackEnd, Context, RelativeTransform };
use interpolation::lerp;
use colors;

// Local crate.
use character;
use action;
use settings;
use bar::Bar;
use player;
use player::Player;
use air_bottle::AirBottle;
use snake;
use snake::Snake;
use text;
use snakeapp::{
    current_bars,
    current_player,
    current_snakes,
    current_air_bottles
};

#[derive(Debug)]
pub enum Data {
    Player,
    Snake(usize),
    AirBottle(usize),
    Bar(usize),
    BarBackground,
}

/// All objects are of same kind.
/// Makes it easier to write game logic.
pub struct Object {
    pub layer: usize,
    pub pos: [f64; 2],
    pub vel: [f64; 2],
    pub acc: [f64; 2],
    pub acceleration_h: [f64; 2],
    pub acceleration_v: [f64; 2],
    pub radius: f64,
    pub test_color: [f32; 4],
    pub data: Data,
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
            data: Data::BarBackground,
        }
    }

    pub fn air_bottle(
        pos: [f64; 2]
    ) -> Object {
        let i = current_air_bottles().len();
        current_air_bottles().push(AirBottle {
            fill_up: settings::AIR_BOTTLE_FILL_UP,
        });
        Object {
            pos: pos,
            layer: 0,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: [0.0, 0.0],
            acceleration_v: [0.0, 0.0],
            radius: settings::AIR_BOTTLE_RADIUS,
            test_color: settings::AIR_BOTTLE_TEST_COLOR,
            data: Data::AirBottle(i),
        }
    }

    pub fn snake(
        pos: [f64; 2],
        settings: settings::SnakeSettings
    ) -> Object {

        let n = 256;
        let mut tail = Vec::with_capacity(n * 2);
        for _ in range(0, n) {
            tail.push(pos[0]);
            tail.push(pos[1]);
        }
        let i = current_snakes().len();
        current_snakes().push(Snake {
            sensor_distance: settings.sensor_distance,
            state: settings.initial_state,
            bite_damage: settings.bite_damage,
            attack_distance: settings.attack_distance,
            tail: tail,
            wait_seconds_before_initial_attack: settings.wait_seconds_before_initial_attack,
            wait_seconds_before_repeat_attack: settings.wait_seconds_before_repeat_attack,
        });
        Object {
            layer: 0,
            pos: pos,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: [settings.acceleration_left, settings.acceleration_right],
            acceleration_v: [settings.acceleration_up, settings.acceleration_down],
            radius: settings.radius,
            test_color: settings.test_color,
            data: Data::Snake(i),
        }
    }

    pub fn bar(
        pos: [f64; 2],
        text: &'static str,
        text_color: [f32; 4],
        background_color: [f32; 4],
        bar_color: [f32; 4],
        value: fn () -> f64
    ) -> Object {

        let i = current_bars().len();
        current_bars().push(Bar {
            text: text,
            value: value,
            text_color: text_color,
            background_color: background_color,
            bar_color: bar_color,
        });
        Object {
            layer: 2,
            pos: pos,
            radius: 0.0,
            vel: [0.0, 0.0],
            acc: [0.0, 0.0],
            acceleration_h: [0.0, 0.0],
            acceleration_v: [0.0, 0.0],
            test_color: colors::BLACK,
            data: Data::Bar(i),
        }
    }

    fn render_snake<B: BackEnd>(
        &self,
        snake: &Snake,
        x: f64,
        y: f64,
        rad: f64,
        cam: &Context,
        _c: &Context,
        gl: &mut B
    ) {

        // cam.square_centered(x, y, rad).color(self.test_color).draw(gl);
        graphics::Ellipse::new(self.test_color)
            .draw(graphics::ellipse::circle(x, y, rad), cam, gl);
        let n = snake.tail.len() / 2;
        let black = graphics::Ellipse::new(colors::BLACK);
        let tail = graphics::Ellipse::new(settings::SNAKE_TAIL_COLOR);
        for i in range(0, n) {
            let x = snake.tail[i * 2];
            let y = snake.tail[i * 2 + 1];
            if (i / 8) % 2 == 1 {
                black.draw(graphics::ellipse::circle(x, y, rad), cam, gl);
            } else {
                tail.draw(graphics::ellipse::circle(x, y, rad), cam, gl);
            }
        }
    }

    fn render_player<B: BackEnd>(
        &self,
        player: &Player,
        x: f64,
        y: f64,
        cam: &Context,
        _c: &Context,
        gl: &mut B
    ) {

        // cam.square_centered(x, y, rad).color(self.test_color).draw(gl);
        match player.state {
            player::PlayerState::Normal => {
                character::draw_character(
                    &graphics::Polygon::new(settings::PLAYER_COLOR),
                    player.tween_factor,
                    &cam.trans(x, y).zoom(0.002),
                    gl
                );
            },
            player::PlayerState::Bitten(sec) => {
                let t = 1.0 - sec / settings::PLAYER_BITTEN_FADE_OUT_SECONDS;
                let color = lerp(&settings::PLAYER_BITTEN_COLOR, &settings::PLAYER_COLOR, &(t as f32));
                character::draw_character(
                    &graphics::Polygon::new(color),
                    player.tween_factor,
                    &cam
                        .trans(x, y)
                        .zoom(0.002),
                    gl
                );
            },
        }
    }

    fn render_air_bottle<B: BackEnd>(
        &self,
        air_bottle: &AirBottle,
        x: f64,
        y: f64,
        rad: f64,
        cam: &Context,
        _c: &Context,
        gl: &mut B
    ) {
        if air_bottle.fill_up == 0.0 { return; }
        graphics::Rectangle::new(self.test_color)
            .draw(graphics::rectangle::centered_square(x, y, rad), cam, gl);
        text::text(
            "air",
            &graphics::Polygon::new(settings::AIR_BOTTLE_TEXT_COLOR),
            &cam.trans(x, y)
                .flip_v()
                .trans(-0.075, -0.03)
                .zoom(0.001),
            gl
        );
    }

    pub fn render<B: BackEnd>(
        &self, cam: &Context, c: &Context, gl: &mut B
    ) {
        let x = self.pos[0];
        let y = self.pos[1];
        let rad = self.radius;

        match self.data {
            Data::Snake(i) => self.render_snake(
                &current_snakes()[i], x, y, rad, cam, c, gl),
            Data::Player => self.render_player(
                &*current_player(), x, y, cam, c, gl),
            Data::AirBottle(i) => self.render_air_bottle(
                &current_air_bottles()[i], x, y, rad, cam, c, gl),
            Data::Bar(i) => {
                current_bars()[i].render(&c.trans(x, y), gl);
            },
            Data::BarBackground => {
                // Render round rectangle around bars.
                let bar_bgh = settings::BAR_BACKGROUND_HEIGHT;
                let bar_color = settings::BAR_BACKGROUND_COLOR;
                let bar_color_2 = settings::BAR_BACKGROUND_COLOR_2;
                let margin = settings::BAR_BACKGROUND_MARGIN;
                let margin_2 = settings::BAR_BACKGROUND_MARGIN_2;
                let rect = graphics::rectangle::margin(
                        [-1.0, 1.0 - bar_bgh, 2.0, bar_bgh],
                        margin
                    );
                graphics::Rectangle::round(bar_color, 0.1)
                    .draw(rect, c, gl);
                let rect_2 = graphics::rectangle::margin(
                        [-1.0, 1.0 - bar_bgh, 2.0, bar_bgh],
                        margin_2
                    );
                graphics::Rectangle::round(bar_color_2, 0.1)
                    .draw(rect_2, c, gl);
            }
        };
    }

    fn move_snake(&mut self, state: snake::SnakeState, player_dx: f64, player_dy: f64) {
        match state {
            snake::SnakeState::ChasingPlayer => {
                if player_dx > 0.0 { self.move_right(); }
                else { self.move_left(); }

                if player_dy > 0.0 { self.move_up(); }
                else { self.move_down(); }
            },
            _ => {},
       }
    }

    pub fn update(&mut self, dt: f64, player_pos: [f64; 2]) -> action::Action {
        use std::num::Float;

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

        let mut action = action::Action::Passive;
        // Update object state.
        let (player_dx, player_dy) = (player_pos[0] - self.pos[0], player_pos[1] - self.pos[1]);
        match self.data {
            Data::Snake(i) => {
                action = current_snakes()[i].update(dt, player_pos, self.pos);
            },
            _ => {},
        }

        // Move object.
        match self.data {
            Data::Snake(i) => {
                let &Snake { state, .. } = &current_snakes()[i];
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
