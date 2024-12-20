use crate::OrderedF32;

impl OrderedF32 {
    #[inline]
    pub fn abs(&self) -> Self {
        OrderedF32(self.0.abs())
    }

    pub fn abs_sub(&self, other: &Self) -> Self {
        OrderedF32(&self.0 - &other.0)
    }

    #[inline]
    pub fn signum(&self) -> Self {
        OrderedF32(self.0.signum())
    }
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0 > 0.0
    }
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 < 0.0
    }
}
