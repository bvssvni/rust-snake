use player::Player;

pub enum SharkState {
    Ignorant,
    ChasingPlayer,
    WaitForAttack(f64),
    Dead,
}

pub enum SharkAction {
    Passive,
    Attack(f64),
}

pub struct Shark {
    pub sensor_distance: f64,
    pub state: SharkState,
    pub bite_damage: f64,
    pub attack_distance: f64,
    pub wait_seconds_before_initial_attack: f64,
    pub wait_seconds_before_repeat_attack: f64,
}

impl Shark {
    pub fn update(
        &mut self, dt: f64,  
        player_pos: [f64, ..2], 
        shark_pos: [f64, ..2]) -> SharkAction {

        let (dx, dy) = (player_pos[0] - shark_pos[0], player_pos[1] - shark_pos[1]);
        let d = (dx * dx + dy * dy).sqrt();
        
        let mut action = Passive;
        self.state = match self.state {
            Ignorant => if d < self.sensor_distance { ChasingPlayer } else { Ignorant },
            ChasingPlayer =>
                if d < self.attack_distance { WaitForAttack(self.wait_seconds_before_initial_attack) }
                else { ChasingPlayer },
            WaitForAttack(seconds) =>
                if seconds - dt <= 0.0 {
                     action = Attack(self.bite_damage);
                    WaitForAttack(self.wait_seconds_before_repeat_attack) 
                } else { WaitForAttack(seconds - dt) },
            Dead => Dead,
        };
        action
    }
}

