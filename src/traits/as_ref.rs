use crate::OrderedF32;

impl AsRef<f32> for OrderedF32 {
    #[inline]
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}
