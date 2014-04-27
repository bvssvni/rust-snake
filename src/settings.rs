use snake;
use game_state;

pub static NUMBER_OF_LAYERS: uint = 4;
pub static WATER_COLOR: [f32, ..4] = [0.0, 0.0, 0.45, 1.0];
pub static WATER_FRICTION: f64 = 0.2;
pub static RADIUS: f64 = 0.1;
pub static SURFACE_Y: f64 = 0.8;
pub static INITIAL_GAME_STATE: game_state::GameState = game_state::Play;
pub static INITIAL_CAMERA_POS: [f64, ..2] = [0.0, 0.0];
pub static CAMERA_FOLLOW_PERCENTAGE: f64 = 0.2;

pub static YOU_WIN_TEXT: &'static str = "you win";
pub static YOU_WIN_TEXT_COLOR: [f32, ..4] = GREEN;
pub static YOU_WIN_POS: [f64, ..2] = [-0.42, 0.1];

pub static YOU_LOOSE_TEXT: &'static str = "you loose";
pub static YOU_LOOSE_TEXT_COLOR: [f32, ..4] = RED;
pub static YOU_LOOSE_POS: [f64, ..2] = [-0.55, 0.1];

pub static PLAYER_LOOSE_AIR_SPEED: f64 = 0.1;
pub static PLAYER_COLOR: [f32, ..4] = YELLOW;
pub static PLAYER_INITIAL_BLOOD: f64 = 1.0;
pub static PLAYER_INITIAL_AIR: f64 = 1.0;
pub static PLAYER_RADIUS: f64 = 0.1;
pub static PLAYER_ACCELERATION_RIGHT: f64 = 0.1;
pub static PLAYER_ACCELERATION_LEFT: f64 = 0.1;
pub static PLAYER_ACCELERATION_UP: f64 = 0.1;
pub static PLAYER_ACCELERATION_DOWN: f64 = 0.1;
pub static PLAYER_INITIAL_TWEEN_FACTOR: f64 = 0.0;
pub static PLAYER_TWEEN_SPEED: f64 = 1.0;

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

pub static SNAKE_TAIL_COLOR: [f32, ..4] = [0.8, 0.6, 0.6, 0.4];


pub static SNAKE_TAIL_DISTANCE: f64 = 0.01;
pub static SNAKE_OVERALL_ACCELERATION: f64 = 2.0;
pub static SNAKE_SETTINGS: SnakeSettings = SnakeSettings {
    acceleration_left: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_right: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_up: 0.1 * SNAKE_OVERALL_ACCELERATION,
    acceleration_down: 0.1 * SNAKE_OVERALL_ACCELERATION,
    initial_state: snake::Ignorant,
    bite_damage: 0.1,
    sensor_distance: 2.0,
    attack_distance: 2.0 * PLAYER_RADIUS,
    wait_seconds_before_initial_attack: 1.0,
    wait_seconds_before_repeat_attack: 1.0,
    test_color: BLACK,
    radius: 0.03,
};

pub static SNAKE_SPEED_RIGHT: f64 = 0.1;
pub static SNAKE_SPEED_LEFT: f64 = 0.1;
pub static SNAKE_SPEED_UP: f64 = 0.1;
pub static SNAKE_SPEED_DOWN: f64 = 0.1;

// snake 1.
pub static SNAKE_1_SETTINGS: SnakeSettings = SNAKE_SETTINGS;
pub static SNAKE_1_POS: [f64, ..2] = [-0.8, 0.8];
pub static SNAKE_1_ADD: bool = true;

// snake 2.
pub static SNAKE_2_SETTINGS: SnakeSettings = SNAKE_SETTINGS;
pub static SNAKE_2_POS: [f64, ..2] = [0.8, 0.8];
pub static SNAKE_2_ADD: bool = true;

pub static BAR_RECTANGLE: [f64, ..4] = [0.3, -0.06, 1.5, 0.05];
pub static BAR_MARGIN: f64 = 0.01;
pub static BAR_TEXT_COLOR: [f32, ..4] = WHITE;

pub static AIR_BAR_INITIAL_VALUE: f64 = 0.5;
pub static AIR_BAR_POS: [f64, ..2] = [-0.9, 0.95];
pub static AIR_BAR_TEXT_COLOR: [f32, ..4] = BAR_TEXT_COLOR;
pub static AIR_BAR_BACKGROUND_COLOR: [f32, ..4] = DARK_BLUE;
pub static AIR_BAR_BAR_COLOR: [f32, ..4] = LIGHT_BLUE;

pub static BLOOD_BAR_INITIAL_VALUE: f64 = 0.5;
pub static BLOOD_BAR_POS: [f64, ..2] = [-0.9, 0.85];
pub static BLOOD_BAR_TEXT_COLOR: [f32, ..4] = BAR_TEXT_COLOR;
pub static BLOOD_BAR_BACKGROUND_COLOR: [f32, ..4] = DARK_RED;
pub static BLOOD_BAR_BAR_COLOR: [f32, ..4] = LIGHT_RED;

pub static BAR_BACKGROUND_HEIGHT: f64 = 0.275;
pub static BAR_BACKGROUND_COLOR: [f32, ..4] = [0.1, 0.1, 0.1, 0.9];
pub static BAR_BACKGROUND_COLOR_2: [f32, ..4] = [0.4, 0.4, 0.4, 0.8];
pub static BAR_BACKGROUND_MARGIN: f64 = 0.01;
pub static BAR_BACKGROUND_MARGIN_2: f64 = 0.02;

pub static BLACK: [f32, ..4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
pub static GRAY: [f32, ..4] = [0.5, 0.5, 0.5, 1.0];
pub static DARK_GRAY: [f32, ..4] = [0.2, 0.2, 0.2, 1.0];

pub static RED: [f32, ..4] = [1.0, 0.0, 0.0, 1.0];
pub static DARK_RED: [f32, ..4] = [0.5, 0.0, 0.0, 1.0];
pub static LIGHT_RED: [f32, ..4] = [1.0, 0.0, 0.0, 1.0];

pub static GREEN: [f32, ..4] = [0.0, 1.0, 0.0, 1.0];

pub static BLUE: [f32, ..4] = [0.0, 0.0, 1.0, 1.0];
pub static DARK_BLUE: [f32, ..4] = [0.0, 0.0, 0.5, 1.0];
pub static LIGHT_BLUE: [f32, ..4] = [0.5, 0.5, 1.0, 1.0];

pub static YELLOW: [f32, ..4] = [0.0, 1.0, 1.0, 1.0];

pub static ORIGIN: [f64, ..2] = [0.0, 0.0];

