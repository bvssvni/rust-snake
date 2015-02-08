
use settings;
use action::Action;

#[derive(Copy)]
pub enum SnakeState {
    Ignorant,
    ChasingPlayer,
    WaitForAttack(f64),
    Dead,
}

pub struct Snake {
    pub sensor_distance: f64,
    pub state: SnakeState,
    pub bite_damage: f64,
    pub attack_distance: f64,
    pub wait_seconds_before_initial_attack: f64,
    pub wait_seconds_before_repeat_attack: f64,
    pub tail: Vec<f64>,
}

impl Snake {
    pub fn update(
        &mut self, dt: f64,
        player_pos: [f64; 2],
        snake_pos: [f64; 2]
    ) -> Action {
        use std::num::Float;

        self.simulate_tail(snake_pos[0], snake_pos[1], dt);
        let (dx, dy) = (player_pos[0] - snake_pos[0], player_pos[1] - snake_pos[1]);
        let d = (dx * dx + dy * dy).sqrt();

        let mut action = Action::Passive;
        self.state = match self.state {
            SnakeState::Ignorant =>
                if d < self.sensor_distance {
                    SnakeState::ChasingPlayer
                } else {
                    SnakeState::Ignorant
                },
            SnakeState::ChasingPlayer =>
                if d < self.attack_distance {
                    SnakeState::WaitForAttack(self.wait_seconds_before_initial_attack)
                } else {
                    SnakeState::ChasingPlayer
                },
            SnakeState::WaitForAttack(seconds) =>
                if seconds - dt <= 0.0 {
                    if d >= self.attack_distance { SnakeState::ChasingPlayer }
                    else {
                        action = Action::Attack(self.bite_damage);
                        SnakeState::WaitForAttack(self.wait_seconds_before_repeat_attack)
                    }
                } else { SnakeState::WaitForAttack(seconds - dt) },
            SnakeState::Dead => SnakeState::Dead,
        };
        action
    }

    pub fn simulate_tail(&mut self, x: f64, y: f64, _dt: f64) {
        use std::num::Float;

        let mut x = x;
        let mut y = y;
        let dist = settings::SNAKE_TAIL_DISTANCE;
        let n = self.tail.len() / 2;
        for i in range(0, n) {
            let x2 = self.tail[i * 2];
            let y2 = self.tail[i * 2 + 1];
            let dx = x - x2;
            let dy = y - y2;
            let d = dx * dx + dy * dy;
            if d > dist * dist {
                let d = d.sqrt();
                let dx = dx / d;
                let dy = dy / d;
                let dx = dx * (dist - d);
                let dy = dy * (dist - d);
                self.tail[i * 2] -= dx;
                self.tail[i * 2 + 1] -= dy;
            }

            x = x2;
            y = y2;
        }
    }
}
