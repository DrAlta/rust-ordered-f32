use crate::OrderedF32;

pub trait PowPow<T> {
    fn powpow(&self, rhs: T) -> Self;
}

impl PowPow<OrderedF32> for OrderedF32 {
    fn powpow(&self, rhs: OrderedF32) -> Self {
        OrderedF32(self.0.powf(rhs.0))
    }
}

impl PowPow<&OrderedF32> for OrderedF32 {
    fn powpow(&self, rhs: &OrderedF32) -> Self {
        OrderedF32(self.0.powf(rhs.0))
    }
}

impl PowPow<f32> for OrderedF32 {
    fn powpow(&self, rhs: f32) -> Self {
        OrderedF32(self.0.powf(rhs))
    }
}

impl OrderedF32 {
    pub fn pow<T>(self, rhs: T) -> Self
    where
        OrderedF32: PowPow<T>,
    {
        self.powpow(rhs)
    }
}
