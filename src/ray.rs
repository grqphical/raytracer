use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    /// Creates an empty Ray
    pub fn new() -> Self {
        Self {
            origin: Vector3::new(),
            direction: Vector3::new(),
        }
    }
    
    /// Creates a ray from an origin and direction
    pub fn from(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction
        }
    }
    /// Gets a point on the ray based on a given T value. Uses the formula *P(t) = A + tB*
    /// 
    /// ## Arguments
    /// - `t` Represents how far along the ray you wish to travel
    pub fn at(&self, t: f64) -> Vector3 {
        let result = self.origin + t * self.direction;
        return result;
    }
}
