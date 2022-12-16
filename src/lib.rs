mod cell;
mod food;
mod genes;
mod randoms;
mod renderer;
mod simulator;

use once_cell::sync::Lazy;
use simulator::Reproduction;
use std::sync::{Mutex, MutexGuard};
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Rust module loaded"));

    Ok(())
}

static SIMULATOR: Lazy<Mutex<simulator::Simulator>> =
    Lazy::new(|| Mutex::new(simulator::Simulator::new(Reproduction::Asexual)));
fn get_simulator() -> MutexGuard<'static, simulator::Simulator> {
    SIMULATOR.lock().unwrap()
}

#[wasm_bindgen]
pub fn simulate_step() {
    let mut simulator = get_simulator();
    simulator.simulate_step();
    // console::log_2(&JsValue::from_str("Simulated step %d"), &simulator.get_steps().into());
}

#[wasm_bindgen]
pub fn set_food_density(density: u32) {
    get_simulator().get_config_mut().food_density = density;

    console::log_2(
        &JsValue::from_str("Set food density to %d"),
        &density.into(),
    );
}

#[wasm_bindgen]
pub fn set_reproductive_method(repro_method: &str) {
    get_simulator().get_config_mut().reproduction = if repro_method == "asexual" {
        simulator::Reproduction::Asexual
    } else {
        simulator::Reproduction::Sexual
    };

    console::log_2(
        &JsValue::from_str("Set food density to %s"),
        &repro_method.into(),
    );
}

#[wasm_bindgen]
pub fn get_cells_data_csv() -> String {
    let simulator = get_simulator();
    let mut result = format!(
        "Step #{}\nCell #,x,y,size,flagellum size,stomach size,steps until child born",
        simulator.get_steps()
    )
    .to_string();

    let cells = simulator.get_cells();
    let mut avg_size = 0.0;
    let mut avg_flagellum_size = 0.0;
    let mut avg_stomach_size = 0.0;
    let mut avg_steps_until_child_born = 0.0;
    for i in 0..cells.len() {
        let cell = &cells[i];
        let genes = cell.genes;

        avg_size += genes.size;
        avg_flagellum_size += genes.flagellum_size;
        avg_stomach_size += genes.stomach_size;
        avg_steps_until_child_born += genes.steps_until_child_born;

        result = format!(
            "{}\n{},{},{},{},{},{},{}",
            result,
            i,
            cell.x,
            cell.y,
            genes.size,
            genes.flagellum_size,
            genes.stomach_size,
            genes.steps_until_child_born
        );
    }

    let cells_len = cells.len() as f64;
    avg_size /= cells_len;
    avg_flagellum_size /= cells_len;
    avg_stomach_size /= cells_len;
    avg_steps_until_child_born /= cells_len;

    result = format!(
        "{}\nAVERAGE,,,{},{},{},{}",
        result, avg_size, avg_flagellum_size, avg_stomach_size, avg_steps_until_child_born
    );

    return result;
}

const NUM_SIMULATIONS: usize = 10;
#[wasm_bindgen]
pub fn get_results_csv(
    repro_method: &str,
    beginning_food_density: u32,
    switched_food_density: u32,
) -> String {
    // setup simulator with provided config
    let repro_method = if repro_method == "asexual" {
        simulator::Reproduction::Asexual
    } else {
        simulator::Reproduction::Sexual
    };
    let mut simulators: Vec<simulator::Simulator> = vec![(); NUM_SIMULATIONS]
        .iter()
        .map(|_| {
            let mut simulator = simulator::Simulator::new(repro_method);
            simulator.get_config_mut().food_density = beginning_food_density;
            simulator
        })
        .collect();

    let mut result = format!(
        "Step #,Population Size,% Food Available,Avg. Size,Avg. Flagellum Size,Avg. Stomach Size,Avg. Gestation Steps"
    );

    // loop 1 mil times, separated so we don't save every step just every 1k
    for i in 0..1_000 {
        let mut all_sims_avg_population_size = 0.0;
        let mut all_sims_avg_per_food_avail = 0.0;
        let mut all_sims_avg_size = 0.0;
        let mut all_sims_avg_fla_size = 0.0;
        let mut all_sims_avg_sto_size = 0.0;
        let mut all_sims_avg_gest_steps = 0.0;

        for simulator in simulators.iter_mut() {
            if i == 500 {
                simulator.get_config_mut().food_density = switched_food_density;
            }
            for _ in 0..1_000 {
                simulator.simulate_step();
            }

            let population_size: usize = simulator.get_cells().len();
            let total_food_avail: usize = simulator
                .get_food()
                .iter()
                .map(|food_row| food_row.iter().filter(|food| food.is_some()).count())
                .sum();
            let total_food: usize = simulator
                .get_food()
                .get(0)
                .expect("could not get food row 0")
                .len()
                * simulator.get_food().len();

            let per_food_avail = (total_food_avail as f64 / total_food as f64) * 100.0;

            // calculate averages for this simulation
            let mut avg_size = 0.0;
            let mut avg_fla_size = 0.0;
            let mut avg_sto_size = 0.0;
            let mut avg_gest_steps = 0.0;
            for cell in simulator.get_cells() {
                avg_size += cell.genes.size;
                avg_fla_size += cell.genes.flagellum_size;
                avg_sto_size += cell.genes.stomach_size;
                avg_gest_steps += cell.genes.steps_until_child_born;
            }

            let cells_len = simulator.get_cells().len() as f64;
            avg_size /= cells_len;
            avg_fla_size /= cells_len;
            avg_sto_size /= cells_len;
            avg_gest_steps /= cells_len;

            // add this simulation's averages to the total averages
            all_sims_avg_population_size += population_size as f64;
            all_sims_avg_per_food_avail += per_food_avail;
            all_sims_avg_size += avg_size;
            all_sims_avg_fla_size += avg_fla_size;
            all_sims_avg_sto_size += avg_sto_size;
            all_sims_avg_gest_steps += avg_gest_steps;
        }

        all_sims_avg_population_size /= NUM_SIMULATIONS as f64;
        all_sims_avg_per_food_avail /= NUM_SIMULATIONS as f64;
        all_sims_avg_size /= NUM_SIMULATIONS as f64;
        all_sims_avg_fla_size /= NUM_SIMULATIONS as f64;
        all_sims_avg_sto_size /= NUM_SIMULATIONS as f64;
        all_sims_avg_gest_steps /= NUM_SIMULATIONS as f64;

        // record data
        let step = (i + 1) * 1_000;
        result = format!(
            "{}\n{},{},{},{},{},{},{}",
            result,
            step,
            all_sims_avg_population_size,
            all_sims_avg_per_food_avail,
            all_sims_avg_size,
            all_sims_avg_fla_size,
            all_sims_avg_sto_size,
            all_sims_avg_gest_steps
        );

        // this is an extreme hack but it works
        let completion_percentage = i as f64 / 10.0;
        if completion_percentage.fract() == 0.0 {
            // only update on exact percentages
            let evaled = js_sys::eval(&String::from(format!(
                "self.onCompletionPercentChange({});",
                completion_percentage
            )));
            if let Err(err) = evaled {
                console::error_1(&err);
            }
        }
    }

    return result;
}
