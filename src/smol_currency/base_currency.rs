use std::ops::Neg;

use creusot_contracts::{logic, prelude::ensures, open};
use const_decimal::{Decimal, ParseDecimalError};
use num_traits::{Num, One, Signed, Zero};

use super::{Currency, MarginCurrency, Mon, QuoteCurrency};

/// Representation of a Base currency,
/// e.g in the symbol BTCUSD, the prefix BTC is the `BaseCurrency` and the postfix `USD` is the `QuoteCurrency`.
///
/// # Generics:
/// - `I`: The numeric data type of `Decimal`.
/// - `D`: The constant decimal precision
#[derive(
    Debug,
    Clone,
    Default,
    Copy,
    creusot_contracts::prelude::PartialEq,
    creusot_contracts::model::DeepModel,
    Eq,
    PartialOrd,
    Ord,
    std::hash::Hash,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Mul,
    derive_more::Div,
    derive_more::Neg,
    derive_more::From,
    derive_more::AsRef,
)]
#[mul(forward)]
#[div(forward)]
#[repr(transparent)]
pub struct BaseCurrency<I, const D: u8>(Decimal<I, D>)
where
    I: Mon<D> + Sized;

impl<I, const D: u8> BaseCurrency<I, D>
where
    I: Mon<D>,
{
    /// Create a new instance from an `integer` and a `scale`.
    pub fn new(integer: I, scale: u8) -> Self {
        Self(
            Decimal::try_from_scaled(integer, scale)
                .expect("Can construct `Decimal` from `integer` and `scale`"),
        )
    }

    /// Round a number to a multiple of a given `quantum` toward zero.
    /// general ref: <https://en.wikipedia.org/wiki/Quantization_(signal_processing)>
    ///
    /// By default, rust is rounding towards zero and so does this method.
    ///
    #[inline]
    #[must_use]
    pub fn quantize_round_to_zero(&self, quantum: Self) -> Self {
        Self(self.0.quantize_round_to_zero(*quantum.as_ref()))
    }
}

/// # Generics:
/// - `I`: The numeric data type of `Decimal`.
/// - `D`: The constant decimal precision of the `BaseCurrency`.
impl<I, const D: u8> Currency<I, D> for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    // Generic over the decimal points in paired currency.
    type PairedCurrency = QuoteCurrency<I, D>;

    fn convert_from(units: Self::PairedCurrency, price_per_unit: QuoteCurrency<I, D>) -> Self {
        debug_assert!(units >= Zero::zero());
        debug_assert!(price_per_unit > Zero::zero());
        BaseCurrency(*units.as_ref() / *price_per_unit.as_ref())
    }
}

/// Inverse futures where the `Base` currency is used as margin currency.
///
/// # Generics:
/// - `I`: The numeric data type of `Decimal`.
/// - `D`: The constant decimal precision of the `BaseCurrency`.
impl<I, const D: u8> MarginCurrency<I, D> for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    #[inline]
    fn pnl(
        entry_price: QuoteCurrency<I, D>,
        exit_price: QuoteCurrency<I, D>,
        quantity: QuoteCurrency<I, D>,
    ) -> BaseCurrency<I, D> {
        debug_assert!(entry_price > Zero::zero());
        debug_assert!(exit_price > Zero::zero());
        BaseCurrency::convert_from(quantity, entry_price)
            - BaseCurrency::convert_from(quantity, exit_price)
    }
}

impl<I, const D: u8> Zero for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    #[inline]
    fn zero() -> Self {
        Self(Decimal::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<I, const D: u8> One for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    #[inline]
    fn one() -> Self {
        Self(Decimal::one())
    }

    #[inline]
    fn set_one(&mut self) {
        *self = One::one();
    }

    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

impl<I, const D: u8> Num for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    type FromStrRadixErr = ParseDecimalError<I>;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(Decimal::from_str_radix(str, radix)?))
    }
}

impl<I, const D: u8> Signed for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    #[inline]
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        Self(self.0.abs_sub(&other.0))
    }

    #[inline]
    fn signum(&self) -> Self {
        use std::cmp::Ordering::*;
        match self.0.cmp(&Decimal::zero()) {
            Less => Self(Decimal::one().neg()),
            Equal => Self(Decimal::zero()),
            Greater => Self(Decimal::one()),
        }
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.0 > Decimal::zero()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.0 < Decimal::zero()
    }
}

impl<I, const D: u8> std::ops::Mul<Decimal<I, D>> for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Decimal<I, D>) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<I, const D: u8> std::ops::Div<Decimal<I, D>> for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Decimal<I, D>) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<I, const D: u8> std::ops::Rem for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<I, const D: u8> std::fmt::Display for BaseCurrency<I, D>
where
    I: Mon<D>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Base", self.0)
    }
}

impl<I, const D: u8> From<BaseCurrency<I, D>> for f64
where
    I: Mon<D>,
{
    #[inline]
    fn from(val: BaseCurrency<I, D>) -> Self {
        val.0.to_f64()
    }
}
