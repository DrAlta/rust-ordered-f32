use core::cmp::Ordering;

use crate::OrderedF32;

impl PartialOrd for OrderedF32 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        !other.ge(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        // We consider all NaNs equal, and NaN is the largest possible
        // value. Thus if self is NaN we always return true. Otherwise
        // self >= other is correct. If other is also not NaN it is trivially
        // correct, and if it is we note that nothing can be greater or
        // equal to NaN except NaN itself, which we already handled earlier.
        self.0.is_nan() | (self.0 >= other.0)
    }
}
