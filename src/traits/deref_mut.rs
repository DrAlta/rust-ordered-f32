use core::ops::DerefMut;

use crate::OrderedF32;

impl DerefMut for OrderedF32 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
