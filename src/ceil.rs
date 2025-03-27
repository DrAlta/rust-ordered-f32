use crate::OrderedF32;

impl OrderedF32 {
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
}
