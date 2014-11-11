use settings;

pub enum PlayerState {
    Normal,
    Bitten(f64),
}

pub struct Player {
    pub blood: f64,
    pub air: f64,
    pub tween_factor: f64,
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Player {
        Player {
            blood: 0.0,
            air: 0.0,
            tween_factor: 0.0,
            state: Normal,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.tween_factor += dt * settings::PLAYER_TWEEN_SPEED;
        self.state = match self.state {
            Normal => Normal,
            Bitten(sec) => {
                if sec - dt < 0.0 {
                    Normal
                } else {
                    Bitten(sec - dt)
                }
            }
        }
    }

    pub fn bite(&mut self, damage: f64) {
        if damage > 0.0 {
            self.state = Bitten(settings::PLAYER_BITTEN_FADE_OUT_SECONDS);
            self.blood -= damage;
        }
    }
}
