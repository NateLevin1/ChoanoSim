use std::collections::HashMap;

use crate::cell;
use crate::food;
use crate::genes::Genes;
use js_sys::Math::random;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulator {
    config: SimulatorConfig,
    steps: u32,
    cells: Vec<cell::Cell>,
    food: Vec<Vec<Option<food::Food>>>,
}

impl Simulator {
    pub fn new() -> Self {
        let config = SimulatorConfig::new();

        let cells = (0..config.cell_number)
            .map(|_| {
                cell::Cell::new(
                    Genes::default(),
                    cell::Cell::get_random_pos_in_bounds(&config),
                    cell::Cell::get_random_pos_in_bounds(&config),
                    5.0,
                    &config,
                )
            })
            .collect();

        let food = (0..config.width / config.food_spacing)
            .map(|_| {
                (0..config.height / config.food_spacing)
                    .map(|_| None)
                    .collect()
            })
            .collect();

        let mut new_self = Self {
            cells,
            food,
            config,
            steps: 0,
        };
        new_self.fill_food();

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

        self.add_food();

        let food_spacing = self.config.food_spacing as f64;

        // cell death works by storing all the dead cells after looping,
        // then removing them from the list at the end
        let mut i = 0;
        let mut indexes_to_remove: Vec<usize> = Vec::new();

        let mut reproduction_locations: HashMap<(u32, u32), usize> = HashMap::new();

        for cell_index in 0..self.cells.len() {
            i += 1;

            // to access the current cell, use `&mut self.cells[cell_index]`
            // this is necessary to avoid multiple mutable borrows
            // see https://www.reddit.com/r/rust/comments/y1qefw/question_avoiding_cannot_borrow_as_mutable_more/
            // perf impact should be very low because it is just a vec lookup

            // if not alive, skip simulating
            if !(&self.cells[cell_index]).alive {
                indexes_to_remove.push(i - 1);
                continue;
            }

            (&mut self.cells[cell_index]).find_food_and_eat(&mut self.food, food_spacing);

            // reproduction
            match self.config.reproduction {
                Reproduction::Asexual => {
                    // if asexual then reproduce if possible
                    let cell = &mut self.cells[cell_index];
                    if cell.reproduction_cooldown == 0 {
                        cell.start_reproduction(&cell.genes.clone(), &self.config);
                    }
                }
                Reproduction::Sexual => {
                    let genes = self.cells[cell_index].genes;
                    if let Some(index_to_impregnate) = (&mut self.cells[cell_index])
                        .find_mate_and_reproduce(
                            cell_index,
                            &mut reproduction_locations,
                            &self.config,
                        )
                    {
                        let cell_to_impregnate = &mut self.cells[index_to_impregnate];
                        cell_to_impregnate.start_reproduction(&genes, &self.config);
                    }
                }
            }

            let possibly_new_cell = (&mut self.cells[cell_index]).simulate_step(&self.config);
            if let Some(new_cell) = possibly_new_cell {
                self.cells.push(new_cell);
            }
        }

        // after removing food we need to adjust all the indexes after -- so we do this hack
        i = 0;
        for index in indexes_to_remove {
            self.cells.remove(index - i);
            i += 1;
        }
    }

    fn fill_food(&mut self) {
        let width = self.config.width;
        let height = self.config.height;
        let food_spacing = self.config.food_spacing;
        let food_offset = food_spacing;

        for row in 0..width / food_spacing {
            let food_row = self
                .food
                .get_mut(row as usize)
                .expect("food row did not exist");
            for col in 0..height / food_spacing {
                food_row[col as usize] = Some(food::Food::new(
                    row * food_spacing + food_offset,
                    col * food_spacing + food_offset,
                ));
            }
        }
    }

    fn add_food(&mut self) {
        let width = self.config.width;
        let height = self.config.height;
        let food_spacing = self.config.food_spacing;
        let food_offset = food_spacing;

        let spawn_chance = 1.0 / (self.config.food_density as f64);

        for row in 0..width / food_spacing {
            let food_row = self
                .food
                .get_mut(row as usize)
                .expect("food row did not exist");
            for col in 0..height / food_spacing {
                if food_row[col as usize].is_none() && random() < spawn_chance {
                    food_row[col as usize] = Some(food::Food::new(
                        row * food_spacing + food_offset,
                        col * food_spacing + food_offset,
                    ));
                }
            }
        }
    }

    pub fn add_cell(&mut self, new_cell: cell::Cell) {
        self.cells.push(new_cell);
    }
}

#[derive(Copy, Clone)]
pub struct SimulatorConfig {
    pub reproduction: Reproduction,
    pub food_density: u32,
    pub width: u32,
    pub height: u32,
    pub cell_number: u32,
    pub food_spacing: u32,
    pub reproduction_cooldown: u32,
    pub mutation_chance: f64,
    pub mutation_percent_change: f64,
}

impl SimulatorConfig {
    pub fn new() -> Self {
        Self {
            reproduction: Reproduction::default(),
            food_density: 240,
            width: 1_600,
            height: 1_600,
            cell_number: 12,
            food_spacing: 40,
            reproduction_cooldown: 200,
            mutation_chance: 0.01,
            mutation_percent_change: 0.1,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum Reproduction {
    #[default]
    Asexual,
    Sexual,
}
