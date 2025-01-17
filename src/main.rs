use rand::Rng;
use std::io;

fn roll_d6() -> u8 {
    rand::thread_rng().gen_range(1..=6)
}
fn monte_carlo_simulation(
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

    // Sort the results in ascending order to compute the worst-case for the given threshold percentage
    successful_attacks_count.sort_unstable();

    let threshold_index = ((num_iterations as f64 * (1.0 - threshold_percent / 100.0)).ceil()
        as usize)
        .saturating_sub(1);
    let threshold = successful_attacks_count[threshold_index];

    (average, threshold)
}

fn main() {
    let mut input = String::new();

    println!("Enter the number of attacks:");
    io::stdin().read_line(&mut input).unwrap();
    let num_attacks: usize = input.trim().parse().expect("Please enter a valid number.");

    input.clear();
    println!("Enter the minimum roll to hit (2-6):");
    io::stdin().read_line(&mut input).unwrap();
    let min_to_hit: u8 = input
        .trim()
        .parse()
        .expect("Please enter a valid number between 2 and 6.");

    input.clear();
    println!("Enter the minimum roll to wound (2-6):");
    io::stdin().read_line(&mut input).unwrap();
    let min_to_wound: u8 = input
        .trim()
        .parse()
        .expect("Please enter a valid number between 2 and 6.");

    input.clear();
    println!("Enter the minimum roll to save (2-6):");
    io::stdin().read_line(&mut input).unwrap();
    let min_to_save: u8 = input
        .trim()
        .parse()
        .expect("Please enter a valid number between 2 and 6.");

    input.clear();
    println!("Enter the threshold percentage (e.g., 80 for 80%):");
    io::stdin().read_line(&mut input).unwrap();
    let threshold_percent: f64 = input
        .trim()
        .parse()
        .expect("Please enter a valid percentage.");

    let num_iterations = 10000;

    let (average, threshold) = monte_carlo_simulation(
        num_attacks,
        min_to_hit,
        min_to_wound,
        min_to_save,
        threshold_percent,
        num_iterations,
    );

    println!("Average successful attacks: {:.2}", average);
    println!(
        "At least {} attacks are successful in {}% of the cases.",
        threshold, threshold_percent
    );
}
