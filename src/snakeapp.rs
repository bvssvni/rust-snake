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
    player_index: Option<uint>,
    blood_bar_index: Option<uint>,
    // Contains the game objects.
    objects: Vec<Object>,
}

impl Game for SnakeApp {
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    
    fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        for obj in self.objects.iter() {
            obj.render(c, gl);
        }
   
        // TEST 
        text::text("restart", &c.flip_v_local().zoom(0.001).color(settings::BLACK), gl); 
    }
    
    fn update(&mut self, dt: f64) {
        if self.player_index == None { return; }        

        // Update states of objects.
        let player_index = self.player_index.unwrap();
        let player_pos = self.objects.get(player_index).pos;
        let mut attack_damage: f64 = 0.0;
        for obj in self.objects.mut_iter() {
            match obj.update(dt, player_pos) {
                action::Passive => {},
                action::Attack(attack) => { attack_damage += attack; },
            }
        }

        // When player reaches surface, win.
        if player_pos[1] >= self.surface_y.unwrap() {
            self.game_state = Some(game_state::Win);
            return;
        }

        // Decrease the players life with attacks.
        *self.objects.get_mut(player_index).blood_mut().unwrap() -= attack_damage;   
 
        if self.blood_bar_index == None { return; }

        // Show blood.
        let player_blood = self.objects.get(player_index).blood().unwrap();
        let blood_bar_index = self.blood_bar_index.unwrap();
        let blood_bar = self.objects.get_mut(blood_bar_index);
        match blood_bar.data {
            object::BarData(ref mut bar) => {
                bar.value = player_blood;
            },
            _ => {},
        }
    }

    fn load(&mut self) {
        self.surface_y = Some(settings::SURFACE_Y);
        self.game_state = Some(settings::INITIAL_GAME_STATE);    
    
        // Add player.
        self.objects.push(Object::player(
            settings::ORIGIN, 
            settings::BLUE,
            settings::PLAYER_INITIAL_BLOOD,
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
    }
}

impl SnakeApp {
    pub fn new() -> SnakeApp { 
        let exit_on_esc = true;
        let background_color = [1.0, 1.0, 1.0, 1.0];
        SnakeApp {
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
}

