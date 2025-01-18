use crate::logic::dice::roll_d6;

pub fn monte_carlo_simulation(
    num_attacks: usize,
    min_to_hit: u8,
    min_to_wound: u8,
    min_to_save: u8,
    threshold_percent: f64,
    num_iterations: usize,
) -> (f64, usize) {
    let mut successful_attacks_count = vec![0; num_iterations];

    for i in 0..num_iterations {
        let mut successful_attacks = 0;

        for _ in 0..num_attacks {
            let hit_roll = roll_d6();
            if hit_roll >= min_to_hit {
                let wound_roll = roll_d6();
                if wound_roll >= min_to_wound {
                    let save_roll = roll_d6();
                    if save_roll < min_to_save {
                        successful_attacks += 1;
                    }
                }
            }
        }

        successful_attacks_count[i] = successful_attacks;
    }

    let average =
        successful_attacks_count.iter().copied().sum::<usize>() as f64 / num_iterations as f64;

    successful_attacks_count.sort_unstable();

    let threshold_index = ((num_iterations as f64 * (1.0 - threshold_percent / 100.0)).ceil()
        as usize)
        .saturating_sub(1);
    let threshold = successful_attacks_count[threshold_index];

    (average, threshold)
}
