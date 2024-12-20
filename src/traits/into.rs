use crate::OrderedF32;

impl Into<f32> for OrderedF32 {
    fn into(self) -> f32 {
        self.0
    }
}
impl Into<f32> for &OrderedF32 {
    fn into(self) -> f32 {
        self.0
    }
}