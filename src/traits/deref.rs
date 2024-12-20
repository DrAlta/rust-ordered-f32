use core::ops::Deref;

use crate::OrderedF32;

impl Deref for OrderedF32 {
    type Target = f32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
