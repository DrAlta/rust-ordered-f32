use core::str::FromStr;

use crate::OrderedF32;

impl FromStr for OrderedF32 {
    type Err = <f32 as FromStr>::Err;

    /// Convert a &str to `OrderedFloat`. Returns an error if the string fails to parse.
    ///
    /// ```
    /// use ordered_f32::OrderedF32;
    ///
    /// assert!("-10".parse::<OrderedF32>().is_ok());
    /// assert!("abc".parse::<OrderedF32>().is_err());
    /// assert!("NaN".parse::<OrderedF32>().is_ok());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        f32::from_str(s).map(OrderedF32)
    }
}
