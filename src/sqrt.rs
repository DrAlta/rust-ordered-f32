use crate::OrderedF32;

impl OrderedF32{
    pub fn sqrt(&self) -> OrderedF32{
        OrderedF32(self.0.sqrt())
    }
}