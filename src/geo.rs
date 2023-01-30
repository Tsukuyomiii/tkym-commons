//! Contains utilites for working with geometric concepts.
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: u32,
    pub y: u32,
}

impl Vector2 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

impl AddAssign<Self> for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        };
    }
}

impl Sub<Self> for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y)
        }
    }
}

impl SubAssign<Self> for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y)
        };
    }
}

impl From<(u32, u32)> for Vector2 {
    fn from(value: (u32, u32)) -> Self {
        Vector2::new(value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect2 {
    pub width:  u32,
    pub height: u32,
}

impl Rect2 {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn area(&self) -> u32 {
        self.height * self.width
    }
}

impl From<(u32, u32)> for Rect2 {
    fn from(value: (u32, u32)) -> Self {
        Rect2::new(value.0, value.0)
    }
}

