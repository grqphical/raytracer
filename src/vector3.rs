use std::{ops, fmt::Display};

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        return (1.0 / rhs) * self
    }
}

impl Vector3 {
    /// Creates an empty Vector
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    /// Creates a Vector from a set of floats
    /// 
    /// ## Arguments
    /// - `x` X Coordinate of the Vector
    /// - `y` Y Coordinate of the Vector
    /// - `z` Z Coordinate of the Vector
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    /// Returns the length of the Vector
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt()
    }
    
    /// Returns the length of the vector squared
    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.x * self.z;
    }
}

/// Finds the dot product of two Vectors
/// 
/// ## Arguments
/// - `a` The First Vector of the product
/// - `b` The Second Vector of the product
pub fn dot_product(a: Vector3, b: Vector3) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z 
}

/// Finds the cross product of two Vectors
/// 
/// ## Arguments
/// - `a` The First Vector of the product
/// - `b` The Second Vector of the product
pub fn cross_product(a: Vector3, b: Vector3) -> Vector3 {
    return Vector3 { x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z , z: a.x * b.y - a.y * b.x }
}