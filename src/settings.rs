use shark;

pub static RADIUS: f64 = 0.1;

pub static PLAYER_INITIAL_BLOOD: f64 = 1.0;
pub static PLAYER_RADIUS: f64 = 0.1;
pub static PLAYER_SPEED_RIGHT: f64 = 0.1;
pub static PLAYER_SPEED_LEFT: f64 = 0.1;
pub static PLAYER_SPEED_UP: f64 = 0.1;
pub static PLAYER_SPEED_DOWN: f64 = 0.1;

pub struct SharkSettings {
    pub initial_state: shark::SharkState,    
    pub bite_damage: f64,
    pub sensor_distance: f64,
    pub attack_distance: f64,
    pub wait_seconds_before_initial_attack: f64,
    pub wait_seconds_before_repeat_attack: f64,
    pub test_color: [f32, ..4],
    pub radius: f64,
}

pub static SHARK_SETTINGS: SharkSettings = SharkSettings {
    initial_state: shark::Ignorant,
    bite_damage: 0.1,
    sensor_distance: 0.2,
    attack_distance: 2.0 * PLAYER_RADIUS,
    wait_seconds_before_initial_attack: 1.0,
    wait_seconds_before_repeat_attack: 1.0,
    test_color: BLACK,
    radius: 0.1,
};

pub static SHARK_SPEED_RIGHT: f64 = 0.1;
pub static SHARK_SPEED_LEFT: f64 = 0.1;
pub static SHARK_SPEED_UP: f64 = 0.1;
pub static SHARK_SPEED_DOWN: f64 = 0.1;

// shark 1.
pub static SHARK_1_SETTINGS: SharkSettings = SHARK_SETTINGS;
pub static SHARK_1_POS: [f64, ..2] = [-0.1, 0.1];

pub static BAR_RECTANGLE: [f64, ..4] = [0.3, -0.06, 1.5, 0.05];
pub static BAR_MARGIN: f64 = 0.01;

pub static AIR_BAR_INITIAL_VALUE: f64 = 0.5;
pub static AIR_BAR_POS: [f64, ..2] = [-0.9, 0.95];
pub static AIR_BAR_TEXT_COLOR: [f32, ..4] = BLACK;
pub static AIR_BAR_BACKGROUND_COLOR: [f32, ..4] = DARK_BLUE;
pub static AIR_BAR_BAR_COLOR: [f32, ..4] = LIGHT_BLUE;

pub static BLOOD_BAR_INITIAL_VALUE: f64 = 0.5;
pub static BLOOD_BAR_POS: [f64, ..2] = [-0.9, 0.85];
pub static BLOOD_BAR_TEXT_COLOR: [f32, ..4] = BLACK;
pub static BLOOD_BAR_BACKGROUND_COLOR: [f32, ..4] = DARK_RED;
pub static BLOOD_BAR_BAR_COLOR: [f32, ..4] = LIGHT_RED;

pub static BLACK: [f32, ..4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];

pub static RED: [f32, ..4] = [1.0, 0.0, 0.0, 1.0];
pub static DARK_RED: [f32, ..4] = [0.5, 0.0, 0.0, 1.0];
pub static LIGHT_RED: [f32, ..4] = [1.0, 0.0, 0.0, 1.0];

pub static GREEN: [f32, ..4] = [0.0, 1.0, 0.0, 1.0];

pub static BLUE: [f32, ..4] = [0.0, 0.0, 1.0, 1.0];
pub static DARK_BLUE: [f32, ..4] = [0.0, 0.0, 0.5, 1.0];
pub static LIGHT_BLUE: [f32, ..4] = [0.5, 0.5, 1.0, 1.0];

pub static ORIGIN: [f64, ..2] = [0.0, 0.0];

