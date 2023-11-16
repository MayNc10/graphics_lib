use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

static EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
static UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };

impl Interval {
    pub fn empty() -> Interval { EMPTY }
    pub fn new(min: f32, max: f32) -> Interval { Interval { min, max } }
    pub fn new_from_intervals(a: Interval, b: Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    pub fn contains(&self, x: f32) -> bool { self.min <= x && x <= self.max }
    pub fn surrounds(&self, x: f32) -> bool { self.min < x && x < self.max }
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { self.min } else if x > self.max { self.max } else { x }
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }

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

