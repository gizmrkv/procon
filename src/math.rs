pub mod gcd;

use std::ops::*;

pub use gcd::*;

/// Defines an additive identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a + 0 = a       ∀ a ∈ Self
/// 0 + a = a       ∀ a ∈ Self
/// ```
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`, `0`.
    /// This function should return the same result at all times.
    /// This cannot be an associated constant, because of bignums.
    fn zero() -> Self;

    /// Sets `self` to the additive identity element of `Self`, `0`.
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }

    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::zero()
    }
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> $t {
                $v
            }
        }
    };
}

impl_zero!(usize, 0);
impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);

impl_zero!(isize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

/// Defines a multiplicative identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a * 1 = a       ∀ a ∈ Self
/// 1 * a = a       ∀ a ∈ Self
/// ```
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    /// This function should return the same result at all times.
    /// This cannot be an associated constant, because of bignums.
    fn one() -> Self;

    /// Sets `self` to the multiplicative identity element of `Self`, `1`.
    fn set_one(&mut self) {
        *self = One::one();
    }

    /// Returns `true` if `self` is equal to the multiplicative identity.
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

macro_rules! impl_one {
    ($t:ty, $v:expr) => {
        impl One for $t {
            fn one() -> $t {
                $v
            }
        }
    };
}

impl_one!(usize, 1);
impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);

impl_one!(isize, 1);
impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);

impl_one!(f32, 1.0);
impl_one!(f64, 1.0);

/// Generic trait for types implementing basic numeric operations
///
/// This is automatically implemented for types which implement the operators.
pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

pub trait Num: Clone + Copy + PartialOrd + PartialEq + Eq + Zero + One + NumOps {}

impl<T> Num for T where T: Clone + Copy + PartialOrd + PartialEq + Eq + Zero + One + NumOps {}
