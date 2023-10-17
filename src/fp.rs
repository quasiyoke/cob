use fixnum::{typenum::U18, FixedPoint};

type Precision = U18;

pub type Fp = FixedPoint<i128, Precision>;

macro_rules! fp {
    ($value:literal) => {
        ::fixnum::fixnum_const!($value, 18)
    };
}

pub(crate) use fp;
