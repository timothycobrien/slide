#![allow(clippy::suspicious_arithmetic_impl)]
use crate::bignum::Bignum;
use std::ops;
use std::ops::Neg;

impl ops::Div for Bignum {
    type Output = Bignum;

    fn div(self, rhs: Bignum) -> Bignum {
        // trying to bound denominator within [0.5, 1.0]
        let one: Bignum = Bignum::new(String::from("1.0")).unwrap();
        let two: Bignum = Bignum::new(String::from("2.0")).unwrap();
        let one_half: Bignum = Bignum::new(String::from("0.5")).unwrap();

        let (lower, upper) = (one_half.clone(), one.clone());
        let neg_result: bool = rhs.is_neg;
        let mut num = self.clone();
        let mut den = if neg_result {rhs.clone().neg()} else {rhs.clone()};

        // divide num and den by two
        while den > upper {
            num  = num.clone() * one_half.clone();
            den = den.clone() * one_half.clone();
            println!("{}", &num);
            println!("{}", &den);
            println!("{}", &upper);
            println!("{}", den > upper);
        }
        // multiply num and den by two
        while den < lower {
            num = num.clone() * two.clone();
            den = den.clone() * two.clone();
        }

        println!("{}", &num);
        println!("{}", &den);

        const A1: f64 = 140.0/33.0; // @todo denom should be 33.0
        const B1: f64 = -64.0/11.0; // @todo denom should be 11.0
        const C1: f64 = 256.0/99.0; // @todo denom should be 99.0

        let a: Bignum = Bignum::new(A1.to_string()).unwrap();
        let b: Bignum = Bignum::new(B1.to_string()).unwrap();
        let c: Bignum = Bignum::new(C1.to_string()).unwrap();
        let zero: Bignum = Bignum::new(String::from("0")).unwrap();
        let one: Bignum = Bignum::new(String::from("1")).unwrap();

        let mut x: Bignum = a.clone() + (den.clone() * (b.clone() + (den.clone() * c.clone())));
        let mut y: Bignum;
        let mut err: Bignum = one.clone() - (den.clone() * x.clone());
        while (err.clone() - zero.clone()) < Bignum::new(String::from(".000001")).unwrap() {
            y = x.clone() * err.clone();
            x = x.clone() + y.clone() + (y.clone() * err);
            err = one.clone() - (den.clone() * x.clone());
        }

        return if neg_result {(x * num).neg()} else {x * num};
    }
}

#[cfg(test)]
mod tests {
    macro_rules! bignum_test_divide {
        ($($name: ident: $lhs:expr, $rhs:expr, $program:expr)*) => {
        $(
            #[test]
            fn $name() {
                use crate::bignum::Bignum;
                let lhs = Bignum::new($lhs.to_string()).unwrap();
                let rhs = Bignum::new($rhs.to_string()).unwrap();
                let result = $program.to_string();
                assert_eq!((lhs/rhs).to_string(), result);
            }
        )*
        }
    }


    mod divide {
        bignum_test_divide! {
            int1: "5", "5", "1"
            int2: "5", "1", "5"
            int3: "100", "20", "5"
            int4: "0", "1", "0"
            int5: "0", "-1", "0"
            int6: "-5", "5", "-1"
            int7: "-9", "-3", "3"
            float1: ".1", ".1", "1"
            float2: "1", "5", ".2"
            float3: "-1", "10", "-.1"
            mixed1: "87", "5", "17.4"
            mixed2: "-10", "-2.5", "4"
            mixed3: "762.9145996941", "8.31333333", "91.77"
        }
    }
}