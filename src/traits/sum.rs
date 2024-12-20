use core::iter::Sum;

use crate::OrderedF32;

impl Sum for OrderedF32 {
    fn sum<I: Iterator<Item = OrderedF32>>(iter: I) -> Self {
        OrderedF32(iter.map(|v| v.0).sum())
    }
}

impl<'a> Sum<&'a OrderedF32> for OrderedF32 {
    #[inline]
    fn sum<I: Iterator<Item = &'a OrderedF32>>(iter: I) -> Self {
        iter.cloned().sum()
    }
}
