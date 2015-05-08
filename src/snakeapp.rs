// Extern crates.
use graphics::{ Context, Graphics };
use graphics;
use piston::input::keyboard;
use current::{ Current, CurrentGuard };
use start_piston;
use glium_graphics::GliumGraphics;

// Local crate.
use action;
use settings;
use object;
use object::Object;
use text as text_mod;
use game_state::GameState;
use player::{ Player };
use colors;
use snake::Snake;
use air_bottle::AirBottle;
use bar::Bar;

pub fn current_cam() -> Current<Cam> { unsafe { Current::new() } }
pub fn current_game_state() -> Current<GameState> { unsafe { Current::new() } }
pub fn current_objects() -> Current<Vec<Object>> { unsafe { Current::new() } }
pub fn current_index() -> Current<Index> { unsafe { Current::new() } }
pub fn current_settings() -> Current<Settings> { unsafe { Current::new() } }
pub fn current_player() -> Current<Player> { unsafe { Current::new() } }
pub fn current_snakes() -> Current<Vec<Snake>> { unsafe { Current::new() } }
pub fn current_air_bottles() -> Current<Vec<AirBottle>> { unsafe { Current::new() } }
pub fn current_bars() -> Current<Vec<Bar>> { unsafe { Current::new() } }

pub fn app() {
    let mut cam = Cam([0.0, 0.0]);
    let mut game_state = GameState::Play;
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

/// The graphics back-end to use for rendering.
#[derive(Debug)]
pub enum GraphicsBackEnd {
    /// Use Gfx to render
    Gfx,
    /// Use OpenGL to render
    OpenGL,
    /// Use Glium to render
    Glium,
}

fn start() {
    use piston::event::{ RenderEvent, UpdateEvent, PressEvent, ReleaseEvent };
    use piston::window::Window;

    let mut back_end = GraphicsBackEnd::Gfx;
    println!("Running with graphics backend {:?}", back_end);
    println!("Use 'S' to swap back-end");

    load();
    let mut iter = 0;
    let iter_end = 10000;
    let bench = false;
    if bench { println!("Benchmarking...") };
    for e in start_piston::events()
        .max_fps(120)
        .swap_buffers(false)
        .bench_mode(bench)
    {
        use piston::input::Key;
        use piston::input::Button::Keyboard;

        if bench {
            iter += 1;
            if iter > iter_end { break; }
        }

        if let Some(Keyboard(Key::S)) = e.press_args() {
            back_end = match back_end {
                GraphicsBackEnd::Gfx => {
                    println!("Swapped to OpenGL");
                    GraphicsBackEnd::OpenGL
                }
                GraphicsBackEnd::OpenGL => {
                    println!("Swapped to Glium");
                    GraphicsBackEnd::Glium
                }
                GraphicsBackEnd::Glium => {
                    println!("Swapped to Gfx");
                    GraphicsBackEnd::Gfx
                }
            };
        };
        e.render(|args| {
            match back_end {
                GraphicsBackEnd::Gfx => {
                    {
                        start_piston::render_2d_gfx(Some(settings::WATER_COLOR), |c, g| {
                            render(&c, g)
                        });
                    }
                    {
                        let window = start_piston::current_window();
                        let mut window = window.borrow_mut();
                        window.swap_buffers();
                    }
                }
                GraphicsBackEnd::OpenGL => {
                    {
                        start_piston::render_2d_opengl(Some(settings::WATER_COLOR), |c, g| {
                            render(&c, g)
                        });
                    }
                    {
                        let window = start_piston::current_window();
                        let mut window = window.borrow_mut();
                        window.swap_buffers();
                    }
                }
                GraphicsBackEnd::Glium => {
                    let glium_window = start_piston::current_glium_window();
                    let glium_window = glium_window.borrow();
                    let glium_2d = start_piston::current_glium_2d();
                    let mut glium_2d = glium_2d.borrow_mut();
                    let mut target = glium_window.draw();
                    {
                        let mut g = GliumGraphics::new(&mut *glium_2d, &mut target);
                        graphics::clear(settings::WATER_COLOR, &mut g);
                        let c = Context::new_viewport(args.viewport());
                        render(&c, &mut g);
                    }
                    target.finish();
                }
            };
            start_piston::set_title(start_piston::fps_tick().to_string());
        });
        e.update(|args| {
            update(args.dt);
        });
        if let Some(Keyboard(key)) = e.press_args() {
            key_press(key);
        }
        if let Some(Keyboard(key)) = e.release_args() {
            key_release(key);
        }
    }
}

pub struct Cam(pub [f64; 2]);

impl Cam {
    fn pos(&self) -> [f64; 2] {
        let Cam(pos) = *self;
        pos
    }

    fn set(&mut self, val: [f64; 2]) {
        *self = Cam(val);
    }

    /// Make camera follow position.
    fn follow_pos(&mut self, dt: f64, follow_percentage: f64, pos: [f64; 2]) {
        let camera_pos = self.pos();
        let (dx, dy) = (pos[0] - camera_pos[0], pos[1] - camera_pos[1]);
        let dx = follow_percentage * dt * dx;
        let dy = follow_percentage * dt * dy;
        self.set([camera_pos[0] + dx, camera_pos[1] + dy]);
    }
}

pub struct Index {
    pub player: Option<usize>,
    pub blood_bar: Option<usize>,
    pub air_bar: Option<usize>,
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

pub fn render<G: Graphics>(c: &Context, gl: &mut G) {
    use graphics::*;

    let c = &c.reset();

    // Get camera coordinates.
    let cam_pos = current_cam().pos();
    let cam_x = cam_pos[0];
    let mut cam_y = cam_pos[1];
    cam_y += 0.4;

    // Render surface.
    let surface_y = current_settings().surface_y.unwrap();
    graphics::Rectangle::new(colors::BLUE)
        .draw([-1.0, surface_y - cam_y, 2.0, 0.05],
            &c.draw_state, c.transform, gl);

    // Render objects in layers.
    let cam = &c.trans(-cam_x, -cam_y);
    for i in 0usize..settings::NUMBER_OF_LAYERS {
        for obj in current_objects().iter() {
            if obj.layer == i { obj.render(cam, c, gl); }
        }
    }

    let text_c = c.flip_v();
    let text_c = text_c.zoom(0.0025);
    match *current_game_state() {
        GameState::Win => {
            let pos = settings::YOU_WIN_POS;
            text_mod::text(settings::YOU_WIN_TEXT,
                &graphics::Polygon::new(settings::YOU_WIN_TEXT_COLOR),
                &text_c
                .trans(pos[0], pos[1])
            , gl);
        },
        GameState::Lose => {
            let pos = settings::YOU_LOSE_POS;
            text_mod::text(settings::YOU_LOSE_TEXT,
                &graphics::Polygon::new(settings::YOU_LOSE_TEXT_COLOR),
                &text_c
                .trans(pos[0], pos[1])
            , gl);
        },
        GameState::Play => {

        },
    }
}

pub fn update(dt: f64) {

    fn player_pos() -> [f64; 2] {
        let player_index = current_index().player.unwrap();
        current_objects()[player_index].pos
    }

    fn update_objects(dt: f64) {
        if *current_game_state() != GameState::Play { return; }

        current_player().update(dt);

        // Update states of objects.
        let player_pos = player_pos();
        let mut attack_damage: f64 = 0.0;
        for obj in current_objects().iter_mut() {
            match obj.update(dt, player_pos) {
                action::Action::Passive => {},
                action::Action::Attack(attack) => { attack_damage += attack; },
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
                object::Data::AirBottle(i) => {
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
            *current_game_state() = GameState::Win;
            return;
        }
    }

    win();

    fn lose() {
        let player = &*current_player();
        let blood = player.blood;
        let air = player.air;
        if blood < 0.0 || air < 0.0 {
            *current_game_state() = GameState::Lose;
            return;
        }
    }

    lose();

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
    current_objects().clear();
    current_objects().push(Object::bar_background());
    current_index().player = Some(0);

    fn add_bars() {
        fn air() -> f64 { current_player().air }
        fn blood() -> f64 { current_player().blood }

        let objects = &mut *current_objects();
        objects.push(Object::bar(
            settings::AIR_BAR_POS,
            "air",
            settings::AIR_BAR_TEXT_COLOR,
            settings::AIR_BAR_BACKGROUND_COLOR,
            settings::AIR_BAR_BAR_COLOR,
            air
        ));
        current_index().air_bar = Some(objects.len() - 1);
        objects.push(Object::bar(
            settings::BLOOD_BAR_POS,
            "blood",
            settings::BLOOD_BAR_TEXT_COLOR,
            settings::BLOOD_BAR_BACKGROUND_COLOR,
            settings::BLOOD_BAR_BAR_COLOR,
            blood
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
    use piston::input::Key;

    if *current_game_state() != GameState::Play { return; }

    match (key, current_index().player) {
        (Key::Right, Some(player_index)) => {
            current_objects()[player_index].move_right();
        },
        (Key::Up, Some(player_index)) => {
            current_objects()[player_index].move_up();
        },
        (Key::Left, Some(player_index)) => {
            current_objects()[player_index].move_left();
        },
        (Key::Down, Some(player_index)) => {
            current_objects()[player_index].move_down();
        },
        _ => {},
    }
}

pub fn key_release(key: keyboard::Key) {
    use piston::input::Key;

    if key == Key::Return || key == Key::Space {
        match *current_game_state() {
            GameState::Win | GameState::Lose => restart(),
            _ => {},
        }
    }
}
