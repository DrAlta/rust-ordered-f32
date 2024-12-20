use core::cmp::Ordering;

use crate::OrderedF32;

impl Ord for OrderedF32 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::comparison_chain)]
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
