use std::collections::HashMap;

use js_sys::Math;

use crate::{
    food,
    genes::Genes,
    random,
    simulator::{self, SimulatorConfig},
};

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub radians: f64,
    pub color: String,

    pub genes: Genes,
    child_genes: Option<Genes>,

    pub alive: bool,
    pub display_seed: f64,
    pub reproduction_cooldown: u32,
    pub remaining_steps_until_child_born: u32,

    stomach_amount: f64,
    rotation_chance: f64,
}

impl Cell {
    pub fn new(genes: Genes, x: u32, y: u32, config: &SimulatorConfig) -> Self {
        Self {
            x,
            y,
            radians: (random(360) as f64).to_radians(),
            stomach_amount: 5.0,
            rotation_chance: 0.0,
            remaining_steps_until_child_born: 0,
            reproduction_cooldown: config.reproduction_cooldown,
            alive: true,

            genes,
            child_genes: None,

            display_seed: Math::random(),
            color: format!(
                "rgb({}, {}, {})",
                50 + random(100),
                random(100),
                100 + random(100)
            ),
        }
    }

    pub fn get_random_pos_in_bounds(config: &SimulatorConfig) -> u32 {
        let dist_from_wall = config.food_spacing;
        dist_from_wall / 2 + random(config.width - dist_from_wall)
    }

    pub fn simulate_step(&mut self, config: &SimulatorConfig) -> Option<Cell> {
        self.simulate_movement(config);

        let possibly_new_child = self.simulate_reproduction(config);

        self.take_food(self.get_energy_usage());

        return possibly_new_child;
    }

    pub fn simulate_movement(&mut self, config: &SimulatorConfig) {
        // Explanation:
        // 1. calculate the dx and dy, move if won't go out of bounds
        // 2. slightly increase the chance it will turn
        // 3. if the move would have made the cell go OOB, greatly increase the chance it will turn
        // 4. if a random number is less than that value, rotate randomly

        let speed = self.get_speed();
        let dx = self.radians.cos() * speed;
        let dy = self.radians.sin() * speed;
        let dist_from_wall = config.food_spacing as i32 / 2i32;

        self.rotation_chance += DEFAULT_ROTATION_CHANCE_CHANGE;

        let new_x = self.x as i32 + dx as i32;
        if new_x >= dist_from_wall && new_x < config.width as i32 - dist_from_wall {
            self.x = new_x as u32;
        } else {
            // self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
            self.radians += (60.0 + random(70) as f64) * 3.14 / 180.0;
        }

        let new_y = self.y as i32 + dy as i32;
        if new_y >= dist_from_wall && new_y < config.height as i32 - dist_from_wall {
            self.y = new_y as u32;
        } else {
            // self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
            self.radians += (60.0 + random(70) as f64) * 3.14 / 180.0;
        }

        if js_sys::Math::random() < self.rotation_chance {
            self.radians = (random(360) as f64).to_radians();
            self.rotation_chance = 0.0;
        }
    }

    pub fn simulate_reproduction(&mut self, config: &SimulatorConfig) -> Option<Cell> {
        if self.reproduction_cooldown > 0 {
            self.reproduction_cooldown -= 1;
        }

        if self.remaining_steps_until_child_born > 0 {
            self.remaining_steps_until_child_born -= 1;
            self.take_food(CHILD_DEVELOPMENT_FOOD_DECREASE);
            if self.remaining_steps_until_child_born == 0 {
                return self.reproduce(config);
            }
        }

        None
    }

    pub fn find_food_and_eat(
        &mut self,
        all_food: &mut Vec<Vec<Option<food::Food>>>,
        food_spacing: f64,
    ) {
        let x = self.x as f64;
        let y = self.y as f64;
        let nearest_food_x = ((x - food_spacing) / food_spacing).round() as usize;
        let nearest_food_y = ((y - food_spacing) / food_spacing).round() as usize;

        let nearest_food = all_food
            .get(nearest_food_x)
            .and_then(|row| row.get(nearest_food_y));

        if let Some(food) = nearest_food {
            if let Some(food) = food {
                let dist = ((self.x.abs_diff(food.x) + self.y.abs_diff(food.y)) as f64).sqrt();
                if dist < self.get_eating_distance() {
                    // cell eats the food
                    self.eat_food();
                    // remove food
                    all_food.get_mut(nearest_food_x).unwrap()[nearest_food_y] = None;
                }
            }
        }
    }

