#![allow(non_upper_case_globals)]

use snake;
use game_state;
use player;
use object::Object;
use snakeapp::{ current_objects, current_settings, current_index };
use colors;

pub const NUMBER_OF_LAYERS: uint = 4;
pub const WATER_COLOR: [f32, ..4] = [0.0, 0.0, 0.2, 1.0];
pub const WATER_FRICTION: f64 = 0.2;
pub const SURFACE_Y: f64 = 10.0;
pub const INITIAL_GAME_STATE: game_state::GameState = game_state::GameState::Play;
pub const INITIAL_CAMERA_POS: [f64, ..2] = [0.0, 0.0];
pub const CAMERA_FOLLOW_PERCENTAGE: f64 = 0.4;

pub const YOU_WIN_TEXT: &'static str = "you win";
pub const YOU_WIN_TEXT_COLOR: [f32, ..4] = colors::GREEN;
pub const YOU_WIN_POS: [f64, ..2] = [-160.0, -64.0];

pub const YOU_LOSE_TEXT: &'static str = "you lose";
pub const YOU_LOSE_TEXT_COLOR: [f32, ..4] = colors::RED;
pub const YOU_LOSE_POS: [f64, ..2] = [-200.0, -64.0];

pub const AIR_BOTTLE_RADIUS: f64 = 0.1;
pub const AIR_BOTTLE_TEST_COLOR: [f32, ..4] = colors::WHITE;
pub const AIR_BOTTLE_TEXT_COLOR: [f32, ..4] = colors::BLACK;
pub const AIR_BOTTLE_FILL_UP: f64 = 0.5;

pub const PLAYER_LOSE_AIR_SPEED: f64 = 0.1;
pub const PLAYER_COLOR: [f32, ..4] = [0.4, 0.4, 0.4, 1.0];
pub const PLAYER_BITTEN_COLOR: [f32, ..4] = colors::RED;
pub const PLAYER_INITIAL_BLOOD: f64 = 1.0;
pub const PLAYER_INITIAL_AIR: f64 = 1.0;
pub const PLAYER_RADIUS: f64 = 0.1;
pub const PLAYER_TWEEN_SPEED: f64 = 1.0;
pub const PLAYER_BITTEN_FADE_OUT_SECONDS: f64 = 2.0;

pub struct SnakeSettings {
    pub acceleration_left: f64,
    pub acceleration_right: f64,
    pub acceleration_up: f64,
    pub acceleration_down: f64,
    pub initial_state: snake::SnakeState,
    pub bite_damage: f64,
    pub sensor_distance: f64,
    pub attack_distance: f64,
    pub wait_seconds_before_initial_attack: f64,
    pub wait_seconds_before_repeat_attack: f64,
    pub test_color: [f32, ..4],
    pub radius: f64,
}

pub const SNAKE_TAIL_COLOR: [f32, ..4] = [0.8, 0.0, 0.0, 0.8];
pub const SNAKE_TAIL_DISTANCE: f64 = 0.01;
pub const SNAKE_OVERALL_ACCELERATION: f64 = 2.0;
pub const SNAKE_SETTINGS: SnakeSettings = SnakeSettings {
    acceleration_left: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_right: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_up: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_down: 0.1 * SNAKE_OVERALL_ACCELERATION,
    initial_state: snake::SnakeState::Ignorant,
    bite_damage: 0.3,
    sensor_distance: 2.0,
    attack_distance: 0.5 * PLAYER_RADIUS,
    wait_seconds_before_initial_attack: 1.0,
    wait_seconds_before_repeat_attack: 1.0,
    test_color: [1.0, 1.0, 1.0, 1.0],
    radius: 0.03,
};

pub const BAR_RECTANGLE: [f64, ..4] = [0.3, -0.06, 1.5, 0.05];
pub const BAR_MARGIN: f64 = 0.01;
pub const BAR_TEXT_COLOR: [f32, ..4] = colors::WHITE;

pub const AIR_BAR_POS: [f64, ..2] = [-0.9, 0.95];
pub const AIR_BAR_TEXT_COLOR: [f32, ..4] = BAR_TEXT_COLOR;
pub const AIR_BAR_BACKGROUND_COLOR: [f32, ..4] = colors::DARK_BLUE;
pub const AIR_BAR_BAR_COLOR: [f32, ..4] = colors::LIGHT_BLUE;

pub const BLOOD_BAR_POS: [f64, ..2] = [-0.9, 0.85];
pub const BLOOD_BAR_TEXT_COLOR: [f32, ..4] = BAR_TEXT_COLOR;
pub const BLOOD_BAR_BACKGROUND_COLOR: [f32, ..4] = colors::DARK_RED;
pub const BLOOD_BAR_BAR_COLOR: [f32, ..4] = colors::LIGHT_RED;

pub const BAR_BACKGROUND_HEIGHT: f64 = 0.275;
pub const BAR_BACKGROUND_COLOR: [f32, ..4] = [0.1, 0.1, 0.1, 0.9];
pub const BAR_BACKGROUND_COLOR_2: [f32, ..4] = [0.4, 0.4, 0.4, 0.8];
pub const BAR_BACKGROUND_MARGIN: f64 = 0.01;
pub const BAR_BACKGROUND_MARGIN_2: f64 = 0.02;

pub const ORIGIN: [f64, ..2] = [0.0, 0.0];

pub fn player(
    blood: f64,
    air: f64,
) {
    use object::Data;
    use player::Player;
    use snakeapp::current_player;

    *current_player() = Player {
        blood: blood,
        air: air,
        tween_factor: 0.0,
        state: player::PlayerState::Normal,
    };
    let i = current_objects().len();
    current_objects().push(Object {
        layer: 0,
        pos: ORIGIN,
        vel: [0.0, 0.0],
        acc: [0.0, 0.0],
        acceleration_h: [0.1, 0.1],
        acceleration_v: [0.1, 0.1],
        radius: PLAYER_RADIUS,
        test_color: colors::BLUE,
        data: Data::Player,
    });
    current_index().player = Some(i);
}

pub fn level_1() {
    current_settings().surface_y = Some(SURFACE_Y);
    player(
        PLAYER_INITIAL_BLOOD,
        PLAYER_INITIAL_AIR,
    );

    fn add_air_bottles() {
        let air_bottles = &[
            0.5, 0.3,
            0.3, 0.8,
            0.6, 1.4,
            0.3, 2.2,
            0.1, 3.0,
            0.3, 4.0,
            0.3, 5.0,
            0.6, 5.5,
            0.6, 6.1,
            0.7, 6.8,
            0.5, 8.0,
            0.2, 8.2,
            0.6, 8.8,
        ];
        let n = air_bottles.len() / 2;
        let objects = &mut *current_objects();
        for i in range(0, n) {
            objects.push(Object::air_bottle([air_bottles[i * 2], air_bottles[i * 2 + 1]]));
        }
    }

    add_air_bottles();

    fn add_snakes() {
        current_objects().push(
            Object::snake([-0.8, 0.8], SNAKE_SETTINGS));
        current_objects().push(
            Object::snake([0.8, 0.8], SNAKE_SETTINGS));
    }

    add_snakes();
}
