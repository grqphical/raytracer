pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn from(min: f64, max: f64) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max }
        return x;
    }
}
