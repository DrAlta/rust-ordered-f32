use crate::OrderedF32;

impl OrderedF32 {
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }
}
