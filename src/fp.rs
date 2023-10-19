pub use fixnum::ops::Zero;
use fixnum::{typenum::U18, FixedPoint};

type Precision = U18;

pub type Fp = FixedPoint<i128, Precision>;

#[cfg(test)]
macro_rules! fp {
    ($value:literal) => {
        ::fixnum::fixnum!($value, 18)
    };
}

#[cfg(test)]
pub(crate) use fp;
