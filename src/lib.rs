//! Wrappers for total order on Floats.  See the [`OrderedF32`] and [`NotNan`] docs for details.

mod ordered_f32;
use num_traits::Float;
pub use ordered_f32::OrderedF32;
mod traits;

mod atan;
mod atan2;
mod ceil;
pub mod consts;
mod floor;
mod into_inner;
mod is_nan;
mod nan;
mod pow;
mod powi;
mod signed;
mod sqrt;
mod to_f32;

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

// masks for the parts of the IEEE 754 float
const SIGN_MASK: u64 = 0x8000000000000000u64;
const EXP_MASK: u64 = 0x7ff0000000000000u64;
const MAN_MASK: u64 = 0x000fffffffffffffu64;

// canonical raw bit patterns (for hashing)
const CANONICAL_NAN_BITS: u64 = 0x7ff8000000000000u64;

#[inline(always)]
fn canonicalize_signed_zero(x: f32) -> f32 {
    // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
    // which Rust guarantees. Thus by adding a positive zero we
    // canonicalize signed zero without any branches in one instruction.
    x + 0.0
}

#[inline]
/// Used for hashing. Input must not be zero or NaN.
fn raw_double_bits(f: &f32) -> u64 {
    let (man, exp, sign) = f.integer_decode();
    let exp_u64 = exp as u16 as u64;
    let sign_u64 = (sign > 0) as u64;
    (man & MAN_MASK) | ((exp_u64 << 52) & EXP_MASK) | ((sign_u64 << 63) & SIGN_MASK)
}
/// A wrapper around floats providing implementations of `Eq`, `Ord`, and `Hash`.
///
/// NaN is sorted as *greater* than all other values and *equal*
/// to itself, in contradiction with the IEEE standard.
///
/// ```
/// use ordered_f32::OrderedF32;
/// use std::f32::NAN;
///
/// let mut v = [OrderedF32(NAN), OrderedF32(2.0), OrderedF32(1.0)];
/// v.sort();
/// assert_eq!(v, [OrderedF32(1.0), OrderedF32(2.0), OrderedF32(NAN)]);
/// ```
///
/// Because `OrderedFloat` implements `Ord` and `Eq`, it can be used as a key in a `HashSet`,
/// `HashMap`, `BTreeMap`, or `BTreeSet` (unlike the primitive `f32` or `f64` types):
///
/// ```
/// # use ordered_f32::OrderedF32;
/// # use std::collections::HashSet;
/// # use std::f32::NAN;
/// let mut s: HashSet<OrderedF32> = HashSet::new();
/// s.insert(OrderedF32(NAN));
/// assert!(s.contains(&OrderedF32(NAN)));
/// ```
///
/// Some non-identical values are still considered equal by the [`PartialEq`] implementation,
/// and will therefore also be considered equal by maps, sets, and the `==` operator:
///
/// * `-0.0` and `+0.0` are considered equal.
///   This different sign may show up in printing, or when dividing by zero (the sign of the zero
///   becomes the sign of the resulting infinity).
/// * All NaN values are considered equal, even though they may have different
///   [bits](https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits), and therefore
///   different [sign](https://doc.rust-lang.org/std/primitive.f64.html#method.is_sign_positive),
///   signaling/quiet status, and NaN payload bits.
///   
/// Therefore, `OrderedFloat` may be unsuitable for use as a key in interning and memoization
/// applications which require equal results from equal inputs, unless these cases make no
/// difference or are canonicalized before insertion.
///
/// # Representation
///
/// `OrderedFloat` has `#[repr(transparent)]` and permits any value, so it is sound to use
/// [transmute](core::mem::transmute) or pointer casts to convert between any type `T` and
/// `OrderedFloat<T>`.
/// However, consider using [`bytemuck`] as a safe alternative if possible.
///

macro_rules! impl_ordered_float_from {
    ($dst:ty, $src:ty) => {
        impl From<$src> for OrderedF32 {
            fn from(val: $src) -> Self {
                OrderedF32(val.into())
            }
        }
    };
}
impl_ordered_float_from! {f32, i8}
impl_ordered_float_from! {f32, i16}
impl_ordered_float_from! {f32, u8}
impl_ordered_float_from! {f32, u16}

macro_rules! impl_ordered_float_binop {
    ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
        impl $imp for OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                OrderedF32((self.0).$method(other.0))
            }
        }

        // Work around for: https://github.com/reem/rust-ordered-float/issues/91
        impl<'a> $imp<Self> for &'a OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                OrderedF32((self.0).$method(other.0))
            }
        }

        impl $imp<f32> for OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: f32) -> Self::Output {
                OrderedF32((self.0).$method(other))
            }
        }

        impl<'a> $imp<&'a f32> for OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: &'a f32) -> Self::Output {
                OrderedF32((self.0).$method(other))
            }
        }

        impl<'a> $imp<&'a Self> for OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: &'a Self) -> Self::Output {
                OrderedF32((self.0).$method(&other.0))
            }
        }

        impl<'a> $imp<OrderedF32> for &'a OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: OrderedF32) -> Self::Output {
                OrderedF32((self.0).$method(other.0))
            }
        }

        impl<'a> $imp<f32> for &'a OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: f32) -> Self::Output {
                OrderedF32((self.0).$method(other))
            }
        }

        impl<'a> $imp<&'a f32> for &'a OrderedF32 {
            type Output = OrderedF32;

            #[inline]
            fn $method(self, other: &'a f32) -> Self::Output {
                OrderedF32((self.0).$method(other))
            }
        }

        impl $assign_imp<f32> for OrderedF32 {
            #[inline]
            fn $assign_method(&mut self, other: f32) {
                (self.0).$assign_method(other);
            }
        }

        impl<'a> $assign_imp<&'a f32> for OrderedF32 {
            #[inline]
            fn $assign_method(&mut self, other: &'a f32) {
                (self.0).$assign_method(other);
            }
        }

        impl $assign_imp for OrderedF32 {
            #[inline]
            fn $assign_method(&mut self, other: Self) {
                (self.0).$assign_method(other.0);
            }
        }

        impl<'a> $assign_imp<&'a Self> for OrderedF32 {
            #[inline]
            fn $assign_method(&mut self, other: &'a Self) {
                (self.0).$assign_method(&other.0);
            }
        }
    };
}

impl_ordered_float_binop! {Add, add, AddAssign, add_assign}
impl_ordered_float_binop! {Sub, sub, SubAssign, sub_assign}
impl_ordered_float_binop! {Mul, mul, MulAssign, mul_assign}
impl_ordered_float_binop! {Div, div, DivAssign, div_assign}
impl_ordered_float_binop! {Rem, rem, RemAssign, rem_assign}
