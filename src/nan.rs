use core::f32;

use crate::OrderedF32;

impl OrderedF32 {
    pub fn nan() -> Self {
        OrderedF32(f32::NAN)
    }
}
