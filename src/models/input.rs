use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimulationInput {
    pub num_attacks: usize,
    pub min_to_hit: u8,
    pub min_to_wound: u8,
    pub min_to_save: u8,
    pub threshold_percent: f64,
}
