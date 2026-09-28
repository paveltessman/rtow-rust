pub struct Interval {
    pub min : f64,
    pub max : f64,
}

impl Interval {
    pub fn new(min : f64, max : f64) -> Interval {
        return Interval { min, max };
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x : f64) -> bool {
        return self.min <= x && x <= self.max;
    }
}
