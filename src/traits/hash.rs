use core::hash::Hasher;
use core::hash::Hash;

use crate::canonicalize_signed_zero;
use crate::raw_double_bits;
use crate::OrderedF32;
use crate::CANONICAL_NAN_BITS;

impl Hash for OrderedF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = if self.is_nan() {
            CANONICAL_NAN_BITS
        } else {
            raw_double_bits(&canonicalize_signed_zero(self.0))
        };

        bits.hash(state)
    }
}