    pub fn find_mate_and_reproduce(
        &mut self,
        index_to_add: usize,
        reproduction_locations: &mut HashMap<(u32, u32), usize>,
        config: &simulator::SimulatorConfig,
    ) -> Option<usize> {
        // if recently reproduced, don't try to reproduce again.
        if self.reproduction_cooldown != 0 {
            return None;
        }

        // cell reproduction works by finding all cells within REPRODUCTION_DISTANCE in a hashmap
        // if a cell tries to insert at an already existing key, it will try to reproduce
        // with the cell there. After successfully reproducing it will remove the value at that
        // key so other cells can reproduce in the same area.
        let box_x = (self.x as i32 - REPRODUCTION_DISTANCE) / REPRODUCTION_DISTANCE;
        let box_y = (self.y as i32 - REPRODUCTION_DISTANCE) / REPRODUCTION_DISTANCE;
        let box_loc = (box_x as u32, box_y as u32);
        return if let Some(other_cell) = reproduction_locations.get(&box_loc) {
            self.reproduction_cooldown = config.reproduction_cooldown;
            Some(*other_cell)
        } else {
            reproduction_locations.insert(box_loc, index_to_add);
            None
        };
    }

    pub fn eat_food(&mut self) {
        self.stomach_amount += STOMACH_INCREASE_FROM_FOOD_AMOUNT;

        // if more than full, set to max
        if self.stomach_amount > self.genes.stomach_size {
            self.stomach_amount = self.genes.stomach_size;
        }
    }

    pub fn start_reproduction(&mut self, other: &Genes, config: &SimulatorConfig) {
        // this runs when `self` is impregnated

        self.child_genes = Some(self.genes.mix(other, config));
        self.remaining_steps_until_child_born = self.genes.steps_until_child_born as u32;
        self.reproduction_cooldown = config.reproduction_cooldown;
    }

    fn reproduce(&mut self, config: &SimulatorConfig) -> Option<Self> {
        self.take_food(CHILDBIRTH_FOOD_DECREASE);
        // prevent fast reproduction having no downside -- no birth if lower production
        if Math::random() < 0.18 * self.genes.steps_until_child_born.cbrt() {
            Some(Cell::new(self.child_genes.unwrap(), self.x, self.y, config))
        } else {
            None
        }
    }

    pub fn take_food(&mut self, amount: f64) {
        self.stomach_amount -= amount;

        // if less than min, then this cell has died :'(
        if self.stomach_amount < 0.0 {
            self.die();
        }
    }

    pub fn get_fullness(&self) -> f64 {
        return self.stomach_amount / self.genes.stomach_size;
    }

    pub fn get_speed(&self) -> f64 {
        self.genes.flagellum_size * FLAGELLUM_SPEED_MULTIPLIER
            + self.genes.stomach_size * STOMACH_SIZE_SPEED_MULTIPLIER
    }

    pub fn get_eating_distance(&self) -> f64 {
        self.genes.size * SIZE_EATING_DISTANCE_MULTIPLIER + self.genes.flagellum_size
    }

    pub fn get_energy_usage(&self) -> f64 {
        STOMACH_DECREASE_FROM_SPEED_AMOUNT * self.get_speed()
            + STOMACH_DECREASE_FROM_SIZE_AMOUNT * self.genes.size as f64
    }

    pub fn die(&mut self) {
        self.alive = false;
    }
}

const DEFAULT_ROTATION_CHANCE_CHANGE: f64 = 0.0006;
// const ROTATION_CHANCE_CHANGE_ON_WALL_HIT: f64 = 0.3;

const STOMACH_INCREASE_FROM_FOOD_AMOUNT: f64 = 0.5;
const STOMACH_DECREASE_FROM_SPEED_AMOUNT: f64 = 0.007;
const STOMACH_DECREASE_FROM_SIZE_AMOUNT: f64 = 0.0006;

const FLAGELLUM_SPEED_MULTIPLIER: f64 = 1.5;
const STOMACH_SIZE_SPEED_MULTIPLIER: f64 = 0.25;

const SIZE_EATING_DISTANCE_MULTIPLIER: f64 = 0.02;

const REPRODUCTION_DISTANCE: i32 = 50;

const CHILDBIRTH_FOOD_DECREASE: f64 = 1.0;
const CHILD_DEVELOPMENT_FOOD_DECREASE: f64 = 0.01;
