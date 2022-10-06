use js_sys::Math;

use crate::{simulator::SimulatorConfig, random};

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub color: String,
    pub seed: f64,
    pub flagellum_size: f64,
    pub direction: f64
}

impl Cell {
    pub fn new(config: &SimulatorConfig) -> Self {
        Self {
            x: random(config.width),
            y: random(config.height),
            
            size: 30,
            flagellum_size: 3.0,
            direction: random(360) as f64,

            seed: Math::random(),
            color: format!("rgb({}, {}, {})", 60+random(50), 150+random(50), 150+random(70))
        }
    }
}