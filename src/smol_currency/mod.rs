mod base_currency;
mod margin_currency_trait;
mod quote_currency;

pub use base_currency::BaseCurrency;
use const_decimal::{Decimal, ScaledInteger};
pub use margin_currency_trait::MarginCurrency;
pub use quote_currency::QuoteCurrency;

/// A trait for monetary values.
pub trait Mon<const D: u8>:
    ScaledInteger<D>
    + Default
    + std::ops::Rem
    + std::ops::Neg
    + num_traits::CheckedNeg
    + std::ops::SubAssign
    + std::hash::Hash
    + std::fmt::Debug
    + num_traits::Signed
{
}

impl<const D: u8> Mon<D> for i32 {}
impl<const D: u8> Mon<D> for i64 {}
impl<const D: u8> Mon<D> for i128 {}

/// A currency must be marked as it can be either a `Base` or `Quote` currency.
///
/// # Generics:
/// - `I` is the numeric type
/// - `D` is the decimal precision.
pub trait Currency<I, const D: u8>:
    Clone
    + Copy
    + Default
    + std::fmt::Debug
    + std::fmt::Display
    + std::cmp::PartialOrd
    + Eq
    + Ord
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::Mul<Decimal<I, D>, Output = Self>
    + std::hash::Hash
    + num_traits::Zero
    + num_traits::One
    + num_traits::Signed
    + Into<f64>
    + From<Decimal<I, D>>
    + AsRef<Decimal<I, D>>
where
    I: Mon<D>,
{
    /// The paired currency in the `Symbol` with generic decimal precision `DP`.
    type PairedCurrency: Currency<I, D, PairedCurrency = Self>;

    /// Convert from one currency to another at a given price per unit.
    fn convert_from(units: Self::PairedCurrency, price_per_unit: QuoteCurrency<I, D>) -> Self;
}
