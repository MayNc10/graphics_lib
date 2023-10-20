#[must_use]
#[repr(transparent)]
pub struct MustUse<T>(T);

impl<T> From<T> for MustUse<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> MustUse<T> {
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}