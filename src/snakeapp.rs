use settings;
use glfw;
use Settings = piston::game::Settings;
use Game = piston::game::Game;
use piston::gl::Gl;
use graphics;
use graphics::*;
use Object = object::Object;
use text;

pub struct SnakeApp {
    settings: Settings,
    player_index: Option<uint>,
    // Contains the game objects.
    objects: Vec<Object>,
}

impl Game for SnakeApp {
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }
    
    fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        for obj in self.objects.iter() {
            obj.render(c, gl);
        }
    }
    
    fn update(&mut self, dt: f64) {
        for obj in self.objects.mut_iter() {
            obj.update(dt);
        }
    }

    fn load(&mut self) {
        self.add_sharks();
        self.objects.push(Object::player(settings::ORIGIN, settings::BLUE));
        self.player_index = Some(1);
        self.add_bars();
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
            objects: Vec::new(),
            player_index: None,
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
    }

    pub fn add_sharks(&mut self) {
        self.objects.push(Object::shark(
            settings::SHARK_1_POS, 
            settings::SHARK_1_TEST_COLOR, 
            settings::SHARK_1_SENSOR_DISTANCE
        ));
    }
}

