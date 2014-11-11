// Extern crates.
use graphics::*;
use input::keyboard;
use current::{ Usage, UseCurrent };

// Local crate.
use action;
use settings;
use object;
use object::Object;
use text;
use game_state;

pub fn current_cam() -> Usage<'static, Cam> { UseCurrent }
pub fn current_game_state()
    -> Usage<'static, game_state::GameState> { UseCurrent }
pub fn current_objects() -> Usage<'static, Vec<Object>> { UseCurrent }
pub fn current_index() -> Usage<'static, Index> { UseCurrent }
pub fn current_settings() -> Usage<'static, Settings> { UseCurrent }

pub fn app() {
    use std::cell::RefCell;
    use current::Current;

    let cam = Cam([0.0, 0.0]);
    let game_state = game_state::Play;
    let objects: Vec<Object> = Vec::new();
    let index = Index::new();
    let settings = Settings::new();

    let cam = RefCell::new(cam);
    let game_state = RefCell::new(game_state);
    let objects = RefCell::new(objects);
    let index = RefCell::new(index);
    let settings = RefCell::new(settings);

    let cam_guard = cam.set_current();
    let game_state_guard = game_state.set_current();
    let objects_guard = objects.set_current();
    let index_guard = index.set_current();
    let settings_guard = settings.set_current();

    start();

    drop(cam_guard);
    drop(game_state_guard);
    drop(objects_guard);
    drop(index_guard);
    drop(settings_guard);
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
    c.rect(-1.0, surface_y - cam_y, 2.0, 0.05).color(settings::BLUE).draw(gl);

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

    fn player_air() -> f64 {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].air().unwrap()
    }

    fn set_player_air(val: f64) {
        let player_index = current_index().player.unwrap();
        *current_objects()[player_index].air_mut().unwrap() = val;
    }

    fn bite_player(damage: f64) {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].player_mut().unwrap().bite(damage);
    }

    fn player_blood() -> f64 {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].blood().unwrap()
    }

    fn player_pos() -> [f64, ..2] {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].pos
    }

    fn update_objects(dt: f64) {
        if *current_game_state() != game_state::Play { return; }

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
        bite_player(attack_damage);

        // Decrease the player's air with time.
        let air = player_air();
        set_player_air(air - dt * settings::PLAYER_LOSE_AIR_SPEED);
    }

    update_objects(dt);

    fn fill_air() {
        let player_pos = player_pos();
        let mut air = player_air();
        for obj in current_objects().iter_mut() {
            let pos = obj.pos;
            match obj.air_bottle_mut() {
                None => {},
                Some(air_bottle) => {
                    let dx = pos[0] - player_pos[0];
                    let dy = pos[1] - player_pos[1];
                    let d = dx * dx + dy * dy;
                    if d <= settings::AIR_BOTTLE_RADIUS * settings::AIR_BOTTLE_RADIUS {
                        air += air_bottle.fill_up;
                        air_bottle.fill_up = 0.0;
                    }
                },
            }
        }

        air = if air > 1.0 { 1.0 } else { air };
        set_player_air(air);
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
        let blood = player_blood();
        let air = player_air();
        if blood < 0.0 || air < 0.0 {
            *current_game_state() = game_state::Lose;
            return;
        }
    }

    lose();

    fn show_blood() {
        if current_index().blood_bar == None { return; }
        // Show blood.
        let player_blood = player_blood();
        let blood_bar_index = current_index().blood_bar.unwrap();
        let objects = &mut *current_objects();
        let blood_bar = objects.get_mut(blood_bar_index).unwrap();
        match blood_bar.data {
            object::BarData(ref mut bar) => {
                bar.value = player_blood;
            },
            _ => {},
        }
    }

    show_blood();

    fn show_air() {
        if current_index().air_bar == None { return; }
        // Show air.
        let player_air = player_air();
        let air_bar_index = current_index().air_bar.unwrap();
        let objects = &mut *current_objects();
        let ref mut air_bar = objects.get_mut(air_bar_index).unwrap();
        match air_bar.data {
            object::BarData(ref mut bar) => {
                bar.value = player_air;
            },
            _ => {},
        }
    }

    show_air();

    fn follow_player(dt: f64) {
        // Make camera follow player.
        let camera_pos = current_cam().pos();
        let camera_follow_percentage = current_settings().camera_follow_percentage.unwrap();
        let player_pos = player_pos();
        let (dx, dy) = (player_pos[0] - camera_pos[0], player_pos[1] - camera_pos[1]);
        let dx = camera_follow_percentage * dt * dx;
        let dy = camera_follow_percentage * dt * dy;
        current_cam().set([camera_pos[0] + dx, camera_pos[1] + dy]);
    }

    follow_player(dt);
}

pub fn load() {
    current_cam().set(settings::INITIAL_CAMERA_POS);
    current_settings().camera_follow_percentage = Some(settings::CAMERA_FOLLOW_PERCENTAGE);
    *current_game_state() = settings::INITIAL_GAME_STATE;

    // Add player.
    current_objects().push(Object::player(
        settings::ORIGIN,
        settings::BLUE,
        settings::PLAYER_INITIAL_BLOOD,
        settings::PLAYER_INITIAL_AIR,
        [settings::PLAYER_ACCELERATION_LEFT, settings::PLAYER_ACCELERATION_RIGHT],
        [settings::PLAYER_ACCELERATION_UP, settings::PLAYER_ACCELERATION_DOWN]
    ));
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
