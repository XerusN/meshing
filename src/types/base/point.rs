use crate::types::base::vector::*;
use std::ops::{Add, Div, Mul, Sub};

/// Point (f64) with basic operations.
/// For now only implemented in 2D.
/// Cartesian direct coordinate system.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Performs a addition between coordinates points.
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(2.0, 2.0);
/// let b = build_point(4.0, 3.0);
///
/// assert_eq!(&a + &b, build_point(2.0 + 4.0, 2.0 + 3.0));
/// ```
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Performs a substraction between coordinates points.
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(2.0, 2.0);
/// let b = build_point(4.0, 3.0);
///
/// assert_eq!(&a - &b, build_point(2.0 - 4.0, 2.0 - 3.0));
/// ```
impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Performs a dot product between coordinates points (not mathematically accurate but useful in some computations).
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(2.0, 1.0);
/// let b = build_point(4.0, 3.0);
///
/// assert_eq!(&a * &b, 2.0*4.0 + 1.0*3.0);
/// ```
impl Mul for &Point {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Performs a division between coordinates and a float.
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(1.0, 3.0);
///
/// assert_eq!(&a * 2.0, build_point(2.0 * 1.0, 2.0 * 3.0));
/// ```
impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Performs a division between coordinates and a float.
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(4.0, 2.0);
///
/// assert_eq!(&a / 2.0, build_point(2.0, 1.0));
/// ```
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
    /// Creates a vector between two points (similar to a substraction but returns a Vector).
    ///
    /// # Example
    ///
    /// ```rust
    /// use meshing::*;
    ///
    /// let a = build_point(2.0, 2.0);
    /// let b = build_point(4.0, 3.0);
    ///
    /// assert_eq!(b.segment_to(&a), build_vector(2.0 - 4.0, 2.0 - 3.0));
    /// ```
    pub fn segment_to(&self, other: &Self) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

/// Creates a new point
///
/// # Example
///
/// ```rust
/// use meshing::*;
///
/// let a = build_point(4.0, 2.0);
///
/// assert_eq!(a.x, 4.0 as f64);
/// assert_eq!(a.y, 2.0 as f64);
/// ```
pub fn build_point(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
}
