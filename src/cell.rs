use js_sys::Math;

use crate::{simulator::{SimulatorConfig, self}, random};

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub radians: f64,
    pub size: u32,
    pub display_seed: f64,
    pub flagellum_size: f64,
    pub speed: f64,
    pub stomach_size: f64,
    pub alive: bool,

    stomach_amount: f64,
    rotation_chance: f64
}

impl Cell {
    pub fn new(config: &SimulatorConfig) -> Self {
        Self {
            x: random(config.width),
            y: random(config.height),
            radians: (random(360) as f64).to_radians(),
            stomach_amount: 5.0,
            rotation_chance: 0.0,
            alive: true,
            
            size: 30,
            flagellum_size: 6.0,
            speed: 10.0,
            stomach_size: 10.0,

            display_seed: Math::random(),
        }
    }

    pub fn simulate_step(&mut self, config: &simulator::SimulatorConfig) {
        // simulate motion

        // Explanation:
        // 1. calculate the dx and dy, move if won't go out of bounds
        // 2. slightly increase the chance it will turn
        // 3. if the move would have made the cell go OOB, greatly increase the chance it will turn
        // 4. if a random number is less than that value, rotate randomly

        let dx = self.radians.cos() * self.speed;
        let dy = self.radians.sin() * self.speed;

        self.rotation_chance += DEFAULT_ROTATION_CHANCE_CHANGE;

        let new_x = self.x as i32 + dx as i32;
        if new_x >= 0 && new_x < config.width as i32 {
            self.x = new_x as u32;
        } else {
            // self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
            self.radians += (60.0 + random(70) as f64) * 3.14 / 180.0;
        }
        
        let new_y = self.y as i32 + dy as i32;
        if new_y >= 0 && new_y < config.height as i32 {
            self.y = new_y as u32;
        } else {
            // self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
            self.radians += (60.0 + random(70) as f64) * 3.14 / 180.0;
        }

        if js_sys::Math::random() < self.rotation_chance {
            self.radians = (random(360) as f64).to_radians();
            self.rotation_chance = 0.0;
        }

        self.take_food(FOOD_STOMACH_DECREASE_AMOUNT * self.speed);
    }

    pub fn eat_food(&mut self) {
        self.stomach_amount += FOOD_STOMACH_INCREASE_AMOUNT;
        
        // if more than full, set to max
        if self.stomach_amount > self.stomach_size {
            self.stomach_amount = self.stomach_size;
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
        return self.stomach_amount / self.stomach_size;
    }

    pub fn die(&mut self) {
        self.alive = false;
    }
}

const DEFAULT_ROTATION_CHANCE_CHANGE: f64 = 0.0006;
// const ROTATION_CHANCE_CHANGE_ON_WALL_HIT: f64 = 0.3;
const FOOD_STOMACH_INCREASE_AMOUNT: f64 = 2.5;
const FOOD_STOMACH_DECREASE_AMOUNT: f64 = 0.007;
