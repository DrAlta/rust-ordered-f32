use crate::OrderedF32;

impl OrderedF32 {
    pub fn atan(self) -> Self {
        OrderedF32(self.0.atan())
    }

}