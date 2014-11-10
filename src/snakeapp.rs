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

pub fn current_app() -> Usage<'static, SnakeApp> { UseCurrent }
pub fn current_cam() -> Usage<'static, Cam> { UseCurrent }
pub fn current_game_state()
    -> Usage<'static, game_state::GameState> { UseCurrent }
pub fn current_objects() -> Usage<'static, Vec<Object>> { UseCurrent }
pub fn current_index() -> Usage<'static, Index> { UseCurrent }

pub fn app(f: ||) {
    use std::cell::RefCell;
    use current::Current;

    let app = SnakeApp::new();
    let cam = Cam([0.0, 0.0]);
    let game_state = game_state::Play;
    let objects: Vec<Object> = Vec::new();
    let index = Index::new();

    let app = RefCell::new(app);
    let cam = RefCell::new(cam);
    let game_state = RefCell::new(game_state);
    let objects = RefCell::new(objects);
    let index = RefCell::new(index);

    let app_guard = app.set_current();
    let cam_guard = cam.set_current();
    let game_state_guard = game_state.set_current();
    let objects_guard = objects.set_current();
    let index_guard = index.set_current();

    f();

    drop(index_guard);
    drop(objects_guard);
    drop(game_state_guard);
    drop(app_guard);
    drop(cam_guard);
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

pub struct SnakeApp {
    // Tells where the surface is.
    surface_y: Option<f64>,
    camera_follow_percentage: Option<f64>,
}

impl SnakeApp {
    pub fn render<B: BackEnd<I>, I: ImageSize>(
        &self, c: &Context, gl: &mut B
    ) {
        let c = &c.reset();

        // Get camera coordinates.
        let [cam_x, mut cam_y] = current_cam().pos();
        cam_y += 0.4;

        // Render surface.
        let surface_y = self.surface_y.unwrap();
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

    pub fn update(&mut self, dt: f64) {
        self.update_objects(dt);
        self.fill_air();
        self.win();
        self.lose();
        self.show_blood();
        self.show_air();
        self.follow_player(dt);
    }

    pub fn load(&mut self) {
        self.camera_follow_percentage = Some(settings::CAMERA_FOLLOW_PERCENTAGE);
        current_cam().set(settings::INITIAL_CAMERA_POS);
        self.surface_y = Some(settings::SURFACE_Y);
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

        // Add blood and air bar.
        self.add_bars();

        // Add air bottles.
        self.add_air_bottles();

        // Add snakes.
        self.add_snakes();
    }

    pub fn key_press(&mut self, key: keyboard::Key) {
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

    pub fn key_release(&mut self, key: keyboard::Key) {
        if key == keyboard::Return || key == keyboard::Space {
            match *current_game_state() {
                game_state::Win | game_state::Lose => self.restart(),
                _ => {},
            }
        }
    }

    pub fn new() -> SnakeApp {
        SnakeApp {
            camera_follow_percentage: None,
            surface_y: None,
        }
    }

    pub fn add_bars(&mut self) {
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

    pub fn add_snakes(&mut self) {
        if settings::SNAKE_1_ADD { current_objects().push(
            Object::snake(settings::SNAKE_1_POS, settings::SNAKE_1_SETTINGS)); }
        if settings::SNAKE_2_ADD { current_objects().push(
            Object::snake(settings::SNAKE_2_POS, settings::SNAKE_2_SETTINGS)); }
    }

    pub fn add_air_bottles(&mut self) {
        let air_bottles = settings::AIR_BOTTLE_POS;
        let n = air_bottles.len() / 2;
        let objects = &mut *current_objects();
        for i in range(0, n) {
            objects.push(Object::air_bottle([air_bottles[i * 2], air_bottles[i * 2 + 1]]));
        }
    }

    fn follow_player(&mut self, dt: f64) {
        // Make camera follow player.
        let camera_pos = current_cam().pos();
        let camera_follow_percentage = self.camera_follow_percentage.unwrap();
        let player_pos = self.player_pos();
        let (dx, dy) = (player_pos[0] - camera_pos[0], player_pos[1] - camera_pos[1]);
        let dx = camera_follow_percentage * dt * dx;
        let dy = camera_follow_percentage * dt * dy;
        current_cam().set([camera_pos[0] + dx, camera_pos[1] + dy]);
    }

    fn show_blood(&mut self) {
        if current_index().blood_bar == None { return; }
        // Show blood.
        let player_blood = self.player_blood();
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

    fn show_air(&mut self) {
        if current_index().air_bar == None { return; }
        // Show air.
        let player_air = self.player_air();
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

    fn win(&mut self) {
        let player_pos = self.player_pos();
        // When player reaches surface, win.
        if player_pos[1] >= self.surface_y.unwrap() {
            *current_game_state() = game_state::Win;
            return;
        }
    }

    fn lose(&mut self) {
        let blood = self.player_blood();
        let air = self.player_air();
        if blood < 0.0 || air < 0.0 {
            *current_game_state() = game_state::Lose;
            return;
        }
    }

    fn restart(&mut self) {
        *self = SnakeApp::new();
        self.load();
    }

    fn update_objects(&mut self, dt: f64) {
        if *current_game_state() != game_state::Play { return; }

        // Update states of objects.
        let player_pos = self.player_pos();
        let mut attack_damage: f64 = 0.0;
        for obj in current_objects().iter_mut() {
            match obj.update(dt, player_pos) {
                action::Passive => {},
                action::Attack(attack) => { attack_damage += attack; },
            }
        }
        // Bite player.
        self.bite_player(attack_damage);

        // Decrease the player's air with time.
        let air = self.player_air();
        self.set_player_air(air - dt * settings::PLAYER_LOSE_AIR_SPEED);
    }

    fn player_pos(&self) -> [f64, ..2] {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].pos
    }

    fn player_blood(&self) -> f64 {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].blood().unwrap()
    }

    fn player_air(&self) -> f64 {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].air().unwrap()
    }

    fn set_player_air(&mut self, val: f64) {
        let player_index = current_index().player.unwrap();
        *current_objects()[player_index].air_mut().unwrap() = val;
    }

    fn bite_player(&mut self, damage: f64) {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].player_mut().unwrap().bite(damage);
    }

    fn fill_air(&mut self) {
        let player_pos = self.player_pos();
        let mut air = self.player_air();
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
        self.set_player_air(air);
    }
}
