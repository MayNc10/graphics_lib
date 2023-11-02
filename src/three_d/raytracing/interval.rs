#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

static EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
static UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };

impl Interval {
    pub fn empty() -> Interval { EMPTY }
    pub fn new(min: f32, max: f32) -> Interval { Interval { min, max } }
    pub fn contains(&self, x: f32) -> bool { self.min <= x && x <= self.max }
    pub fn surrounds(&self, x: f32) -> bool { self.min < x && x < self.max }
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { self.min } else if x > self.max { self.max } else { x }
    }

    pub fn replace_max(&mut self, max: f32) -> Interval {
        self.max = max;
        *self
    }
}

