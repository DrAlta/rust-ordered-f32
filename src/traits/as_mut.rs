use crate::OrderedF32;

impl AsMut<f32> for OrderedF32 {
    #[inline]
    fn as_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}
