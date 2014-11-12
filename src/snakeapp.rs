// Extern crates.
use graphics::*;
use input::keyboard;
use current::{ Current, CurrentGuard };

// Local crate.
use action;
use settings;
use object;
use object::Object;
use text;
use game_state;
use player::{ Player };
use colors;
use snake::Snake;
use air_bottle::AirBottle;
use bar::Bar;

pub fn current_cam() -> Current<Cam> { Current }
pub fn current_game_state() -> Current<game_state::GameState> { Current }
pub fn current_objects() -> Current<Vec<Object>> { Current }
pub fn current_index() -> Current<Index> { Current }
pub fn current_settings() -> Current<Settings> { Current }
pub fn current_player() -> Current<Player> { Current }
pub fn current_snakes() -> Current<Vec<Snake>> { Current }
pub fn current_air_bottles() -> Current<Vec<AirBottle>> { Current }
pub fn current_bars() -> Current<Vec<Bar>> { Current }

pub fn app() {
    let mut cam = Cam([0.0, 0.0]);
    let mut game_state = game_state::Play;
    let mut objects: Vec<Object> = Vec::new();
    let mut index = Index::new();
    let mut settings = Settings::new();
    let mut player = Player::new();
    let mut snakes: Vec<Snake> = Vec::new();
    let mut air_bottles: Vec<AirBottle> = Vec::new();
    let mut bars: Vec<Bar> = Vec::new();

    let cam_guard = CurrentGuard::new(&mut cam);
    let game_state_guard = CurrentGuard::new(&mut game_state);
    let objects_guard = CurrentGuard::new(&mut objects);
    let index_guard = CurrentGuard::new(&mut index);
    let settings_guard = CurrentGuard::new(&mut settings);
    let player_guard = CurrentGuard::new(&mut player);
    let snakes_guard = CurrentGuard::new(&mut snakes);
    let air_bottles_guard = CurrentGuard::new(&mut air_bottles);
    let bars_guard = CurrentGuard::new(&mut bars);

    start();

    drop(cam_guard);
    drop(game_state_guard);
    drop(objects_guard);
    drop(index_guard);
    drop(settings_guard);
    drop(player_guard);
    drop(snakes_guard);
    drop(air_bottles_guard);
    drop(bars_guard);
}

fn start() {
    use event::{ RenderEvent, UpdateEvent, PressEvent, ReleaseEvent };
    use input;

    load();
    for e in ::events() {
        ::swap_backend(&e);
        e.render(|args| {
            ::render(args);
            ::fps_tick();
        });
        e.update(|args| {
            update(args.dt);
        });
        e.press(|button| {
            if let input::Keyboard(key) = button {
                key_press(key);
            }
        });
        e.release(|button| {
            if let input::Keyboard(key) = button {
                key_release(key);
            }
        });
    }
}

pub struct Cam(pub [f64, ..2]);

impl Cam {
    fn pos(&self) -> [f64, ..2] {
        let Cam(pos) = *self;
        pos
    }

    fn set(&mut self, val: [f64, ..2]) {
        *self = Cam(val);
    }

    /// Make camera follow position.
    fn follow_pos(&mut self, dt: f64, follow_percentage: f64, pos: [f64, ..2]) {
        let camera_pos = self.pos();
        let (dx, dy) = (pos[0] - camera_pos[0], pos[1] - camera_pos[1]);
        let dx = follow_percentage * dt * dx;
        let dy = follow_percentage * dt * dy;
        self.set([camera_pos[0] + dx, camera_pos[1] + dy]);
    }
}

pub struct Index {
    pub player: Option<uint>,
    pub blood_bar: Option<uint>,
    pub air_bar: Option<uint>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            player: None,
            blood_bar: None,
            air_bar: None
        }
    }
}

pub struct Settings {
    // Tells where the surface is.
    pub surface_y: Option<f64>,
    pub camera_follow_percentage: Option<f64>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            surface_y: None,
            camera_follow_percentage: None,
        }
    }
}

pub fn render<B: BackEnd<I>, I: ImageSize>(c: &Context, gl: &mut B) {
    let c = &c.reset();

    // Get camera coordinates.
    let [cam_x, mut cam_y] = current_cam().pos();
    cam_y += 0.4;

    // Render surface.
    let surface_y = current_settings().surface_y.unwrap();
    c.rect(-1.0, surface_y - cam_y, 2.0, 0.05).color(colors::BLUE).draw(gl);

    // Render objects in layers.
    let cam = &c.trans(-cam_x, -cam_y);
    for i in range(0u, settings::NUMBER_OF_LAYERS) {
        for obj in current_objects().iter() {
            if obj.layer == i { obj.render(cam, c, gl); }
        }
    }

    let text_c = c.flip_v();
    let text_c = text_c.zoom(0.0025);
    match *current_game_state() {
        game_state::Win => {
            let pos = settings::YOU_WIN_POS;
            text::text(settings::YOU_WIN_TEXT,
                &text_c
                .trans(pos[0], pos[1])
                .color(settings::YOU_WIN_TEXT_COLOR)
            , gl);
        },
        game_state::Lose => {
            let pos = settings::YOU_LOSE_POS;
            text::text(settings::YOU_LOSE_TEXT,
                &text_c
                .trans(pos[0], pos[1])
                .color(settings::YOU_LOSE_TEXT_COLOR)
            , gl);
        },
        game_state::Play => {

        },
    }
}

