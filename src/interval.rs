pub struct Interval {
    pub min : f64,
    pub max : f64,
}

impl Interval {
    pub const fn new(min : f64, max : f64) -> Interval {
        return Interval { min, max };
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x : f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn clamp(&self, x : f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }
}
