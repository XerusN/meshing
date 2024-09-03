use std::ops::{Add, Sub, Mul, Div};

/// Vector (f64) with basic operations.
/// For now only implemented in 2D.
/// Cartesian direct coordinate system.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x : f64,
    pub y : f64,
}

/// Performs a addition between vectors.
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(2.0, 2.0);
/// let b = build_vector(4.0, 3.0);
/// 
/// assert_eq!(&a + &b, build_vector(2.0 + 4.0, 2.0 + 3.0));
/// ```
impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Performs a substraction between vectors.
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(2.0, 2.0);
/// let b = build_vector(4.0, 3.0);
/// 
/// assert_eq!(&a - &b, build_vector(2.0 - 4.0, 2.0 - 3.0));
/// ```
impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Performs a dot product between vectors.
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(2.0, 1.0);
/// let b = build_vector(4.0, 3.0);
/// 
/// assert_eq!(&a * &b, 2.0*4.0 + 1.0*3.0);
/// ```
impl Mul for &Vector {
    type Output = f64;
    
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Performs a division between a vector and a float.
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(1.0, 3.0);
/// 
/// assert_eq!(&a * 2.0, build_vector(2.0 * 1.0, 2.0 * 3.0));
/// ```
impl Mul<f64> for &Vector {
    type Output = Vector;
    
    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Performs a division between a vector and a float.
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(4.0, 2.0);
/// 
/// assert_eq!(&a / 2.0, build_vector(2.0, 1.0));
/// ```
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
    
    /// Computes the Euclidean norm of the vector
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use meshing::*;
    /// 
    /// let a = build_vector(4.0, 2.0);
    /// 
    /// assert_eq!(a.norm(), (a.x * a.x + a.y * a.y).sqrt());
    /// ```
    pub fn norm(&self) -> f64 {
        (self * self).sqrt()
    }
    
    /// Normalizes the vector. The norm will equal to 1.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use meshing::*;
    /// 
    /// let a = build_vector(4.0, 2.0);
    /// let b = a.normalize();
    /// 
    /// assert_eq!(b, &a / a.norm());
    /// ```
    pub fn normalize(&self) -> Vector {
        let norm = self.norm();
        self / norm
    }
    
    /// Returns an orthogonal vector counter-clockwise
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use meshing::*;
    /// 
    /// let a = build_vector(4.0, 2.0);
    /// let b = build_vector(2.0, -4.0);
    /// 
    /// assert_eq!(b, a.orthogonal_vector());
    /// ```
    // With z pointing away from the drawing
    pub fn orthogonal_vector(&self) -> Vector {
        Vector {
            x: self.y,
            y: - self.x,
        }
    }
    
}

/// Creates a new vector
/// 
/// # Example
/// 
/// ```rust
/// use meshing::*;
/// 
/// let a = build_vector(4.0, 2.0);
/// 
/// assert_eq!(a.x, 4.0 as f64);
/// assert_eq!(a.y, 2.0 as f64);
/// ```
pub fn build_vector(x: f64, y: f64) -> Vector {
    Vector {
        x: x,
        y: y,
    }
}