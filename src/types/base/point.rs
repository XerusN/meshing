use std::ops::{Add, Sub, Mul, Div};
use crate::types::base::vector::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    // Cartesian direct coordinate system
    pub x : f64,
    pub y : f64,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for &Point {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Point {
    
    pub fn segment_to(&self, other : &Self) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
    
}

pub fn build_point(x: f64, y: f64) -> Point {
    Point {
        x: x,
        y: y,
    }
}