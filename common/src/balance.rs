use crate::Amount;
use crate::Fixed;
use codec::{Compact, CompactAs, Decode, Encode};
use num_traits::{CheckedNeg, Num};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedShl, CheckedShr, CheckedSub,
    IntegerSquareRoot, One, Saturating, Unsigned, Zero,
};
use sp_arithmetic::FixedPointNumber;

use sp_core::U256;
use sp_runtime::FixedPointOperand;
use sp_std::convert::TryFrom;
use sp_std::fmt::Display;
use sp_std::ops::*;
use sp_std::str::FromStr;

use static_assertions::_core::fmt::Formatter;
use static_assertions::{assert_eq_align, assert_eq_size};

///Fixed point balance
#[derive(Debug, Clone, Copy, Encode, Decode, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Balance(pub Fixed);

impl From<Fixed> for Balance {
    fn from(_: Fixed) -> Self {
        Self(Fixed)
    }
}

///Error type for conversion
#[derive(Debug, Eq, PartialEq)]
pub enum ConvertError {
    ///Overflow encountered
    Overflow,
    ///Input is not acceptable for conversion
    Invalid,
}

impl TryFrom<U256> for Balance {
    type Error = ConvertError;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value > U256::from(Fixed::max_value().into_inner()) {
            Err(ConvertError)
        } else {
            Ok(Balance(Fixed::from_inner(value.low_u128())))
        }
    }
}

#[cfg(feature = "std")]
impl FromStr for Balance {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Balance(Fixed::from_str(s)?))
    }
}

#[cfg(feature = "std")]
impl Display for Balance {
    fn fmt(&self, f: &mut Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Rem for Balance {
    type Output = Balance;

    ///Division always occurs without reminder
    fn rem(self, _: Self) -> Self::Output {
        Balance::zero()
    }
}

impl Add for Balance {
    type Output = Balance;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul for Balance {
    type Output = Balance;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Balance {
    type Output = Balance;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Sub for Balance {
    type Output = Balance;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Shl<u32> for Balance {
    type Output = Balance;

    fn shl(self, _: u32) -> Self::Output {
        //todo not implemented
        self
    }
}

impl Shr<u32> for Balance {
    type Output = Balance;

    fn shr(self, _: u32) -> Self::Output {
        //todo not implemented
        self
    }
}

impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0)
    }
}

impl DivAssign for Balance {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0);
    }
}

impl RemAssign for Balance {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl Bounded for Balance {
    fn min_value() -> Self {
        Self(Fixed::min_value())
    }

    fn max_value() -> Self {
        Self(Fixed::max_value())
    }
}

impl Zero for Balance {
    fn zero() -> Self {
        Self(Fixed::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Balance {
    fn one() -> Self {
        Self(Fixed::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl IntegerSquareRoot for Balance {
    fn integer_sqrt_checked(&self) -> Option<Self> where
        Self: Sized {
        //todo not implemented
        None
    }
}

impl CheckedAdd for Balance {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(&v.0).map(Self)
    }
}

impl CheckedSub for Balance {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(&v.0).map(Self)
    }
}

impl CheckedMul for Balance {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.0.checked_mul(&v.0).map(Self)
    }
}

impl CheckedDiv for Balance {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(&v.0).map(Self)
    }
}

impl CheckedShl for Balance {
    fn checked_shl(&self, _: u32) -> Option<Self> {
        //todo not implemented
        None
    }
}

impl CheckedShr for Balance {
    fn checked_shr(&self, _: u32) -> Option<Self> {
        //todo not implemented
        None
    }
}

impl Saturating for Balance {
    fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    fn saturating_mul(self, rhs: Self) -> Self {
        Self(self.0.saturating_mul(rhs.0))
    }

    fn saturating_pow(self, exp: usize) -> Self {
        Self(self.0.saturating_pow(exp))
    }
}

impl Num for Balance {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        //todo not implemented
        Err(())
    }
}

impl Unsigned for Balance {}

impl From<usize> for Balance {
    fn from(v: usize) -> Self {
        Self(Fixed::from(v as <Fixed as FixedPointNumber>::Inner))
    }
}

impl Into<usize> for Balance {
    fn into(self) -> usize {
        self.0.saturating_mul_int(1u128) as usize
    }
}


impl Into<Amount> for Balance {
    fn into(self) -> Amount {
        <Self as Into<u64>>::into(self) as i128
    }
}

impl TryFrom<Amount> for Balance {
    type Error = ();

    fn try_from(value: Amount) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(())
        } else {
            Ok(Self::from(value as u128))
        }
    }
}

impl CheckedNeg for Balance {
    fn checked_neg(&self) -> Option<Self> {
        None
    }
}

impl FixedPointOperand for Balance {}

impl From<Compact<Balance>> for Balance {
    fn from(c: Compact<Balance>) -> Self {
        c.0
    }
}

impl CompactAs for Balance {
    type As = <Fixed as FixedPointNumber>::Inner;

    fn encode_as(&self) -> &Self::As {
        assert_eq_size!(Fixed, <Fixed as FixedPointNumber>::Inner);
        assert_eq_align!(Fixed, <Fixed as FixedPointNumber>::Inner);
        unsafe {
            sp_std::mem::transmute::<&Fixed, &<Fixed as FixedPointNumber>::Inner>(&self.0)
        }
    }

    fn decode_from(v: Self::As) -> Self {
        Balance(Fixed::from_inner(v))
    }
}

#[cfg(test)]
mod tests {
    use super::{Balance, CompactAs, FixedPointNumber, One};

    #[test]
    fn balance_encode_as_should_equal_fixed_inner() {
        let balance = Balance::one();
        assert_eq!(balance.0.into_inner(), *balance.encode_as())
    }
}

macro_rules! impl_primitive_conversion {
    ($t:ty) => {
        impl From<$t> for Balance {
            fn from(v : $t) -> Balance {
                Balance(Fixed::from(v as <Fixed as FixedPointNumnber>::Inner))
            }
        }

        impl Into<$t> for Balance {
            fn into(self) -> $t {
                self.0.saturating_mul_int(1 as $t)
            }
        }
    };
}


macro_rules! impl_primitive_conversion_any {
    ($($t:ty)+) => ($(
        impl_primitive_conversion!($t);
    )+)
}

impl_primitive_conversion_any!(u8 u16 u32 u64 u128);