use core::fmt;

use crate::OrderedF32;

impl fmt::LowerExp for OrderedF32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
