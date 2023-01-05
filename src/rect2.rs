#[derive(Debug, Clone, Copy)]
pub struct Rect2 {
    pub width: u32,
    pub height: u32,
}

impl Rect2 {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

