use crate::OrderedF32;

impl PartialEq for OrderedF32 {
    #[inline]
    fn eq(&self, other: &OrderedF32) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}

impl PartialEq<f32> for OrderedF32 {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}
