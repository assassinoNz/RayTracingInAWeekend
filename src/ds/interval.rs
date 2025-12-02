pub struct Interval {
    start: f64,
    end: f64,
}

impl Interval {
    pub fn new(start: f64, end: f64) -> Interval {
        Interval { start, end }
    }

    pub fn new_empty() -> Interval {
        Interval {
            start: f64::INFINITY,
            end: f64::NEG_INFINITY,
        }
    }

    pub fn new_universe() -> Interval {
        Interval {
            start: f64::NEG_INFINITY,
            end: f64::INFINITY,
        }
    }

    pub fn len(&self) -> f64 {
        self.end - self.start
    }

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.start {
            self.start
        } else if x > self.end {
            self.end
        } else {
            x
        }
    }
}
