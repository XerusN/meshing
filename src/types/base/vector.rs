use std::ops::{Add, Sub, Mul, Div};

pub struct Vector {
    // Cartesian direct coordinate system
    pub x : f64,
    pub y : f64,
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for &Vector {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Vector {
    
    pub fn norm(&self) -> f64 {
        (self * self).sqrt()
    }
    
    pub fn normalize(&self) -> Vector {
        let norm = self.norm();
        self / norm
    }
    
    // With z pointing away from the drawing
    pub fn orthognal_segment(&self) -> Vector {
        Vector {
            x: self.y,
            y: - self.x,
        }
    }
    
    pub fn angle_with(&self, other: &Self) -> f64 {
        (self * other / (self.norm() * other.norm())).acos()
    }
    
}

pub fn build_vector(x: f64, y: f64) -> Vector {
    Vector {
        x: x,
        y: y,
    }
}