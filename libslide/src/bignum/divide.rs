#![allow(clippy::suspicious_arithmetic_impl)]
use crate::bignum::Bignum;
use std::ops;

impl ops::Div for Bignum {
    type Output = Bignum;

    fn div(self, rhs: Bignum) -> Bignum {
        // trying to bound denominator within [0.5, 1.0]
        const LOWER: Bignum = Bignum::new("0.5".to_owned())?;
        const UPPER: Bignum = Bignum::new("1.0".to_owned())?;

        let (mut num, mut den) = (self, rhs);

        // divide num and den by two
        while den > UPPER {
            num >>= 1;
            den >>= 1;
        }
        // multiply num and den by two
        while den < LOWER {
            num <<= 1;
            den <<= 1;
        }

        const A: Bignum = Bignum::new((140/33).to_string())?;
        const B: Bignum = Bignum::new((-64/11).to_string())?;
        const C: Bignum = Bignum::new((256/99).to_string())?;
        const ZERO: Bignum = Bignum::new(0.to_string())?;
        const ONE: Bignum = Bignum::new(1.to_string())?;

        let mut x: Bignum = A + (&den * (B + (&den * C)));
        let mut y: Bignumm;
        let mut err: Bignum = ONE - (&den * x);
        while err != ZERO {
            y = &x * &err;
            x = &x + y + (&y * &err);
            err = ONE - (&den * x);
        }

        return &x * num;
    }
}