use wasm_bindgen::prelude::*;
use crate::cell;
use crate::food;

#[wasm_bindgen]
pub struct Simulator {
    config: SimulatorConfig,
    steps: u32,
    cells: Vec<cell::Cell>,
    food: Vec<Vec<Option<food::Food>>>
}

impl Simulator {
    pub fn new() -> Self {
        let config = SimulatorConfig::new();
        let mut new_self = Self {
            cells: (0..config.cell_number).map(|_| cell::Cell::new(&config)).collect(),
            food: (0..config.width/config.food_spacing).map(|_| (0..config.height/config.food_spacing).map(|_| None).collect()).collect(),
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
    pub fn get_food(&self) -> &Vec<Vec<Option<food::Food>>> {
        &self.food
    }
    
    pub fn simulate_step(&mut self) {
        self.steps += 1;

        if self.steps % self.config.food_density == 0 {
            self.add_food();
        }

        let food_spacing = self.config.food_spacing as f64;

        // cell death works by storing all the dead cells after looping,
        // then removing them from the list at the end
        let mut i = 0;
        let mut indexes_to_remove: Vec<usize> = Vec::new();

        for cell in &mut self.cells {
            i += 1;
            if !cell.alive {
                indexes_to_remove.push(i-1);
                continue;
            }
            let x = cell.x as f64;
            let y = cell.y as f64;
            let nearest_food_x = ((x - food_spacing) / food_spacing).round() as usize;
            let nearest_food_y = ((y - food_spacing) / food_spacing).round() as usize;

            let nearest_food = self.food.get(nearest_food_x)
                .and_then(|row|row.get(nearest_food_y));

            if let Some(food) = nearest_food {
                if let Some(food) = food {
                    let dist = ((cell.x.abs_diff(food.x) + cell.y.abs_diff(food.y)) as f64).sqrt();
                    if dist < cell.flagellum_size {
                        // cell eats the food
                        cell.eat_food();
                        // remove food
                        self.food.get_mut(nearest_food_x).unwrap()[nearest_food_y] = None;
                    }
                }
            }

            cell.simulate_step(&self.config);
        }

        // after removing we need to adjust all the indexes after -- so we do this hack
        i = 0;
        for index in indexes_to_remove {
            self.cells.remove(index - i);
            i += 1;
        }
    }
    fn add_food(&mut self) {
        let width = self.config.width;
        let height = self.config.height;
        let food_spacing = self.config.food_spacing;
        let food_offset = food_spacing;
        
        for row in 0..width/food_spacing {
            let food_row = self.food.get_mut(row as usize).expect("food row did not exist");
            for col in 0..height/food_spacing {
               food_row[col as usize] = Some(food::Food::new(row * food_spacing + food_offset, col * food_spacing + food_offset));
            }
        }
    }
}

pub struct SimulatorConfig {
    pub reproduction: Reproduction,
    pub food_density: u32,
    pub width: u32,
    pub height: u32,
    pub cell_number: u32,
    pub food_spacing: u32
}


impl SimulatorConfig {
    pub fn new() -> Self {
        Self {
            reproduction: Reproduction::default(),
            food_density: 50,
            width: 800,
            height: 800,
            cell_number: 10,
            food_spacing: 40
        }
    }
}

#[derive(Default)]
pub enum Reproduction {
    #[default] Asexual,
    Sexual
}
