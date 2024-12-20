use core::iter::Product;

use crate::OrderedF32;

impl Product for OrderedF32 {
    fn product<I: Iterator<Item = OrderedF32>>(iter: I) -> Self {
        OrderedF32(iter.map(|v| v.0).product())
    }
}

impl<'a> Product<&'a OrderedF32> for OrderedF32 {
    #[inline]
    fn product<I: Iterator<Item = &'a OrderedF32>>(iter: I) -> Self {
        iter.cloned().product()
    }
}
