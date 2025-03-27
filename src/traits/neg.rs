use core::ops::Neg;

use crate::OrderedF32;
impl<'a> Neg for &'a OrderedF32
{
    type Output = OrderedF32;

    #[inline]
    fn neg(self) -> Self::Output {
        OrderedF32(-(&self.0))
    }
}
impl Neg for OrderedF32
{
    type Output = OrderedF32;

    #[inline]
    fn neg(self) -> Self::Output {
        OrderedF32(-(&self.0))
    }
}
