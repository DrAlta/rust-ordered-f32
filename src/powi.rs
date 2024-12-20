use crate::OrderedF32;

impl OrderedF32 {
    pub fn powi(self, n: i32) -> Self {
        OrderedF32(self.0.powi(n))
    }
}