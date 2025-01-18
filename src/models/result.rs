use serde::Serialize;

#[derive(Serialize)]
pub struct SimulationResult {
    pub average_successful_attacks: f64,
    pub threshold_successful_attacks: usize,
}
