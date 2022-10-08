use js_sys::Math;

use crate::{simulator::{SimulatorConfig, self}, random};

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub color: String,
    pub seed: f64,
    pub flagellum_size: f64,
    pub radians: f64,
    pub speed: f64,
    rotation_chance: f64
}

impl Cell {
    pub fn new(config: &SimulatorConfig) -> Self {
        Self {
            x: random(config.width),
            y: random(config.height),
            rotation_chance: 0.0,
            
            size: 30,
            flagellum_size: 3.0,
            speed: 10.0,
            radians: (random(360) as f64).to_radians(),

            seed: Math::random(),
            color: format!("rgb({}, {}, {})", 60+random(50), 150+random(50), 150+random(70))
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
            self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
        }
        
        let new_y = self.y as i32 + dy as i32;
        if new_y >= 0 && new_y < config.height as i32 {
            self.y = new_y as u32;
        } else {
            self.rotation_chance += ROTATION_CHANCE_CHANGE_ON_WALL_HIT;
        }

        if js_sys::Math::random() < self.rotation_chance {
            self.radians = (random(360) as f64).to_radians();
            self.rotation_chance = 0.0;
        }
    }
}

const DEFAULT_ROTATION_CHANCE_CHANGE: f64 = 0.0006;
const ROTATION_CHANCE_CHANGE_ON_WALL_HIT: f64 = 0.2;
