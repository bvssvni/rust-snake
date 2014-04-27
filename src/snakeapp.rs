use action;
use settings;
use glfw;
use Settings = piston::game::Settings;
use Game = piston::game::Game;
use piston::gl::Gl;
use graphics;
use graphics::*;
use object;
use Object = object::Object;
use text;
use game_state;

pub struct SnakeApp {
    settings: Settings,
    // Tells where the surface is.
    surface_y: Option<f64>,
    game_state: Option<game_state::GameState>,
    camera_pos: Option<[f64, ..2]>,
    camera_follow_percentage: Option<f64>,
    player_index: Option<uint>,
    blood_bar_index: Option<uint>,
    // Contains the game objects.
    objects: Vec<Object>,
}

impl Game for SnakeApp {
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    
    fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        // Get camera coordinates.
        let (cam_x, cam_y) = if self.camera_pos.is_some() {
                let camera_pos = self.camera_pos.unwrap();
                (camera_pos[0], camera_pos[1])
            } else { (0.0, 0.0) };
        
        // Render surface.
        let surface_y = self.surface_y.unwrap();
        c.rect(-1.0, surface_y - cam_y, 2.0, 0.05).color(settings::BLUE).fill(gl);

        // Render round rectangle around bars.
        let bar_bgh = settings::BAR_BACKGROUND_HEIGHT;
        let bar_color = settings::BAR_BACKGROUND_COLOR;
        c.rect(-1.0, 1.0 - bar_bgh, 2.0, bar_bgh).round(0.1).color(bar_color).fill(gl);

        // Render objects.
        let cam = &c.trans(-cam_x, -cam_y);
        for obj in self.objects.iter() {
            obj.render(cam, c, gl);
        }
  
        let text_c = c.flip_v_local();
        let text_c = text_c.zoom(0.0025);
        match self.game_state.unwrap() {
            game_state::Win => {
                let pos = settings::YOU_WIN_POS;
                text::text("you win", 
                    &text_c
                    .trans(pos[0], pos[1])
                    .color(settings::YOU_WIN_TEXT_COLOR)
                , gl);
            },
            game_state::Loose => {
                let pos = settings::YOU_LOOSE_POS;
                text::text("you loose",
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

    fn update(&mut self, dt: f64) {
        self.update_objects(dt);
        self.win();
        self.loose();
        self.show_blood(); 
        self.follow_player(dt); 
    }

    fn load(&mut self) {
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
            [settings::PLAYER_SPEED_LEFT, settings::PLAYER_SPEED_RIGHT],
            [settings::PLAYER_SPEED_UP, settings::PLAYER_SPEED_DOWN]
        ));
        self.player_index = Some(0);
        
        // Add blood and air bar.
        self.add_bars();
 
        // Add sharks.
        self.add_sharks();
    }

    fn key_press(&mut self, key: glfw::Key) {
        // TEST
        // println!("Key pressed {}", key);

        if self.game_state.unwrap() != game_state::Play { return; }

        match (key, self.player_index) {
            (glfw::KeyRight, Some(player_index)) => {
                self.objects.get_mut(player_index).move_right();
            },
            (glfw::KeyUp, Some(player_index)) => {
                self.objects.get_mut(player_index).move_up();
            },
            (glfw::KeyLeft, Some(player_index)) => {
                self.objects.get_mut(player_index).move_left();
            },
            (glfw::KeyDown, Some(player_index)) => {
                self.objects.get_mut(player_index).move_down();
            },
            _ => {},
        }
    }

    fn key_release(&mut self, key: glfw::Key) {
        // TEST
        // println!("Key released {}", key);
    
        if key == glfw::KeyEnter || key == glfw::KeySpace {
            match self.game_state.unwrap() {
                game_state::Win | game_state::Loose => self.restart(),
                _ => {},
            }
        }
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp { 
        let exit_on_esc = true;
        let background_color = [1.0, 1.0, 1.0, 1.0];
        SnakeApp {
            camera_pos: None,
            camera_follow_percentage: None,
            settings: Settings::new(exit_on_esc, background_color),
            game_state: None,
            surface_y: None,
            objects: Vec::new(),
            player_index: None,
            blood_bar_index: None,
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

    pub fn add_sharks(&mut self) {
        self.objects.push(Object::shark(settings::SHARK_1_POS, settings::SHARK_1_SETTINGS));
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

    fn restart(&mut self) {
        *self = SnakeApp::new();
        self.load();
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
        // Decrease the players life with attacks.
        let blood = self.player_blood();
        self.set_player_blood(blood - attack_damage);   
    }
    
    fn player_pos(&self) -> [f64, ..2] {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).pos
    }

    fn player_blood(&self) -> f64 {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).blood().unwrap()
    }

    fn set_player_blood(&mut self, val: f64) {
        let player_index = self.player_index.unwrap();
        *self.objects.get_mut(player_index).blood_mut().unwrap() = val;
    }

    fn player_air(&self) -> f64 {
        let player_index = self.player_index.unwrap();
        self.objects.get(player_index).air().unwrap()
    }

    fn set_player_air(&mut self, val: f64) {
        let player_index = self.player_index.unwrap();
        *self.objects.get_mut(player_index).air_mut().unwrap() = val;
    }
}

