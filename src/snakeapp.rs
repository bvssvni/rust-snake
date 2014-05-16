// Extern crates.
use piston::*;
use graphics::*;

// Local crate.
use action;
use settings;
use object;
use Object = object::Object;
use text;
use game_state;

pub struct SnakeApp {
    // Tells where the surface is.
    surface_y: Option<f64>,
    game_state: Option<game_state::GameState>,
    camera_pos: Option<[f64, ..2]>,
    camera_follow_percentage: Option<f64>,
    player_index: Option<uint>,
    blood_bar_index: Option<uint>,
    air_bar_index: Option<uint>,
    // Contains the game objects.
    objects: Vec<Object>,
}

impl Game for SnakeApp {
    fn render(&self, c: &Context, gl: &mut Gl) {
        // Get camera coordinates.
        let (cam_x, cam_y) = if self.camera_pos.is_some() {
                let camera_pos = self.camera_pos.unwrap();
                (camera_pos[0], camera_pos[1] + 0.4)
            } else { (0.0, 0.0) };

        // Render surface.
        let surface_y = self.surface_y.unwrap();
        c.rect(-1.0, surface_y - cam_y, 2.0, 0.05).color(settings::BLUE).fill(gl);

        // Render objects in layers.
        let cam = &c.trans(-cam_x, -cam_y);
        for i in range(0u, settings::NUMBER_OF_LAYERS) {
            for obj in self.objects.iter() {
                if obj.layer == i { obj.render(cam, c, gl); }
            }
        }

        let text_c = c.flip_v_local();
        let text_c = text_c.zoom(0.0025);
        match self.game_state.unwrap() {
            game_state::Win => {
                let pos = settings::YOU_WIN_POS;
                text::text(settings::YOU_WIN_TEXT,
                    &text_c
                    .trans(pos[0], pos[1])
                    .color(settings::YOU_WIN_TEXT_COLOR)
                , gl);
            },
            game_state::Loose => {
                let pos = settings::YOU_LOOSE_POS;
                text::text(settings::YOU_LOOSE_TEXT,
                    &text_c
                    .trans(pos[0], pos[1])
                    .color(settings::YOU_LOOSE_TEXT_COLOR)
                , gl);
            },
            game_state::Play => {

            },
        }

        // TEST
        // text::text("restart", &c.flip_v_local().zoom(0.001).color(settings::BLACK), gl);
    }

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        self.update_objects(dt);
        self.fill_air();
        self.win();
        self.loose();
        self.show_blood();
        self.show_air();
        self.follow_player(dt);
    }

    fn load(&mut self, _asset_store: &mut AssetStore) {
        self.camera_follow_percentage = Some(settings::CAMERA_FOLLOW_PERCENTAGE);
        self.camera_pos = Some(settings::INITIAL_CAMERA_POS);
        self.surface_y = Some(settings::SURFACE_Y);
        self.game_state = Some(settings::INITIAL_GAME_STATE);

        // Add player.
        self.objects.push(Object::player(
            settings::ORIGIN,
            settings::BLUE,
            settings::PLAYER_INITIAL_BLOOD,
            settings::PLAYER_INITIAL_AIR,
            [settings::PLAYER_ACCELERATION_LEFT, settings::PLAYER_ACCELERATION_RIGHT],
            [settings::PLAYER_ACCELERATION_UP, settings::PLAYER_ACCELERATION_DOWN]
        ));
        self.objects.push(Object::bar_background());
        self.player_index = Some(0);

        // Add blood and air bar.
        self.add_bars();

        // Add air bottles.
        self.add_air_bottles();

        // Add snakes.
        self.add_snakes();
    }

    fn key_press(&mut self, key: keyboard::Key, _asset_store: &mut AssetStore) {
        // TEST
        // println!("Key pressed {}", key);

        if self.game_state.unwrap() != game_state::Play { return; }

        match (key, self.player_index) {
            (keyboard::Right, Some(player_index)) => {
                self.objects.get_mut(player_index).move_right();
            },
            (keyboard::Up, Some(player_index)) => {
                self.objects.get_mut(player_index).move_up();
            },
            (keyboard::Left, Some(player_index)) => {
                self.objects.get_mut(player_index).move_left();
            },
            (keyboard::Down, Some(player_index)) => {
                self.objects.get_mut(player_index).move_down();
            },
            _ => {},
        }
    }

    fn key_release(&mut self, key: keyboard::Key, asset_store: &mut AssetStore) {
        // TEST
        // println!("Key released {}", key);

        if key == keyboard::Enter || key == keyboard::Space {
            match self.game_state.unwrap() {
                game_state::Win | game_state::Loose => self.restart(asset_store),
                _ => {},
            }
        }
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp {
        SnakeApp {
            camera_pos: None,
            camera_follow_percentage: None,
            game_state: None,
            surface_y: None,
            objects: Vec::new(),
            player_index: None,
            blood_bar_index: None,
            air_bar_index: None,
        }
    }

    pub fn add_bars(&mut self) {
        self.objects.push(Object::bar(
            settings::AIR_BAR_POS,
            "air",
            settings::AIR_BAR_TEXT_COLOR,
            settings::AIR_BAR_BACKGROUND_COLOR,
            settings::AIR_BAR_BAR_COLOR,
            settings::AIR_BAR_INITIAL_VALUE
        ));
        self.air_bar_index = Some(self.objects.len() - 1);
        self.objects.push(Object::bar(
            settings::BLOOD_BAR_POS,
            "blood",
            settings::BLOOD_BAR_TEXT_COLOR,
            settings::BLOOD_BAR_BACKGROUND_COLOR,
            settings::BLOOD_BAR_BAR_COLOR,
            settings::BLOOD_BAR_INITIAL_VALUE
        ));
        self.blood_bar_index = Some(self.objects.len() - 1);
    }

    pub fn add_snakes(&mut self) {
        if settings::SNAKE_1_ADD { self.objects.push(Object::snake(settings::SNAKE_1_POS, settings::SNAKE_1_SETTINGS)); }
        if settings::SNAKE_2_ADD { self.objects.push(Object::snake(settings::SNAKE_2_POS, settings::SNAKE_2_SETTINGS)); }
    }

    pub fn add_air_bottles(&mut self) {
        let air_bottles = settings::AIR_BOTTLE_POS;
        let n = air_bottles.len() / 2;
        for i in range(0, n) {
            self.objects.push(Object::air_bottle([air_bottles[i * 2], air_bottles[i * 2 + 1]]));
        }
    }

    fn follow_player(&mut self, dt: f64) {
        if self.camera_pos.is_none() { return; }
        // Make camera follow player.
        let camera_pos = self.camera_pos.unwrap();
        let camera_follow_percentage = self.camera_follow_percentage.unwrap();
        let player_pos = self.player_pos();
        let (dx, dy) = (player_pos[0] - camera_pos[0], player_pos[1] - camera_pos[1]);
        let dx = camera_follow_percentage * dt * dx;
        let dy = camera_follow_percentage * dt * dy;
        self.camera_pos = Some([camera_pos[0] + dx, camera_pos[1] + dy]);
    }

    fn show_blood(&mut self) {
        if self.blood_bar_index == None { return; }
        // Show blood.
        let player_blood = self.player_blood();
        let blood_bar_index = self.blood_bar_index.unwrap();
        let blood_bar = self.objects.get_mut(blood_bar_index);
        match blood_bar.data {
            object::BarData(ref mut bar) => {
                bar.value = player_blood;
            },
            _ => {},
        }
    }

    fn show_air(&mut self) {
        if self.air_bar_index == None { return; }
        // Show air.
        let player_air = self.player_air();
        let air_bar_index = self.air_bar_index.unwrap();
        let air_bar = self.objects.get_mut(air_bar_index);
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
            self.game_state = Some(game_state::Win);
            return;
        }
    }

    fn loose(&mut self) {
        let blood = self.player_blood();
        let air = self.player_air();
        if blood < 0.0 || air < 0.0 {
            self.game_state = Some(game_state::Loose);
            return;
        }
    }

    fn restart(&mut self, asset_store: &mut AssetStore) {
        *self = SnakeApp::new();
        self.load(asset_store);
    }

    fn update_objects(&mut self, dt: f64) {
        if self.game_state.unwrap() != game_state::Play { return; }

        // Update states of objects.
        let player_pos = self.player_pos();
        let mut attack_damage: f64 = 0.0;
        for obj in self.objects.mut_iter() {
            match obj.update(dt, player_pos) {
                action::Passive => {},
                action::Attack(attack) => { attack_damage += attack; },
            }
        }
        // Bite player.
        self.bite_player(attack_damage);

        // Decrease the player's air with time.
        let air = self.player_air();
        self.set_player_air(air - dt * settings::PLAYER_LOOSE_AIR_SPEED);
    }

    fn player_pos(&self) -> [f64, ..2] {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).pos
    }

    fn player_blood(&self) -> f64 {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).blood().unwrap()
    }

    fn player_air(&self) -> f64 {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).air().unwrap()
    }

    fn set_player_air(&mut self, val: f64) {
        let player_index = self.player_index.unwrap();
        *self.objects.get_mut(player_index).air_mut().unwrap() = val;
    }

    fn bite_player(&mut self, damage: f64) {
        let player_index = self.player_index.unwrap();
        self.objects.get_mut(player_index).player_mut().unwrap().bite(damage);
    }

    fn fill_air(&mut self) {
        let player_pos = self.player_pos();
        let mut air = self.player_air();
        for obj in self.objects.mut_iter() {
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

