
pub enum SharkState {
    Ignorant,
    ChasingPlayer,
    Dead,
}

pub struct Shark {
    pub sensor_distance: f64,
    pub state: SharkState,
}

