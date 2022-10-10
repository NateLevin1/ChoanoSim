use js_sys::Math;

use crate::{simulator::{SimulatorConfig, self}, random};

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub radians: f64,
    pub size: u32,
    pub display_seed: f64,
    pub flagellum_size: f64,
    pub stomach_size: f64,
    pub alive: bool,

    stomach_amount: f64,
    rotation_chance: f64
}

impl Cell {
    pub fn new(config: &SimulatorConfig) -> Self {
        let dist_from_wall = config.food_spacing;
        Self {
            x: dist_from_wall / 2 + random(config.width - dist_from_wall),
            y: dist_from_wall / 2 + random(config.height - dist_from_wall),
            radians: (random(360) as f64).to_radians(),
            stomach_amount: 5.0,
            rotation_chance: 0.0,
            alive: true,
            
            size: 30,
            flagellum_size: 5.0,
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

        self.take_food(self.get_energy_usage());
    }

    pub fn eat_food(&mut self) {
        self.stomach_amount += STOMACH_INCREASE_FROM_FOOD_AMOUNT;
        
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

    pub fn get_speed(&self) -> f64 {
        self.flagellum_size * FLAGELLUM_SPEED_MULTIPLIER
        + self.stomach_size * STOMACH_SIZE_SPEED_MULTIPLIER
    }

    pub fn get_eating_distance(&self) -> f64 {
        self.size as f64 * SIZE_EATING_DISTANCE_MULTIPLIER
        + self.flagellum_size
    }

    pub fn get_energy_usage(&self) -> f64 {
        STOMACH_DECREASE_FROM_SPEED_AMOUNT * self.get_speed()
        + STOMACH_DECREASE_FROM_SIZE_AMOUNT * self.size as f64
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

const SIZE_EATING_DISTANCE_MULTIPLIER: f64 = 0.03;
