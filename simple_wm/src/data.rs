use std::ops::Sub;

pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Sub for Position {
    type Output = Vector2D;
    fn sub(self, other: Position) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    pub fn add_vec(&self, vec: &Vector2D) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl Size {
    pub fn add_vec(&self, vec: &Vector2D) -> Self {
        Self {
            width: self.width + vec.x as u32,
            height: self.width + vec.y as u32,
        }
    }
}
