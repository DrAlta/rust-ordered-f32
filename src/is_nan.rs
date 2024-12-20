use crate::OrderedF32;

impl OrderedF32{
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }
}