use crate::logic::simulation::monte_carlo_simulation;
use crate::models::input::SimulationInput;
use crate::models::result::SimulationResult;
use actix_web::{web, HttpResponse, Responder};

pub async fn simulate(input: web::Json<SimulationInput>) -> impl Responder {
    if !(2..=6).contains(&input.min_to_hit) {
        return HttpResponse::BadRequest().body("min_to_hit must be between 2 and 6.");
    }
    if !(2..=6).contains(&input.min_to_wound) {
        return HttpResponse::BadRequest().body("min_to_wound must be between 2 and 6.");
    }
    if !(2..=7).contains(&input.min_to_save) {
        return HttpResponse::BadRequest().body("min_to_save must be between 2 and 7.");
    }
    if !(0.0..=100.0).contains(&input.threshold_percent) {
        return HttpResponse::BadRequest().body("threshold_percent must be between 0 and 100.");
    }

    let num_iterations = 10_000;

    let (average, threshold) = monte_carlo_simulation(
        input.num_attacks,
        input.min_to_hit,
        input.min_to_wound,
        input.min_to_save,
        input.threshold_percent,
        num_iterations,
    );

    HttpResponse::Ok().json(SimulationResult {
        average_successful_attacks: average,
        threshold_successful_attacks: threshold,
    })
}
