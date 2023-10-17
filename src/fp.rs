use fixnum::typenum::{Unsigned, U24};

type Precision = U18;

pub type Fp = fixnum::FixedPoint<i128, Precision>;

macro_rules! fp {
    ($value:literal) => {
        ::fixnum::fixnum_const!($value, 18)
    };
}

pub use fp;
