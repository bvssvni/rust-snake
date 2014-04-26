use player::Player;

pub enum SharkState {
    Ignorant,
    ChasingPlayer,
    WaitForAttack(f64),
    Dead,
}

pub struct Shark {
    pub sensor_distance: f64,
    pub state: SharkState,
    pub bite_damage: f64,
    pub wait_seconds_before_initial_attack: f64,
}

impl Shark {
    pub fn update(&mut self, player_pos: [f64, ..2], shark_pos: [f64, ..2]) {
        let (dx, dy) = (player_pos[0] - shark_pos[0], player_pos[1] - shark_pos[1]);
        let d = (dx * dx + dy * dy).sqrt();
        if d < self.sensor_distance {
            self.state = ChasingPlayer;
        }
    }

    pub fn attack(&mut self, player: &mut Player) {
        player.blood -= self.bite_damage;
    }
}

