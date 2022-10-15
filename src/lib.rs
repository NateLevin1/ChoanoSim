mod cell;
mod food;
mod genes;
mod renderer;
mod simulator;

use once_cell::sync::Lazy;
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
    Lazy::new(|| Mutex::new(simulator::Simulator::new()));
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

    avg_size /= cells.len() as f64;
    avg_flagellum_size /= cells.len() as f64;
    avg_stomach_size /= cells.len() as f64;
    avg_steps_until_child_born /= cells.len() as f64;

    result = format!(
        "{}\nAVERAGE,,,{},{},{},{}",
        result, avg_size, avg_flagellum_size, avg_stomach_size, avg_steps_until_child_born
    );

    return result;
}

pub fn random(max: u32) -> u32 {
    (js_sys::Math::random() * max as f64) as u32
}
