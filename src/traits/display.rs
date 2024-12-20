use core::fmt;

use crate::OrderedF32;

impl fmt::Display for OrderedF32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
