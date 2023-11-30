use std::ops::Add;

/// A struct representing an interval between two numbers
#[derive(Copy, Clone, Debug)]
pub struct Interval {
    /// The minimum value of the interval
    pub min: f32,
    /// The maximum value of the interval
    pub max: f32,
}

static EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
static UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };

impl Interval {
    /// Create an empty interval, which contains no numbers
    pub fn empty() -> Interval { EMPTY }
    /// Create an interval, given two endpoints
    pub fn new(min: f32, max: f32) -> Interval { Interval { min, max } }
    /// Create a super-interval that encapsulates two intervals
    pub fn new_from_intervals(a: Interval, b: Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    /// Check if the interval contains a number
    pub fn contains(&self, x: f32) -> bool { self.min <= x && x <= self.max }
    /// Check if the interval surrounds the number
    /// This method rejects values that are on the endpoints
    pub fn surrounds(&self, x: f32) -> bool { self.min < x && x < self.max }
    /// Clamp a value to be within the array
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { self.min } else if x > self.max { self.max } else { x }
    }
    /// Compute the size of the interval
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    /// Expand the interval, adding half of delta on each side
    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }
    /// Replace the maximum value with a new value, and then return the interval

    pub fn replace_max(&mut self, max: f32) -> Interval {
        self.max = max;
        *self
    }
}
impl Add<f32> for Interval {
    type Output = Interval;
    fn add(self, rhs: f32) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f32 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        Interval::new(self + rhs.min, self + rhs.max)
    }
}

