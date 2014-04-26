
pub enum SharkState {
    Ignorant,
    ChasingPlayer,
    Dead,
}

pub struct Shark {
    pub sensor_distance: f64,
    pub state: SharkState,
}

impl Shark {
    pub fn update(&mut self, player_pos: [f64, ..2], shark_pos: [f64, ..2]) {
        let (dx, dy) = (player_pos[0] - shark_pos[0], player_pos[1] - shark_pos[1]);
        let d = (dx * dx + dy * dy).sqrt();
        if d < self.sensor_distance {
            self.state = ChasingPlayer;
        }
    }
}
