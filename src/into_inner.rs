use crate::OrderedF32;

impl OrderedF32 {
    /// Get the value out.
    #[inline]
    pub fn into_inner(self) -> f32 {
        self.0
    }
}
