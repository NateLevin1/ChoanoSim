use wasm_bindgen::prelude::*;
use crate::cell;
use crate::food;

#[wasm_bindgen]
pub struct Simulator {
    config: SimulatorConfig,
    steps: u32,
    cells: Vec<cell::Cell>,
    food: Vec<Vec<food::Food>>
}

impl Simulator {
    pub fn new() -> Self {
        let config = SimulatorConfig::new();
        let mut new_self = Self {
            cells: (0..config.cell_number).map(|_| cell::Cell::new(&config)).collect(),
            food: (0..config.width/config.food_spacing).map(|_| Vec::new()).collect(),
            config,
            steps: 0,
        };
        new_self.add_food();

        new_self
    }
    pub fn get_steps(&self) -> u32 {
        self.steps
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
    pub fn get_food(&self) -> &Vec<Vec<food::Food>> {
        &self.food
    }
    
    pub fn simulate_step(&mut self) {
        self.steps += 1;

        if let FoodDensity::Value(n_steps) = self.config.food_density {
            if self.steps % n_steps == 0 {
                self.add_food();
            }
        } else {
            self.add_food();
        }

        for cell in &mut self.cells {
            cell.simulate_step(&self.config);
        }
    }
    fn add_food(&mut self) {
        let width = self.config.width;
        let height = self.config.height;
        let food_spacing = self.config.food_spacing;
        let food_offset = food_spacing;
        
        for row in 0..width/food_spacing {
            let food_row = self.food.get_mut(row as usize);
            if let Some(food_row) = food_row {
                for col in 0..height/food_spacing {
                    if food_row.get(col as usize).is_none() {
                        food_row.insert(col as usize, food::Food::new(row * food_spacing + food_offset, col * food_spacing + food_offset));
                    }
                }
            } else {
                self.food.insert(row as usize, Vec::new());
            }
        }
    }
}

pub struct SimulatorConfig {
    pub reproduction: Reproduction,
    pub food_density: FoodDensity,
    pub width: u32,
    pub height: u32,
    pub cell_number: u32,
    pub food_spacing: u32
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
            width: 1_000,
            height: 800,
            cell_number: 20,
            food_spacing: 100
        }
    }
}

#[derive(Default)]
pub enum Reproduction {
    #[default] Asexual,
    Sexual
}
