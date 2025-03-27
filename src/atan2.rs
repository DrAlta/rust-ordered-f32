use crate::OrderedF32;

pub trait TanTan {
    fn tantan(lhs: &OrderedF32, rhs: Self) -> OrderedF32;
}

impl TanTan for &OrderedF32 {
    fn tantan(lhs: &OrderedF32, rhs: Self) -> OrderedF32 {
        OrderedF32(lhs.0.atan2(rhs.0))
    }
}

impl TanTan for OrderedF32 {
    fn tantan(lhs: &OrderedF32, rhs: Self) -> OrderedF32 {
        OrderedF32(lhs.0.atan2(rhs.0))
    }
}

impl TanTan for f32 {
    fn tantan(lhs: &OrderedF32, rhs: Self) -> OrderedF32 {
        OrderedF32(lhs.0.atan2(rhs))
    }
}

impl OrderedF32 {
    pub fn atan2<T: TanTan>(&self, other: T) -> Self {
        TanTan::tantan(self, other)
    }
}
