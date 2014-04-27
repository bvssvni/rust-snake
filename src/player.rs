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
    pub fn update(&mut self, dt: f64) {
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

    pub fn bitten(&mut self) {
        self.state = Bitten(settings::PLAYER_BITTEN_FADE_OUT_SECONDS);
    }
}


