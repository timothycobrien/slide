#![allow(clippy::suspicious_arithmetic_impl)]
use crate::bignum::utils::abs;
use crate::bignum::utils::normalize_vecs;
use crate::bignum::utils::recast_vec;
use crate::bignum::utils::truncate_zeros;
use crate::bignum::Bignum;
use std::ops;

impl ops::Sub for Bignum {
    type Output = Bignum;

    fn sub(self, other: Bignum) -> Bignum {
        let mut is_neg: bool = false;
        let lhs: Bignum;
        let rhs: Bignum;
        if !self.is_neg && other.is_neg {
            return self + abs(&other);
        } else if self.is_neg && !other.is_neg {
            return self + (-other);
        }
        // make rhs the bigger number
        if abs(&self) > abs(&other) {
            if self.is_neg {
                is_neg = true;
            }
            lhs = other;
            rhs = self;
        } else {
            if !(self.is_neg || other.is_neg) {
                is_neg = true;
            }
            lhs = self;
            rhs = other;
        }

        // make the decimal vectors the same size
        // fill lhs and rhs vectors as well
        let (res_dec_equal, lhs_dec_equal) = normalize_vecs(rhs.dec, lhs.dec);
        let (mut res_int, lhs_int, mut res_dec, lhs_dec): (Vec<i8>, Vec<i8>, Vec<i8>, Vec<i8>) = (
            recast_vec(rhs.int),
            recast_vec(lhs.int),
            recast_vec(res_dec_equal),
            recast_vec(lhs_dec_equal),
        );
        // 1. Decimal

        let mut res: i8;
        let mut carry: i8 = 0;
        for i in (0..res_dec.len()).rev() {
            res = res_dec[i] - lhs_dec[i] - carry;
            carry = (res < 0) as i8;
            res += carry * 10;
            res_dec[i] = res;
        }

        // 2. Integer
        for i in 0..lhs_int.len() {
            res = res_int[i] - lhs_int[i] - carry;
            carry = (res < 0) as i8;
            res += carry * 10;
            res_int[i] = res;
        }

        // Propogate the rest of the carry
        let mut i = lhs_int.len();
        while i < res_int.len() {
            res = res_int[i] as i8 - carry;
            carry = (res < 0) as i8;
            res += carry * 10;
            res_int[i] = res;
            i += 1;
        }

        // remove preceeding zeros
        let int: Vec<u8> = truncate_zeros(recast_vec(res_int));
        let dec: Vec<u8> = truncate_zeros(recast_vec(res_dec));

        // quick optimization to remove -0 from possible results.
        if is_neg && int.is_empty() && dec.is_empty() {
            is_neg = false;
        }

        Bignum { is_neg, dec, int }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! bignum_test_sub {
        ($($name: ident: $lhs:expr, $rhs:expr, $program:expr)*) => {
        $(
            #[test]
            fn $name() {
                use crate::bignum::Bignum;
                let lhs = Bignum::new($lhs.to_string()).unwrap();
                let rhs = Bignum::new($rhs.to_string()).unwrap();
                let result = $program.to_string();
                assert_eq!((lhs-rhs).to_string(), result);
            }
        )*
        }
    }

    mod sub {
        bignum_test_sub! {
            int1: "1", "2", "-1"
            int2: "2", "1", "1"
            int3: "10", "1", "9"
            int4: "10000", "9999", "1"
            int5: "9000000000", "101000000000000000000", "-100999999991000000000"
            int6: "5", "10", "-5"
            float1: "0.1", "0.1", "0"
            float2: "0.2", "0.1", "0.1"
            float3: "0.1", "0.2", "-0.1"
            float4: "0.10", "0.09", "0.01"
            float5: "0.10099", "0.10099", "0"
            float6: "0.1", "0.0000099", "0.0999901"
            float7: "0.999999999999999999999900000000000000", "0.55555539999999999999", "0.4444446000000000000099"
            mixed1: "1.1", "1.1", "0"
            mixed2: "1.999999", "9999.999", "-9997.999001"
            mixed3: "99999999999999.999999999999", "100000000000000000000000000.", "-99999999999900000000000000.000000000001"
            negative_int1: "-5", "5", "-10"
            negative_int2: "-10", "5", "-15"
            negative_int3: "5", "-10", "15"
            negative_int4: "-555555", "999999", "-1555554"
            negative_int5: "-10", "-10", "0"
            negative_float1: "-0.1", "0.1", "-0.2"
            negative_float2: "-0.2", "0.1", "-0.3"
            negative_float3: "-0.1", "0.2", "-0.3"
            negative_float4: "-0.1", "-0.1", "0"
            negative_mixed1: "-12332.55", "1.0", "-12333.55"
            negative_mixed2: "-12332.55", "-1.0", "-12331.55"
            negative_mixed3: "1.0", "-12332.55", "12333.55"
            negative_mixed4: "-1.0", "-12332.55", "12331.55"
            negative_mixed5: "1.0", "-12332.55", "12333.55"
            zero1: "0000.00", "0", "0"
            zero2: "0.0000", "0", "0"
            zero3: "0", "0.000", "0"
            zero4: "0.0", "00.00", "0"
            zero5: "-0", "00.00", "0"
            zero6: "0.0000", "-00000.000", "0"
            zero7: "-0.00", "-000.0000", "0"
            trailing_zero1: "5.55", "5.55", "0"
        }
    }
}