pub fn update(dt: f64) {

    fn player_pos() -> [f64, ..2] {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].pos
    }

    fn update_objects(dt: f64) {
        if *current_game_state() != game_state::Play { return; }

        current_player().update(dt);

        // Update states of objects.
        let player_pos = player_pos();
        let mut attack_damage: f64 = 0.0;
        for obj in current_objects().iter_mut() {
            match obj.update(dt, player_pos) {
                action::Passive => {},
                action::Attack(attack) => { attack_damage += attack; },
            }
        }
        // Bite player.
        current_player().bite(attack_damage);

        // Decrease the player's air with time.
        let air = current_player().air;
        current_player().air = air - dt * settings::PLAYER_LOSE_AIR_SPEED;
    }

    update_objects(dt);

    fn fill_air() {
        let player_pos = player_pos();
        let mut air = current_player().air;
        for obj in current_objects().iter_mut() {
            let pos = obj.pos;
            match obj.data {
                object::AirBottleData(i) => {
                    let dx = pos[0] - player_pos[0];
                    let dy = pos[1] - player_pos[1];
                    let d = dx * dx + dy * dy;
                    if d <= settings::AIR_BOTTLE_RADIUS * settings::AIR_BOTTLE_RADIUS {
                        let air_bottle = &mut current_air_bottles()[i];
                        air += air_bottle.fill_up;
                        air_bottle.fill_up = 0.0;
                    }
                },
                _ => {}
            }
        }

        air = if air > 1.0 { 1.0 } else { air };
        current_player().air = air;
    }

    fill_air();

    fn win() {
        let player_pos = player_pos();
        // When player reaches surface, win.
        if player_pos[1] >= current_settings().surface_y.unwrap() {
            *current_game_state() = game_state::Win;
            return;
        }
    }

    win();

    fn lose() {
        let player = &*current_player();
        let blood = player.blood;
        let air = player.air;
        if blood < 0.0 || air < 0.0 {
            *current_game_state() = game_state::Lose;
            return;
        }
    }

    lose();

    fn show_blood() {
        if current_index().blood_bar == None { return; }
        // Show blood.
        let player_blood = current_player().blood;
        let blood_bar_index = current_index().blood_bar.unwrap();
        let objects = &mut *current_objects();
        let blood_bar = objects.get_mut(blood_bar_index).unwrap();
        match blood_bar.data {
            object::BarData(i) => {
                current_bars()[i].value = player_blood;
            },
            _ => {},
        }
    }

    show_blood();

    fn show_air() {
        if current_index().air_bar == None { return; }
        // Show air.
        let player_air = current_player().air;
        let air_bar_index = current_index().air_bar.unwrap();
        let objects = &mut *current_objects();
        let ref mut air_bar = objects.get_mut(air_bar_index).unwrap();
        match air_bar.data {
            object::BarData(i) => {
                current_bars()[i].value = player_air;
            },
            _ => {},
        }
    }

    show_air();

    fn follow_player(dt: f64) {
        // Make camera follow player.
        let follow_percentage = current_settings().camera_follow_percentage.unwrap();
        current_cam().follow_pos(dt, follow_percentage, player_pos());
    }

    follow_player(dt);
}

pub fn load() {
    current_cam().set(settings::INITIAL_CAMERA_POS);
    current_settings().camera_follow_percentage = Some(settings::CAMERA_FOLLOW_PERCENTAGE);
    *current_game_state() = settings::INITIAL_GAME_STATE;

    // Add player.
    current_objects().push(Object::bar_background());
    current_index().player = Some(0);

    fn add_bars() {
        let objects = &mut *current_objects();
        objects.push(Object::bar(
            settings::AIR_BAR_POS,
            "air",
            settings::AIR_BAR_TEXT_COLOR,
            settings::AIR_BAR_BACKGROUND_COLOR,
            settings::AIR_BAR_BAR_COLOR,
            settings::AIR_BAR_INITIAL_VALUE
        ));
        current_index().air_bar = Some(objects.len() - 1);
        objects.push(Object::bar(
            settings::BLOOD_BAR_POS,
            "blood",
            settings::BLOOD_BAR_TEXT_COLOR,
            settings::BLOOD_BAR_BACKGROUND_COLOR,
            settings::BLOOD_BAR_BAR_COLOR,
            settings::BLOOD_BAR_INITIAL_VALUE
        ));
        current_index().blood_bar = Some(objects.len() - 1);
    }

    // Add blood and air bar.
    add_bars();

    settings::level_1();
}

fn restart() {
    load();
}

pub fn key_press(key: keyboard::Key) {
    if *current_game_state() != game_state::Play { return; }

    match (key, current_index().player) {
        (keyboard::Right, Some(player_index)) => {
            current_objects()[player_index].move_right();
        },
        (keyboard::Up, Some(player_index)) => {
            current_objects()[player_index].move_up();
        },
        (keyboard::Left, Some(player_index)) => {
            current_objects()[player_index].move_left();
        },
        (keyboard::Down, Some(player_index)) => {
            current_objects()[player_index].move_down();
        },
        _ => {},
    }
}

pub fn key_release(key: keyboard::Key) {
    if key == keyboard::Return || key == keyboard::Space {
        match *current_game_state() {
            game_state::Win | game_state::Lose => restart(),
            _ => {},
        }
    }
}
