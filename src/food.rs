#[derive(Debug)]
pub struct Food {
    pub x: u32,
    pub y: u32
}

impl Food {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }
}