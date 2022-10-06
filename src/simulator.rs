use wasm_bindgen::prelude::*;
use crate::cell;

#[wasm_bindgen]
pub struct Simulator {
    config: SimulatorConfig,
    steps: u32,
    cells: Vec<cell::Cell>,
}

impl Simulator {
    pub fn new() -> Self {
        let config = SimulatorConfig::new();
        Self {
            cells: (0..config.cell_number).map(|_| cell::Cell::new(&config)).collect(),
            config,
            steps: 0,
        }
    }
    pub fn get_steps(&self) -> u32 {
        self.steps
    }
    pub fn simulate_step(&mut self) {
        self.steps += 1;
    }
    pub fn get_config(&self) -> &SimulatorConfig {
        &self.config
    }
    pub fn get_config_mut(&mut self) -> &mut SimulatorConfig {
        &mut self.config
    }
    pub fn get_cells(&self) -> &Vec<cell::Cell> {
        &self.cells
    }
}

pub struct SimulatorConfig {
    pub reproduction: Reproduction,
    pub food_density: FoodDensity,
    pub width: u32,
    pub height: u32,
    pub cell_number: u32,
}

pub enum FoodDensity {
    Value(u32),
    Infinite,
}

impl SimulatorConfig {
    pub fn new() -> Self {
        Self {
            reproduction: Reproduction::default(),
            food_density: FoodDensity::Value(5),
            width: 800,
            height: 800,
            cell_number: 10
        }
    }
}

#[derive(Default)]
pub enum Reproduction {
    #[default] Asexual,
    Sexual
}
